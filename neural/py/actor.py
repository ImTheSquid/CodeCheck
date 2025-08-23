from collections import deque

import numpy as np
import torch
import torch.nn as nn
import torch.nn.functional as F
from numpy._typing import NDArray
from torch.distributions import Bernoulli
from torch_geometric.nn import GATv2Conv
from torch_geometric.nn.norm import LayerNorm
from torch_geometric.utils import degree, subgraph

from utils import Transition, calculate_line_spans, compute_reward, diou_loss


def find_closest_surviving_node(
    removed_node_index: int, pre_pool_edge_index, perm_set
) -> int:
    visited = set()
    queue = deque([removed_node_index])

    while queue:
        node = queue.popleft()
        # print(f"dequeue {node}")
        if node not in visited:
            visited.add(node)

            # Is this node in `perm`? If so it survived the pooling and is the nearest node
            if node in perm_set:
                # print(f"NODE {node} FOUND AS NEAREST NEIGHBOR")
                return node

            # Find neighbors
            # Neighbors for which `node` is a source
            source_neighbors = pre_pool_edge_index[
                1, pre_pool_edge_index[0] == node
            ]
            # Neighbors for which `node` is a target
            target_neighbors = pre_pool_edge_index[
                0, pre_pool_edge_index[1] == node
            ]

            # Do target neighbors first to work up the tree
            for neighbor in torch.cat(
                [target_neighbors, source_neighbors]
            ).tolist():
                if neighbor not in visited:
                    # print(f"NEIGHBOR {neighbor}")
                    queue.append(neighbor)

    return -1


class Actor(nn.Module):
    def __init__(
        self,
        in_dim: int,
        hidden_dims: list[int],
        num_heads: list[int],
        critic: nn.Module,
        selection_dropout: float = 0.3,
        alpha: float = 0.5,
        beta: float = 0.5,
    ):
        """
        alpha: The weight between the policy and degree prior for handling node biases within the AST
        beta: Weight for depth
        """

        super().__init__()
        self.num_layers = len(num_heads)
        self.gats = nn.ModuleList()
        self.norms = nn.ModuleList()
        # self.pools = nn.ModuleList()
        self.policy_heads = nn.ModuleList()
        self.alpha = alpha
        self.beta = beta
        self.critic = critic

        dims = [in_dim] + hidden_dims

        for i in range(self.num_layers):
            self.gats.append(
                GATv2Conv(
                    dims[i] * (num_heads[i - 1] if i > 0 else 1),
                    dims[i + 1],
                    heads=num_heads[i],
                )
            )
            self.norms.append(LayerNorm(dims[i + 1] * num_heads[i]))

            # self.pools.append(TopKPooling(dims[i+1], ratio=pool_ratios[i]))
            self.policy_heads.append(
                nn.Sequential(
                    nn.Linear(dims[i + 1] * num_heads[i], dims[i + 1]),
                    nn.GELU(),
                    nn.Linear(dims[i + 1], dims[i + 1] // 2),
                    nn.GELU(),
                    nn.Dropout(p=selection_dropout),
                    nn.Linear(dims[i + 1] // 2, dims[i + 1] // 4),
                    nn.GELU(),
                    nn.Linear(dims[i + 1] // 4, 1),
                )
            )  # Node selection score

        self.reducer = nn.Sequential(
            nn.Linear(dims[-1] * num_heads[-1], dims[-1]), nn.GELU()
        )

    def compute_depths(self, edge_index, batch):
        num_nodes = batch.size(0)
        device = edge_index.device
        depths = torch.full((num_nodes,), -1, dtype=torch.float, device=device)

        for g in batch.unique():
            mask = batch == g
            local_indices = mask.nonzero(as_tuple=True)[0]
            if local_indices.numel() == 0:
                continue

            root = local_indices[0].item()
            visited = set()
            queue = deque([(root, 0)])

            while queue:
                node, depth = queue.popleft()
                if node not in visited:
                    visited.add(node)
                    depths[node] = depth

                    neighbors = edge_index[1][edge_index[0] == node]
                    for neighbor in neighbors.tolist():
                        if neighbor not in visited:
                            queue.append((neighbor, depth + 1))

        # Normalize depth per graph
        norm_depths = torch.zeros_like(depths)
        for g in batch.unique():
            mask = batch == g
            local_depths = depths[mask]
            mean = local_depths.mean()
            std = local_depths.std(unbiased=False) + 1e-6
            gauss = torch.exp(-0.5 * ((local_depths - mean) / std) ** 2)
            gauss = (gauss - gauss.min()) / (gauss.max() - gauss.min() + 1e-6)
            norm_depths[mask] = gauss

        return norm_depths

    def bias_nodes_per_degree(
        self,
        x: torch.Tensor,
        edge_index: torch.Tensor,
        batch: torch.Tensor,
        probabilities: torch.Tensor,
    ) -> torch.Tensor:
        # Compute node degree (per node)
        deg = degree(
            edge_index[0], x.size(0), dtype=x.dtype
        )  # size: [num_nodes]

        # Optional: normalize per graph
        centrality_prior = torch.zeros_like(deg)
        for g in batch.unique():
            mask = batch == g
            local_deg = deg[mask]

            # Rescale: Gaussian bump around the middle
            sorted_deg, _ = local_deg.sort(descending=True)
            mean = sorted_deg.mean()
            std = sorted_deg.std(unbiased=False) + 1e-6
            gaussian = torch.exp(-0.5 * ((local_deg - mean) / std) ** 2)

            # Normalize
            gaussian = (gaussian - gaussian.min()) / (
                gaussian.max() - gaussian.min() + 1e-6
            )
            centrality_prior[mask] = gaussian

        prior = self.beta * centrality_prior + (
            1 - self.beta
        ) * self.compute_depths(edge_index, batch)

        # Mix centrality bias with learnable policy
        return self.alpha * probabilities + (1 - self.alpha) * prior

    def forward(
        self,
        x: torch.Tensor,
        edge_index: torch.Tensor,
        batch: torch.Tensor,
        key_batch: list[int],
        keys_1d: list[NDArray],
        persistent_to_batch_id_map: list[tuple[int, int]],
        selected_spans: NDArray,
    ):
        N0 = x.size(0)
        merge_map = torch.arange(N0, device=x.device)  # global merge_map
        global_map = torch.arange(N0, device=x.device)  # local→global map
        perm = 0

        # Sanity check
        b_start = torch.unique(torch.clone(batch))
        start_num_graphs = b_start.shape[0]

        logp_terms = []
        logp_last = None

        transitions = []

        for i in range(self.num_layers):
            # 1) GAT + score
            x = self.gats[i](x, edge_index)
            x = F.relu(x)
            x = self.norms[i](x)
            logits = self.policy_heads[i](x).squeeze(-1)
            probs = torch.sigmoid(logits)
            probs = self.bias_nodes_per_degree(x, edge_index, batch, probs)
            dist = Bernoulli(probs)
            actions = dist.sample()
            for g in batch.unique():
                mask = batch == g
                mask_indices = mask.nonzero(as_tuple=True)[0]
                if mask_indices.numel() == 0:
                    continue  # just in case
                if actions[mask_indices].sum() == 0:
                    top_idx = probs[mask_indices].argmax()
                    actions[mask_indices[top_idx]] = 1.0

            logp = dist.log_prob(actions)
            logp_last = logp
            logp_terms.append(logp)

            keep_mask = actions.bool()
            perm = keep_mask.nonzero(as_tuple=True)[0]

            # 2) Save pre‐pool state for BFS
            pre_edge_index = edge_index.clone().detach()
            # pre_batch      = batch
            pre_global_map = global_map.clone().detach()

            # 3) Pool (returns new x, new edge_index, new batch, perm, _)
            # x, edge_index, _, batch, perm, _ = self.pools[i](
            #     x, edge_index,
            #     batch=batch,
            #     attn=scores
            # )

            edge_index, _ = subgraph(
                perm, edge_index, relabel_nodes=True, num_nodes=x.size(0)
            )
            logp_last = logp_last[perm]
            x = x[perm]
            batch = batch[perm]

            # 4) Survivors in this layer, in original indexing
            surv_global = pre_global_map[perm]  # shape = [# kept nodes]

            # 5) Which local nodes were removed *this* layer?
            all_local = torch.arange(pre_global_map.size(0), device=x.device)
            removed_local = all_local[~torch.isin(all_local, perm)]

            perm_set = set(perm.tolist())
            # 6) For each removed local node, find its BFS‐nearest surviving *local* node:
            for loc in removed_local.tolist():
                rep_loc = find_closest_surviving_node(
                    loc, pre_edge_index, perm_set
                )
                rep_glob = pre_global_map[rep_loc]
                orig = pre_global_map[loc]
                merge_map[orig] = rep_glob

            # 7) Ensure survivors map to self
            merge_map[surv_global] = surv_global

            # 8) Shrink your local→global map for the next layer
            global_map = surv_global.clone()

            x = F.gelu(x)

            predicted_reward = self.critic(x, edge_index, batch)

            line_spans = calculate_line_spans(
                merge_map=merge_map.cpu().numpy(),
                selected_indices_for_batch=persistent_to_batch_id_map,
                batch=batch.cpu().numpy(),
                perm=perm.cpu().numpy(),
                selected_line_assignments_for_batch=selected_spans,
            )

            diou_l = diou_loss(
                line_mappings=line_spans,
                edge_index=edge_index.cpu().numpy(),
                batch=batch.cpu().numpy(),
                keys=np.vstack(keys_1d),
                key_batch_associations=np.vstack(key_batch).squeeze(1),
                k=10,
                decay_alpha=0.7,
            ).to(x.device)

            reward_g = compute_reward(diou_l, batch)  # [G]

            transition = Transition(
                logp=logp_last,
                reward=reward_g,
                value=predicted_reward,
                batch=batch,
            )
            transitions.append(transition)

        x = self.reducer(x)

        # Sanity check
        end_num_graphs = torch.unique(batch).shape[0]
        assert start_num_graphs == end_num_graphs, (
            f"Graph quantity mismatch! {start_num_graphs} != {end_num_graphs}, Removed: {b_start[~torch.isin(b_start, torch.unique(batch))]}"
        )

        # All graphs have now been processed. It should theoretically be impossible for any graph to have
        # nodes that failed to find a survivor.
        assert not torch.any(merge_map == -1), (
            "Some nodes failed to find a survivor!"
        )

        return (
            x,
            edge_index,
            merge_map,
            batch,
            perm,
            logp_last,
            transitions,
        )
