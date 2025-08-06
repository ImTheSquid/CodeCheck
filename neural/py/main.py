import gc
import itertools
import os
from collections import defaultdict, deque
from pathlib import Path
from typing import Literal, Optional

import numpy as np
import psutil
import torch
import torch.nn.functional as F
from diskcache import Cache
from numpy.typing import NDArray
from progress.bar import Bar
from sklearn.cluster import HDBSCAN
from tabulate import tabulate
from torch import Tensor, nn, optim
from torch.utils.data import random_split
from torch.utils.data.dataset import Dataset
from torch_geometric.data import Data
from torch_geometric.loader import DataLoader
from torch_geometric.nn import global_mean_pool
from torch_geometric.utils import k_hop_subgraph, subgraph

from actor import Actor
from critic import MergeCritic
from embedding import EmbeddingPredictor, GatGraphEmbedding

DEVICE = torch.device(
    "cuda"
    if torch.cuda.is_available()
    else "mps"
    if torch.backends.mps.is_available()
    else "cpu"
)


BYTES_TO_GB = 1024**3


def memory_usage() -> tuple[int, int | None]:
    return psutil.Process().memory_info().rss, (
        torch.cuda.memory_allocated()
        if torch.cuda.is_available()
        else torch.mps.current_allocated_memory()
        if torch.mps.is_available
        else None
    )


def empty_cache():
    if torch.cuda.is_available():
        torch.cuda.empty_cache()
    elif torch.mps.is_available():
        torch.mps.empty_cache()


def sync_device():
    if torch.cuda.is_available():
        torch.cuda.synchronize()
    elif torch.mps.is_available():
        torch.mps.synchronize()


def cleanup():
    sync_device()
    gc.collect()
    empty_cache()


class GraphDataset(Dataset):
    def __init__(self, dataset, languages: NDArray):
        super().__init__()
        self.dataset = dataset
        self.languages = languages
        # for i, graph in enumerate(self.graphs):
        #     graph.key_index = i

    def __len__(self):
        return len(self.dataset)

    def get(self, idx: int):
        d: dict[str, NDArray] = self.dataset.get(idx)
        data = Data(
            x=torch.tensor(d["features"], dtype=torch.float),
            edge_index=torch.tensor(d["edge_index"], dtype=torch.long),
            # This is the only way I could get this to not freak out for some reason
            key_index=f"{idx}",
            language=torch.tensor([self.languages[idx]], dtype=torch.long),
            lines=torch.tensor(d["feature_spans"], dtype=torch.long),
        )
        del d
        return data

    def __getitem__(self, lookup):
        return self.get(lookup)


NUM_EPISODES = 25


def make_splits(dataset_sz: int) -> tuple[int, int, int]:
    # Define split sizes (80% Train, 10% Val, 10% Test)
    train_size = int(0.8 * dataset_sz)
    val_size = int(0.1 * dataset_sz)
    test_size = dataset_sz - train_size - val_size  # Ensure correct total
    return train_size, val_size, test_size


def find_relevant_keys_for_clustering(
    keys: dict[tuple[int, int], NDArray],
    selected_indices_for_batch: list[tuple[int, int]],
) -> dict[tuple[int, int], NDArray]:
    """
    Takes the main `keys` dict and filters to only graph indices present in the batch
    """
    persistent_graph_ids = list(map(lambda i: i[0], selected_indices_for_batch))
    pairs = set(itertools.combinations(persistent_graph_ids, 2))
    out = {k: keys[k] for k in keys.keys() if k in pairs}
    return out


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


def mean_pool_same_lines(
    x: NDArray, batch: NDArray, lines: NDArray
) -> tuple[NDArray, NDArray, NDArray]:
    """
    Mean pool all of the x values that are on the same line in the same graph
    """

    batch_with_lines = np.hstack([lines, np.expand_dims(batch, 1)])
    meaned = []
    batch_lines = []
    for collection in np.unique(batch_with_lines, axis=0):
        mask = np.all(batch_with_lines == collection, axis=1)
        all_x = x[mask]
        all_x_mean = all_x.mean(axis=0)
        meaned.append(all_x_mean)
        batch_lines.append(collection)

    meaned = np.vstack(meaned)
    batch_lines = np.vstack(batch_lines)

    return meaned, batch_lines[:, 2], batch_lines[:, :2]


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

    return recombine_per_graph_spans(
        batch, line_spans, selected_indices_for_batch
    )


def cluster_and_calculate_reward(
    nodes: NDArray,
    keys: dict[tuple[int, int], NDArray],
    selected_indices_for_batch: list[tuple[int, int]],
    batch: NDArray,
    line_spans: NDArray,
) -> float:
    assert not np.isnan(np.sum(nodes)), "NaN in nodes! Bad training :("
    assert batch.shape[0] == nodes.shape[0], (
        "Something is wrong, nodes must match batch"
    )

    nodes, batch, line_spans = mean_pool_same_lines(nodes, batch, line_spans)

    relevant = find_relevant_keys_for_clustering(
        keys, selected_indices_for_batch
    )
    print(
        f"Graphs in this batch are:\n{tabulate(selected_indices_for_batch, headers=['Persistent ID', 'Batch ID'])}"
    )
    print(f"Relevant keys for clustering: {relevant}")
    # If there aren't any relevant keys in this dataset then no change in score
    if len(relevant) == 0:
        return 0.0

    selected_indices_dictionary = {
        gid: pid for (pid, gid) in selected_indices_for_batch
    }

    print(
        tabulate(
            list(
                zip(
                    batch.tolist(),
                    map(lambda bid: selected_indices_dictionary[bid], batch),
                    map(lambda arr: f"{arr[0]}, {arr[1]}", line_spans),
                )
            ),
            headers=["Batch", "Persistent ID", "Line Assignments"],
        )
    )

    # 1) Cluster
    # clusterer = HDBSCAN(min_cluster_size=2, cluster_selection_method='leaf')
    clusterer = HDBSCAN(
        min_cluster_size=2,
        # n_jobs=-1,
        # prediction_data=True,
        cluster_selection_method="leaf",
        allow_single_cluster=True,
    )
    labels = clusterer.fit_predict(nodes)  # -1 = noise

    print(f"{labels.max()} clusters created")

    for label in np.unique(labels):
        mask = label == labels
        print(
            f"\n\nMEMBERS IN CLUSTER {label} (total {np.count_nonzero(mask)}):\n"
        )

        line_data = line_spans[mask]
        bid = batch[mask]
        pid = map(lambda b: selected_indices_dictionary[b], bid)
        print(
            tabulate(
                list(
                    zip(
                        bid,
                        pid,
                        map(lambda arr: f"{arr[0]}, {arr[1]}", line_data),
                    )
                ),
                headers=["Batch", "Persistent ID", "Line Assignments"],
            )
        )

    # 2) Recover per‐node spans
    #    If you want "true" spans per node from your original mapping,
    #    you can call the helper above. Otherwise assume line_spans is already per node.

    # 3) Build lookup: local_idx -> (pid,graph)
    # pid_graph = {i: selected_indices_for_batch[i] for i in range(len(selected_indices_for_batch))}

    # 4) Count TP/FP and track which key‐rows we match exactly
    TP = FP = 0
    matched = set()  # (pair, (si,sj,ei,ej))

    # all_keys = set(keys.keys())

    for cluster in set(labels):
        if cluster < 0:
            continue
        members = np.where(labels == cluster)[0]
        for i, j in itertools.combinations(members, 2):
            g_i = batch[i]
            g_j = batch[j]

            # print(f"SEL {i}, {j}")
            # pid_i, g_i = pid_graph[i]
            # pid_j, g_j = pid_graph[j]
            if g_i == g_j:
                continue

            pid_i = pid_j = -1

            for pid, g in selected_indices_for_batch:
                if g == g_i:
                    pid_i = pid
                elif g == g_j:
                    pid_j = pid
                # Early return if both found
                if pid_i > -1 and pid_j > -1:
                    break

            span_i = tuple(line_spans[i].tolist())
            span_j = tuple(line_spans[j].tolist())

            pair = (pid_i, pid_j)
            arr = keys.get(pair, None)
            if arr is None or arr.size == 0:
                # no true spans → false positive
                FP += 1
                continue

            # want to see [si, sj, ei, ej]
            want1 = (span_i[0], span_j[0], span_i[1], span_j[1])
            want2 = (span_j[0], span_i[0], span_j[1], span_i[1])

            found = False
            for row in arr:
                row_t = (int(row[0]), int(row[1]), int(row[2]), int(row[3]))
                if row_t == want1 or row_t == want2:
                    TP += 1
                    matched.add((pair, row_t))
                    found = True
                    break
            if not found:
                FP += 1

    # 5) Count FN: ground‐truth rows never matched
    FN = 0
    for pair, arr in keys.items():
        if arr.size == 0:
            continue
        for row in arr:
            row_t = (int(row[0]), int(row[1]), int(row[2]), int(row[3]))
            if (pair, row_t) not in matched:
                FN += 1

    # 6) Precision/Recall/F1
    precision = TP / (TP + FP) if TP + FP > 0 else 0.0
    recall = TP / (TP + FN) if TP + FN > 0 else 0.0
    f1 = (
        2 * precision * recall / (precision + recall)
        if precision + recall > 0
        else 0.0
    )

    print(
        f"True Positives: {TP}\nFalse Positives: {FP}\nFalse Negatives: {FN}\nF1: {f1}"
    )

    return f1


def train(
    dataset: Dataset,
    embedder: nn.Module,
    actor: nn.Module,
    critic: nn.Module,
    actor_optim: optim.Optimizer,
    critic_optim: optim.Optimizer,
    episodes: int,
    keys: dict[tuple[int, int], NDArray],
):
    train_set, val_set, test_set = random_split(
        dataset,
        make_splits(len(dataset)),  # type: ignore
    )

    train_data = DataLoader(train_set, batch_size=25, shuffle=True)  # type: ignore
    val_data = DataLoader(val_set, batch_size=12)  # type: ignore
    test_data = DataLoader(test_set, batch_size=12)  # type: ignore

    for epoch in range(episodes):
        print(f"\n⏰ EPOCH {epoch}")
        embedder.train()
        actor.train()
        critic.train()

        total_actor_loss, total_critic_loss = 0.0, 0.0

        # Train
        for batch in train_data:
            print("\n\nNEXT BATCH ->")

            # Selected graph indices are actually different than the assignments given by PyTorch
            # Zip them together for processing later
            selected_batch_indices = list(range(torch.max(batch.batch) + 1))
            persistent_to_batch_id_map = list(
                zip(map(int, batch.key_index), selected_batch_indices)
            )

            selected_spans = batch.lines
            batch = batch.to(DEVICE)

            x = embedder(batch.x, batch.edge_index)

            (
                a_x,
                a_edge_index,
                merge_map,
                a_batch,
                a_perm,
                a_logp_sum,
                a_logp_last,
            ) = actor(x, batch.edge_index, batch.batch)

            # pred_reward = critic(batch.x, batch.edge_index, a_x, a_edge_index)

            line_spans = calculate_line_spans(
                merge_map=merge_map.cpu().numpy(),
                selected_indices_for_batch=persistent_to_batch_id_map,
                batch=a_batch.cpu().numpy(),
                perm=a_perm.cpu().numpy(),
                selected_line_assignments_for_batch=selected_spans.cpu().numpy(),
            )
            assert line_spans.shape[0] == a_batch.shape[0], (
                f"line_spans.shape[0] ({line_spans.shape[0]}) != a_batch.shape[0] ({a_batch.shape[0]})"
            )

            # reward = cluster_and_calculate_reward(
            #     nodes=a_x.cpu().detach().numpy(),
            #     keys=keys,
            #     selected_indices_for_batch=persistent_to_batch_id_map,
            #     batch=a_batch.cpu().numpy(),
            #     line_spans=line_spans
            # )

            # reward = torch.tensor(reward).to(DEVICE)

            # Keys from this batch specifically along with their associations
            keys_1d = []
            key_batch = []

            def add_key_to_keys_and_batch(val: NDArray, gid, target_left: bool):
                nonlocal keys_1d, key_batch
                if target_left:
                    keys_1d.append(val[:, [0, 2]])
                else:
                    keys_1d.append(val[:, [1, 3]])
                key_batch += [gid] * val.shape[0]

            for pid, gid in persistent_to_batch_id_map:
                for left, right in keys.keys():
                    if pid == left or pid == right:
                        add_key_to_keys_and_batch(
                            keys[(left, right)], gid, pid == left
                        )

            # TODO: Deduplicate keys_1d
            diou_l = diou_loss(
                line_mappings=line_spans,
                edge_index=a_edge_index.cpu().numpy(),
                batch=a_batch.cpu().numpy(),
                keys=np.vstack(keys_1d),
                key_batch_associations=np.vstack(key_batch).squeeze(1),
                k=10,
                decay_alpha=0.7,
            )
            diou_l = (diou_l - diou_l.mean()) / (diou_l.std() + 1e-6)
            diou_l = diou_l.to(DEVICE)

            together = torch.cat(
                [a_x.detach(), a_logp_last.detach().unsqueeze(1)], dim=1
            )

            pred_diou_l = critic(together, a_edge_index, a_batch)

            critic_loss = F.mse_loss(pred_diou_l.squeeze(1), diou_l)
            critic_optim.zero_grad()
            critic_loss.backward()
            critic_optim.step()

            # advantage = (reward - pred_reward).detach()
            # actor_loss = -(advantage * a_logp_sum).mean() + embedding_loss

            # actor_optim.zero_grad()
            # actor_loss.backward()
            # actor_optim.step()

            # critic_loss = nn.HuberLoss()(pred_reward, reward)
            # critic_optim.zero_grad()
            # critic_loss.backward()
            # critic_optim.step()

            advantage = diou_l.detach() - pred_diou_l.detach().squeeze(1)
            actor_loss = -(advantage * a_logp_sum).mean()
            actor_optim.zero_grad()
            actor_loss.backward()
            actor_optim.step()

            total_actor_loss += actor_loss
            total_critic_loss += critic_loss
            print(
                "*" * 10
                + f"\nBatch Loss:\nActor: {actor_loss}\nCritic: {critic_loss}\n"
                + "*" * 10
            )

        # Val
        for batch in val_data:
            pass

        print(
            f"~~\nTotal Loss:\nActor: {total_actor_loss}\nCritic: {total_critic_loss}\n~~"
        )

    # Test
    actor.eval()
    critic.eval()
    for batch in test_data:
        pass

    print("Training complete")


def find_root_nodes(edge_index: Tensor, node_indices: Tensor) -> list[int]:
    """
    Finds root nodes in a given graph using node indices.
    A root node is one with no incoming edges.
    """
    incoming = defaultdict(int)
    for src, dst in edge_index.t().tolist():
        if dst in node_indices:
            incoming[dst] += 1

    return [
        int(node.item())
        for node in node_indices
        if incoming[int(node.item())] == 0
    ]


def preorder_traversal(edge_index: Tensor, root: int) -> list[int]:
    """
    Performs a preorder traversal (node → children) of a tree rooted at `root`.
    `edge_index` is expected to be a 2 x E tensor of directed edges.
    """
    tree = defaultdict(list)
    for src, dst in edge_index.t().tolist():
        tree[src].append(dst)

    visited = set()
    order = []

    def dfs(node):
        if node in visited:
            return
        visited.add(node)
        order.append(node)
        for child in tree[node]:
            dfs(child)

    dfs(root)
    return order


def get_projections_and_targets_for_each_batch(proj, batch, cache: Cache):
    tgt = torch.argmax(batch.x, dim=1)

    selected_batch_indices = list(range(torch.max(batch.batch) + 1))
    persistent_to_batch_id_map = list(
        zip(map(int, batch.key_index), selected_batch_indices)
    )

    batch_to_persistent_id_map = {
        bid: pid for pid, bid in persistent_to_batch_id_map
    }

    for batch_id in torch.unique(batch.batch):
        if batch_to_persistent_id_map[int(batch_id)] in cache:
            preorder = cache[batch_to_persistent_id_map[int(batch_id)]]
        else:
            subgraph_mask = batch.batch == batch_id
            node_indices = subgraph_mask.nonzero(as_tuple=True)[0]
            subgraph_edge_index, _ = subgraph(
                edge_index=batch.edge_index,
                subset=node_indices,
                relabel_nodes=True,
            )

            # Find root node(s)
            roots = find_root_nodes(
                subgraph_edge_index, torch.arange(node_indices.shape[0])
            )
            assert len(roots) == 1, "AST should have exactly one root"

            # Traverse
            preorder = preorder_traversal(subgraph_edge_index, roots[0])
            cache[batch_to_persistent_id_map[int(batch_id)]] = preorder

        from collections import deque

        def window(seq, n=2):
            it = iter(seq)
            win = deque((next(it, None) for _ in range(n)), maxlen=n)
            yield win
            append = win.append
            for e in it:
                append(e)
                yield win

        projs = []
        tgts = []
        for a, b in window(preorder):
            projs.append(proj[a])
            tgts.append(tgt[b])

        projs = torch.stack(projs)
        tgts = torch.stack(tgts)
        yield projs, tgts


def mine_triplets(
    x: torch.Tensor,
    embeddings: torch.Tensor,
    top_k: int = 3,
    distance_metric: str = "cosine",
) -> torch.Tensor:
    """
    Args:
        x: (N, D) one-hot node type matrix.
        embeddings: (D, E) embedding lookup for node types.
        top_k: number of types to mine for positives and negatives.
        distance_metric: 'cosine' or 'euclidean'.

    Returns:
        triplets: (T, 3) LongTensor with anchor, positive, and negative indices.
    """
    node_types = x.argmax(dim=1)  # (N,)
    N, D = x.shape

    # === 1. Compute (D, D) distance matrix between types ===
    if distance_metric == "cosine":
        type_embs = torch.nn.functional.normalize(embeddings, dim=1)
        dist_type_type = 1 - type_embs @ type_embs.T
    elif distance_metric == "euclidean":
        diff = embeddings[:, None, :] - embeddings[None, :, :]
        dist_type_type = torch.norm(diff, dim=2)
    else:
        raise ValueError("Unsupported distance metric")

    # Exclude self-types using NaN (so they don't appear in topk either way)
    dist_type_type.fill_diagonal_(float("nan"))

    # === 2. Precompute type -> indices map ===
    type_to_indices = [[] for _ in range(D)]
    for idx, t in enumerate(node_types.tolist()):
        type_to_indices[t].append(idx)

    # === 3. Mine triplets ===
    triplets = []

    for i in range(N):
        anchor_type = node_types[i]
        dists = dist_type_type[anchor_type]  # (D,)

        if dists.isnan().all():
            continue  # no other types to compare to

        # Top-k closest and farthest *types* (excluding self via NaN)
        _, closest_types = torch.topk(dists, top_k, largest=False)
        _, farthest_types = torch.topk(dists, top_k, largest=True)

        for pt in closest_types.tolist():
            pos_candidates = type_to_indices[pt]
            if not pos_candidates:
                continue
            pos_idx = pos_candidates[torch.randint(len(pos_candidates), (1,))]

            for nt in farthest_types.tolist():
                neg_candidates = type_to_indices[nt]
                if not neg_candidates:
                    continue
                neg_idx = neg_candidates[
                    torch.randint(len(neg_candidates), (1,))
                ]
                triplets.append((i, pos_idx, neg_idx))

    assert len(triplets) > 0, "Triples array empty, probably bad"

    return torch.tensor(triplets, dtype=torch.long)


class AverageAccumulator:
    def __init__(self, init: float | Tensor = 0.0) -> None:
        self.sum = init
        self.num = 0

    def __iadd__(self, o):
        self.sum += o
        self.num += 1
        return self

    def mean(self) -> float | Tensor:
        return self.sum / max(self.num, 1)


def train_embeddings(
    dataset: Dataset,
    num_episodes: int,
    embedder: nn.Module,
    optimizer: optim.Optimizer,
    artifact_dir: Path,
    ast_embeddings: Tensor,
    top_k: int,
    traversal_cache: Cache,
):
    train_set, val_set, test_set = random_split(
        dataset,
        make_splits(len(dataset)),  # type: ignore
    )

    checkpoint_dir = artifact_dir / "embedding_checkpoints"
    if checkpoint_dir.exists():
        import shutil

        shutil.rmtree(checkpoint_dir)
    os.makedirs(checkpoint_dir)

    train_data = DataLoader(
        train_set,  # type: ignore
        batch_size=80,
        shuffle=True,
        num_workers=10,
    )
    val_data = DataLoader(val_set, batch_size=100, num_workers=10)  # type: ignore
    test_data = DataLoader(test_set, batch_size=100, num_workers=10)  # type: ignore

    LANG_LOSS_SCALE = 2.0
    TRIPLET_LOSS_SCALE = 4.0
    NEXT_NODE_LOSS_SCALE = 1.0

    for epoch in range(num_episodes):
        total_loss = AverageAccumulator()
        bar = Bar()
        for batch in bar.iter(train_data):
            cpu, gpu = memory_usage()
            bar.suffix = f"Batch %(index)d/%(max)d Epoch {epoch} CPU: {(cpu / BYTES_TO_GB):.02f} GB GPU: {(gpu / BYTES_TO_GB if gpu else 0.0):.02f} GB"

            batch = batch.to(DEVICE)

            triplets = mine_triplets(batch.x, ast_embeddings, top_k=top_k)

            proj, lang = embedder(batch.x, batch.edge_index)

            triplet_loss = F.triplet_margin_loss(
                proj[triplets[0]], proj[triplets[1]], proj[triplets[2]]
            )

            lang = global_mean_pool(lang, batch.batch)

            next_node_loss = AverageAccumulator()
            for projs, tgts in get_projections_and_targets_for_each_batch(
                proj, batch, traversal_cache
            ):
                next_node_loss += F.cross_entropy(projs, tgts)

            batch_loss = (
                F.cross_entropy(lang, batch.language) * LANG_LOSS_SCALE
                + triplet_loss * TRIPLET_LOSS_SCALE
                + next_node_loss.mean() * NEXT_NODE_LOSS_SCALE
            )

            batch_loss.backward()
            optimizer.step()
            optimizer.zero_grad(set_to_none=True)
            total_loss += batch_loss.item()

            del (
                batch,
                triplets,
                proj,
                lang,
                batch_loss,
                triplet_loss,
                next_node_loss,
            )

            cleanup()

        print(f"Epoch {epoch} mean loss {total_loss.mean()}")

        torch.save(
            embedder.embedding_model,
            checkpoint_dir / f"{epoch}.pt",
        )

        val_loss = AverageAccumulator()
        for batch in val_data:
            with torch.no_grad():
                batch = batch.to(DEVICE)
                triplets = mine_triplets(batch.x, ast_embeddings, top_k=top_k)

                proj, lang = embedder(batch.x, batch.edge_index)

                triplet_loss = F.triplet_margin_loss(
                    proj[triplets[0]], proj[triplets[1]], proj[triplets[2]]
                )
                lang = global_mean_pool(lang, batch.batch)

                next_node_loss = AverageAccumulator()
                for (
                    projs,
                    tgts,
                ) in get_projections_and_targets_for_each_batch(
                    proj, batch, traversal_cache
                ):
                    next_node_loss += F.cross_entropy(projs, tgts)

                batch_loss = (
                    F.cross_entropy(lang, batch.language) * LANG_LOSS_SCALE
                    + triplet_loss * TRIPLET_LOSS_SCALE
                    + next_node_loss.mean() * NEXT_NODE_LOSS_SCALE
                )
                val_loss += batch_loss.item()

                del (
                    batch,
                    triplets,
                    proj,
                    lang,
                    batch_loss,
                    triplet_loss,
                    next_node_loss,
                )

                cleanup()
        print(f"Mean val loss {val_loss.mean()}")
        cleanup()

    test_loss = AverageAccumulator()
    for batch in test_data:
        batch = batch.to(DEVICE)
        with torch.no_grad():
            triplets = mine_triplets(batch.x, ast_embeddings, top_k=top_k)
            proj, lang = embedder(batch.x, batch.edge_index)
            lang = global_mean_pool(lang, batch.batch)

            triplet_loss = F.triplet_margin_loss(
                proj[triplets[0]], proj[triplets[1]], proj[triplets[2]]
            )

            next_node_loss = AverageAccumulator()
            for projs, tgts in get_projections_and_targets_for_each_batch(
                proj, batch, traversal_cache
            ):
                next_node_loss += F.cross_entropy(projs, tgts)

            batch_loss = (
                F.cross_entropy(lang, batch.language) * LANG_LOSS_SCALE
                + triplet_loss * TRIPLET_LOSS_SCALE
                + next_node_loss.mean() * NEXT_NODE_LOSS_SCALE
            )

            test_loss += batch_loss.item()

            del (
                batch,
                triplets,
                proj,
                lang,
                batch_loss,
                triplet_loss,
                next_node_loss,
            )

            cleanup()

    print(f"Mean test loss: {test_loss.mean()}")
    print("Saving to artifact directory")

    torch.save(embedder.embedding_model, artifact_dir / "embeddings.pt")


def test_embeddings(embedder: nn.Module, dataset: Dataset, artifact_dir: Path):
    import datetime

    import matplotlib.pyplot as plt
    import pandas as pd
    import plotly.express as px
    from sklearn.manifold import TSNE

    plot_dir = (
        datetime.datetime.now()
        .replace(microsecond=0)
        .isoformat()
        .replace(":", "_")
    )

    print(f"Saving plots to {artifact_dir / plot_dir}")

    os.makedirs(artifact_dir / plot_dir)

    data = DataLoader(dataset, batch_size=50, shuffle=True)  # type: ignore
    for i, batch in enumerate(data):
        batch = batch.to(DEVICE)
        print(f"Batch {i}")
        embeddings = embedder(batch.x, batch.edge_index)

        labels = batch.language[batch.batch].detach().cpu().numpy()

        labels = np.array(["C", "C++", "Java", "Python"])[labels]

        tsne = TSNE(n_components=3, perplexity=30)
        X_tsne = tsne.fit_transform(embeddings.cpu())
        df = pd.DataFrame(
            {
                "TSNE1": X_tsne[:, 0],
                "TSNE2": X_tsne[:, 1],
                "TSNE3": X_tsne[:, 2],
                "Label": labels,  # or y_encoded if you want numeric labels
            }
        )

        fig = plt.figure()
        ax = fig.add_subplot(111, projection="3d")
        scatter = ax.scatter(
            df["TSNE1"],
            df["TSNE2"],
            df["TSNE3"],
            c=pd.factorize(df["Label"])[0],  # Color by label
            cmap="tab10",
            alpha=0.8,
        )

        # Optional: Add a legend with label names
        legend_labels = df["Label"].unique()
        ax.legend(
            handles=scatter.legend_elements()[0], labels=legend_labels.tolist()
        )

        ax.set_xlabel("TSNE1")
        ax.set_ylabel("TSNE2")
        ax.set_zlabel("TSNE3")  # pyright: ignore
        plt.tight_layout()
        fig.savefig(artifact_dir / plot_dir / f"{i}.png")
        fig.clear()

        fig = px.scatter_3d(
            df,
            x="TSNE1",
            y="TSNE2",
            z="TSNE3",
            color="Label",
            title="3D t-SNE Visualization",
            opacity=0.8,
        )

        # Save to interactive HTML file
        fig.write_html(str(artifact_dir / plot_dir / f"{i}.html"))


def rust_train(
    dataset,
    artifact_dir: str,
    ast_embeddings: NDArray,
    mode: Literal["train", "embed", "embed-test"] = "train",
    top_k: int = 3,
    device: Optional[str] = None,
):
    global DEVICE
    if device:
        DEVICE = torch.device(device)

    torch.manual_seed(0xDEADBEEF)
    print(
        f"✅ Python initialization successful. Beginning training on device {DEVICE}..."
    )
    artifact_dir: Path = Path(artifact_dir)
    # print(f"KEYS\n\n{keys}\n\n")
    # dataset = generate_dataset(edges, features, feature_spans, languages)
    languages = dataset.languages
    keys = dataset.keys
    dataset = GraphDataset(dataset.loader, languages)

    embedding_in_features = dataset[0].x.shape[1]  # pyright: ignore reportOptionalMemberAccess

    # embedder = GraphEmbedding(
    #     embedding_in_features=embedding_in_features, embedding_dim=12
    # ).to(DEVICE)

    embedding_dim = min(500, embedding_in_features // 2)

    embedder = GatGraphEmbedding(
        in_channels=embedding_in_features, embedding_dim=embedding_dim
    ).to(DEVICE)

    match mode:
        case "embed-test":
            embedder = torch.load(
                artifact_dir / "embeddings.pt", weights_only=False
            )
            with torch.no_grad():
                test_embeddings(
                    embedder=embedder,
                    dataset=dataset,
                    artifact_dir=artifact_dir,
                )
        case "embed":
            print(
                f"Embedding with top k={top_k} from dim {embedding_in_features} to dim {embedding_dim}"
            )
            embedder = EmbeddingPredictor(
                embedding_model=embedder,
                embedding_dim=embedding_dim,
                hidden_dim=embedding_dim // 2,
                out_dim=embedding_in_features,
                num_langs=np.max(languages) + 1,
            ).to(DEVICE)

            with Cache() as traversal_cache:
                train_embeddings(
                    dataset,
                    num_episodes=50,
                    embedder=embedder,
                    optimizer=optim.Adam(
                        embedder.parameters(), lr=0.0005, weight_decay=0.01
                    ),
                    artifact_dir=artifact_dir,
                    ast_embeddings=torch.Tensor(ast_embeddings).to(DEVICE),
                    top_k=top_k,
                    traversal_cache=traversal_cache,
                )
        case "train":
            if os.path.exists(artifact_dir / "embeddings.pt"):
                print("Loading embeddings model into memory")
                embedder = torch.load(
                    artifact_dir / "embeddings.pt", weights_only=False
                )

            actor = Actor(
                in_dim=12,
                hidden_dims=[12, 12, 12, 10, 10],
                num_heads=[4, 4, 4, 4, 4],
                # pool_ratios=[0.5, 0.6, 0.8, 0.8, 0.8],
                alpha=0.5,
                beta=0.7,
                selection_dropout=0.2,
            ).to(DEVICE)
            # critic = Critic(in_dim=features[0].shape[1], hidden_dim=20, num_heads=8).to(DEVICE)
            critic = MergeCritic(in_dim=11, hidden_dim=10).to(DEVICE)

            train(
                dataset,
                embedder=embedder,
                actor=actor,
                critic=critic,
                actor_optim=optim.Adam(
                    actor.parameters(), lr=0.001, weight_decay=0.01
                ),
                critic_optim=optim.Adam(
                    critic.parameters(), lr=0.0001, weight_decay=0.01
                ),
                episodes=NUM_EPISODES,
                keys=keys,
            )
