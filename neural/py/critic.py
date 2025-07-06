import torch
import torch.nn as nn
from torch_geometric.nn import GATv2Conv
from torch_geometric.nn.norm import BatchNorm

class Critic(nn.Module):
    def __init__(self, in_dim, hidden_dim, num_heads):
        super().__init__()
        self.gat = GATv2Conv(in_dim, hidden_dim, heads=num_heads, concat=False)
        self.norm = BatchNorm(hidden_dim)
        self.value_head = nn.Linear(hidden_dim, 1)  # Value estimation

    def forward(self, x, edge_index, a_x, a_edge_index):
        h = self.gat(x, edge_index)
        h = self.norm(h)
        value = self.value_head(h).mean()  # Graph-level value score
        # print(f'VALUE {type(value)}')
        return value
