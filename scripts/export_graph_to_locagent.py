#!/usr/bin/env python3
"""
export_graph_to_locagent.py

Thread-23: Convert CDSAgent JSON graph export to LocAgent-compatible pickle format

Usage:
    python scripts/export_graph_to_locagent.py \
        --input graph.json \
        --output graph.pkl

Purpose:
    - Read CDSAgent JSON graph (from Rust DependencyGraph::export_to_json)
    - Convert to NetworkX MultiDiGraph compatible with LocAgent
    - Serialize as .pkl file for comparison

Node ID Format Conversion:
    CDSAgent: "repo::path/file.py::ClassName::method_name"
    LocAgent: "path/file.py:ClassName.method_name"

Edge Type Mapping:
    CDSAgent "contain" → LocAgent "contains"
    (Other edge types match: import, invoke, inherit)
"""

import argparse
import json
import pickle
import networkx as nx
from pathlib import Path


def convert_node_id_to_locagent(cdsagent_id: str) -> str:
    """
    Convert CDSAgent node ID format to LocAgent format

    CDSAgent: "repo::path/file.py::ClassName::method_name"
    LocAgent: "path/file.py:ClassName.method_name"
    """
    # Remove repo prefix
    if "::" in cdsagent_id:
        parts = cdsagent_id.split("::")
        # First part is repo name, skip it
        path_parts = parts[1:]

        # File path (may contain multiple parts like "django/db/models.py")
        file_path = path_parts[0]

        # Entity path (ClassName.method_name)
        if len(path_parts) > 1:
            entity_parts = path_parts[1:]
            entity_path = ".".join(entity_parts)
            return f"{file_path}:{entity_path}"
        else:
            return file_path
    else:
        return cdsagent_id


def convert_edge_kind_to_locagent(cdsagent_kind: str) -> str:
    """
    Convert CDSAgent edge kind to LocAgent edge type

    CDSAgent "contain" → LocAgent "contains"
    """
    if cdsagent_kind == "contain":
        return "contains"
    else:
        return cdsagent_kind


def convert_node_kind_to_locagent(cdsagent_kind: str) -> str:
    """
    Convert CDSAgent NodeKind to LocAgent node type

    Both use same names (lowercase): directory, file, class, function
    """
    return cdsagent_kind.lower()


def load_cdsagent_json(json_path: Path) -> dict:
    """Load CDSAgent graph JSON export"""
    with open(json_path, 'r') as f:
        return json.load(f)


def convert_to_locagent_graph(cdsagent_data: dict) -> nx.MultiDiGraph:
    """
    Convert CDSAgent JSON to LocAgent-compatible NetworkX graph

    Node attributes expected by LocAgent:
        - 'type': 'directory' | 'file' | 'class' | 'function'
        - 'code': str (optional, source code)
        - 'start_line': int (optional)
        - 'end_line': int (optional)

    Edge attributes:
        - 'type': 'contains' | 'imports' | 'invokes' | 'inherits'
    """
    G = nx.MultiDiGraph()

    # Add nodes
    for node in cdsagent_data['nodes']:
        locagent_id = convert_node_id_to_locagent(node['id'])

        node_attrs = {
            'type': convert_node_kind_to_locagent(node['kind'])
        }

        # Add source range if present
        if node.get('range'):
            node_attrs['start_line'] = node['range']['start_line']
            node_attrs['end_line'] = node['range']['end_line']

        # Add code content from attributes if present
        if 'code' in node.get('attributes', {}):
            node_attrs['code'] = node['attributes']['code']

        G.add_node(locagent_id, **node_attrs)

    # Add edges
    for edge in cdsagent_data['edges']:
        source_id = convert_node_id_to_locagent(edge['source'])
        target_id = convert_node_id_to_locagent(edge['target'])
        edge_type = convert_edge_kind_to_locagent(edge['kind'])

        edge_attrs = {'type': edge_type}

        # Add alias if present (for import edges)
        if edge.get('alias'):
            edge_attrs['alias'] = edge['alias']

        G.add_edge(source_id, target_id, **edge_attrs)

    return G


def export_to_pickle(graph: nx.MultiDiGraph, output_path: Path):
    """Serialize NetworkX graph to pickle file"""
    with open(output_path, 'wb') as f:
        pickle.dump(graph, f)


def main():
    parser = argparse.ArgumentParser(
        description='Convert CDSAgent JSON graph to LocAgent pickle format'
    )
    parser.add_argument(
        '--input',
        required=True,
        type=Path,
        help='Path to CDSAgent JSON graph export'
    )
    parser.add_argument(
        '--output',
        required=True,
        type=Path,
        help='Path to output LocAgent-compatible .pkl file'
    )
    parser.add_argument(
        '--verbose',
        action='store_true',
        help='Print conversion statistics'
    )

    args = parser.parse_args()

    # Load CDSAgent JSON
    print(f"Loading CDSAgent graph from {args.input}...")
    cdsagent_data = load_cdsagent_json(args.input)

    # Convert to LocAgent format
    print(f"Converting to LocAgent format...")
    locagent_graph = convert_to_locagent_graph(cdsagent_data)

    # Export to pickle
    print(f"Exporting to {args.output}...")
    export_to_pickle(locagent_graph, args.output)

    if args.verbose:
        print(f"\nConversion Statistics:")
        print(f"  Nodes: {locagent_graph.number_of_nodes()}")
        print(f"  Edges: {locagent_graph.number_of_edges()}")

        # Count nodes by type
        node_types = {}
        for node, data in locagent_graph.nodes(data=True):
            ntype = data.get('type', 'unknown')
            node_types[ntype] = node_types.get(ntype, 0) + 1

        print(f"\n  Nodes by type:")
        for ntype, count in sorted(node_types.items()):
            print(f"    {ntype}: {count}")

        # Count edges by type
        edge_types = {}
        for u, v, data in locagent_graph.edges(data=True):
            etype = data.get('type', 'unknown')
            edge_types[etype] = edge_types.get(etype, 0) + 1

        print(f"\n  Edges by type:")
        for etype, count in sorted(edge_types.items()):
            print(f"    {etype}: {count}")

    print(f"\n✅ Successfully exported LocAgent-compatible graph to {args.output}")


if __name__ == '__main__':
    main()
