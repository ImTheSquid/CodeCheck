from collections import deque
import torch
import torch.nn as nn
from torch_geometric.nn import GATv2Conv, TopKPooling

class Actor(nn.Module):
    def __init__(self, in_dim: int, hidden_dims: list[int], num_heads: list[int], pool_ratios: list[float]):
        super().__init__()
        self.num_layers = len(num_heads)
        self.gats = nn.ModuleList()
        self.pools = nn.ModuleList()
        self.policy_heads = nn.ModuleList()

        dims = [in_dim] + hidden_dims

        for i in range(self.num_layers):
            self.gats.append(GATv2Conv(dims[i], dims[i + 1], heads=num_heads[i], concat=False))
            self.pools.append(TopKPooling(dims[i+1], ratio=pool_ratios[i]))
            self.policy_heads.append(nn.Linear(dims[i + 1], 1)) # Node selection score

    # ChatGPT generated this
    # Is there likely a better way of doing this? Yeah
    # Do I feel like trying to figure it out right now? Nah
    def batch_update_merge_map(self, merge_map, perm, edge_index):
        """
        Updates merge_map by running a BFS from removed nodes to the nearest surviving node.
        """
        num_nodes = merge_map.size(0)
        all_nodes = torch.arange(num_nodes)
        removed_nodes = all_nodes[~torch.isin(all_nodes, perm)]  # Nodes that got removed
        perm_set = set(perm.tolist())  # Surviving nodes

        # ----  BFS for each removed node in parallel ----
        for node in removed_nodes.tolist():
            queue = deque([node])
            visited = set([node])

            while queue:
                current = queue.popleft()

                # Get neighbors of the current node
                neighbors = edge_index[1][edge_index[0] == current].tolist()

                for neighbor in neighbors:
                    if neighbor in perm_set:  # Found the nearest surviving node!
                        merge_map[node] = neighbor
                        queue.clear()  # Stop BFS for this node
                        break
                    if neighbor not in visited:
                        visited.add(neighbor)
                        queue.append(neighbor)

        # Ensure all pooled nodes point to themselves
        merge_map[perm] = perm

        # **Resolve multi-step merges iteratively**
        for node in range(len(merge_map)):
            while merge_map[node] != merge_map[merge_map[node]]:  # Follow merges until stable
                merge_map[node] = merge_map[merge_map[node]]

        return merge_map

    def forward(self, x: torch.Tensor, edge_index: torch.Tensor, batch: torch.Tensor):
        # Merge map keeps track of which node is merged where
        # At the beginning no nodes are merged, so each node points to itself
        merge_map = torch.arange(x.shape[0])

        for i in range(self.num_layers):
            x = self.gats[i](x, edge_index)
            scores = torch.sigmoid(self.policy_heads[i](x))
            x, edge_index, _, batch, perm, _ = self.pools[i](x, edge_index, batch=batch, attn=scores)
            # Possible footgun: is edge_index passed here supposed to be the one from before or after pooling?
            merge_map = self.batch_update_merge_map(merge_map, perm, edge_index)

        return x, edge_index, merge_map
