from typing import Any, Tuple

import torch
import torch.nn.functional as F
from torch import Tensor, nn
from torch_geometric.nn import MessagePassing
from torch_geometric.utils import softmax


# https://arxiv.org/pdf/2111.00604
class GraphHAMLayer(MessagePassing):
    def __init__(
        self,
        in_features: int,
        out_features: int,
        num_groups: int,
        num_heads: int,
        temperature: float = 1.0,
        non_linearity: str = "relu",
        non_linearity_args: list[Any] = [],
        negative_slope: float = 0.2,
        *args,
        **kwargs,
    ) -> None:
        super().__init__(*args, **kwargs)

        self.in_features = in_features
        self.out_features = out_features

        self.negative_slope = negative_slope
        self.num_groups = num_groups
        self.phi = nn.Parameter(torch.empty([num_groups, in_features]))
        # tau
        self.temperature = temperature

        # Attention weights
        self.num_heads = num_heads
        self.attn_weights = nn.Parameter(
            torch.empty([num_heads, in_features, out_features])
        )
        self.non_linearity = getattr(F, non_linearity)
        self.non_linearity_args = non_linearity_args

        self.attn_weight_a = nn.Parameter(torch.empty([2 * out_features]))

        self.context_proj = nn.Linear(in_features, num_groups * in_features)

        self.reset_parameters()

    def reset_parameters(self):
        nn.init.xavier_uniform_(self.phi)
        nn.init.xavier_uniform_(self.attn_weights)
        nn.init.normal_(self.attn_weight_a)
        self.context_proj.reset_parameters()

    def forward(self, x: Tensor, edge_index: Tensor) -> Tuple[Tensor, Tensor]:
        # print(f"X: {x.shape} EI: {edge_index.shape}")
        # Group assignments
        # membership_distribution = Dirichlet(F.softmax(self.phi * x))
        # membership_vectors = membership_distribution.sample()
        membership_vectors = (self.phi @ x.T).T
        # gumbel = Gumbel(torch.tensor([0.0]), torch.tensor([1.0]))
        # gumbel_sample = cast(Tensor, gumbel.sample())
        # group_assignments = torch.exp(
        #     (membership_vectors + gumbel_sample) / self.temperature
        # )
        group_assignments = F.gumbel_softmax(
            membership_vectors, tau=self.temperature, hard=False
        )

        # Attention
        phi_z = group_assignments @ self.phi
        # print(f"PHI_Z {phi_z.shape}")
        # print(f"ATTN_WEIGHTS: {self.attn_weights.shape}")
        # attn_groups = self.attn_weights @ group_assignments
        # attn_nodes = self.attn_weights @ x
        attn_group_proj = torch.einsum("hfo,nf->nho", self.attn_weights, phi_z)
        # print(f"ATTN G PROJ {attn_group_proj.shape}")

        x_proj = torch.einsum("hfo,nf->nho", self.attn_weights, x)  # [N, H, F_out]

        # Compute context vectors and append to input
        N = x.shape[0]
        context_raw = self.context_proj(x).view(
            N, self.num_groups, self.in_features
        )  # [N, G, F]
        z_index = torch.argmax(group_assignments, dim=-1)  # [N]
        # context_vec = context_raw[torch.arange(N), z_index]  # [N, F]

        # Include context in message passing input
        # x_with_context = torch.cat(
        #     [x, context_vec], dim=-1
        # )  # Only used to compute loss
        # Compute context loss
        context_loss = self.compute_context_loss(x, edge_index, context_raw, z_index)

        return (
            self.propagate(
                edge_index,
                x=x_proj.view(N, self.num_heads * self.out_features),
                attn_group=attn_group_proj.view(
                    N, self.num_heads * self.out_features
                ),  # attn_node=attn_nodes removed because x is the same thing
            ),
            context_loss,
        )

    def message(  # pyright: ignore[reportIncompatibleMethodOverride]
        self,
        x_j: Tensor,
        x_i: Tensor,
        index,
        # ptr,
        attn_group_i,
        attn_group_j,
        # attn_node_i,
        # attn_node_j,
    ) -> Tensor:
        x_i = x_i.view(-1, self.num_heads, self.out_features)
        x_j = x_j.view(-1, self.num_heads, self.out_features)
        attn_group_i = attn_group_i.view(-1, self.num_heads, self.out_features)
        attn_group_j = attn_group_j.view(-1, self.num_heads, self.out_features)
        # print(f"X_I, J: {x_i.shape} {x_j.shape}")
        # print(f"Index: {index.shape} Attn Group {attn_group.shape}")
        # print(f"Attn_group_i, j: {attn_group_i.shape}, {attn_group_j.shape}")
        # 1. Concatenate group and node features: [E, H, 2 * F_out]
        group_cat = torch.cat([attn_group_i, attn_group_j], dim=-1)
        node_cat = torch.cat([x_i, x_j], dim=-1)
        # print(f"Group / Node Cat {group_cat.shape}, {node_cat.shape}")

        # 2. Reshape attention vector: [H, 2 * F_out]
        # attn_vec = self.attn_weight_a.view(self.num_heads, -1)  # [H, 2F_out]
        attn_vec = self.attn_weight_a.unsqueeze(0)
        # print(f"Attn vec {attn_vec.shape}")

        # 3. Compute attention scores:
        # Elementwise multiply then sum over features: [E, H]
        lambda_ij = F.leaky_relu(
            (group_cat * attn_vec).sum(dim=-1), self.negative_slope
        )
        alpha_ij = F.leaky_relu((node_cat * attn_vec).sum(dim=-1), self.negative_slope)

        # 4. Normalize with softmax over neighbors
        lambda_ij = softmax(lambda_ij, index)  # [E, H]
        alpha_ij = softmax(alpha_ij, index)  # [E, H]

        # 5. Combine and apply to message (x_j): [E, H, F_out]
        attention = lambda_ij * alpha_ij  # [E, H]
        out = x_j * attention.unsqueeze(-1)  # broadcasting over F_out

        # print(f"Out Shape: {out.shape}")
        out = out.view(-1, self.num_heads * self.out_features)

        return out  # shape [E, H, F_out]

    def update(self, inputs: Tensor) -> Tensor:
        return self.non_linearity(
            inputs.view(-1, self.num_heads, self.out_features).mean(dim=1),
            *self.non_linearity_args,
        )

    def compute_context_loss(
        self,
        x: Tensor,
        edge_index: Tensor,
        context_vectors: Tensor,
        z_index: Tensor,
        num_neg_samples: int = 5,
    ) -> Tensor:
        i, j = edge_index
        q_j = context_vectors[j, z_index[i]]
        h_i = x[i]
        pos = torch.sum(h_i * q_j, dim=-1)
        # print(f"PL {pos}")

        N = x.size(0)
        j_neg = torch.randint(0, N, (num_neg_samples * len(i),), device=x.device)
        z_neg = z_index[i.repeat(num_neg_samples)]
        q_neg = context_vectors[j_neg, z_neg]
        h_i_neg = x[i.repeat(num_neg_samples)]
        neg = torch.sum(h_i_neg * q_neg, dim=-1)

        pos_loss = -F.logsigmoid(pos)
        neg_loss = -F.logsigmoid(-neg).mean()
        return (pos_loss + neg_loss).sum()
