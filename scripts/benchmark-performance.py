#!/usr/bin/env python3
"""
Benchmark LocAgent performance metrics for parity validation.

Measures:
- Graph build time
- Search query latency (p50, p95, p99)
- Traversal latency
- Memory usage

Usage:
    python scripts/benchmark-performance.py \
        --repo-path tmp/LocAgent \
        --output tests/fixtures/parity/golden_outputs/performance_baselines.json \
        --append

Output format:
    {
        "repo": "LocAgent",
        "graph_build": {"duration_s": X, "memory_mb": Y, "nodes": N, "edges": E},
        "search": {"p50_ms": X, "p95_ms": Y, "p99_ms": Z, "queries": N},
        "traverse": {"p50_ms": X, "p95_ms": Y, "p99_ms": Z, "scenarios": N},
        "timestamp": "ISO8601"
    }
"""

import argparse
import json
import sys
import time
from pathlib import Path
from typing import Dict, Any
import statistics
from datetime import datetime

sys.path.insert(0, str(Path(__file__).parent.parent / "tmp" / "LocAgent"))

try:
    import psutil
    from dependency_graph.build_graph import build_graph
    from dependency_graph.traverse_graph import traverse_graph_structure
    from plugins.location_tools.retriever.bm25_retriever import (
        build_code_retriever_from_repo
    )
except ImportError as e:
    print(f"Error: Missing dependencies: {e}", file=sys.stderr)
    print("Install with: pip install psutil", file=sys.stderr)
    sys.exit(1)


def measure_memory_usage():
    """Get current process memory usage in MB."""
    process = psutil.Process()
    return process.memory_info().rss / (1024 * 1024)


def benchmark_graph_build(repo_path: Path) -> Dict[str, Any]:
    """Benchmark graph construction."""
    print("Benchmarking graph build...")

    start_mem = measure_memory_usage()
    start_time = time.time()

    graph = build_graph(str(repo_path))

    duration = time.time() - start_time
    end_mem = measure_memory_usage()
    mem_delta = end_mem - start_mem

    return {
        "duration_s": round(duration, 3),
        "memory_mb": round(mem_delta, 2),
        "nodes": graph.number_of_nodes(),
        "edges": graph.number_of_edges(),
    }


def benchmark_search(repo_path: Path, num_queries: int = 50) -> Dict[str, Any]:
    """Benchmark search latency."""
    print(f"Benchmarking search ({num_queries} queries)...")

    # Workaround for llama-index SimpleDirectoryReader limitation:
    # It validates required_exts at the root level before recursing. For repos where
    # Python code is in subdirectories (django/django/, sklearn/sklearn/), this fails.
    # Solution: Temporarily create a dummy .py file to satisfy the validator.
    dummy_file = repo_path / "dummy_llamaindex_workaround.py"
    created_dummy = False

    try:
        # Check if we need the workaround (no .py files at root except setup.py/conftest.py)
        root_py_files = list(repo_path.glob("*.py"))
        substantial_files = [f for f in root_py_files
                           if f.name not in ("setup.py", "conftest.py", "__init__.py")]

        if not substantial_files:
            # Create dummy file to satisfy SimpleDirectoryReader validator
            dummy_file.write_text("# Temporary file for llama-index SimpleDirectoryReader validation\n")
            created_dummy = True
            print(f"  ✓ Created dummy file: {dummy_file.name} (llama-index workaround)")

        retriever = build_code_retriever_from_repo(str(repo_path))

    finally:
        # Clean up dummy file
        if created_dummy and dummy_file.exists():
            dummy_file.unlink()
            print(f"  ✓ Cleaned up dummy file")

    # Use diverse queries
    queries = [
        "graph builder", "dependency traversal", "BM25 search",
        "code parser", "AST parsing", "entity extraction",
        "import resolution", "function analysis", "class inheritance",
        "error handling", "search ranking", "result formatting",
        "test utilities", "logging", "configuration",
        "parse Python", "build graph", "create index",
        "tokenize code", "extract signature", "resolve names",
        "traverse graph", "filter files", "serialize data",
        "rank results", "format output", "handle errors",
        "configure logger", "run tests", "measure performance",
        "find class", "locate function", "search imports",
        "find calls", "locate errors", "search docs",
        "find tests", "locate config", "search main",
        "find handlers", "locate utils", "search constants",
        "find annotations", "locate exports", "search decorators",
        "validate input", "process data", "initialize state",
        "raise exception", "catch error", "return result",
    ][:num_queries]

    latencies = []
    for query in queries:
        start = time.time()
        _ = retriever.retrieve(query)[:10]
        latency = (time.time() - start) * 1000  # Convert to ms
        latencies.append(latency)

    # Calculate percentiles safely
    latencies_sorted = sorted(latencies)
    n = len(latencies)

    return {
        "p50_ms": round(latencies_sorted[n // 2], 2),
        "p95_ms": round(latencies_sorted[int(n * 0.95)], 2),
        "p99_ms": round(latencies_sorted[int(n * 0.99)], 2),
        "queries": len(latencies),
        "mean_ms": round(statistics.mean(latencies), 2),
    }


def benchmark_traverse(graph, num_scenarios: int = 10) -> Dict[str, Any]:
    """Benchmark graph traversal latency."""
    print(f"Benchmarking traversal ({num_scenarios} scenarios)...")

    # Sample entities
    functions = [n for n, d in graph.nodes(data=True)
                 if d.get("type") == "function"][:num_scenarios]

    # Handle empty function list (very small repos)
    if not functions:
        print("  ⚠️  No function nodes found, skipping traversal benchmark")
        return {
            "p50_ms": 0.0,
            "p95_ms": 0.0,
            "p99_ms": 0.0,
            "scenarios": 0,
            "mean_ms": 0.0,
            "skipped": True,
        }

    latencies = []
    for func in functions:
        start = time.time()
        _ = traverse_graph_structure(
            graph,
            roots=[func],
            direction="downstream",
            hops=2,
            edge_type_filter=["invokes"]
        )
        latency = (time.time() - start) * 1000
        latencies.append(latency)

    # Calculate percentiles safely (guaranteed len(latencies) >= 1)
    latencies_sorted = sorted(latencies)
    n = len(latencies)

    return {
        "p50_ms": round(latencies_sorted[n // 2], 2) if n > 0 else 0.0,
        "p95_ms": round(latencies_sorted[int(n * 0.95)], 2) if n > 0 else 0.0,
        "p99_ms": round(latencies_sorted[int(n * 0.99)], 2) if n > 0 else 0.0,
        "scenarios": len(latencies),
        "mean_ms": round(statistics.mean(latencies), 2) if n > 0 else 0.0,
    }


def benchmark_repository(repo_path: Path, repo_name: str) -> Dict[str, Any]:
    """Run all benchmarks for a repository."""
    print(f"\nBenchmarking {repo_name}...")

    # Graph build
    graph_metrics = benchmark_graph_build(repo_path)

    # Search
    search_metrics = benchmark_search(repo_path, num_queries=50)

    # Traversal (reuse graph from graph_build)
    graph = build_graph(str(repo_path))
    traverse_metrics = benchmark_traverse(graph, num_scenarios=10)

    return {
        "repo": repo_name,
        "graph_build": graph_metrics,
        "search": search_metrics,
        "traverse": traverse_metrics,
        "timestamp": datetime.utcnow().isoformat() + "Z",
    }


def main():
    parser = argparse.ArgumentParser(description="Benchmark performance")
    parser.add_argument("--repo-path", type=Path, required=True)
    parser.add_argument("--repo-name", required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--append", action="store_true")

    args = parser.parse_args()

    if not args.repo_path.exists():
        print(f"Error: {args.repo_path} does not exist", file=sys.stderr)
        sys.exit(1)

    # Run benchmarks
    try:
        metrics = benchmark_repository(args.repo_path, args.repo_name)
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)

    # Save results
    args.output.parent.mkdir(parents=True, exist_ok=True)

    if args.append and args.output.exists():
        with open(args.output) as f:
            data = json.load(f)
            if not isinstance(data, list):
                data = [data]
        data.append(metrics)
    else:
        data = [metrics]

    with open(args.output, "w") as f:
        json.dump(data, f, indent=2)

    print(f"\n✓ Performance baseline captured:")
    print(f"  Repository: {metrics['repo']}")
    print(f"  Graph build: {metrics['graph_build']['duration_s']}s "
          f"({metrics['graph_build']['nodes']} nodes)")
    print(f"  Search p95: {metrics['search']['p95_ms']}ms")
    print(f"  Traverse p95: {metrics['traverse']['p95_ms']}ms")
    print(f"  Output: {args.output}")


if __name__ == "__main__":
    main()
