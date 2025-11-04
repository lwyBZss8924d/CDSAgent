#!/usr/bin/env python3
"""
compare_graphs.py

Thread-23: Compare CDSAgent and LocAgent graphs to identify entity extraction gaps

Usage:
    python scripts/compare_graphs.py \
        --cdsagent graph_cdsagent.pkl \
        --locagent graph_locagent.pkl \
        --output comparison_report.json

Purpose:
    - Load both CDSAgent and LocAgent graphs
    - Compare node counts by type
    - Compare edge counts by type
    - Identify missing/extra nodes in CDSAgent
    - Identify missing/extra edges in CDSAgent
    - Generate diagnostic report with top 10 gaps

Expected Output:
    - JSON report with detailed statistics
    - Human-readable summary
    - Top 10 entity extraction gaps
"""

import argparse
import json
import pickle
import networkx as nx
from pathlib import Path
from collections import defaultdict
from typing import Dict, List, Tuple, Set


def load_graph_pickle(pkl_path: Path) -> nx.MultiDiGraph:
    """Load NetworkX graph from pickle file"""
    with open(pkl_path, 'rb') as f:
        return pickle.load(f)


def count_nodes_by_type(G: nx.MultiDiGraph) -> Dict[str, int]:
    """Count nodes by type attribute"""
    counts = defaultdict(int)
    for node, data in G.nodes(data=True):
        ntype = data.get('type', 'unknown')
        counts[ntype] += 1
    return dict(counts)


def count_edges_by_type(G: nx.MultiDiGraph) -> Dict[str, int]:
    """Count edges by type attribute"""
    counts = defaultdict(int)
    for u, v, data in G.edges(data=True):
        etype = data.get('type', 'unknown')
        counts[etype] += 1
    return dict(counts)


def get_nodes_by_type(G: nx.MultiDiGraph, node_type: str) -> Set[str]:
    """Get set of node IDs by type"""
    return {
        node
        for node, data in G.nodes(data=True)
        if data.get('type') == node_type
    }


def compare_node_sets(
    cdsagent_nodes: Set[str],
    locagent_nodes: Set[str],
    node_type: str
) -> Dict:
    """
    Compare two sets of nodes and identify differences

    Returns:
        {
            'missing_in_cdsagent': list,
            'extra_in_cdsagent': list,
            'overlap_count': int,
            'overlap_percent': float
        }
    """
    missing = locagent_nodes - cdsagent_nodes
    extra = cdsagent_nodes - locagent_nodes
    overlap = cdsagent_nodes & locagent_nodes

    overlap_percent = 0.0
    if locagent_nodes:
        overlap_percent = (len(overlap) / len(locagent_nodes)) * 100

    return {
        'node_type': node_type,
        'missing_in_cdsagent': sorted(list(missing)),
        'extra_in_cdsagent': sorted(list(extra)),
        'overlap_count': len(overlap),
        'cdsagent_count': len(cdsagent_nodes),
        'locagent_count': len(locagent_nodes),
        'overlap_percent': round(overlap_percent, 2),
        'missing_count': len(missing),
        'extra_count': len(extra)
    }


def compare_edge_sets(
    cdsagent_G: nx.MultiDiGraph,
    locagent_G: nx.MultiDiGraph,
    edge_type: str
) -> Dict:
    """
    Compare edges of a specific type between two graphs

    Returns:
        {
            'missing_in_cdsagent': list of (source, target),
            'extra_in_cdsagent': list of (source, target),
            'overlap_count': int,
            'overlap_percent': float
        }
    """
    # Get edges of specific type
    cdsagent_edges = {
        (u, v)
        for u, v, data in cdsagent_G.edges(data=True)
        if data.get('type') == edge_type
    }

    locagent_edges = {
        (u, v)
        for u, v, data in locagent_G.edges(data=True)
        if data.get('type') == edge_type
    }

    missing = locagent_edges - cdsagent_edges
    extra = cdsagent_edges - locagent_edges
    overlap = cdsagent_edges & locagent_edges

    overlap_percent = 0.0
    if locagent_edges:
        overlap_percent = (len(overlap) / len(locagent_edges)) * 100

    return {
        'edge_type': edge_type,
        'missing_in_cdsagent': sorted([f"{u} -> {v}" for u, v in missing]),
        'extra_in_cdsagent': sorted([f"{u} -> {v}" for u, v in extra]),
        'overlap_count': len(overlap),
        'cdsagent_count': len(cdsagent_edges),
        'locagent_count': len(locagent_edges),
        'overlap_percent': round(overlap_percent, 2),
        'missing_count': len(missing),
        'extra_count': len(extra)
    }


def identify_top_gaps(node_comparisons: List[Dict], edge_comparisons: List[Dict]) -> List[Dict]:
    """
    Identify top 10 entity extraction gaps based on missing counts

    Priority:
        1. Functions (most critical for code localization)
        2. Classes (important for structure understanding)
        3. Edges (relationship gaps)
        4. Files (usually fewer gaps)
    """
    gaps = []

    # Add node gaps
    for comp in node_comparisons:
        if comp['missing_count'] > 0:
            gaps.append({
                'type': 'node',
                'category': comp['node_type'],
                'missing_count': comp['missing_count'],
                'overlap_percent': comp['overlap_percent'],
                'severity': 'HIGH' if comp['node_type'] in ['function', 'class'] else 'MEDIUM',
                'examples': comp['missing_in_cdsagent'][:5]  # Top 5 examples
            })

    # Add edge gaps
    for comp in edge_comparisons:
        if comp['missing_count'] > 0:
            gaps.append({
                'type': 'edge',
                'category': comp['edge_type'],
                'missing_count': comp['missing_count'],
                'overlap_percent': comp['overlap_percent'],
                'severity': 'MEDIUM' if comp['edge_type'] in ['invokes', 'imports'] else 'LOW',
                'examples': comp['missing_in_cdsagent'][:5]  # Top 5 examples
            })

    # Sort by severity (HIGH -> MEDIUM -> LOW) then by missing count (descending)
    severity_order = {'HIGH': 0, 'MEDIUM': 1, 'LOW': 2}
    gaps.sort(key=lambda x: (severity_order[x['severity']], -x['missing_count']))

    return gaps[:10]  # Top 10


def generate_comparison_report(
    cdsagent_G: nx.MultiDiGraph,
    locagent_G: nx.MultiDiGraph
) -> Dict:
    """Generate comprehensive comparison report"""

    # Overall statistics
    report = {
        'summary': {
            'cdsagent': {
                'node_count': cdsagent_G.number_of_nodes(),
                'edge_count': cdsagent_G.number_of_edges(),
                'nodes_by_type': count_nodes_by_type(cdsagent_G),
                'edges_by_type': count_edges_by_type(cdsagent_G)
            },
            'locagent': {
                'node_count': locagent_G.number_of_nodes(),
                'edge_count': locagent_G.number_of_edges(),
                'nodes_by_type': count_nodes_by_type(locagent_G),
                'edges_by_type': count_edges_by_type(locagent_G)
            }
        },
        'node_comparisons': [],
        'edge_comparisons': [],
        'top_10_gaps': []
    }

    # Node comparisons by type
    node_types = set(list(report['summary']['cdsagent']['nodes_by_type'].keys()) +
                     list(report['summary']['locagent']['nodes_by_type'].keys()))

    for ntype in sorted(node_types):
        cdsagent_nodes = get_nodes_by_type(cdsagent_G, ntype)
        locagent_nodes = get_nodes_by_type(locagent_G, ntype)
        comparison = compare_node_sets(cdsagent_nodes, locagent_nodes, ntype)
        report['node_comparisons'].append(comparison)

    # Edge comparisons by type
    edge_types = set(list(report['summary']['cdsagent']['edges_by_type'].keys()) +
                     list(report['summary']['locagent']['edges_by_type'].keys()))

    for etype in sorted(edge_types):
        comparison = compare_edge_sets(cdsagent_G, locagent_G, etype)
        report['edge_comparisons'].append(comparison)

    # Identify top 10 gaps
    report['top_10_gaps'] = identify_top_gaps(
        report['node_comparisons'],
        report['edge_comparisons']
    )

    return report


def print_summary(report: Dict):
    """Print human-readable summary of comparison"""

    print("\n" + "=" * 80)
    print("GRAPH COMPARISON SUMMARY")
    print("=" * 80)

    print("\nOverall Statistics:")
    print(f"  CDSAgent: {report['summary']['cdsagent']['node_count']} nodes, "
          f"{report['summary']['cdsagent']['edge_count']} edges")
    print(f"  LocAgent: {report['summary']['locagent']['node_count']} nodes, "
          f"{report['summary']['locagent']['edge_count']} edges")

    print("\nNode Overlap by Type:")
    for comp in report['node_comparisons']:
        print(f"  {comp['node_type']:12} "
              f"{comp['overlap_percent']:6.2f}% overlap "
              f"({comp['overlap_count']}/{comp['locagent_count']}) "
              f"[missing: {comp['missing_count']}, extra: {comp['extra_count']}]")

    print("\nEdge Overlap by Type:")
    for comp in report['edge_comparisons']:
        print(f"  {comp['edge_type']:12} "
              f"{comp['overlap_percent']:6.2f}% overlap "
              f"({comp['overlap_count']}/{comp['locagent_count']}) "
              f"[missing: {comp['missing_count']}, extra: {comp['extra_count']}]")

    print("\nTop 10 Entity Extraction Gaps:")
    for i, gap in enumerate(report['top_10_gaps'], 1):
        print(f"\n{i}. {gap['severity']} - {gap['type'].upper()}: {gap['category']}")
        print(f"   Missing: {gap['missing_count']} ({gap['overlap_percent']:.2f}% overlap)")
        if gap['examples']:
            print(f"   Examples:")
            for ex in gap['examples'][:3]:  # Show top 3
                print(f"     - {ex}")

    print("\n" + "=" * 80)


def main():
    parser = argparse.ArgumentParser(
        description='Compare CDSAgent and LocAgent graphs for parity analysis'
    )
    parser.add_argument(
        '--cdsagent',
        required=True,
        type=Path,
        help='Path to CDSAgent graph pickle'
    )
    parser.add_argument(
        '--locagent',
        required=True,
        type=Path,
        help='Path to LocAgent graph pickle'
    )
    parser.add_argument(
        '--output',
        required=True,
        type=Path,
        help='Path to output JSON report'
    )

    args = parser.parse_args()

    # Load graphs
    print(f"Loading CDSAgent graph from {args.cdsagent}...")
    cdsagent_G = load_graph_pickle(args.cdsagent)

    print(f"Loading LocAgent graph from {args.locagent}...")
    locagent_G = load_graph_pickle(args.locagent)

    # Generate comparison
    print(f"\nComparing graphs...")
    report = generate_comparison_report(cdsagent_G, locagent_G)

    # Save report
    print(f"\nSaving report to {args.output}...")
    with open(args.output, 'w') as f:
        json.dump(report, f, indent=2)

    # Print summary
    print_summary(report)

    print(f"\n✅ Comparison complete! Full report saved to {args.output}")


if __name__ == '__main__':
    main()
