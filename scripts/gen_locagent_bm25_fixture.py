#!/usr/bin/env python3
"""Generate LocAgent BM25 golden fixtures for a repository."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Iterable, List

from plugins.location_tools.retriever.bm25_retriever import build_code_retriever_from_repo


def load_queries(path: Path) -> List[str]:
    queries: List[str] = []
    with path.open() as handle:
        for raw in handle:
            raw = raw.strip()
            if not raw:
                continue
            payload = json.loads(raw)
            query = payload.get("query")
            if not query:
                raise ValueError(f"Missing 'query' field in {path}: {payload}")
            queries.append(query.strip())
    if not queries:
        raise ValueError(f"No queries loaded from {path}")
    return queries


def normalize_path(candidate: str | None, repo_root: Path) -> str:
    if not candidate:
        return ""
    cand_path = Path(candidate)
    if cand_path.is_absolute():
        try:
            return str(cand_path.resolve().relative_to(repo_root))
        except ValueError:
            return str(cand_path)
    return candidate.replace('\\', '/').lstrip('./')


def span_name(metadata: dict) -> str:
    spans: Iterable[str] | None = metadata.get("span_ids")
    if spans:
        for span in spans:
            if span:
                return span
    return metadata.get("node_id", "")


def format_hit(hit, repo_root: Path) -> dict:
    node = hit.node
    metadata = dict(node.metadata)
    file_path = metadata.get("file_path") or metadata.get("file")
    return {
        "file": normalize_path(file_path, repo_root),
        "name": span_name(metadata),
        "type": "codeblock",
        "score": float(hit.score),
        "line": metadata.get("start_line"),
        "text": node.get_content() or getattr(node, "text", ""),
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo-path', required=True, help='Path to the repo under tmp/smoke/')
    parser.add_argument('--repo-name', required=True, help='Logical repo name used in fixtures')
    parser.add_argument('--queries', required=True, help='JSONL file with {"query": ...}')
    parser.add_argument('--output', required=True, help='Destination JSONL file')
    parser.add_argument('--top-k', type=int, default=10, help='Number of hits to record per query')
    parser.add_argument('--progress', action='store_true', help='Show retriever progress output')
    args = parser.parse_args()

    repo_root = Path(args.repo_path).expanduser().resolve()
    queries_path = Path(args.queries).expanduser().resolve()
    output_path = Path(args.output).expanduser().resolve()
    output_path.parent.mkdir(parents=True, exist_ok=True)

    queries = load_queries(queries_path)

    retriever = build_code_retriever_from_repo(
        str(repo_root),
        similarity_top_k=args.top_k,
        show_progress=args.progress,
    )

    with output_path.open("w") as handle:
        for query in queries:
            results = retriever.retrieve(query)
            top_hits = [format_hit(hit, repo_root) for hit in results[: args.top_k]]
            record = {
                "repo": args.repo_name,
                "query": query,
                "top_10": top_hits,
                "total_results": len(results),
            }
            handle.write(json.dumps(record))
            handle.write("\n")


if __name__ == '__main__':
    main()
