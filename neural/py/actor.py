from collections import deque

import numpy as np
import torch
import torch.nn as nn
import torch.nn.functional as F
from torch.distributions import Bernoulli
from torch_geometric.nn import GATv2Conv
from torch_geometric.nn.norm import LayerNorm
from torch_geometric.utils import degree, subgraph


def min_max_indices(tensor: torch.Tensor):
    """
    Find the minimum and maximum indices for each unique number in the tensor.

    Args:
        tensor (torch.Tensor): Input tensor with numbers.

    Returns:
        results (dict): A dictionary where keys are unique values, and values
                        are tuples containing the min and max index of occurrences.
    """
    # Get all unique values
    unique_values = torch.unique(tensor)
    results = {}

    for value in unique_values:
        # Find indices of this value in the tensor
        indices = torch.where(tensor == value)[0]

        # Calculate min and max indices
        min_index = torch.min(indices).item()
        max_index = torch.max(indices).item()

        # Store results
        results[value.item()] = (min_index, max_index)

    return results


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
        # pool_ratios: list[float],
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

    def batch_update_merge_map(
        self,
        merge_map: torch.Tensor,
        perm: torch.Tensor,
        pre_pool_edge_index: torch.Tensor,
        batch: torch.Tensor,
        new_batch: torch.Tensor,
    ):
        """
        Updates merge_map so that each removed node is assigned to the nearest surviving node.
        Uses BFS on the pre-pooling graph. Falls back to nearest surviving index if no path is found.

        Args:
            merge_map (Tensor): [N] Original merge map (each node points to itself at first).
            perm (Tensor): [M] Node indices that survived pooling.
            pre_pool_edge_index (Tensor): [2, E] Original edge index (before pooling).
            batch (Tensor): [N] Original batch graph IDs per node.
            new_batch (Tensor): [M] Batch graph IDs per node after pooling.

        Returns:
            new_merge_map (Tensor): Updated merge map with all nodes pointing to surviving ones.
        """
        # To already have been removed is to have an index i such that merge_map[i] != i.
        # To find the removed node indices that need to be processed,
        # first take the merge map and remove nodes that already
        # point to other nodes, as they were removed in previous iterations. Then remove nodes in
        # common with perm, as they survived. The remaining nodes are the ones that were removed.
        new_merge_map = merge_map.clone()
        # num_nodes = batch.shape[0]
        # all_nodes = torch.arange(num_nodes, device=merge_map.device)

        # removed_nodes = all_nodes[~torch.isin(all_nodes, perm)]  # Nodes that were removed
        all_nodes = torch.arange(merge_map.shape[0], device=merge_map.device)
        not_yet_merged = merge_map == all_nodes
        removed_this_layer = ~torch.isin(all_nodes, perm)
        removed_node_indices = torch.where(not_yet_merged & removed_this_layer)[
            0
        ]
        # removed_node_indices = torch.where(~torch.isin(all_nodes, perm))[0]
        perm_set = set(perm.tolist())
        print(
            f"There were {len(all_nodes)} nodes before pooling, {len(perm_set)} have survived."
        )
        print(
            f"These {len(removed_node_indices)} nodes have been REMOVED: {removed_node_indices} (diff {all_nodes.shape[0] - len(removed_node_indices)})"
        )
        # print(f'SURVIVORS: {perm_set}')

        # unresolved_nodes = []

        def find_closest_surviving_node(removed_node_index: int) -> int:
            nonlocal pre_pool_edge_index
            nonlocal perm_set

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

        assert removed_node_indices.ndim == 1
        # print(f'{removed_node_indices.shape[0]} NODES REMOVED')
        perm_to_batch = torch.full(
            (merge_map.shape[0],), -1, dtype=torch.long, device=perm.device
        )
        perm_to_batch[perm] = new_batch

        for node_index in removed_node_indices.tolist():
            removed_node_graph_id = batch[node_index]
            surviving_node = find_closest_surviving_node(node_index)
            assert surviving_node in perm or surviving_node == -1, (
                "BFS did not work"
            )
            new_merge_map[node_index] = surviving_node

            surviving_node_graph_id = perm_to_batch[surviving_node]
            assert (
                surviving_node_graph_id == removed_node_graph_id
                or surviving_node == -1
            ), (
                f"CROSSOVER DETECTED: Node association {node_index} -> {surviving_node} crosses graph boundary {removed_node_graph_id.item()} -> {surviving_node_graph_id.item()}"
            )

        for graph_id in torch.unique(batch):
            graph_node_indices = np.where(batch == graph_id)[0]
            relevant_merge_map = new_merge_map[graph_node_indices]
            assert torch.all(relevant_merge_map == -1) or not torch.any(
                relevant_merge_map == -1
            ), f"Graph {graph_id} has mixed merged map!"

        # Ensure all surviving nodes map to themselves
        new_merge_map[perm] = perm

        # Resolve multi-step chains
        for i in range(len(new_merge_map)):
            while new_merge_map[i] != new_merge_map[new_merge_map[i]]:
                new_merge_map[i] = new_merge_map[new_merge_map[i]]

        return new_merge_map

    def forward(
        self,
        x: torch.Tensor,
        edge_index: torch.Tensor,
        batch: torch.Tensor,
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

        logp_total = torch.cat(logp_terms).sum()

        return x, edge_index, merge_map, batch, perm, logp_total, logp_last
