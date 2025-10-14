import csv
import datetime
import gc
import itertools
import multiprocessing
import os
import shutil
from collections import defaultdict
from multiprocessing.pool import Pool
from pathlib import Path
from typing import Literal, Optional

import numpy as np
import psutil
import torch
import torch.nn.functional as F
from diskcache import Cache
from numpy.typing import NDArray
from omegaconf import OmegaConf
from progress.bar import Bar
from sklearn.cluster import HDBSCAN
from torch import Tensor, nn, optim
from torch.utils.data import random_split
from torch.utils.data.dataset import Dataset
from torch_geometric.data import Data
from torch_geometric.loader import DataLoader
from torch_geometric.nn import global_mean_pool

from actor import Actor, LearningData
from critic import MergeCritic
from embedding import EmbeddingPredictor, GatGraphEmbedding
from utils import (
    DATA_WORKERS,
    MAX_POOL_TASKS,
    AverageAccumulator,
    LossConfig,
    Metrics,
    ModelConfig,
    actor_critic_loss,
    auxiliary_embedding_loss_helper,
    build_gats_and_layer_norms_with_pooling,
    diou_loss,
    focal_loss,
    get_projections_and_targets_for_each_batch,
    lerp,
    make_persistent_to_batch_id_map,
    mine_triplets_from_aux_embeddings,
    persistent_to_batch_id_map_and_keys,
    try_load_schema_from_file,
)

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


NUM_EPISODES = 10
NUM_EMBEDDING_EPISODES = 50


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


def calculate_node_inside_ranges(
    line_associations: Tensor,
    ranges: Tensor,
    mode: Literal["any", "sum", "all"] = "all",
) -> Tensor:
    """
    line_associations: [N, 2]
    ranges: [M, 2]
    is_in_any: bool if true, only return how many ranges each node is in

    Returns [N*M, 1], the portion of the range they take up, 0 if outside
    """

    assert torch.all(line_associations[:, 1] - line_associations[:, 0] > 0), (
        "line associations have zero-length items!"
    )
    assert torch.all(ranges[:, 1] - ranges[:, 0] > 0), (
        "ranges have zero-length items!"
    )

    lines = torch.repeat_interleave(line_associations, ranges.shape[0], dim=0)
    r = ranges.repeat((line_associations.shape[0], 1))
    assert lines.shape == r.shape, (
        f"lines.shape={lines.shape}, r.shape={r.shape}"
    )

    inside = (lines[:, 0] >= r[:, 0]) & (lines[:, 1] <= r[:, 1])
    if mode == "any":
        return torch.zeros(
            line_associations.shape[0],
            device=line_associations.device,
            dtype=torch.long,
        ).scatter_add(
            0,
            torch.repeat_interleave(
                torch.arange(
                    line_associations.shape[0], device=line_associations.device
                ),
                ranges.shape[0],
            ),
            inside.long(),
        )

    proportion = (lines[:, 1] - lines[:, 0]) / (r[:, 1] - r[:, 0])
    assert proportion.shape == inside.shape, (
        f"proportion.shape={proportion.shape}, inside.shape={inside.shape}"
    )

    proportion[~inside] = 0
    assert (
        proportion.shape[0] == line_associations.shape[0] * ranges.shape[0]
    ), (
        f"proportion.shape={proportion.shape}, line_associations.shape={line_associations.shape}, ranges.shape={ranges.shape}"
    )

    if mode == "sum":
        return torch.zeros(
            line_associations.shape[0], device=line_associations.device
        ).scatter_add(
            0,
            torch.repeat_interleave(
                torch.arange(
                    line_associations.shape[0], device=line_associations.device
                ),
                ranges.shape[0],
            ),
            proportion,
        )

    return proportion


def clustering_loss(
    x: Tensor,
    batch: Tensor,
    line_associations: Tensor,
    persistent_to_batch_id_map: dict[int, int],
    keys: dict[tuple[int, int], NDArray],
    negative_k: int = 3,
):
    """
    Ensures graphs that have keys are clustered closer together.
    Positive clustering for nodes within key ranges,
    negative clustering for nodes outside key ranges.
    """

    # Filter keys to only those present in this batch
    valid_keys = [
        (k, torch.as_tensor(v, device=x.device, dtype=torch.long))
        for k, v in keys.items()
        if k[0] in persistent_to_batch_id_map
        and k[1] in persistent_to_batch_id_map
    ]

    print(f"LA: {line_associations.shape[0]} X: {x.shape}")

    if not valid_keys:
        print("WARNING: No keys present for batch")
        return torch.tensor(0.0, device=x.device)

    losses = []
    for k, v in valid_keys:
        left_ranges, right_ranges = v[:, [0, 2]], v[:, [1, 3]]

        left_mask = batch == persistent_to_batch_id_map[k[0]]
        right_mask = batch == persistent_to_batch_id_map[k[1]]

        assert torch.any(left_mask), (
            f"No nodes in batch for key {k[0]} (batch_id={persistent_to_batch_id_map[k[0]]})"
        )
        assert torch.any(right_mask), (
            f"No nodes in batch for key {k[1]} (batch_id={persistent_to_batch_id_map[k[1]]})"
        )

        left_nodes, right_nodes = x[left_mask], x[right_mask]
        left_assoc, right_assoc = (
            line_associations[left_mask],
            line_associations[right_mask],
        )

        # Expand ranges until both have something
        expansion_amount = 0
        while True:
            lr = left_ranges
            lr[:, 0] -= expansion_amount
            lr[:, 1] += expansion_amount
            rr = right_ranges
            rr[:, 0] -= expansion_amount
            rr[:, 1] += expansion_amount
            relevant_left = (
                calculate_node_inside_ranges(left_assoc, lr, mode="any") > 0
            )
            relevant_right = (
                calculate_node_inside_ranges(right_assoc, rr, mode="any") > 0
            )

            if torch.any(relevant_left) and torch.any(relevant_right):
                break
            else:
                expansion_amount += 1

        pos_left, pos_right = (
            left_nodes[relevant_left],
            right_nodes[relevant_right],
        )
        neg_left, neg_right = (
            left_nodes[~relevant_left],
            right_nodes[~relevant_right],
        )

        if pos_left.numel() == 0 or pos_right.numel() == 0:
            print(
                f"WARNING: No positive nodes found for graph pair {persistent_to_batch_id_map[k[0]], persistent_to_batch_id_map[k[1]]}"
            )
            continue

        # --- Vectorized positive pairing ---
        # All pairs between pos_left and pos_right
        pl = pos_left.unsqueeze(1).expand(
            -1, pos_right.size(0), -1
        )  # (L, R, D)
        pr = pos_right.unsqueeze(0).expand(
            pos_left.size(0), -1, -1
        )  # (L, R, D)

        # flatten into (L*R, D)
        pl = pl.reshape(-1, x.size(-1))
        pr = pr.reshape(-1, x.size(-1))

        # symmetric anchors/positives
        anchors = [pl, pr]
        positives = [pr, pl]

        # --- Negative sampling ---
        def sample_negatives(pool: Tensor, count: int) -> Tensor:
            if pool.numel() == 0:
                rand_idx = torch.randint(
                    0, x.size(0), (count,), device=x.device
                )
                return x[rand_idx]
            else:
                idx = torch.randint(0, pool.size(0), (count,), device=x.device)
                return pool[idx]

        n_total = pl.size(0)  # same for pr
        neg_r = sample_negatives(neg_right, n_total * negative_k)
        neg_l = sample_negatives(neg_left, n_total * negative_k)

        negatives = [
            neg_r.view(n_total, negative_k, -1).mean(1),
            neg_l.view(n_total, negative_k, -1).mean(1),
        ]

        anchors = torch.cat(anchors, dim=0)
        positives = torch.cat(positives, dim=0)
        negatives = torch.cat(negatives, dim=0)

        # trim negatives if oversampled
        negatives = negatives[: anchors.size(0)]

        # Divide by expansion amount to reward larger spans
        losses.append(
            F.triplet_margin_loss(
                anchors, positives, negatives, margin=1.0, p=2
            )
            / (np.log1p(expansion_amount) + 1)
        )

    if not losses:
        print("WARNING: No anchors present for batch")
        return torch.tensor(0.0, device=x.device)

    return torch.stack(losses).mean()


def relevance_loss(
    x: Tensor,
    batch: Tensor,
    line_associations: Tensor,
    persistent_to_batch_id_map: dict[int, int],
    keys_1d: list[NDArray],
    keys_batch: list[int],
    config: LossConfig,
):
    """
    Calculates loss relevant to the keys
    Finds nodes where key ranges overlap exactly
    """

    scores = torch.zeros((x.shape[0]), device=x.device)
    for range, graph in zip(keys_1d, keys_batch):
        mask = batch == graph
        masked_scores = calculate_node_inside_ranges(
            line_associations[mask],
            torch.tensor(
                range, device=line_associations.device, dtype=torch.long
            ),
            mode="sum",
        )
        scores[mask] += masked_scores

    # Weight loss based on span width, so bigger spans contribute more to the loss and are more important to get correct
    weights = line_associations[:, 1] - line_associations[:, 0]  # span length
    loss = (
        focal_loss(
            x, scores, alpha=config.focal_alpha, gamma=config.focal_gamma
        )
        * torch.log1p(weights)
    ).mean()
    return loss


class TraditionalModel(nn.Module):
    def __init__(
        self,
        in_dim: int,
        hidden_dims: list[int],
        num_heads: list[int],
        config: ModelConfig,
    ) -> None:
        super().__init__()
        dims = [in_dim] + hidden_dims

        self.num_layers = len(num_heads)

        self.gats, self.norms, self.pools, out_dim = (
            build_gats_and_layer_norms_with_pooling(
                dims,
                num_heads,
                config.pool_every_n,
                config.pool_threshold,
                config.pool_offset,
                config.pool_multiplier,
            )
        )

        self.importance_head = nn.Sequential(
            nn.Linear(out_dim, out_dim),
            nn.GELU(),
            nn.Linear(out_dim, out_dim),
            nn.GELU(),
            nn.Dropout(p=config.actor.selection_dropout),
            nn.Linear(out_dim, out_dim // 2),
            nn.GELU(),
            nn.Linear(out_dim // 2, out_dim // 4),
            nn.GELU(),
            nn.Linear(out_dim // 4, 1),
            nn.Softplus(),
        )

        self.latent_head = nn.Sequential(
            nn.Linear(out_dim, out_dim),
            nn.Dropout(p=config.actor.selection_dropout),
            nn.GELU(),
            nn.Linear(out_dim, out_dim),
            nn.GELU(),
            nn.Linear(out_dim, out_dim),
        )

        self.config = config
        self.reset_parameters()

    def reset_parameters(self):
        for layer in self.importance_head:
            if isinstance(layer, nn.Linear):
                nn.init.xavier_normal_(layer.weight)

        for layer in self.latent_head:
            if isinstance(layer, nn.Linear):
                nn.init.xavier_normal_(layer.weight)

    def forward(
        self,
        x: torch.Tensor,
        edge_index: torch.Tensor,
        batch: torch.Tensor,
        persistent_to_batch_id_map: list[tuple[int, int]],
        feature_spans: Tensor,
    ):
        print(f"start: {x.shape}")
        for i in range(self.num_layers):
            x = self.gats[i](x, edge_index)
            # x = self.norms[i](x)
            x = F.gelu(x)

            if p := self.pools[i]:
                print(f"POOL at {i}")
                x, edge_index, _, batch, perm, _ = p(
                    x=x, edge_index=edge_index, batch=batch
                )
                feature_spans = feature_spans[perm]

            print(x.shape)

        return (
            self.latent_head(x),
            self.importance_head(x).squeeze(-1),
            edge_index,
            batch,
            feature_spans,
        )


def eval_traditional(
    model: TraditionalModel, embedder: nn.Module, dataset: Dataset, keys
):
    embedder.eval()
    model.eval()

    eval_data = DataLoader(
        dataset,  # type: ignore
        batch_size=25,
        shuffle=False,
        num_workers=DATA_WORKERS,
    )

    xs = []
    spanss = []
    relevances = []
    persistent_idss = []
    with torch.no_grad():
        for batch in eval_data:
            p_to_b_id_dict = dict(make_persistent_to_batch_id_map(batch))

            batch_to_persistent = torch.tensor(
                list(map(int, batch.key_index))
            ).to(DEVICE)

            batch = batch.to(DEVICE)

            embs = embedder(batch.x, batch.edge_index)

            latent, relevance, _, b, feature_spans = model(
                embs,
                batch.edge_index,
                batch.batch,
                p_to_b_id_dict,
                batch.lines,
            )

            assert (
                latent.shape[0] == relevance.shape[0] == feature_spans.shape[0]
            )

            persistent_ids = batch_to_persistent[b]
            xs.append(latent)
            spanss.append(feature_spans)
            relevances.append(relevance)
            persistent_idss.append(persistent_ids)

    x = torch.cat(xs, dim=0)
    spans = torch.cat(spanss, dim=0).cpu().numpy()
    relevance = torch.cat(relevances, dim=0).cpu().numpy()
    persistent_ids = torch.cat(persistent_idss, dim=0).cpu().numpy()

    hdbscan = HDBSCAN(cluster_selection_method="leaf")
    hdbscan.fit(x.cpu().numpy())

    assert len(hdbscan.labels_) == len(spans)
    detections = defaultdict(set)
    for i, label in enumerate(hdbscan.labels_):
        row = spans[i]
        detections[label].add((persistent_ids[i], row[0], row[1], relevance[i]))

    for k, v in detections.items():
        print(
            "These items are not members of any cluster"
            if k == -1
            else f"Items in cluster {k}"
        )

        graphs_present = set(map(lambda x: x[0], v))
        successful_graphs = {}
        if keys is not None:
            for k_k, k_v in keys.items():
                a, b = k_k
                if a in graphs_present and b in graphs_present:
                    print(k_v)
                    successful_graphs[a] = (
                        b,
                        np.hstack([k_v[:, 0].T, k_v[:, 2].T]),
                    )
                    successful_graphs[b] = (
                        a,
                        np.hstack([k_v[:, 1].T, k_v[:, 3].T]),
                    )

        for graph, start, end, relevance in v:
            print(
                f"Graph: {graph}, Start: {start}, End: {end}, Relevance: {relevance} {f'✅ {successful_graphs[graph]}' if graph in successful_graphs else ''}"
            )

    print(
        f"The provided dataset yielded {len(detections.keys()) - 1} positive clusters"
    )

    # return detections


def eval(actor: Actor, embedder: nn.Module, dataset: Dataset):
    embedder.eval()
    actor.eval()

    eval_data = DataLoader(
        dataset,  # type: ignore
        batch_size=25,
        shuffle=False,
        num_workers=DATA_WORKERS,
    )

    xs = []
    spanss = []
    persistent_idss = []
    with torch.no_grad():
        with Pool(
            DATA_WORKERS, maxtasksperchild=MAX_POOL_TASKS
        ) as closest_node_pool:
            for batch in eval_data:
                persistent_to_batch_id_map = make_persistent_to_batch_id_map(
                    batch
                )
                batch_to_persistent = torch.tensor(
                    list(map(int, batch.key_index))
                ).to(DEVICE)

                selected_spans = batch.lines
                batch = batch.to(DEVICE)

                embs = embedder(batch.x, batch.edge_index)

                x, line_spans, batch_idxs = actor(
                    embs,
                    batch.edge_index,
                    batch.batch,
                    persistent_to_batch_id_map=persistent_to_batch_id_map,
                    feature_spans=selected_spans,
                    learning_data=None,
                    closest_node_pool=closest_node_pool,
                )

                persistent_ids = batch_to_persistent[batch_idxs].cpu().numpy()
                xs.append(x)
                spanss.append(line_spans)
                persistent_idss.append(persistent_ids)

    x = torch.cat(xs, dim=0)
    spans = np.concat(spanss, axis=0)
    persistent_ids = np.concatenate(persistent_idss, axis=0)

    hdbscan = HDBSCAN(cluster_selection_method="leaf")
    hdbscan.fit(x.cpu().numpy())

    detections = defaultdict(set)
    for i, label in enumerate(hdbscan.labels_):
        row = spans[i]
        detections[label].add((persistent_ids[i], row[0], row[1]))

    return detections


def run_model_batch(
    embedder: nn.Module,
    model: TraditionalModel,
    batch,
    keys,
    config: ModelConfig,
) -> Tensor:
    persistent_to_batch_id_map, keys_1d, key_batch = (
        persistent_to_batch_id_map_and_keys(batch, keys)
    )

    p_to_b_id_dict = dict(persistent_to_batch_id_map)

    batch = batch.to(DEVICE)

    embs = embedder(batch.x, batch.edge_index)

    latent, relevance, _, b, feature_spans = model(
        embs,
        batch.edge_index,
        batch.batch,
        persistent_to_batch_id_map,
        batch.lines,
    )

    assert latent.shape[0] == relevance.shape[0] == feature_spans.shape[0]

    l_loss = clustering_loss(
        latent,
        b,
        feature_spans,
        p_to_b_id_dict,
        keys,
    )
    r_loss = relevance_loss(
        relevance,
        b,
        feature_spans,
        p_to_b_id_dict,
        keys_1d,
        key_batch,
        config=config.loss,
    )

    d_loss = diou_loss(
        batch.lines.cpu().numpy(),
        batch.edge_index.cpu().numpy(),
        batch.batch.cpu().numpy(),
        np.vstack(keys_1d),
        np.vstack(key_batch),
        k=config.embedding_loss_top_k,
    )[0].to(DEVICE)

    aux_emb_loss = auxiliary_embedding_loss_helper(
        embs,
        batch=batch.batch,
        diou_loss=d_loss,
        persistent_to_batch_id_map=p_to_b_id_dict,
        keys=keys,
        config=config,
    )

    aux_emb_loss = (
        aux_emb_loss
        if aux_emb_loss is not None
        else torch.tensor(0.0, device=d_loss.device)
    )

    print(
        f"Losses: Latent={l_loss.item()}, Relevance={r_loss.item()}, Auxiliary={aux_emb_loss.item()}"
    )

    loss_weights = config.loss
    return (
        l_loss * loss_weights.latent
        + r_loss * loss_weights.relevance
        + aux_emb_loss * loss_weights.auxiliary
    )


def train_traditional(
    dataset: Dataset,
    embedder: nn.Module,
    model: TraditionalModel,
    model_optim: optim.Optimizer,
    start_epoch: int,
    episodes: int,
    keys: dict[tuple[int, int], NDArray],
    artifact_dir: Path,
    train_checkpoints_dir: Path,
    config: ModelConfig,
    keep_last_n_old_checkpoints: int = 3,
):
    train_set, val_set, test_set = random_split(
        dataset,
        make_splits(len(dataset)),  # type: ignore
    )

    train_data = DataLoader(
        train_set,  # type: ignore
        batch_size=config.batch_sizes.train,
        shuffle=True,
        num_workers=DATA_WORKERS,
    )
    val_data = DataLoader(
        val_set,  # type: ignore
        batch_size=config.batch_sizes.val,
        shuffle=True,
        num_workers=DATA_WORKERS,
    )
    test_data = DataLoader(
        test_set,  # type: ignore
        batch_size=config.batch_sizes.test,
        num_workers=DATA_WORKERS,
    )
    for epoch in range(start_epoch, episodes):
        print(f"\n⏰ EPOCH {epoch}")
        embedder.train()
        model.train()

        total_model_loss = 0.0

        # Train
        for batch in train_data:
            print(f"\n\nNEXT BATCH | EPOCH {epoch} ->")

            model_loss = run_model_batch(embedder, model, batch, keys, config)

            model_optim.zero_grad()
            model_loss.backward()
            model_optim.step()

            model_loss = model_loss.item()
            total_model_loss += model_loss

            print("*" * 10 + f"\nBatch Loss:\nModel: {model_loss}\n" + "*" * 10)

            del batch, model_loss
            cleanup()

        print(
            f"~~\nTotal Training Loss for Epoch {epoch}:\nModel: {total_model_loss}\n~~"
        )

        train_checkpoints_dir_latest = train_checkpoints_dir / f"{epoch}"
        os.makedirs(train_checkpoints_dir_latest)
        torch.save(model, train_checkpoints_dir_latest / "model.pt")
        torch.save(embedder, train_checkpoints_dir_latest / "embedder.pt")

        for epoch_del in range(epoch - keep_last_n_old_checkpoints):
            if os.path.exists(train_checkpoints_dir / f"{epoch_del}"):
                shutil.rmtree(train_checkpoints_dir / f"{epoch_del}")

        total_model_loss = 0.0
        # Val
        with torch.no_grad():
            for batch in val_data:
                model_loss = run_model_batch(
                    embedder, model, batch, keys, config
                )

                total_model_loss += model_loss.item()

                print(
                    "%" * 10
                    + f"\nValidation =====\nBatch Loss:\nModel: {model_loss}\n"
                    + "%" * 10
                )

                del batch, model_loss
                cleanup()

        print(
            f"~~\nTotal Validation Loss for Epoch {epoch}:\nModel: {total_model_loss}\n~~"
        )

    # Test
    total_model_loss = 0.0
    with torch.no_grad():
        for batch in test_data:
            model_loss = run_model_batch(embedder, model, batch, keys, config)

            total_model_loss += model_loss.item()

            print(
                "=" * 10
                + f"\nTest=====\nBatch Loss:\nModel: {model_loss}\n"
                + "=" * 10
            )

            del batch, model_loss
            cleanup()

    print("Training complete")
    torch.save(embedder, artifact_dir / "embedder_tuned.pt")
    torch.save(model, artifact_dir / "model.pt")


def train(
    dataset: Dataset,
    embedder: nn.Module,
    actor: Actor,
    critic: nn.Module,
    actor_optim: optim.Optimizer,
    critic_optim: optim.Optimizer,
    start_epoch: int,
    episodes: int,
    keys: dict[tuple[int, int], NDArray],
    artifact_dir: Path,
    train_checkpoints_dir: Path,
    config: ModelConfig,
    gamma=0.99,
    lam=0.95,
    entropy_coefs: tuple[float, float] = (0.01, 0.001),
    keep_last_n_old_checkpoints: int = 3,
):
    train_set, val_set, test_set = random_split(
        dataset,
        make_splits(len(dataset)),  # type: ignore
    )

    train_data = DataLoader(
        train_set,  # type: ignore
        batch_size=config.batch_sizes.train,
        shuffle=True,
        num_workers=DATA_WORKERS,
    )
    val_data = DataLoader(
        val_set,  # type: ignore
        batch_size=config.batch_sizes.val,
        shuffle=True,
        num_workers=DATA_WORKERS,
    )
    test_data = DataLoader(
        test_set,  # type: ignore
        batch_size=config.batch_sizes.test,
        num_workers=DATA_WORKERS,
    )

    train_f = open(artifact_dir / "train_loss.csv", "w")
    val_f = open(artifact_dir / "val_loss.csv", "w")

    train_csv = csv.writer(train_f)
    val_csv = csv.writer(val_f)

    ROWS = [
        "Actor Loss",
        "Critic Loss",
        "Mean Reward",
        "Reward Standard Deviation",
        "Mean Value",
        "Value Standard Deviation",
        "Mean Absolute Reward",
        "Mean Absolute Value",
        "Mean Entropy",
        "Mean Absolute Entropy",
    ]
    train_csv.writerow(ROWS)
    val_csv.writerow(ROWS)

    def write_metrics(csv, metrics: Metrics):
        csv.writerow(
            [
                metrics.actor_loss.item(),
                metrics.critic_loss.item(),
                metrics.reward.mean().item(),
                metrics.reward.std().item(),
                metrics.value.mean().item(),
                metrics.value.std().item(),
                metrics.reward.abs().mean().item(),
                metrics.value.abs().mean().item(),
                metrics.entropy.mean().item(),
                metrics.entropy.abs().mean().item(),
            ]
        )

    with Pool(
        DATA_WORKERS, maxtasksperchild=MAX_POOL_TASKS
    ) as closest_node_pool:
        for epoch in range(start_epoch, episodes):
            print(f"\n⏰ EPOCH {epoch}")
            embedder.train()
            actor.train()
            critic.train()

            entropy = lerp(entropy_coefs[0], entropy_coefs[1], epoch / episodes)

            total_actor_loss = total_critic_loss = 0.0

            # Train
            for batch in train_data:
                print(f"\n\nNEXT BATCH | EPOCH {epoch} ->")

                persistent_to_batch_id_map, keys_1d, key_batch = (
                    persistent_to_batch_id_map_and_keys(batch, keys)
                )

                p_to_b_id_dict = dict(persistent_to_batch_id_map)

                selected_spans = batch.lines
                batch = batch.to(DEVICE)

                embs = embedder(batch.x, batch.edge_index)

                learning_data = LearningData(
                    critic=critic,
                    keys_1d=keys_1d,
                    key_batch=key_batch,
                )

                transitions = actor(
                    embs,
                    batch.edge_index,
                    batch.batch,
                    persistent_to_batch_id_map,
                    selected_spans,
                    learning_data=learning_data,
                    closest_node_pool=closest_node_pool,
                )
                metrics = actor_critic_loss(
                    transitions,
                    gamma=gamma,
                    lam=lam,
                    entropy_coef=entropy,
                    critic_loss_fn=config.critic_loss_fn,
                )

                aux_emb_loss = auxiliary_embedding_loss_helper(
                    embs,
                    batch=batch,
                    diou_loss=torch.stack(
                        [t.diou_loss for t in transitions]
                    ).mean(dim=0),
                    persistent_to_batch_id_map=p_to_b_id_dict,
                    keys=keys,
                    config=config,
                )

                actor_loss = metrics.actor_loss
                if aux_emb_loss is not None:
                    actor_loss -= aux_emb_loss
                critic_loss = metrics.critic_loss

                actor_optim.zero_grad()
                actor_loss.backward()
                actor_optim.step()

                critic_optim.zero_grad()
                critic_loss.backward()
                critic_optim.step()

                total_actor_loss += actor_loss.item()
                total_critic_loss += critic_loss.item()

                write_metrics(train_csv, metrics)

                print(
                    "*" * 10
                    + f"\nBatch Loss:\nActor: {actor_loss}\nCritic: {critic_loss}\n"
                    + "*" * 10
                )

                del batch, actor_loss, critic_loss, aux_emb_loss, metrics
                cleanup()

            print(
                f"~~\nTotal Training Loss for Epoch {epoch}:\nActor: {total_actor_loss}\nCritic: {total_critic_loss}\n~~"
            )

            train_checkpoints_dir_latest = train_checkpoints_dir / f"{epoch}"
            os.makedirs(train_checkpoints_dir_latest)
            torch.save(actor, train_checkpoints_dir_latest / "actor.pt")
            torch.save(critic, train_checkpoints_dir_latest / "critic.pt")
            torch.save(embedder, train_checkpoints_dir_latest / "embedder.pt")

            for epoch_del in range(epoch - keep_last_n_old_checkpoints):
                if os.path.exists(train_checkpoints_dir / f"{epoch_del}"):
                    shutil.rmtree(train_checkpoints_dir / f"{epoch_del}")

            total_actor_loss = total_critic_loss = 0.0

            # Val
            with torch.no_grad():
                for batch in val_data:
                    persistent_to_batch_id_map, keys_1d, key_batch = (
                        persistent_to_batch_id_map_and_keys(batch, keys)
                    )

                    selected_spans = batch.lines
                    batch = batch.to(DEVICE)

                    embs = embedder(batch.x, batch.edge_index)

                    learning_data = LearningData(
                        critic=critic, key_batch=key_batch, keys_1d=keys_1d
                    )

                    transitions = actor(
                        embs,
                        batch.edge_index,
                        batch.batch,
                        persistent_to_batch_id_map,
                        selected_spans,
                        learning_data=learning_data,
                        closest_node_pool=closest_node_pool,
                    )

                    metrics = actor_critic_loss(
                        transitions,
                        gamma=gamma,
                        lam=lam,
                        entropy_coef=entropy,
                        critic_loss_fn=config.critic_loss_fn,
                    )

                    aux_emb_loss = auxiliary_embedding_loss_helper(
                        embs,
                        batch=batch,
                        diou_loss=torch.stack(
                            [t.diou_loss for t in transitions]
                        ).mean(dim=0),
                        persistent_to_batch_id_map=dict(
                            persistent_to_batch_id_map
                        ),
                        keys=keys,
                        config=config,
                    )

                    actor_loss = metrics.actor_loss
                    if aux_emb_loss is not None:
                        actor_loss += aux_emb_loss
                    critic_loss = metrics.critic_loss
                    critic_loss = metrics.critic_loss

                    total_actor_loss += actor_loss.item()
                    total_critic_loss += critic_loss.item()

                    write_metrics(train_csv, metrics)

                    print(
                        "%" * 10
                        + f"\nValidation =====\nBatch Loss:\nActor: {actor_loss}\nCritic: {critic_loss}\n"
                        + "%" * 10
                    )

                    del batch, actor_loss, critic_loss, aux_emb_loss, metrics
                    cleanup()

            print(
                f"~~\nTotal Validation Loss for Epoch {epoch}:\nActor: {total_actor_loss}\nCritic: {total_critic_loss}\n~~"
            )

        # Test
        total_actor_loss = total_critic_loss = 0.0
        test_actor_losses = test_critic_losses = []
        with torch.no_grad():
            for batch in test_data:
                persistent_to_batch_id_map, keys_1d, key_batch = (
                    persistent_to_batch_id_map_and_keys(batch, keys)
                )

                selected_spans = batch.lines
                batch = batch.to(DEVICE)

                embs = embedder(batch.x, batch.edge_index)

                learning_data = LearningData(
                    critic=critic, keys_1d=keys_1d, key_batch=key_batch
                )

                transitions = actor(
                    embs,
                    batch.edge_index,
                    batch.batch,
                    persistent_to_batch_id_map,
                    selected_spans,
                    learning_data=learning_data,
                    closest_node_pool=closest_node_pool,
                )
                metrics = actor_critic_loss(
                    transitions,
                    gamma=gamma,
                    lam=lam,
                    entropy_coef=entropy_coefs[1],
                    critic_loss_fn=config.critic_loss_fn,
                )

                aux_emb_loss = auxiliary_embedding_loss_helper(
                    embs,
                    batch=batch,
                    diou_loss=torch.stack(
                        [t.diou_loss for t in transitions]
                    ).mean(dim=0),
                    persistent_to_batch_id_map=dict(persistent_to_batch_id_map),
                    keys=keys,
                    config=config,
                )

                actor_loss = metrics.actor_loss
                if aux_emb_loss is not None:
                    actor_loss += aux_emb_loss * config.embedding_loss_weight
                critic_loss = metrics.critic_loss

                total_actor_loss += actor_loss.item()
                test_actor_losses.append(actor_loss.item())
                total_critic_loss += critic_loss.item()
                test_critic_losses.append(critic_loss.item())

                print(
                    "=" * 10
                    + f"\nTest =====\nBatch Loss:\nActor: {actor_loss}\nCritic: {critic_loss}\n"
                    + "=" * 10
                )

                del batch, actor_loss, critic_loss, aux_emb_loss, metrics
                cleanup()

    train_f.close()
    val_f.close()

    with open(artifact_dir / "test_loss.csv", "w") as f:
        csv.writer(f).writerows(
            [["Actor", "Critic"]]
            + list(zip(test_actor_losses, test_critic_losses))
        )

    print(
        f"~~\nTotal Test Loss:\nActor: {total_actor_loss}\nCritic: {total_critic_loss}\n~~"
    )

    print("Training complete")
    torch.save(embedder, artifact_dir / "embedder_tuned.pt")
    torch.save(actor, artifact_dir / "actor.pt")


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
        num_workers=DATA_WORKERS,
    )
    val_data = DataLoader(val_set, batch_size=100, num_workers=DATA_WORKERS)  # type: ignore
    test_data = DataLoader(test_set, batch_size=100, num_workers=DATA_WORKERS)  # type: ignore

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

            triplets = mine_triplets_from_aux_embeddings(
                batch.x, ast_embeddings, top_k=top_k
            )

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
                triplets = mine_triplets_from_aux_embeddings(
                    batch.x, ast_embeddings, top_k=top_k
                )

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
            triplets = mine_triplets_from_aux_embeddings(
                batch.x, ast_embeddings, top_k=top_k
            )
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


def test_embeddings(
    embedder: nn.Module, dataset: Dataset, artifact_dir: Path, perplexity: int
):
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

    data = DataLoader(
        dataset,  # type: ignore
        batch_size=50,
        shuffle=True,
        num_workers=DATA_WORKERS,
    )
    for i, batch in enumerate(data):
        batch = batch.to(DEVICE)
        print(f"Batch {i}")
        embeddings = embedder(batch.x, batch.edge_index)

        labels = batch.language[batch.batch].detach().cpu().numpy()

        labels = np.array(["C", "C++", "Java", "Python"])[labels]

        tsne = TSNE(n_components=3, perplexity=perplexity)
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
    config_dir: str,
    ast_embeddings: NDArray,
    mode: Literal["train", "embed", "embed-test", "eval"] = "train",
    top_k: int = 3,
    device: Optional[str] = None,
    restart_from_checkpoint: bool = True,
):
    try:
        multiprocessing.set_start_method("forkserver", force=True)
    except ValueError:
        # ValueError: "forkserver" not available on this platform
        print(
            'WARNING: "forkserver" start method not available on this platform'
        )

    if "file_system" in torch.multiprocessing.get_all_sharing_strategies():
        torch.multiprocessing.set_sharing_strategy("file_system")
    else:
        print('WARNING: "file_system" sharing not available on this platform')

    global DEVICE
    if device:
        DEVICE = torch.device(device)

    torch.manual_seed(0xDEADBEEF)
    print(
        f"✅ Python initialization successful. Beginning training on device {DEVICE}..."
    )
    artifact_dir: Path = Path(artifact_dir)
    config_dir: Path = Path(config_dir)
    # print(f"KEYS\n\n{keys}\n\n")
    # dataset = generate_dataset(edges, features, feature_spans, languages)
    languages = dataset.languages
    keys = dataset.keys
    print(f"The provided dataset has {len(keys.keys())} keys")
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
                artifact_dir / "embeddings.pt",
                weights_only=False,
                map_location=DEVICE,
            )
            with torch.no_grad():
                test_embeddings(
                    embedder=embedder,
                    dataset=dataset,
                    artifact_dir=artifact_dir,
                    perplexity=25,
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
                    num_episodes=NUM_EMBEDDING_EPISODES,
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
                print("📡 Loading embeddings model into memory")
                embedder = torch.load(
                    artifact_dir / "embeddings.pt",
                    weights_only=False,
                    map_location=DEVICE,
                )

            schema = try_load_schema_from_file(config_dir / "config.yml")

            artifact_suffix = (
                datetime.datetime.now()
                .replace(microsecond=0)
                .isoformat()
                .replace(":", "_")
            )

            os.makedirs(artifact_dir / artifact_suffix)

            with open(
                artifact_dir / artifact_suffix / "config.yml", mode="w"
            ) as f:
                OmegaConf.save(schema, f)

            critic = MergeCritic(
                in_dim=embedding_dim // 2 * schema.actor.num_heads,
                hidden_dim_generator=lambda d: d // 2,
                num_layers=schema.critic_layers,
                p_dropout=schema.critic_dropout,
            ).to(DEVICE)
            actor = Actor(
                in_dim=embedding_dim,
                hidden_dims=[embedding_dim // 2] * schema.actor.num_layers
                + [embedding_dim],
                num_heads=[schema.actor.num_heads] * schema.actor.num_layers
                + [1],
                config=schema,  # pyright: ignore
                # pool_ratios=[0.5, 0.6, 0.8, 0.8, 0.8],
                alpha=schema.actor.alpha,
                beta=schema.actor.beta,
                selection_dropout=schema.actor.selection_dropout,
            ).to(DEVICE)
            # critic = Critic(in_dim=features[0].shape[1], hidden_dim=20, num_heads=8).to(DEVICE)
            model = TraditionalModel(
                in_dim=embedding_dim,
                hidden_dims=[embedding_dim] * schema.actor.num_layers
                + [embedding_dim],
                num_heads=[schema.actor.num_heads] * schema.actor.num_layers
                + [1],
                config=schema,
            ).to(DEVICE)

            start_epoch = 0
            train_checkpoints_dir = artifact_dir / "train_checkpoints"
            if os.path.exists(train_checkpoints_dir):
                files = os.listdir(train_checkpoints_dir)
                sorted_files = list(
                    sorted(
                        map(
                            lambda d: int(d),
                            filter(
                                lambda f: os.path.isdir(
                                    train_checkpoints_dir / f
                                ),
                                files,
                            ),
                        ),
                    )
                )
                if restart_from_checkpoint and sorted_files:
                    last_complete_epoch = str(sorted_files[-1])
                    start_epoch = int(last_complete_epoch) + 1

                    print(f"🏃‍➡️ Restarting from epoch {start_epoch}")

                    schema = try_load_schema_from_file(
                        train_checkpoints_dir / "config.yml"
                    )

                    embedder = torch.load(
                        train_checkpoints_dir
                        / last_complete_epoch
                        / "embedder.pt",
                        weights_only=False,
                        map_location=DEVICE,
                    )
                    model = torch.load(
                        train_checkpoints_dir
                        / last_complete_epoch
                        / "model.pt",
                        weights_only=False,
                        map_location=DEVICE,
                    )
                    # actor = torch.load(
                    #     train_checkpoints_dir
                    #     / last_complete_epoch
                    #     / "actor.pt",
                    #     weights_only=False,
                    #     map_location=DEVICE,
                    # )
                    # critic = torch.load(
                    #     train_checkpoints_dir
                    #     / last_complete_epoch
                    #     / "critic.pt",
                    #     weights_only=False,
                    #     map_location=DEVICE,
                    # )
                else:
                    shutil.rmtree(train_checkpoints_dir)
                    os.makedirs(train_checkpoints_dir)
            else:
                os.makedirs(train_checkpoints_dir)

            with open(train_checkpoints_dir / "config.yml", mode="w") as f:
                OmegaConf.save(schema, f)

            train_traditional(
                dataset,
                embedder=embedder,
                model=model,
                model_optim=optim.Adam(
                    model.parameters(),
                    lr=schema.actor_lr,
                    weight_decay=schema.actor_wd,
                ),
                start_epoch=start_epoch,
                episodes=schema.num_episodes,
                keys=keys,
                artifact_dir=artifact_dir / artifact_suffix,
                train_checkpoints_dir=train_checkpoints_dir,
                config=schema,
            )

            # train(
            #     dataset,
            #     embedder=embedder,
            #     actor=actor,
            #     critic=critic,
            #     actor_optim=optim.Adam(
            #         actor.parameters(),
            #         lr=schema.actor_lr,
            #         weight_decay=schema.actor_wd,
            #     ),
            #     critic_optim=optim.Adam(
            #         critic.parameters(),
            #         lr=schema.critic_lr,
            #         weight_decay=schema.critic_wd,
            #     ),
            #     start_epoch=start_epoch,
            #     episodes=schema.num_episodes,
            #     keys=keys,
            #     artifact_dir=artifact_dir / artifact_suffix,
            #     train_checkpoints_dir=train_checkpoints_dir,
            #     config=schema,
            #     entropy_coefs=(
            #         schema.actor.entropy_start,
            #         schema.actor.entropy_end,
            #     ),
            #     gamma=schema.gae.gamma,
            #     lam=schema.gae.lam,
            # )

            shutil.rmtree(train_checkpoints_dir)
        case "eval":
            if not (
                os.path.exists(artifact_dir / "embedder_tuned.pt")
                and os.path.exists(artifact_dir / "model.pt")
            ):
                raise FileNotFoundError(
                    "Actor or tuned embedding weights not found in artifact dir!"
                )

            embedder = torch.load(
                artifact_dir / "embedder_tuned.pt",
                weights_only=False,
                map_location=DEVICE,
            )
            # actor = torch.load(
            #     artifact_dir / "actor.pt",
            #     weights_only=False,
            #     map_location=DEVICE,
            # )
            model = torch.load(
                artifact_dir / "model.pt",
                weights_only=False,
                map_location=DEVICE,
            )

            # return eval(actor, embedder, dataset)
            eval_traditional(model, embedder, dataset, keys)
