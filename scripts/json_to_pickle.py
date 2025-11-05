#!/usr/bin/env python3
"""
json_to_pickle.py - Convert NetworkX JSON graphs to pickle format

This script converts LocAgent-format JSON graphs (already in NetworkX structure)
directly to pickle format without any format conversion.

Usage:
    python scripts/json_to_pickle.py --input graph.json --output graph.pkl
"""

import argparse
import json
import pickle
import networkx as nx
from pathlib import Path


def load_networkx_json(json_path: Path) -> nx.MultiDiGraph:
    """Load a NetworkX graph from JSON format"""
    with open(json_path, 'r') as f:
        data = json.load(f)

    # Create a new MultiDiGraph
    G = nx.MultiDiGraph()

    # Add nodes
    for node in data['nodes']:
        node_id = node['id']
        # Add all node attributes
        G.add_node(node_id, **{k: v for k, v in node.items() if k != 'id'})

    # Add edges
    for edge in data['edges']:
        source = edge['source']
        target = edge['target']
        # Add all edge attributes
        edge_attrs = {k: v for k, v in edge.items() if k not in ['source', 'target']}
        G.add_edge(source, target, **edge_attrs)

    return G


def save_pickle(graph: nx.MultiDiGraph, pkl_path: Path):
    """Save NetworkX graph to pickle format"""
    with open(pkl_path, 'wb') as f:
        pickle.dump(graph, f, protocol=pickle.HIGHEST_PROTOCOL)


def main():
    parser = argparse.ArgumentParser(
        description='Convert NetworkX JSON graph to pickle format'
    )
    parser.add_argument(
        '--input',
        required=True,
        type=Path,
        help='Path to input JSON file'
    )
    parser.add_argument(
        '--output',
        required=True,
        type=Path,
        help='Path to output pickle file'
    )
    parser.add_argument(
        '--verbose',
        action='store_true',
        help='Print detailed statistics'
    )

    args = parser.parse_args()

    # Load graph
    if args.verbose:
        print(f"Loading NetworkX graph from {args.input}...")
    G = load_networkx_json(args.input)

    # Save pickle
    if args.verbose:
        print(f"Saving to {args.output}...")
    save_pickle(G, args.output)

    if args.verbose:
        print(f"\nConversion Statistics:")
        print(f"  Nodes: {G.number_of_nodes()}")
        print(f"  Edges: {G.number_of_edges()}")

        # Count nodes by type
        node_types = {}
        for _, data in G.nodes(data=True):
            ntype = data.get('type', 'unknown')
            node_types[ntype] = node_types.get(ntype, 0) + 1

        print(f"\n  Nodes by type:")
        for ntype, count in sorted(node_types.items()):
            print(f"    {ntype}: {count}")

        # Count edges by type
        edge_types = {}
        for _, _, data in G.edges(data=True):
            etype = data.get('type', 'unknown')
            edge_types[etype] = edge_types.get(etype, 0) + 1

        print(f"\n  Edges by type:")
        for etype, count in sorted(edge_types.items()):
            print(f"    {etype}: {count}")

        print(f"\n✅ Successfully saved NetworkX graph to {args.output}")


if __name__ == '__main__':
    main()
