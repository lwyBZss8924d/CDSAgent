#!/usr/bin/env python3
"""
Fetch SWE-bench Lite repository instances to ephemeral scratch space.

Usage:
    python scripts/fetch-swe-bench-lite.py --instances <instance_id1> <instance_id2> ...
    python scripts/fetch-swe-bench-lite.py --from-yaml tests/fixtures/parity/swe-bench-lite/samples.yaml

This script:
1. Loads the SWE-bench Lite dataset from Hugging Face
2. Clones specified instances to .artifacts/tmp/swe-bench-lite/<instance_id>/
3. Checks out the exact commit hash from the dataset
4. Does NOT commit repos to git (kept in gitignored .artifacts/tmp/)

Environment:
    - Requires: pip install datasets gitpython
    - HF_TOKEN environment variable (if dataset requires authentication)
"""

import argparse
import os
import sys
from pathlib import Path
from typing import List, Dict, Any

try:
    from datasets import load_dataset
    from git import Repo
    import yaml
except ImportError as e:
    print(f"Error: Missing dependencies. Install with: pip install datasets gitpython pyyaml", file=sys.stderr)
    sys.exit(1)


def load_swe_bench_lite() -> List[Dict[str, Any]]:
    """Load SWE-bench Lite dataset from Hugging Face."""
    print("Loading SWE-bench Lite dataset from Hugging Face...")
    try:
        dataset = load_dataset("princeton-nlp/SWE-bench_Lite", split="test")
        return list(dataset)
    except Exception as e:
        print(f"Error loading dataset: {e}", file=sys.stderr)
        print("Ensure HF_TOKEN is set if authentication is required.", file=sys.stderr)
        sys.exit(1)


def get_instance_metadata(dataset: List[Dict[str, Any]], instance_id: str) -> Dict[str, Any]:
    """Extract metadata for a specific instance."""
    for item in dataset:
        if item.get("instance_id") == instance_id:
            return {
                "instance_id": instance_id,
                "repo": item.get("repo"),
                "base_commit": item.get("base_commit"),
                "problem_statement": item.get("problem_statement", ""),
                "created_at": item.get("created_at", ""),
            }
    raise ValueError(f"Instance {instance_id} not found in SWE-bench Lite dataset")


def clone_instance(metadata: Dict[str, Any], target_dir: Path) -> None:
    """Clone a SWE-bench Lite instance to target directory."""
    instance_id = metadata["instance_id"]
    repo_url = f"https://github.com/{metadata['repo']}"
    commit_hash = metadata["base_commit"]

    instance_dir = target_dir / instance_id

    if instance_dir.exists():
        print(f"  ✓ {instance_id} already exists at {instance_dir}")
        return

    print(f"  Cloning {repo_url} to {instance_dir}...")
    try:
        repo = Repo.clone_from(repo_url, instance_dir, no_checkout=True)
        print(f"  Checking out commit {commit_hash[:8]}...")
        repo.git.checkout(commit_hash)
        print(f"  ✓ {instance_id} cloned successfully")
    except Exception as e:
        print(f"  ✗ Failed to clone {instance_id}: {e}", file=sys.stderr)
        raise


def main():
    parser = argparse.ArgumentParser(description="Fetch SWE-bench Lite instances")
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--instances", nargs="+", help="Instance IDs to fetch")
    group.add_argument("--from-yaml", type=Path, help="Load instance IDs from YAML file")
    parser.add_argument("--output-dir", type=Path, default=Path(".artifacts/tmp/swe-bench-lite"),
                        help="Output directory (default: .artifacts/tmp/swe-bench-lite)")

    args = parser.parse_args()

    # Load dataset
    dataset = load_swe_bench_lite()

    # Get instance IDs
    if args.from_yaml:
        with open(args.from_yaml) as f:
            config = yaml.safe_load(f)
            instance_ids = [s["instance_id"] for s in config.get("samples", [])]
    else:
        instance_ids = args.instances

    print(f"\nFetching {len(instance_ids)} instances to {args.output_dir}/\n")

    # Create output directory
    args.output_dir.mkdir(parents=True, exist_ok=True)

    # Clone each instance
    for instance_id in instance_ids:
        try:
            metadata = get_instance_metadata(dataset, instance_id)
            clone_instance(metadata, args.output_dir)
        except Exception as e:
            print(f"Failed to process {instance_id}: {e}", file=sys.stderr)
            continue

    print(f"\n✓ Fetch complete. Instances available at: {args.output_dir}/")
    print(f"Note: This directory is gitignored and ephemeral.")


if __name__ == "__main__":
    main()
