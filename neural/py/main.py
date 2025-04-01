from numpy.typing import NDArray
from torch import nn, optim
from torch_geometric.data import Batch, Data
from actor import Actor
from critic import Critic

def main():
    print("Hello from py!")

def train(features, edges, actor: nn.Module, critic: nn.Module, actor_optim: optim.Optimizer, critic_optim: optim.Optimizer, episodes: int):
    pass

def generate_dataset(edges: NDArray, features: NDArray) -> Data:
    assert edges.shape[0] == features.shape[0], "Invalid shape configuration!!! Stack of edges must have same number of graphs as features!"
    assert edges.shape[1] == 2, "Invalid edge shape! Must have shape [B, 2, E]"

    # Iterate over each batch
    graphs = []
    for i in range(edges.shape[0]):
        edge_index = edges[i]
        x = features[i]

        # Padding will always be at the end of these, so easy to just get rid of them
        valid_nodes = ~(x == -1).all(axis=1)  # Boolean mask for real nodes
        x_pruned = x[valid_nodes]  # Filter node features

        valid_edges = ~(edge_index == -1).all(axis=0)  # Boolean mask for real edges
        edge_index_pruned = edge_index[:, valid_edges]  # Remove invalid edges

        graphs.append(Data(x=x_pruned, edge_index=edge_index_pruned))

    return Batch.from_data_list(graphs) # type: ignore

NUM_EPISODES = 10

def rust_train(edges: NDArray, features: NDArray):
    batch = generate_dataset(edges, features)

    actor = Actor(in_dim=features.shape[2], hidden_dims=[20, 10], num_heads=[8, 8], pool_ratios=[0.5, 0.5])
    critic = Critic(in_dim=features.shape[2], hidden_dim=20, num_heads=8)

    train(
        batch.x,
        batch.edge_index,
        actor,
        critic,
        optim.Adam(actor.parameters(), lr=0.001),
        optim.Adam(critic.parameters(), lr=0.001),
        NUM_EPISODES
    )

if __name__ == "__main__":
    main()
