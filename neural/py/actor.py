from collections import deque
from dataclasses import dataclass
from multiprocessing.pool import Pool
from typing import Optional

import numpy as np
import torch
import torch.nn as nn
import torch.nn.functional as F
from numpy._typing import NDArray
from torch_geometric.utils import degree

from utils import (
    ModelConfig,
    RunningNorm,
    Transition,
    build_gats_and_layer_norms,
    compute_node_reward,
    diou_loss,
)


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


@dataclass
class LearningData:
    critic: nn.Module
    keys_1d: list[NDArray]
    key_batch: list[int]


class Actor(nn.Module):
    def __init__(
        self,
        in_dim: int,
        hidden_dims: list[int],
        num_heads: list[int],
        config: ModelConfig,
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
        self.model_config = config

        self.running_reward_norm = RunningNorm()

        dims = [in_dim] + hidden_dims

        self.gats, self.norms, last = build_gats_and_layer_norms(
            dims, num_heads
        )

        for i in range(self.num_layers):
            # self.gats.append(
            #     GATv2Conv(
            #         dims[i] * (num_heads[i - 1] if i > 0 else 1),
            #         dims[i + 1],
            #         heads=num_heads[i],
            #     )
            # )
            # self.norms.append(LayerNorm(dims[i + 1] * num_heads[i]))

            # self.pools.append(TopKPooling(dims[i+1], ratio=pool_ratios[i]))
            self.policy_heads.append(
                nn.Sequential(
                    nn.Linear(dims[i + 1] * num_heads[i], dims[i + 1]),
                    nn.GELU(),
                    nn.Linear(dims[i + 1], dims[i + 1]),
                    nn.GELU(),
                    nn.Linear(dims[i + 1], dims[i + 1] // 2),
                    nn.GELU(),
                    nn.Dropout(p=selection_dropout),
                    nn.Linear(dims[i + 1] // 2, dims[i + 1] // 4),
                    nn.GELU(),
                    nn.Linear(dims[i + 1] // 4, 1),
                )
            )  # Node selection score

        self.reset_parameters()

    def reset_parameters(self):
        for seq in self.policy_heads:  # pyright: ignore
            seq: nn.Sequential
            for layer in seq:
                if isinstance(layer, nn.Linear):
                    nn.init.xavier_normal_(layer.weight)

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

        # prior = self.beta * centrality_prior + (
        #     1 - self.beta
        # ) * self.compute_depths(edge_index, batch)

        # Mix centrality bias with learnable policy
        # return self.alpha * probabilities + (1 - self.alpha) * prior
        return probabilities

    def forward(
        self,
        x: torch.Tensor,
        edge_index: torch.Tensor,
        batch: torch.Tensor,
        persistent_to_batch_id_map: list[tuple[int, int]],
        feature_spans: NDArray,
        learning_data: Optional[LearningData],
        closest_node_pool: Pool,
    ) -> list[Transition] | tuple[torch.Tensor, NDArray, torch.Tensor]:
        if learning_data is None == self.training:
            raise AssertionError("Learning data is None when training")

        graph_has_key = (
            torch.isin(
                torch.unique(batch),
                torch.tensor(learning_data.key_batch, device=batch.device),
            )
            if learning_data is not None
            else None
        )

        no_key_graph_indices = (
            torch.argwhere(~graph_has_key)
            if graph_has_key is not None
            else None
        )

        # Sanity check
        # b_start = torch.unique(torch.clone(batch))
        # start_num_graphs = b_start.shape[0]

        # logp_last = None

        transitions = []
        prev_reward_data = None

        for i in range(self.num_layers):
            # 1) GAT + score
            x = self.gats[i](x, edge_index)
            x = self.norms[i](x)

            x = F.gelu(x)

            # Actor head: outputs one logit per node
            logits = self.policy_heads[i](x)  # shape: (N_nodes,)

            # Bernoulli distribution for each node
            probs = torch.sigmoid(
                logits
            )  # probabilities of selecting each node
            dist = torch.distributions.Bernoulli(probs=probs)

            # Sample binary mask of important nodes
            actions = dist.sample()  # shape: (N_nodes,), entries in {0,1}

            # Log-prob of those actions
            logp = dist.log_prob(actions).sum(dim=-1)  # sum across nodes

            key_batch_associations = keys_stack = None
            if learning_data is not None:
                key_batch_associations = (
                    np.vstack(learning_data.key_batch).squeeze(1)
                    if len(learning_data.key_batch) > 0
                    else np.array([])
                )
                keys_stack = (
                    np.vstack(learning_data.keys_1d)
                    if len(learning_data.keys_1d) > 0
                    else np.array([], dtype=int)
                )

                assert key_batch_associations.shape[0] == keys_stack.shape[0]

            line_spans = feature_spans

            if (
                keys_stack is not None
                and key_batch_associations is not None
                and learning_data is not None
                and no_key_graph_indices is not None
                and graph_has_key is not None
            ):
                diou_l, missing = diou_loss(
                    line_mappings=line_spans,
                    edge_index=edge_index.cpu().numpy(),
                    batch=batch.cpu().numpy(),
                    keys=keys_stack,
                    key_batch_associations=key_batch_associations,
                    k=10,
                    decay_alpha=0.7,
                )

                diou_l = diou_l.to(x.device)

                reward_n = compute_node_reward(
                    diou_l=diou_l,
                    edge_index=edge_index,
                    batch=batch,
                    graph_has_key=graph_has_key,
                    prev_diou_l=prev_reward_data,
                    reward_config=self.model_config.reward,
                )

                prev_reward_data = diou_l
                # self.running_reward_norm.update(reward_n)
                # r = self.running_reward_norm.normalize(reward_n)
                r = reward_n
                # r = reward_g
                # r = (r - r.mean()) / (r.std() + 1e-6)

                predicted_reward = learning_data.critic(
                    x.detach(), edge_index, batch
                )

                assert r.shape == predicted_reward.shape, (
                    f"Reward/value mismatch ({r.shape} != {predicted_reward.shape})"
                )

                r = (
                    (r * actions).sum()
                    - self.model_config.actor.action_reward_scale
                    * actions.sum()
                )
                value = predicted_reward.mean()
                # value = (
                #     (predicted_reward * actions).sum()
                #     - self.model_config.actor.action_reward_scale
                #     * actions.sum()
                # )

                transition = Transition(
                    logp=logp,
                    reward=r,
                    value=value,
                    batch=batch,
                    entropy=dist.entropy().mean(),
                    diou_loss=diou_l,
                )
                transitions.append(transition)
            elif i == self.num_layers - 1:
                assert len(transitions) == 0, (
                    "Transitions added in evaluation run"
                )

                return x, line_spans, batch

        return transitions
