import networkx as nx
from math import prod
from pathlib import Path


def load_data():
    return [line for line in Path("input/day_25.txt").read_text().strip().split("\n")]


def part_one(data):
    G = nx.Graph()
    for line in data:
        a, bs = line.split(": ")
        for b in bs.split():
            G.add_edge(a, b)
    G.remove_edges_from(nx.minimum_edge_cut(G))
    solution = prod(len(c) for c in nx.connected_components(G))
    print(solution)
    return solution


def main():
    data = load_data()
    part_one(data)


if __name__ == "__main__":
    main()
