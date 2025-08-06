import torch.nn.functional as F
from torch import nn
from torch_geometric.nn import GATv2Conv, LayerNorm

from graphham import GraphHAMLayer


class GraphEmbedding(nn.Module):
    def __init__(
        self, embedding_in_features: int, embedding_dim: int, *args, **kwargs
    ) -> None:
        super().__init__(*args, **kwargs)
        self.hams = nn.ModuleList(
            [
                GraphHAMLayer(
                    in_features=embedding_in_features,
                    out_features=embedding_dim,
                    num_heads=8,
                    num_groups=16,
                    temperature=0.5,
                    negative_slope=0.1,
                ),
                GraphHAMLayer(
                    in_features=embedding_dim,
                    out_features=embedding_dim,
                    num_heads=8,
                    num_groups=4,
                    temperature=0.5,
                    negative_slope=0.1,
                ),
            ]
        )
        self.norm = LayerNorm(embedding_dim)

    def forward(self, x, edge_index):
        losses = []
        for ham in self.hams:
            x, loss = ham(x, edge_index)
            x = F.gelu(x)
            x = self.norm(x)
            losses.append(loss)
        return x, losses


class GatGraphEmbedding(nn.Module):
    def __init__(
        self, in_channels: int, embedding_dim: int, p_dropout: float = 0.1
    ) -> None:
        super().__init__()
        self.gats = nn.ModuleList(
            [
                GATv2Conv(
                    in_channels,
                    out_channels=embedding_dim,
                    heads=4,
                ),
                GATv2Conv(
                    embedding_dim * 4,
                    out_channels=embedding_dim * 4,
                    heads=2,
                ),
                GATv2Conv(embedding_dim * 8, embedding_dim, heads=1),
            ]
        )
        self.norms = nn.ModuleList(
            [
                LayerNorm(embedding_dim * 4),
                LayerNorm(embedding_dim * 8),
                LayerNorm(embedding_dim),
            ]
        )
        self.dropout = nn.Dropout(p_dropout)

    def forward(self, x, edge_index):
        for gat, norm in zip(self.gats, self.norms):
            x = gat(x, edge_index)
            x = F.gelu(x)
            x = self.dropout(x)
            x = norm(x)
        return x


class EmbeddingPredictor(nn.Module):
    def __init__(
        self,
        embedding_model: nn.Module,
        embedding_dim: int,
        hidden_dim: int,
        out_dim: int,
        num_langs: int,
    ):
        super().__init__()
        self.embedding_model = embedding_model
        self.projector = nn.Sequential(
            nn.Linear(embedding_dim, hidden_dim),
            nn.ReLU(),
            nn.Linear(hidden_dim, out_dim),
        )
        in_channels = embedding_dim
        self.seq = nn.Sequential(
            nn.Linear(in_channels, in_channels // 2),
            nn.GELU(),
            nn.Linear(in_channels // 2, num_langs),
        )

    def forward(self, x, edge_index):
        h = self.embedding_model(x, edge_index)

        z_proj = self.projector(h)

        return z_proj, self.seq(h)
