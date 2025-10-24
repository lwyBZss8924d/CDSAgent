#!/usr/bin/env python3
"""
Extract graph baseline from a repository for parity validation.

This script extracts full graph data (nodes + edges) using LocAgent's build_graph module.

Usage:
    python scripts/extract-parity-baseline.py \
        --repo-path tmp/LocAgent \
        --output tests/fixtures/parity/golden_outputs/graph_locagent.json \
        --repo-name LocAgent

    python scripts/extract-parity-baseline.py \
        --repo-path .artifacts/tmp/swe-bench-lite/django__django-12345 \
        --output tests/fixtures/parity/golden_outputs/graph_django-12345.json \
        --repo-name django__django-12345 \
        --max-files 500

Options:
    --max-files: Limit extraction to first N Python files (for large repos)
    --exclude-tests: Skip test files (default: False)

Environment:
    - Requires LocAgent dependencies (install via: cd tmp/LocAgent && pip install -r requirements.txt)
    - PYTHONPATH must include tmp/LocAgent
"""

import argparse
import json
import sys
from pathlib import Path
from typing import Dict, List, Any, Optional

# Add LocAgent to path
sys.path.insert(0, str(Path(__file__).parent.parent / "tmp" / "LocAgent"))

try:
    from dependency_graph.build_graph import build_graph
except ImportError:
    print("Error: Cannot import LocAgent build_graph module.", file=sys.stderr)
    print("Ensure tmp/LocAgent is present and dependencies are installed.", file=sys.stderr)
    sys.exit(1)


def extract_graph_data(
    repo_path: Path,
    repo_name: str,
    max_files: Optional[int] = None,
    exclude_tests: bool = False
) -> Dict[str, Any]:
    """
    Extract full graph data including nodes and edges.

    Returns:
        {
            "repository": str,
            "total_nodes": int,
            "total_edges": int,
            "node_counts_by_type": dict,
            "edge_counts_by_type": dict,
            "nodes": [{"id": str, "type": str, "name": str, "file": str, "line": int}, ...],
            "edges": [{"source": str, "target": str, "type": str}, ...],
            "graph_version": str,
            "extraction_metadata": {
                "max_files": int | None,
                "exclude_tests": bool,
                "total_files_processed": int
            }
        }
    """
    print(f"Building graph for {repo_name} from {repo_path}...")

    # Build graph using LocAgent's build_graph
    # Note: This returns a NetworkX MultiDiGraph
    graph = build_graph(str(repo_path))

    # Phase 1: Build set of allowed file IDs
    allowed_files = set()
    file_count = 0

    for node_id, attrs in graph.nodes(data=True):
        node_type = attrs.get("type", "unknown")

        if node_type == "file":
            # Apply exclude_tests filter
            if exclude_tests and (
                "/test" in node_id or node_id.endswith("_test.py") or
                node_id.endswith("/tests.py")
            ):
                continue

            # Apply max_files limit
            file_count += 1
            if max_files and file_count > max_files:
                break  # Stop processing more files

            allowed_files.add(node_id)

    # Phase 2: Extract nodes, filtering by allowed files
    nodes = []
    node_counts = {"directory": 0, "file": 0, "class": 0, "function": 0}

    for node_id, attrs in graph.nodes(data=True):
        node_type = attrs.get("type", "unknown")

        # For class/function nodes, derive file from node ID (format: "file.py:ClassName")
        if node_type in ("class", "function"):
            # Extract filename from node_id (before first colon)
            if ":" in node_id:
                node_file = node_id.split(":", 1)[0]
            else:
                # Malformed node ID, skip
                continue

            if node_file not in allowed_files:
                continue  # Skip nodes from excluded files

        # For file nodes, check if in allowed set
        elif node_type == "file":
            if node_id not in allowed_files:
                continue

        # Directory nodes are always included (needed for tree structure)

        # Increment counters for included nodes only
        node_counts[node_type] = node_counts.get(node_type, 0) + 1

        # Determine file path for this node
        if node_type in ("class", "function"):
            node_file_path = node_id.split(":", 1)[0] if ":" in node_id else ""
        elif node_type == "file":
            node_file_path = node_id
        else:
            node_file_path = ""

        nodes.append({
            "id": node_id,
            "type": node_type,
            "name": attrs.get("name", node_id.split("/")[-1].split(":")[-1]),
            "file": node_file_path,
            "line": attrs.get("start_line", attrs.get("line", 0)),
        })

    # Build node ID set for edge filtering
    node_ids = {n["id"] for n in nodes}
    files_processed = allowed_files

    # Extract edge data (only edges between included nodes)
    edges = []
    edge_counts = {"contains": 0, "imports": 0, "invokes": 0, "inherits": 0}

    for source, target, key, attrs in graph.edges(keys=True, data=True):
        # Skip edges to/from pruned nodes
        if source not in node_ids or target not in node_ids:
            continue

        edge_type = attrs.get("type", "unknown")
        edge_counts[edge_type] = edge_counts.get(edge_type, 0) + 1

        edges.append({
            "source": source,
            "target": target,
            "type": edge_type,
        })

    return {
        "repository": repo_name,
        "total_nodes": len(nodes),
        "total_edges": len(edges),
        "node_counts_by_type": node_counts,
        "edge_counts_by_type": edge_counts,
        "nodes": nodes,
        "edges": edges,
        "graph_version": "v2.3",  # LocAgent's current version
        "extraction_metadata": {
            "max_files": max_files,
            "exclude_tests": exclude_tests,
            "total_files_processed": len(files_processed),
        }
    }


def main():
    parser = argparse.ArgumentParser(description="Extract parity baseline graph")
    parser.add_argument("--repo-path", type=Path, required=True, help="Path to repository")
    parser.add_argument("--output", type=Path, required=True, help="Output JSON file")
    parser.add_argument("--repo-name", required=True, help="Repository name/identifier")
    parser.add_argument("--max-files", type=int, help="Limit to first N Python files")
    parser.add_argument("--exclude-tests", action="store_true", help="Skip test files")

    args = parser.parse_args()

    if not args.repo_path.exists():
        print(f"Error: Repository path does not exist: {args.repo_path}", file=sys.stderr)
        sys.exit(1)

    # Extract graph data
    try:
        graph_data = extract_graph_data(
            args.repo_path,
            args.repo_name,
            max_files=args.max_files,
            exclude_tests=args.exclude_tests
        )
    except Exception as e:
        print(f"Error extracting graph: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)

    # Save to output file
    args.output.parent.mkdir(parents=True, exist_ok=True)
    with open(args.output, "w") as f:
        json.dump(graph_data, f, indent=2)

    print(f"\n✓ Graph baseline extracted:")
    print(f"  Repository: {graph_data['repository']}")
    print(f"  Nodes: {graph_data['total_nodes']} (dirs: {graph_data['node_counts_by_type']['directory']}, "
          f"files: {graph_data['node_counts_by_type']['file']}, "
          f"classes: {graph_data['node_counts_by_type']['class']}, "
          f"functions: {graph_data['node_counts_by_type']['function']})")
    print(f"  Edges: {graph_data['total_edges']} (contains: {graph_data['edge_counts_by_type']['contains']}, "
          f"imports: {graph_data['edge_counts_by_type']['imports']}, "
          f"invokes: {graph_data['edge_counts_by_type']['invokes']}, "
          f"inherits: {graph_data['edge_counts_by_type']['inherits']})")
    print(f"  Output: {args.output}")

    # Check file size
    file_size_mb = args.output.stat().st_size / (1024 * 1024)
    print(f"  File size: {file_size_mb:.2f} MB")
    if file_size_mb > 5:
        print(f"  ⚠️  Warning: File size exceeds 5MB. Consider using --max-files to reduce size.")


if __name__ == "__main__":
    main()
