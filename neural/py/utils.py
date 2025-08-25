from typing import NamedTuple

import numpy as np
import torch
import torch.nn.functional as F
import torch_scatter
from numpy.typing import NDArray
from progress import deque
from torch import Tensor
from torch_geometric.utils import k_hop_subgraph


class Transition(NamedTuple):
    logp: torch.Tensor  # [N]
    reward: torch.Tensor  # [G]
    value: torch.Tensor  # [G]
    batch: torch.Tensor


def compute_reward(
    diou_l: Tensor, batch: Tensor, size_penalty: float = 0.01
) -> Tensor:
    """
    diou_l: [N] – node‑wise DIoU (lower is better)
    batch:  [N] – graph ID for each node
    Returns: [G] – reward per graph
    """
    # Normalise DIoU so that lower = better
    diou_norm = (diou_l - diou_l.mean()) / (diou_l.std() + 1e-6)

    # Node‑wise reward = -DIoU (because lower DIoU means more accurate)
    node_reward = -diou_norm

    # Graph‑wise mean reward
    G = batch.max().item() + 1
    reward_per_graph = torch.zeros(int(G)).to(diou_l.device)
    reward_per_graph = torch_scatter.scatter_mean(
        node_reward, batch, dim=0, out=reward_per_graph
    )

    # Size penalty – encourage fewer nodes
    num_nodes = torch_scatter.scatter_sum(torch.ones_like(diou_l), batch, dim=0)
    reward_per_graph -= size_penalty * num_nodes.float()

    return reward_per_graph


def compute_returns_and_advantages(
    transitions,
    gamma: float = 0.99,
    lam: float = 0.95,
    reward_last_only: bool = False,
):
    """
    transitions : list[Transition] – ordered from first to last layer
    returns    : torch.Tensor  [G]  (discounted return per graph)
    advantages : torch.Tensor  [G]  (G - V)
    """
    G = len(transitions[0].reward)  # number of graphs in the batch

    # Stack per‑layer values & rewards
    V = torch.stack([t.value for t in transitions]).detach()  # [L, G]
    R = torch.stack([t.reward for t in transitions])  # [L, G]

    # If only the last reward is non‑zero, copy it to all layers
    if reward_last_only:
        R = R.clone()
        R[:-1] = R[-1].unsqueeze(0)  # broadcast

    # Compute discounted returns with GAE (lambda)
    returns = torch.zeros_like(R)
    gae = 0.0
    for t in reversed(range(len(R))):
        delta = R[t] + gamma * (V[t + 1] if t + 1 < len(V) else 0) - V[t]
        gae = delta + gamma * lam * gae
        returns[t] = gae + V[t]



    advantages = returns - V
    advantages = torch.clamp(advantages, -10.0, 10.0)
    assert returns.shape[1] == G, (
        f"returns.shape[1] ({returns.shape[1]}) != G ({G})"
    )
    print('RET Mean/Std')
    print(returns.mean().item(), returns.std().item())
    print('Value Mean/Std')
    print(V.mean().item(), V.std().item())
    return returns, advantages


def actor_critic_loss(
    transitions: list[Transition],
    gamma: float = 0.99,
    lam: float = 0.95,
    entropy_coef: float = 0.01,
):
    """
    transitions : list[Transition] – all layers
    returns    : [G]
    advantages : [G]
    """
    returns, advantages = compute_returns_and_advantages(
        transitions,
        gamma,
        lam,
    )

    # Concatenate per‑layer node data
    logp_all = torch.cat([t.logp for t in transitions])  # [∑N_i]
    batch_all = torch.cat([t.batch for t in transitions])  # [∑N_i]

    # Broadcast advantage to nodes
    # adv_per_node = advantages[batch_all]  # [∑N_i]
    # print(f'Adv: {advantages.shape}, BA: {batch_all.shape}, ApN: {adv_per_node.shape}, LpA: {logp_all.shape}')

    # Policy loss (PG)
    actor_loss = 0.0
    for i, transition in enumerate(transitions):
        actor_loss += -(advantages[i][transition.batch] * transition.logp).mean()

    # Critic loss (smooth L1 / Huber)
    V_all = torch.cat([t.value for t in transitions])  # [L, G]
    R_all = torch.cat([returns[t] for t in range(len(returns))])  # [L, G]
    critic_loss = F.mse_loss(V_all, R_all)

    # Entropy bonus
    entropy = -(logp_all * torch.exp(logp_all)).mean()
    actor_loss -= entropy_coef * entropy

    return actor_loss, critic_loss


def diou_loss_1d(a: NDArray, b: NDArray) -> NDArray:
    EPSILON = 1e-6

    if a.ndim == 1:
        a = np.expand_dims(a, 0)
    if b.ndim == 1:
        b = np.expand_dims(b, 0)

    assert a.ndim == 2 and b.ndim == 2 and a.shape[1] == b.shape[1] == 2, (
        f"Bad data: a: {a.shape} b: {b.shape}"
    )
    a, b = np.broadcast_arrays(a, b)

    a_center = np.mean(a, axis=1)
    b_center = np.mean(b, axis=1)
    dist_sq = (a_center - b_center) ** 2
    farthest_end = np.max(
        np.hstack([a[:, 1][:, None], b[:, 1][:, None]]), axis=1
    )
    closest_start = np.min(
        np.hstack([a[:, 0][:, None], b[:, 0][:, None]]), axis=1
    )
    outer_distance_sq = (farthest_end - closest_start) ** 2

    start = np.max(np.hstack([a[:, 0][:, None], b[:, 0][:, None]]), axis=1)
    end = np.min(np.hstack([a[:, 1][:, None], b[:, 1][:, None]]), axis=1)
    valid = start < end
    intersection = np.hstack((start[:, None], end[:, None]))
    intersection[~valid] = 0
    intersection = np.diff(intersection, axis=1)
    union = (np.diff(a, axis=1) + np.diff(b, axis=1)) - intersection
    iou = intersection / (union + EPSILON)

    return 1.0 - iou.squeeze(1) + dist_sq / (outer_distance_sq + EPSILON)


def find_closest_mapping_index(line_mappings: NDArray, target: NDArray) -> int:
    return np.argmin(diou_loss_1d(line_mappings, target)).astype(int)


def diou_loss(
    line_mappings: NDArray,
    edge_index: NDArray,
    batch: NDArray,
    keys: NDArray,
    key_batch_associations: NDArray,
    k: int = 5,
    decay_alpha: float = 0.5,
    log_transform_diou: bool = True,
):
    """
    Calculates DIoU loss for the closest node in the graph to each key (based on DIoU) and fans out `k` hops
    with decay `decay_alpha`
    """

    assert line_mappings.shape[0] == batch.shape[0], (
        f"line_mappings.shape[0] ({line_mappings.shape[0]}) != batch.shape[0] ({batch.shape[0]})"
    )
    num_nodes = line_mappings.shape[0]

    # Shuffle keys to make it slightly more stochastic
    np.random.shuffle(keys)
    # Find the absolute best nodes (lowest DIoU loss) for each key
    best_node_indices = []
    for i, key in enumerate(keys):
        mask = batch == key_batch_associations[i]
        map_to_original = np.arange(batch.shape[0])[mask]
        best_node_indices.append(
            map_to_original[
                find_closest_mapping_index(line_mappings[mask], key)
            ]
        )
        # print(f'Best index for {key} is {best_node_indices[-1]} (with value {line_mappings[best_node_indices[-1]]}) from \n {line_mappings[mask]} w/loss\n {diou_loss_1d(line_mappings[mask], key)}')

    # Do k hop subgraph for each key, assigning the DIoU to each node in the graph
    edge_index: Tensor = torch.tensor(edge_index)
    subset, edge_index, mapping, edge_mask = k_hop_subgraph(
        best_node_indices,
        num_hops=k,
        edge_index=edge_index,
        num_nodes=num_nodes,
    )

    # Always want to add DIoU loss since multiple spans can be close enough to eachother for them to overlap

    # ChatGPT made this
    def decay_fn(depth: int):
        return decay_alpha**depth

    # Initialize full loss vector (global node indices)
    losses_full = torch.zeros(num_nodes, dtype=torch.float)

    # Build adjacency list for full graph
    adj = [[] for _ in range(num_nodes)]
    for src, dst in edge_index.t().tolist():
        adj[src].append(dst)
        adj[dst].append(src)  # if undirected

    # Propagate DIoU loss from each source node
    for i, node_index in enumerate(best_node_indices):
        diou_val = torch.tensor(
            diou_loss_1d(line_mappings[node_index], keys[i])
        )
        if log_transform_diou:
            diou_val = torch.log(diou_val + 1e-8)
        # print(f'Loss for node {node_index} is {diou_val} using {line_mappings[node_index]} and {keys[i]}')

        visited = set()
        queue = deque()
        queue.append((node_index, 0))  # (current_node, depth)

        while queue:
            current, depth = queue.popleft()
            if current in visited or depth > k:
                continue
            visited.add(current)

            losses_full[current] += diou_val.sum(dim=0) * decay_fn(depth)

            if depth < k:
                for neighbor in adj[current]:
                    if neighbor not in visited:
                        queue.append((neighbor, depth + 1))

    return losses_full


def create_line_number_mappings(
    merge_map: NDArray, original_line_mappings: NDArray
) -> NDArray:
    """
    Takes the `merge_map` of shape `[N]` generated by the model run and `original_line_mappings` of shape `[N, 2]`
    Uses an iterative algorithm to determine merges for each item in the merge map.
    The returned tensor contains the feature and it's start and end line numbers
    """
    assert merge_map.shape[0] == original_line_mappings.shape[0], (
        "Invalid arrays!"
    )
    num_features = np.unique(merge_map).shape[0]
    # assert num_features <= merge_map.shape[0], "Num features must be lte size of merge map"
    line_mappings = np.zeros([num_features, 2], dtype=np.long)
    # print(f'Original merge map size {merge_map.shape}')
    # print(f'Made line mappings {line_mappings.shape}')
    # The lookup key is the target merge, the values are all nodes that were merged into it
    lookup = {}
    for i, itm in enumerate(merge_map.tolist()):
        if itm in lookup:
            lookup[itm].append(i)
        else:
            lookup[itm] = [i]

    for target, values in lookup.items():
        min_num = 999999999
        max_num = -1
        for merge_src in values:
            row = original_line_mappings[merge_src]
            low = row[0]
            high = row[1]
            min_num = min(min_num, low)
            max_num = max(max_num, high)

        line_mappings[target] = np.array([min_num, max_num])

    return line_mappings


def recombine_per_graph_spans(
    batch: NDArray,  # [N] graph‐ID per pooled node
    recovered_per_graph: list[
        NDArray
    ],  # list of [n_g,2] arrays, one per graph in same order
    selected_indices_for_batch,
) -> NDArray:
    """
    Given:
      - batch:           length‐N array, batch[i] = graph‐ID of node i
      - recovered_per_graph[k]: array [n_k,2] for graph k,
         in the same order as your `for (pid, graph) in selected_indices`
    Returns:
      - full_spans: np.ndarray of shape [N,2],
         where full_spans[i] is the span for node i.
    """
    N = batch.shape[0]
    full_spans = np.zeros((N, 2), dtype=recovered_per_graph[0].dtype)

    # Suppose your graph‐loop was in this order:
    #   for idx, (pid, graph) in enumerate(selected_indices_for_batch):
    for idx, (_, graph) in enumerate(selected_indices_for_batch):
        mask = batch == graph  # boolean mask of shape [N]
        spans = recovered_per_graph[idx]  # shape [mask.sum(), 2]
        # sanity check:
        assert spans.shape[0] == mask.sum(), (
            f"Spans don't match mask: ({spans.shape[0]} != {mask.sum()})"
        )
        full_spans[mask] = spans  # broadcast assign into those rows

    return full_spans


def calculate_line_spans(
    merge_map: NDArray,
    selected_indices_for_batch: list[tuple[int, int]],
    selected_line_assignments_for_batch: NDArray,
    batch: NDArray,
    perm: NDArray,
):
    line_spans = []

    for _persistent_id, graph in selected_indices_for_batch:
        mask = batch == graph  # select survivors of that graph
        local_pos = np.nonzero(mask)[
            0
        ]  # e.g. [ 0, 3, 5, 9, ... ]  length N_graph

        assert local_pos.size != 0, "Graphs should never be fully removed"

        global_nodes = perm[
            local_pos
        ]  # now these are the true original indices

        # pull out their merge_map entries in the global map:
        merge_map_for_graph = merge_map[global_nodes]

        # now you have numbers in the range 0..N₀–1, but only for this graph’s survivors
        # if you need them 0..N_graph–1, remap:
        uniques = np.unique(merge_map_for_graph)
        remap = {orig: i for i, orig in enumerate(uniques)}
        merge_map_for_graph = np.array([remap[v] for v in merge_map_for_graph])

        # and likewise your line spans:
        line_assignments_for_graph = selected_line_assignments_for_batch[
            global_nodes
        ]

        recovered_assignments = create_line_number_mappings(
            merge_map_for_graph, line_assignments_for_graph
        )

        # Build per-node spans:
        node_spans = recovered_assignments[merge_map_for_graph]

        # assert nodes[mask].shape[0] == node_spans.shape[0], (
        #     f"Nodes for graph don't match assignments! {nodes[mask].shape[0]} != {recovered_assignments.shape[0]}"
        # )
        line_spans.append(node_spans)

    # assert line_spans.shape[0] == nodes.shape[0], "Nodes don't match line assignments!"

    recombined = recombine_per_graph_spans(
        batch, line_spans, selected_indices_for_batch
    )

    assert recombined.shape[0] == batch.shape[0], (
        f"line_spans.shape[0] ({recombined.shape[0]}) != a_batch.shape[0] ({batch.shape[0]})"
    )

    return recombined
