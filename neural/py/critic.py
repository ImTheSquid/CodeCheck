import torch
import torch.nn as nn
import torch.nn.functional as F
from torch import Tensor
from torch_geometric.nn import GATv2Conv, global_mean_pool
from torch_geometric.nn.norm import LayerNorm


class Critic(nn.Module):
    def __init__(self, in_dim, hidden_dim, num_heads):
        super().__init__()
        self.gat = GATv2Conv(in_dim, hidden_dim, heads=num_heads, concat=False)
        self.norm = LayerNorm(hidden_dim)
        self.value_head = nn.Linear(hidden_dim, 1)  # Value estimation

    def forward(self, x, edge_index, a_x, a_edge_index):
        h = self.gat(x, edge_index)
        h = self.norm(h)
        value = self.value_head(h).mean()  # Graph-level value score
        # print(f'VALUE {type(value)}')
        return value


class MergeCritic(nn.Module):
    """
    How good is the merging performance of the actor?

    This critic wants to measure the distance of nodes from optimality.
    If a node is found in the data (same span), it is a "hot point". Points get colder
    the farther away other points are (line-wise). All nodes in the graph are scored.
    If no points in the graph are in the true labels, then it doesn't matter
    """

    def __init__(self, in_dim, hidden_dim_generator):
        super().__init__()
        hidden_dim = hidden_dim_generator(in_dim)
        self.max_hidden_dim = in_dim
        self.gats = nn.ModuleList(
            [
                GATv2Conv(in_dim, hidden_dim, heads=8),
                GATv2Conv(hidden_dim * 8, hidden_dim, heads=4),
                GATv2Conv(hidden_dim * 4, hidden_dim, concat=False),
            ]
        )
        self.norms = nn.ModuleList(
            [
                LayerNorm(hidden_dim * 8),
                LayerNorm(hidden_dim * 4),
                LayerNorm(hidden_dim),
            ]
        )
        self.graph_value_head = nn.Sequential(
            nn.Linear(hidden_dim, hidden_dim // 2),
            nn.GELU(),
            nn.Linear(hidden_dim // 2, hidden_dim // 4),
            nn.GELU(),
            nn.Linear(hidden_dim // 4, 1),
        )
        self.reset_parameters()

    def reset_parameters(self):
        for layer in self.graph_value_head:
            if isinstance(layer, nn.Linear):
                nn.init.xavier_normal_(layer.weight)

    def forward(self, x: Tensor, edge_index: Tensor, batch: Tensor) -> Tensor:
        """
        Takes in the logits and predicts the MSE for each node
        """

        # x may have shape [N, H] where H <= max_hidden_dim
        # Pad or slice to max_hidden_dim so the projection works
        H = x.size(1)
        if H < self.max_hidden_dim:
            # pad with zeros
            pad = torch.zeros(
                x.size(0), self.max_hidden_dim - H, device=x.device
            )
            x = torch.cat([x, pad], dim=1)
        elif H > self.max_hidden_dim:
            raise Exception(
                f"Input dimension {H} exceeds maximum allowed dimension {self.max_hidden_dim}"
            )

        for gat, norm in zip(self.gats, self.norms):
            x = gat(x, edge_index)
            x = F.gelu(x)
            x = norm(x)

        graph_mean = global_mean_pool(x, batch)
        values = self.graph_value_head(graph_mean).squeeze(-1)

        return values
