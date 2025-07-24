import torch.nn as nn
from torch import Tensor
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


class MergeCritic(nn.Module):
    """
    How good is the merging performance of the actor?

    This critic wants to measure the distance of nodes from optimality.
    If a node is found in the data (same span), it is a "hot point". Points get colder
    the farther away other points are (line-wise). All nodes in the graph are scored.
    If no points in the graph are in the true labels, then it doesn't matter
    """

    def __init__(self, in_dim, hidden_dim, num_heads):
        super().__init__()
        self.gat = GATv2Conv(in_dim, hidden_dim, heads=num_heads, concat=False)
        self.norm = BatchNorm(hidden_dim)
        self.value_head = nn.Sequential(nn.Linear(hidden_dim, hidden_dim // 2), nn.ReLU(), nn.Linear(hidden_dim // 2, hidden_dim // 4), nn.ReLU(), nn.Linear(hidden_dim //4,1))

    def forward(self, x: Tensor, edge_index: Tensor, batch: Tensor) -> Tensor:
        """
        Takes in the logits and predicts the MSE for each node
        """
        h = self.gat(x, edge_index)
        h = self.norm(h)
        values = self.value_head(h)

        return values


class EmbeddingCritic(nn.Module):
    """
    How well does each representative vector fit with any other representative vector?

    Want to do something similar to triplet loss here if possible.
    This will always operate on multiple graphs.
    If two vectors from different graphs are plagiarized, try to bring them closer together.
    If there are other plagiarism cases in the other graph, distance the vector from those
    """

    pass
