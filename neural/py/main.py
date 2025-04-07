from numpy.typing import NDArray
from torch import nn, optim
from torch.utils.data import random_split
from torch.utils.data.dataset import Dataset
from torch_geometric.loader import DataLoader
from torch_geometric.data import Data
from actor import Actor
from critic import Critic
from hdbscan import HDBSCAN, all_points_membership_vectors

class GraphDataset(Dataset):
    def __init__(self, graphs: list[Data]):
        super().__init__()
        self.graphs = graphs
        for i, graph in enumerate(self.graphs):
            graph.idx = i

    def __len__(self):
        return len(self.graphs)

    def get(self, idx: int):
        return self.graphs[idx]

    def __getitem__(self, lookup):
        return self.get(lookup)

def generate_dataset(edges: list[NDArray], features: list[NDArray]) -> GraphDataset:
    assert len(edges) == len(features), "Invalid shape configuration!!! Stack of edges must have same number of graphs as features!"
    # assert edges[0].shape[0] == 2, "Invalid edge shape! Must have shape [B, 2, E]"

    # Iterate over each batch
    # Since the y values are only relevant for the entire graph, they will be stored separately
    # However since the indexing is sensitive the y value for each graph will just be its index
    graphs = []
    for i in range(len(edges)):
        edge_index = edges[i]
        x = features[i]

        # Padding will always be at the end of these, so easy to just get rid of them
        valid_nodes = ~(x == -1).all(axis=1)  # Boolean mask for real nodes
        x_pruned = x[valid_nodes]  # Filter node features

        valid_edges = ~(edge_index == -1).all(axis=0)  # Boolean mask for real edges
        edge_index_pruned = edge_index[:, valid_edges]  # Remove invalid edges

        graphs.append(Data(x=x_pruned, edge_index=edge_index_pruned, y=i))

    return GraphDataset(graphs)

NUM_EPISODES = 10

def make_splits(dataset_sz: int) -> tuple[int, int, int]:
    # Define split sizes (80% Train, 10% Val, 10% Test)
    train_size = int(0.8 * dataset_sz)
    val_size = int(0.1 * dataset_sz)
    test_size = dataset_sz - train_size - val_size  # Ensure correct total
    return train_size, val_size, test_size

def cluster_and_calculate_reward(nodes: NDArray, edges: NDArray, keys: dict[tuple[int, int], NDArray], merge_map: NDArray, selected_indices_for_batch: list[int]) -> float:
    clusterer = HDBSCAN(min_cluster_size=2, core_dist_n_jobs=-1).fit(nodes)
    vecs = all_points_membership_vectors(clusterer)
    print(vecs)
    return -100.0

def train(dataset: Dataset, actor: nn.Module, critic: nn.Module, actor_optim: optim.Optimizer, critic_optim: optim.Optimizer, episodes: int):
    train_set, val_set, test_set = random_split(dataset, make_splits(len(dataset))) #type: ignore

    train_data = DataLoader(train_set, batch_size=10)
    val_data = DataLoader(val_set, batch_size=5)
    test_data = DataLoader(test_set, batch_size=5)

    for epoch in range(episodes):
        actor.train()
        critic.train()

        total_actor_loss, total_critic_loss = 0.0, 0.0

        # Train
        for batch in train_data:
            selected_indices = map(lambda g: g.idx, batch)
            pass

        # Val
        for batch in val_data:
            pass

    # Test
    actor.eval()
    critic.eval()
    for batch in test_data:
        pass

    print('Training complete')

def rust_train(features: list[NDArray], edges: list[NDArray], keys: dict[tuple[int, int], NDArray]):
    print('Beginning training...')
    dataset = generate_dataset(edges, features)
    print(f'Dataset generated with {len(dataset)} entries')

    actor = Actor(in_dim=features[0].shape[1], hidden_dims=[20, 10], num_heads=[8, 8], pool_ratios=[0.5, 0.5])
    critic = Critic(in_dim=features[0].shape[1], hidden_dim=20, num_heads=8)

    train(
        dataset,
        actor,
        critic,
        optim.Adam(actor.parameters(), lr=0.001),
        optim.Adam(critic.parameters(), lr=0.001),
        NUM_EPISODES
    )
