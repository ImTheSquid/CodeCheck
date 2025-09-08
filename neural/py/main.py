import csv
import datetime
import gc
import itertools
import os
from collections import defaultdict
from multiprocessing.pool import Pool
from pathlib import Path
from typing import Literal, Optional, cast

import numpy as np
import psutil
import torch
import torch.nn.functional as F
from diskcache import Cache
from numpy.typing import NDArray
from omegaconf import OmegaConf
from progress.bar import Bar
from sklearn.cluster import HDBSCAN
from tabulate import tabulate
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
    Metrics,
    ModelConfig,
    actor_critic_loss,
    auxiliary_embedding_loss_helper,
    get_projections_and_targets_for_each_batch,
    lerp,
    make_persistent_to_batch_id_map,
    mine_triplets_from_aux_embeddings,
    persistent_to_batch_id_map_and_keys,
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


def mean_pool_same_lines(
    x: NDArray, batch: NDArray, lines: NDArray
) -> tuple[NDArray, NDArray, NDArray]:
    """
    Mean pool all of the x values that are on the same line in the same graph
    """

    batch_with_lines = np.hstack([lines, np.expand_dims(batch, 1)])
    meaned = []
    batch_lines = []
    for collection in np.unique(batch_with_lines, axis=0):
        mask = np.all(batch_with_lines == collection, axis=1)
        all_x = x[mask]
        all_x_mean = all_x.mean(axis=0)
        meaned.append(all_x_mean)
        batch_lines.append(collection)

    meaned = np.vstack(meaned)
    batch_lines = np.vstack(batch_lines)

    return meaned, batch_lines[:, 2], batch_lines[:, :2]


def cluster_and_calculate_reward(
    nodes: NDArray,
    keys: dict[tuple[int, int], NDArray],
    selected_indices_for_batch: list[tuple[int, int]],
    batch: NDArray,
    line_spans: NDArray,
) -> float:
    assert not np.isnan(np.sum(nodes)), "NaN in nodes! Bad training :("
    assert batch.shape[0] == nodes.shape[0], (
        "Something is wrong, nodes must match batch"
    )

    nodes, batch, line_spans = mean_pool_same_lines(nodes, batch, line_spans)

    relevant = find_relevant_keys_for_clustering(
        keys, selected_indices_for_batch
    )
    print(
        f"Graphs in this batch are:\n{tabulate(selected_indices_for_batch, headers=['Persistent ID', 'Batch ID'])}"
    )
    print(f"Relevant keys for clustering: {relevant}")
    # If there aren't any relevant keys in this dataset then no change in score
    if len(relevant) == 0:
        return 0.0

    selected_indices_dictionary = {
        gid: pid for (pid, gid) in selected_indices_for_batch
    }

    print(
        tabulate(
            list(
                zip(
                    batch.tolist(),
                    map(lambda bid: selected_indices_dictionary[bid], batch),
                    map(lambda arr: f"{arr[0]}, {arr[1]}", line_spans),
                )
            ),
            headers=["Batch", "Persistent ID", "Line Assignments"],
        )
    )

    # 1) Cluster
    # clusterer = HDBSCAN(min_cluster_size=2, cluster_selection_method='leaf')
    clusterer = HDBSCAN(
        min_cluster_size=2,
        # n_jobs=-1,
        # prediction_data=True,
        cluster_selection_method="leaf",
        allow_single_cluster=True,
    )
    labels = clusterer.fit_predict(nodes)  # -1 = noise

    print(f"{labels.max()} clusters created")

    for label in np.unique(labels):
        mask = label == labels
        print(
            f"\n\nMEMBERS IN CLUSTER {label} (total {np.count_nonzero(mask)}):\n"
        )

        line_data = line_spans[mask]
        bid = batch[mask]
        pid = map(lambda b: selected_indices_dictionary[b], bid)
        print(
            tabulate(
                list(
                    zip(
                        bid,
                        pid,
                        map(lambda arr: f"{arr[0]}, {arr[1]}", line_data),
                    )
                ),
                headers=["Batch", "Persistent ID", "Line Assignments"],
            )
        )

    # 2) Recover per‐node spans
    #    If you want "true" spans per node from your original mapping,
    #    you can call the helper above. Otherwise assume line_spans is already per node.

    # 3) Build lookup: local_idx -> (pid,graph)
    # pid_graph = {i: selected_indices_for_batch[i] for i in range(len(selected_indices_for_batch))}

    # 4) Count TP/FP and track which key‐rows we match exactly
    TP = FP = 0
    matched = set()  # (pair, (si,sj,ei,ej))

    # all_keys = set(keys.keys())

    for cluster in set(labels):
        if cluster < 0:
            continue
        members = np.where(labels == cluster)[0]
        for i, j in itertools.combinations(members, 2):
            g_i = batch[i]
            g_j = batch[j]

            # print(f"SEL {i}, {j}")
            # pid_i, g_i = pid_graph[i]
            # pid_j, g_j = pid_graph[j]
            if g_i == g_j:
                continue

            pid_i = pid_j = -1

            for pid, g in selected_indices_for_batch:
                if g == g_i:
                    pid_i = pid
                elif g == g_j:
                    pid_j = pid
                # Early return if both found
                if pid_i > -1 and pid_j > -1:
                    break

            span_i = tuple(line_spans[i].tolist())
            span_j = tuple(line_spans[j].tolist())

            pair = (pid_i, pid_j)
            arr = keys.get(pair, None)
            if arr is None or arr.size == 0:
                # no true spans → false positive
                FP += 1
                continue

            # want to see [si, sj, ei, ej]
            want1 = (span_i[0], span_j[0], span_i[1], span_j[1])
            want2 = (span_j[0], span_i[0], span_j[1], span_i[1])

            found = False
            for row in arr:
                row_t = (int(row[0]), int(row[1]), int(row[2]), int(row[3]))
                if row_t == want1 or row_t == want2:
                    TP += 1
                    matched.add((pair, row_t))
                    found = True
                    break
            if not found:
                FP += 1

    # 5) Count FN: ground‐truth rows never matched
    FN = 0
    for pair, arr in keys.items():
        if arr.size == 0:
            continue
        for row in arr:
            row_t = (int(row[0]), int(row[1]), int(row[2]), int(row[3]))
            if (pair, row_t) not in matched:
                FN += 1

    # 6) Precision/Recall/F1
    precision = TP / (TP + FP) if TP + FP > 0 else 0.0
    recall = TP / (TP + FN) if TP + FN > 0 else 0.0
    f1 = (
        2 * precision * recall / (precision + recall)
        if precision + recall > 0
        else 0.0
    )

    print(
        f"True Positives: {TP}\nFalse Positives: {FP}\nFalse Negatives: {FN}\nF1: {f1}"
    )

    return f1


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
                )

                batch = batch.to(DEVICE)
                batch_to_persistent = batch_to_persistent.to(DEVICE)

                embs = embedder(batch.x, batch.edge_index)

                x, line_spans, batch_idxs = actor(
                    embs,
                    batch.edge_index,
                    batch.batch,
                    persistent_to_batch_id_map,
                    batch.lines,
                    closest_node_pool,
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

    detections = defaultdict(list)
    for i, label in enumerate(hdbscan.labels_):
        row = spans[i]
        detections[label].append((persistent_ids[i], row[0], row[1]))

    return detections


def train(
    dataset: Dataset,
    embedder: nn.Module,
    actor: Actor,
    critic: nn.Module,
    actor_optim: optim.Optimizer,
    critic_optim: optim.Optimizer,
    episodes: int,
    keys: dict[tuple[int, int], NDArray],
    artifact_dir: Path,
    config: ModelConfig,
    gamma=0.99,
    lam=0.95,
    entropy_coefs: tuple[float, float] = (0.01, 0.001),
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
        num_workers=DATA_WORKERS,
    )
    test_data = DataLoader(
        test_set,  # type: ignore
        batch_size=config.batch_sizes.test,
        num_workers=DATA_WORKERS,
    )

    train_f = open(artifact_dir / "train_loss.csv", "w+")
    val_f = open(artifact_dir / "val_loss.csv", "w+")

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
        "Entropy Standard Deviation",
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
                metrics.entropy.std().item(),
                metrics.entropy.abs().mean().item(),
            ]
        )

    with Pool(
        DATA_WORKERS, maxtasksperchild=MAX_POOL_TASKS
    ) as closest_node_pool:
        for epoch in range(episodes):
            print(f"\n⏰ EPOCH {epoch}")
            embedder.train()
            actor.train()
            critic.train()

            entropy = lerp(entropy_coefs[0], entropy_coefs[1], epoch / episodes)

            total_actor_loss = total_critic_loss = 0.0

            # Train
            for batch in train_data:
                print("\n\nNEXT BATCH ->")

                persistent_to_batch_id_map, keys_1d, key_batch = (
                    persistent_to_batch_id_map_and_keys(batch, keys)
                )

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
                    persistent_to_batch_id_map=persistent_to_batch_id_map,
                    keys=keys,
                    config=config,
                )

                actor_loss = metrics.actor_loss
                if aux_emb_loss is not None:
                    actor_loss += aux_emb_loss
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

                del batch, actor_loss, critic_loss, aux_emb_loss
                cleanup()

            print(
                f"~~\nTotal Training Loss:\nActor: {total_actor_loss}\nCritic: {total_critic_loss}\n~~"
            )

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
                        persistent_to_batch_id_map=persistent_to_batch_id_map,
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

                    del batch, actor_loss, critic_loss, aux_emb_loss
                    cleanup()

            print(
                f"~~\nTotal Validation Loss:\nActor: {total_actor_loss}\nCritic: {total_critic_loss}\n~~"
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
                    persistent_to_batch_id_map=persistent_to_batch_id_map,
                    keys=keys,
                    config=config,
                )

                actor_loss = metrics.actor_loss
                if aux_emb_loss is not None:
                    actor_loss += aux_emb_loss
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

                del batch, actor_loss, critic_loss, aux_emb_loss
                cleanup()

    train_f.close()
    val_f.close()

    with open(artifact_dir / "test_loss.csv", "w+") as f:
        csv.writer(f).writerows(
            [["Actor", "Critic"]]
            + list(zip(test_actor_losses, test_critic_losses))
        )

    print(
        f"~~\nTotal Test Loss:\nActor: {total_actor_loss}\nCritic: {total_critic_loss}\n~~"
    )

    print("Training complete")
    torch.save(embedder.state_dict(), artifact_dir / "embedder_tuned.pt")
    torch.save(actor.state_dict(), artifact_dir / "actor.pt")


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
):
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

            schema = OmegaConf.structured(ModelConfig)

            if os.path.exists(config_dir / "config.yml"):
                cfg = OmegaConf.load(config_dir / "config.yml")
                schema = OmegaConf.merge(schema, cfg)

            artifact_suffix = (
                datetime.datetime.now()
                .replace(microsecond=0)
                .isoformat()
                .replace(":", "_")
            )

            os.makedirs(artifact_dir / artifact_suffix)

            with open(
                artifact_dir / artifact_suffix / "config.yml", mode="w+"
            ) as f:
                OmegaConf.save(schema, f)

            schema = cast(ModelConfig, schema)

            critic = MergeCritic(
                in_dim=embedding_dim // 2 * 4,
                hidden_dim_generator=lambda d: d // 2,
                num_layers=schema.critic_layers,
                p_dropout=schema.critic_dropout,
            ).to(DEVICE)
            actor = Actor(
                in_dim=embedding_dim,
                hidden_dims=[embedding_dim // 2] * schema.actor.num_layers
                + [embedding_dim],
                num_heads=[4] * schema.actor.num_layers + [1],
                config=schema,  # pyright: ignore
                # pool_ratios=[0.5, 0.6, 0.8, 0.8, 0.8],
                alpha=schema.actor.alpha,
                beta=schema.actor.beta,
                selection_dropout=schema.actor.selection_dropout,
            ).to(DEVICE)
            # critic = Critic(in_dim=features[0].shape[1], hidden_dim=20, num_heads=8).to(DEVICE)

            train(
                dataset,
                embedder=embedder,
                actor=actor,
                critic=critic,
                actor_optim=optim.Adam(
                    actor.parameters(),
                    lr=schema.actor_lr,
                    weight_decay=schema.actor_wd,
                ),
                critic_optim=optim.Adam(
                    critic.parameters(),
                    lr=schema.critic_lr,
                    weight_decay=schema.critic_wd,
                ),
                episodes=schema.num_episodes,
                keys=keys,
                artifact_dir=artifact_dir / artifact_suffix,
                config=schema,
                entropy_coefs=(
                    schema.actor.entropy_start,
                    schema.actor.entropy_end,
                ),
                gamma=schema.gae.gamma,
                lam=schema.gae.lam,
            )
        case "eval":
            if not (
                os.path.exists(artifact_dir / "embeddings_tuned.pt")
                and os.path.exists(artifact_dir / "actor.pt")
            ):
                raise FileNotFoundError(
                    "Actor or tuned embedding weights not found in artifact dir!"
                )

            embedder = torch.load(
                artifact_dir / "embeddings_tuned.pt",
                weights_only=False,
                map_location=DEVICE,
            )
            actor = torch.load(
                artifact_dir / "actor.pt",
                weights_only=False,
                map_location=DEVICE,
            )

            return eval(actor, embedder, dataset)
