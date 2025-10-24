#!/usr/bin/env python3
"""
Extract BM25 search baseline for parity validation.

Generates 50 diverse search queries and captures top-10 results from LocAgent's BM25 index.

Usage:
    python scripts/extract-search-baseline.py \
        --repo-path tmp/LocAgent \
        --output tests/fixtures/parity/golden_outputs/search_queries.jsonl \
        --append

Output format (JSONL):
    {"repo": "LocAgent", "query": "...", "top_10": [...], "total_results": N}
"""

import argparse
import json
import sys
from pathlib import Path
from typing import List, Dict, Any

sys.path.insert(0, str(Path(__file__).parent.parent / "tmp" / "LocAgent"))

try:
    from plugins.location_tools.retriever.bm25_retriever import (
        build_code_retriever_from_repo
    )
except ImportError:
    print("Error: Cannot import LocAgent BM25 retriever.", file=sys.stderr)
    sys.exit(1)


# Diverse query templates covering common code localization patterns
QUERY_TEMPLATES = [
    # Entity lookups
    "find class {entity}",
    "search for function {entity}",
    "locate method {entity}",

    # Functionality searches
    "parse {concept}",
    "handle {concept}",
    "process {concept}",
    "validate {concept}",
    "build {concept}",
    "create {concept}",
    "initialize {concept}",

    # Error/exception patterns
    "raise {error_type}",
    "catch {error_type}",
    "handle error in {module}",

    # Import/dependency patterns
    "import {module}",
    "use {library}",

    # Documentation patterns
    "docstring for {entity}",
    "return type of {entity}",
]


def generate_queries(repo_name: str) -> List[str]:
    """
    Generate 50 diverse search queries.

    For demonstration, this uses generic patterns. In production, you'd:
    1. Extract actual entity names from the graph
    2. Use issue/commit descriptions from SWE-bench
    3. Include language-specific patterns
    """
    queries = [
        # Generic patterns (20)
        "graph builder",
        "dependency traversal",
        "BM25 search",
        "code blocks parser",
        "AST parsing",
        "tree-sitter integration",
        "entity extraction",
        "qualified name resolution",
        "import resolution",
        "function call analysis",
        "class inheritance",
        "directory traversal",
        "file filtering",
        "index serialization",
        "search ranking",
        "result formatting",
        "error handling",
        "logging configuration",
        "test utilities",
        "benchmark metrics",

        # Code localization patterns (15)
        "find all functions in module",
        "locate class definition",
        "search for import statements",
        "find function calls to",
        "locate error handling code",
        "search for docstrings",
        "find test cases for",
        "locate configuration",
        "search for main entry point",
        "find exception handlers",
        "locate utility functions",
        "search for constants",
        "find type annotations",
        "locate module exports",
        "search for decorators",

        # Implementation patterns (15)
        "parse Python AST",
        "build dependency graph",
        "create BM25 index",
        "tokenize code blocks",
        "extract function signature",
        "resolve qualified names",
        "traverse call graph",
        "filter directories",
        "serialize graph data",
        "rank search results",
        "format output",
        "handle import errors",
        "configure logger",
        "run integration tests",
        "measure performance",
    ]

    return queries[:50]  # Ensure exactly 50


def extract_search_results(repo_path: Path, repo_name: str) -> List[Dict[str, Any]]:
    """Extract search results for 50 queries."""
    print(f"Building BM25 index for {repo_name}...")

    # Workaround for llama-index SimpleDirectoryReader limitation:
    # It validates required_exts at the root level before recursing. For repos where
    # Python code is in subdirectories (django/django/, sklearn/sklearn/), this fails.
    # Solution: Temporarily create a dummy .py file to satisfy the validator.
    dummy_file = repo_path / "dummy_llamaindex_workaround.py"
    created_dummy = False

    try:
        # Check if we need the workaround (no .py files at root except setup.py/conftest.py)
        root_py_files = list(repo_path.glob("*.py"))
        substantial_files = [f for f in root_py_files if f.name not in ("setup.py", "conftest.py", "__init__.py")]

        if not substantial_files:
            # Create dummy file to satisfy SimpleDirectoryReader validator
            dummy_file.write_text("# Temporary file for llama-index SimpleDirectoryReader validation\n")
            created_dummy = True
            # Verify file was created
            if dummy_file.exists():
                print(f"  ✓ Created dummy file: {dummy_file.name} (size: {dummy_file.stat().st_size} bytes)")
            else:
                print(f"  ⚠️  WARNING: Dummy file creation failed!")

        # Initialize retriever using LocAgent's build function
        # This returns a llama-index retriever
        retriever = build_code_retriever_from_repo(str(repo_path))

    finally:
        # Clean up dummy file
        if created_dummy and dummy_file.exists():
            dummy_file.unlink()
            print(f"  ✓ Cleaned up dummy file")

    queries = generate_queries(repo_name)
    results = []

    print(f"Executing {len(queries)} queries...")
    for i, query in enumerate(queries, 1):
        print(f"  [{i}/50] {query[:50]}...")

        # Get top-10 results (llama-index retrieve returns NodeWithScore objects)
        top_results = retriever.retrieve(query)[:10]

        results.append({
            "repo": repo_name,
            "query": query,
            "top_10": [
                {
                    "file": node.node.metadata.get("file_path", ""),
                    "name": node.node.metadata.get("function_name", ""),
                    "type": "codeblock",
                    "score": float(node.score) if node.score else 0.0,
                    "line": node.node.metadata.get("start_line", 0),
                    "text": node.node.text[:200],  # First 200 chars
                }
                for node in top_results
            ],
            "total_results": len(top_results),
        })

    return results


def main():
    parser = argparse.ArgumentParser(description="Extract search baseline")
    parser.add_argument("--repo-path", type=Path, required=True)
    parser.add_argument("--repo-name", required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--append", action="store_true", help="Append to existing file")

    args = parser.parse_args()

    if not args.repo_path.exists():
        print(f"Error: {args.repo_path} does not exist", file=sys.stderr)
        sys.exit(1)

    # Extract results
    try:
        results = extract_search_results(args.repo_path, args.repo_name)
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

    print(f"\n✓ Search baseline extracted:")
    print(f"  Repository: {args.repo_name}")
    print(f"  Queries: {len(results)}")
    print(f"  Output: {args.output}")


if __name__ == "__main__":
    main()
