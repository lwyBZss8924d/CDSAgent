#!/usr/bin/env python3
"""
Analyze diagnostic JSON files to identify failure mode patterns.

Categories:
1. RANKING_ISSUE: Low overlap@10 but high overlap@20/50 (we find files but rank them poorly)
2. RETRIEVAL_GAP: Low overlap across all cutoffs (we're missing files entirely)
3. PERFORMING_WELL: High overlap@10 (≥75%)
"""

import json
import sys
from pathlib import Path
from typing import Dict, List, Tuple

DIAG_DIR = Path(".artifacts/spec-tasks-T-02-02-sparse-index/diag")

def load_diagnostics(diag_path: Path) -> List[Dict]:
    """Load diagnostic JSON file."""
    with open(diag_path) as f:
        return json.load(f)

def classify_query(query: Dict) -> str:
    """Classify query failure mode based on overlap patterns."""
    ov10 = query["overlap_at_10"]
    ov20 = query["overlap_at_20"]
    ov50 = query["overlap_at_50"]

    # High performance
    if ov10 >= 75.0:
        return "PERFORMING_WELL"

    # Ranking issue: found files but ranked poorly
    # Significant improvement from @10 to @20 or @50
    if (ov20 - ov10 >= 20.0) or (ov50 - ov10 >= 30.0):
        return "RANKING_ISSUE"

    # Retrieval gap: consistently low across all cutoffs
    if ov50 < 60.0:
        return "RETRIEVAL_GAP"

    # Edge case: moderate performance
    return "MODERATE"

def analyze_repo(repo_name: str, diagnostics: List[Dict]) -> Dict:
    """Analyze diagnostics for a single repo."""
    categories = {
        "PERFORMING_WELL": [],
        "RANKING_ISSUE": [],
        "RETRIEVAL_GAP": [],
        "MODERATE": []
    }

    for query in diagnostics:
        category = classify_query(query)
        categories[category].append({
            "query_id": query["query_id"],
            "query_text": query["query_text"],
            "overlap_at_10": query["overlap_at_10"],
            "overlap_at_20": query["overlap_at_20"],
            "overlap_at_50": query["overlap_at_50"],
            "loc_only_count": len(query["loc_only_top10"]),
            "cds_only_count": len(query["cds_only_top10"])
        })

    # Summary statistics
    total_queries = len(diagnostics)
    avg_ov10 = sum(q["overlap_at_10"] for q in diagnostics) / total_queries
    avg_ov20 = sum(q["overlap_at_20"] for q in diagnostics) / total_queries
    avg_ov50 = sum(q["overlap_at_50"] for q in diagnostics) / total_queries

    return {
        "repo": repo_name,
        "total_queries": total_queries,
        "avg_overlap_at_10": round(avg_ov10, 2),
        "avg_overlap_at_20": round(avg_ov20, 2),
        "avg_overlap_at_50": round(avg_ov50, 2),
        "categories": categories,
        "category_counts": {
            category: len(queries)
            for category, queries in categories.items()
        }
    }

def print_summary(results: List[Dict]):
    """Print summary of analysis."""
    print("=" * 80)
    print("DIAGNOSTIC PATTERN ANALYSIS")
    print("=" * 80)
    print()

    for result in results:
        repo = result["repo"]
        total = result["total_queries"]

        print(f"Repository: {repo}")
        print(f"  Total queries: {total}")
        print(f"  Avg overlap@10: {result['avg_overlap_at_10']:.2f}%")
        print(f"  Avg overlap@20: {result['avg_overlap_at_20']:.2f}%")
        print(f"  Avg overlap@50: {result['avg_overlap_at_50']:.2f}%")
        print()
        print(f"  Category breakdown:")
        for category, count in result["category_counts"].items():
            pct = (count / total) * 100
            print(f"    {category:20s}: {count:3d} ({pct:5.1f}%)")
        print()

        # Show top 3 ranking issues for this repo
        ranking_issues = result["categories"]["RANKING_ISSUE"]
        if ranking_issues:
            print(f"  Top 3 ranking issues:")
            sorted_issues = sorted(ranking_issues, key=lambda x: x["overlap_at_20"] - x["overlap_at_10"], reverse=True)
            for i, issue in enumerate(sorted_issues[:3], 1):
                gap = issue["overlap_at_20"] - issue["overlap_at_10"]
                print(f"    {i}. \"{issue['query_text']}\"")
                print(f"       @10={issue['overlap_at_10']:.1f}% @20={issue['overlap_at_20']:.1f}% (gap={gap:.1f}%)")
        print()

    # Global summary
    print("=" * 80)
    print("GLOBAL SUMMARY")
    print("=" * 80)
    total_queries = sum(r["total_queries"] for r in results)
    global_categories = {cat: 0 for cat in ["PERFORMING_WELL", "RANKING_ISSUE", "RETRIEVAL_GAP", "MODERATE"]}
    for result in results:
        for category, count in result["category_counts"].items():
            global_categories[category] += count

    print(f"Total queries across all repos: {total_queries}")
    print()
    print("Global category distribution:")
    for category, count in global_categories.items():
        pct = (count / total_queries) * 100
        print(f"  {category:20s}: {count:3d} ({pct:5.1f}%)")
    print()

    # Key insight
    ranking_pct = (global_categories["RANKING_ISSUE"] / total_queries) * 100
    retrieval_pct = (global_categories["RETRIEVAL_GAP"] / total_queries) * 100
    print(f"Key insight: {ranking_pct:.1f}% of queries have RANKING issues (files found but ranked poorly)")
    print(f"             {retrieval_pct:.1f}% of queries have RETRIEVAL gaps (files missing entirely)")
    print()

def main():
    """Main entry point."""
    # Find all diagnostic JSON files
    diag_files = sorted(DIAG_DIR.glob("*_query_diagnostics.json"))

    if not diag_files:
        print(f"ERROR: No diagnostic JSON files found in {DIAG_DIR}", file=sys.stderr)
        sys.exit(1)

    results = []
    for diag_file in diag_files:
        repo_name = diag_file.stem.replace("_query_diagnostics", "")
        diagnostics = load_diagnostics(diag_file)
        result = analyze_repo(repo_name, diagnostics)
        results.append(result)

    print_summary(results)

    # Write detailed JSON output
    output_file = DIAG_DIR / "analysis_summary.json"
    with open(output_file, "w") as f:
        json.dump(results, f, indent=2)

    print(f"Detailed analysis written to: {output_file}")
    print()

if __name__ == "__main__":
    main()
