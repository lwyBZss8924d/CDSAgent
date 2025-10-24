#!/usr/bin/env python3
"""
Select 5 diverse Python instances from SWE-bench Lite for parity validation.

Selection criteria:
- All Python projects
- Diversity in framework/library type
- Range of repository sizes (small to medium)
- Popular, well-maintained projects

Usage:
    python scripts/select-swe-bench-instances.py \
        --output tests/fixtures/parity/swe-bench-lite/samples.yaml
"""

import argparse
import sys
from pathlib import Path
from typing import List, Dict, Any

try:
    from datasets import load_dataset
    import yaml
except ImportError:
    print("Error: Missing dependencies", file=sys.stderr)
    print("Install with: pip install datasets pyyaml", file=sys.stderr)
    sys.exit(1)


def load_swe_bench_lite() -> List[Dict[str, Any]]:
    """Load SWE-bench Lite dataset."""
    print("Loading SWE-bench Lite dataset...")
    try:
        dataset = load_dataset("princeton-nlp/SWE-bench_Lite", split="test")
        return list(dataset)
    except Exception as e:
        print(f"Error loading dataset: {e}", file=sys.stderr)
        sys.exit(1)


def analyze_repository_distribution(dataset: List[Dict[str, Any]]) -> None:
    """Analyze repository distribution in dataset."""
    repos = {}
    for item in dataset:
        repo = item.get("repo", "unknown")
        repos[repo] = repos.get(repo, 0) + 1

    print(f"\nDataset contains {len(dataset)} instances across "
          f"{len(repos)} repositories:\n")

    # Sort by count
    for repo, count in sorted(repos.items(), key=lambda x: -x[1])[:15]:
        print(f"  {repo}: {count} instances")


def select_diverse_instances(dataset: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
    """
    Select 5 diverse instances.

    Strategy:
    1. Group by repository
    2. Select from different framework types (web, ML, data, testing, CLI)
    3. Prefer smaller repos (<500 files estimated)
    4. Pick one instance per selected repo
    """

    # Manually curate diverse selection based on SWE-bench Lite composition
    # (These are known Python projects in the dataset)
    target_repos = [
        "django/django",          # Web framework
        "scikit-learn/scikit-learn",  # ML library
        "matplotlib/matplotlib",  # Visualization
        "pytest-dev/pytest",      # Testing framework
        "psf/requests",          # HTTP library
    ]

    selected = []
    for repo in target_repos:
        # Find first instance for this repo
        candidates = [item for item in dataset if item.get("repo") == repo]
        if candidates:
            selected.append(candidates[0])
            print(f"  ✓ Selected {repo}: {candidates[0]['instance_id']}")
        else:
            print(f"  ✗ No instances found for {repo}")

    if len(selected) < 5:
        print("\nWarning: Could not find all target repos. "
              "Selecting alternatives...")
        # Fallback: pick from most common repos
        repos_seen = {s["repo"] for s in selected}
        for item in dataset:
            if item["repo"] not in repos_seen:
                selected.append(item)
                repos_seen.add(item["repo"])
                print(f"  ✓ Alternative: {item['repo']}: {item['instance_id']}")
                if len(selected) >= 5:
                    break

    return selected[:5]


def format_sample_metadata(instance: Dict[str, Any]) -> Dict[str, Any]:
    """Format instance metadata for YAML output."""
    return {
        "instance_id": instance["instance_id"],
        "repo": instance["repo"],
        "repo_url": f"https://github.com/{instance['repo']}",
        "base_commit": instance["base_commit"],
        "problem_statement": instance.get("problem_statement", "")[:200] + "...",
        "created_at": instance.get("created_at", ""),
        "language": "python",
    }


def main():
    parser = argparse.ArgumentParser(description="Select SWE-bench Lite instances")
    parser.add_argument("--output", type=Path, required=True,
                        help="Output YAML file")
    parser.add_argument("--analyze", action="store_true",
                        help="Show repository distribution")

    args = parser.parse_args()

    # Load dataset
    dataset = load_swe_bench_lite()

    if args.analyze:
        analyze_repository_distribution(dataset)
        return

    # Select instances
    print("\nSelecting 5 diverse instances...")
    selected = select_diverse_instances(dataset)

    if len(selected) < 5:
        print(f"\nError: Only found {len(selected)}/5 instances", file=sys.stderr)
        sys.exit(1)

    # Format output
    output = {
        "dataset": "SWE-bench Lite",
        "dataset_url": "https://github.com/princeton-nlp/SWE-bench",
        "split": "test",
        "selection_date": "2025-10-24",
        "selection_criteria": "Diversity in framework type, repo size, popularity",
        "samples": [format_sample_metadata(inst) for inst in selected],
    }

    # Write YAML
    args.output.parent.mkdir(parents=True, exist_ok=True)
    with open(args.output, "w") as f:
        yaml.dump(output, f, default_flow_style=False, sort_keys=False)

    print(f"\n✓ Selected 5 instances:")
    for sample in output["samples"]:
        print(f"  - {sample['instance_id']} ({sample['repo']})")

    print(f"\nOutput written to: {args.output}")
    print("\nNext step: Run fetch script to clone repositories")
    print(f"  python scripts/fetch-swe-bench-lite.py "
          f"--from-yaml {args.output}")


if __name__ == "__main__":
    main()
