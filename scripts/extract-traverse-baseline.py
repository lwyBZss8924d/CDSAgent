#!/usr/bin/env python3
"""
Extract graph traversal baseline for parity validation.

Generates 10 diverse traversal scenarios and captures complete outputs.

Usage:
    python scripts/extract-traverse-baseline.py \
        --repo-path tmp/LocAgent \
        --output tests/fixtures/parity/golden_outputs/traverse_samples.jsonl \
        --append

Output format (JSONL):
    {"repo": "LocAgent", "scenario": "...", "start_entity": "...", "depth": N, "results": [...]}
"""

import argparse
import json
import sys
from pathlib import Path
from typing import List, Dict, Any

sys.path.insert(0, str(Path(__file__).parent.parent / "tmp" / "LocAgent"))

try:
    from dependency_graph.build_graph import build_graph
    from dependency_graph.traverse_graph import traverse_graph_structure
except ImportError:
    print("Error: Cannot import LocAgent modules.", file=sys.stderr)
    sys.exit(1)


def generate_traversal_scenarios(graph) -> List[Dict[str, Any]]:
    """
    Generate 10 diverse traversal scenarios.

    Scenarios cover:
    - Callers/callees (1-2 hops)
    - Import dependencies
    - Class inheritance
    - File contains relationships
    """
    # Extract some sample entities from graph
    sample_functions = [n for n, d in graph.nodes(data=True)
                        if d.get("type") == "function"][:5]
    sample_classes = [n for n, d in graph.nodes(data=True)
                      if d.get("type") == "class"][:3]
    sample_files = [n for n, d in graph.nodes(data=True)
                    if d.get("type") == "file"][:2]

    scenarios = []

    # Scenario 1-5: Function call graphs (callees = downstream)
    for i, func in enumerate(sample_functions, 1):
        scenarios.append({
            "scenario": f"callees_1hop_function_{i}",
            "start_entity": func,
            "direction": "downstream",  # Find what this function calls
            "edge_types": ["invokes"],
            "max_depth": 1,
        })

    # Scenario 6-8: Class inheritance (upstream = what inherits from this)
    for i, cls in enumerate(sample_classes, 1):
        scenarios.append({
            "scenario": f"subclasses_class_{i}",
            "start_entity": cls,
            "direction": "upstream",  # Find what inherits from this
            "edge_types": ["inherits"],
            "max_depth": 2,
        })

    # Scenario 9-10: File imports (downstream = what this file imports)
    for i, file in enumerate(sample_files, 1):
        scenarios.append({
            "scenario": f"imports_file_{i}",
            "start_entity": file,
            "direction": "downstream",
            "edge_types": ["imports"],
            "max_depth": 1,
        })

    return scenarios[:10]


def execute_traversal(graph, scenario: Dict[str, Any]) -> Dict[str, Any]:
    """Execute a single traversal scenario using LocAgent's traverse_graph_structure."""
    start = scenario["start_entity"]
    direction = scenario["direction"]
    edge_types = scenario["edge_types"]
    max_depth = scenario["max_depth"]

    # Use LocAgent's traverse_graph_structure function
    # Returns a text representation of the subgraph
    subgraph_text = traverse_graph_structure(
        graph,
        roots=[start],
        direction=direction,
        hops=max_depth,
        edge_type_filter=edge_types
    )

    # Parse the text result to extract entity information
    # (LocAgent returns formatted text, not structured data)
    results = []
    if subgraph_text:
        # Extract entities from the text representation
        for line in subgraph_text.split("\n"):
            if line.strip() and not line.startswith("#"):
                results.append({
                    "text": line.strip(),
                })

    return {
        "scenario": scenario["scenario"],
        "start_entity": start,
        "direction": direction,
        "edge_types": edge_types,
        "max_depth": max_depth,
        "total_results": len(results),
        "results": results,
        "graph_text": subgraph_text[:500] if subgraph_text else "",  # First 500 chars
    }


def extract_traversal_results(repo_path: Path, repo_name: str) -> List[Dict[str, Any]]:
    """Extract traversal results for 10 scenarios."""
    print(f"Building graph for {repo_name}...")
    graph = build_graph(str(repo_path))

    print(f"Generating traversal scenarios...")
    scenarios = generate_traversal_scenarios(graph)

    results = []
    print(f"Executing {len(scenarios)} traversals...")
    for i, scenario in enumerate(scenarios, 1):
        print(f"  [{i}/10] {scenario['scenario']}...")
        try:
            result = execute_traversal(graph, scenario)
            result["repo"] = repo_name
            results.append(result)
        except Exception as e:
            print(f"    ✗ Failed: {e}")
            continue

    return results


def main():
    parser = argparse.ArgumentParser(description="Extract traversal baseline")
    parser.add_argument("--repo-path", type=Path, required=True)
    parser.add_argument("--repo-name", required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--append", action="store_true")

    args = parser.parse_args()

    if not args.repo_path.exists():
        print(f"Error: {args.repo_path} does not exist", file=sys.stderr)
        sys.exit(1)

    # Extract results
    try:
        results = extract_traversal_results(args.repo_path, args.repo_name)
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)

    # Write to JSONL
    args.output.parent.mkdir(parents=True, exist_ok=True)
    mode = "a" if args.append else "w"

    with open(args.output, mode) as f:
        for result in results:
            f.write(json.dumps(result) + "\n")

    print(f"\n✓ Traversal baseline extracted:")
    print(f"  Repository: {args.repo_name}")
    print(f"  Scenarios: {len(results)}")
    print(f"  Output: {args.output}")


if __name__ == "__main__":
    main()
