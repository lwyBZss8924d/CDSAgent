#!/usr/bin/env python3
"""
Export the English stop-word list used by LocAgent's BM25 pipeline
into CDSAgent's parity fixture.

The script attempts to locate `bm25s/stopwords.py` inside the LocAgent
worktree (or its virtual environment) and extracts `STOPWORDS_EN_PLUS`.
If the file cannot be located, the script falls back to the current
fixture contents to avoid producing an empty list.
"""

from __future__ import annotations

import argparse
import ast
import sys
from pathlib import Path
from typing import Iterable


REPO_ROOT = Path(__file__).resolve().parents[1]
LOCAGENT_DIR = REPO_ROOT / "tmp" / "LocAgent"
DEFAULT_SOURCE_PATTERNS: tuple[str, ...] = (
    "bm25s/stopwords.py",
    ".venv/lib/python*/site-packages/bm25s/stopwords.py",
)
DEFAULT_FIXTURE = (
    REPO_ROOT
    / "crates"
    / "cds-index"
    / "tests"
    / "fixtures"
    / "parity"
    / "tokenizer"
    / "stop_words.txt"
)


def discover_stopwords_source(
    extra_candidates: Iterable[Path] | None = None,
) -> Path | None:
    candidates = list(extra_candidates or [])
    for pattern in DEFAULT_SOURCE_PATTERNS:
        candidates.extend(LOCAGENT_DIR.glob(pattern))
        candidates.extend(LOCAGENT_DIR.glob(f"**/{pattern}"))
    for candidate in candidates:
        if candidate.is_file():
            return candidate
    return None


def load_stopwords_from_python(path: Path) -> list[str]:
    data = path.read_text(encoding="utf-8")
    module = ast.parse(data, filename=str(path))
    for node in module.body:
        if isinstance(node, ast.Assign):
            for target in node.targets:
                if (
                    isinstance(target, ast.Name)
                    and target.id == "STOPWORDS_EN_PLUS"
                ):
                    return sorted(
                        {
                            item.strip().lower()
                            for item in ast.literal_eval(node.value)
                        }
                    )
    raise ValueError(f"STOPWORDS_EN_PLUS not found in {path}")


def load_stopwords_from_fixture(path: Path) -> list[str]:
    if not path.exists():
        return []
    return sorted(
        {
            line.strip().lower()
            for line in path.read_text(encoding="utf-8").splitlines()
            if line.strip()
        }
    )


def write_stopwords(path: Path, stopwords: list[str]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8", newline="\n") as handle:
        handle.write("\n".join(stopwords))
        handle.write("\n")


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--output",
        type=Path,
        default=DEFAULT_FIXTURE,
        help=(
            "Destination file for exported stop words "
            f"(default: {DEFAULT_FIXTURE.relative_to(REPO_ROOT)})"
        ),
    )
    parser.add_argument(
        "--source",
        type=Path,
        default=None,
        help="Optional explicit path to LocAgent bm25s/stopwords.py",
    )
    return parser.parse_args(argv)


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    output_path: Path = (
        args.output if args.output.is_absolute() else REPO_ROOT / args.output
    )

    source_path: Path | None = None
    if args.source:
        explicit = (
            args.source
            if args.source.is_absolute()
            else REPO_ROOT / args.source
        )
        if explicit.exists():
            source_path = explicit
        else:
            print(
                f"[WARN] Explicit source {explicit} does not exist",
                file=sys.stderr,
            )
    if source_path is None:
        source_path = discover_stopwords_source()

    if source_path is None:
        print(
            "[WARN] Could not locate bm25s stopwords; "
            "falling back to existing fixture",
            file=sys.stderr,
        )
        stopwords = load_stopwords_from_fixture(output_path)
        if not stopwords:
            print(
                "[ERROR] No stop words available; aborting",
                file=sys.stderr,
            )
            return 1
    else:
        try:
            stopwords = load_stopwords_from_python(source_path)
        except Exception as err:  # pylint: disable=broad-except
            print(
                f"[WARN] Failed to parse {source_path}: {err}",
                file=sys.stderr,
            )
            stopwords = []
        if not stopwords:
            print(
                "[WARN] Parsed stop-word list empty; "
                "falling back to existing fixture",
                file=sys.stderr,
            )
            stopwords = load_stopwords_from_fixture(output_path)
            if not stopwords:
                print(
                    "[ERROR] No stop words available; aborting",
                    file=sys.stderr,
                )
                return 1

    write_stopwords(output_path, stopwords)
    rel_output = output_path.relative_to(REPO_ROOT)
    print(f"Wrote {len(stopwords)} stop words to {rel_output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
