#!/usr/bin/env python3
"""
test_graph_export.py

Thread-23: Test graph export and comparison pipeline

Purpose:
    - Test CDSAgent graph export to JSON
    - Test conversion to LocAgent pickle format
    - Test comparison harness

Usage:
    python scripts/test_graph_export.py --repo-path tmp/LocAgent

This script will:
    1. Build CDSAgent graph for LocAgent repo
    2. Export to JSON
    3. Convert to LocAgent-compatible pickle
    4. Compare with golden LocAgent baseline
    5. Report findings
"""

import argparse
import json
import pickle
import subprocess
import sys
from pathlib import Path
import tempfile


def run_command(cmd: list, description: str):
    """Run a shell command and handle errors"""
    print(f"\n→ {description}")
    print(f"  Command: {' '.join(cmd)}")

    result = subprocess.run(
        cmd,
        capture_output=True,
        text=True
    )

    if result.returncode != 0:
        print(f"  ❌ FAILED")
        print(f"  STDERR: {result.stderr}")
        sys.exit(1)

    print(f"  ✅ SUCCESS")
    if result.stdout:
        print(f"  Output: {result.stdout[:200]}...")

    return result


def main():
    parser = argparse.ArgumentParser(
        description='Test graph export and comparison pipeline'
    )
    parser.add_argument(
        '--repo-path',
        default='tmp/LocAgent',
        type=Path,
        help='Path to repository for testing (default: tmp/LocAgent)'
    )
    parser.add_argument(
        '--keep-temp',
        action='store_true',
        help='Keep temporary files for inspection'
    )

    args = parser.parse_args()

    print("=" * 80)
    print("THREAD-23: Graph Export & Comparison Pipeline Test")
    print("=" * 80)

    # Create temp directory for outputs
    if args.keep_temp:
        temp_dir = Path('.artifacts/spec-tasks-T-02-02-sparse-index/diag/graph_export_test')
        temp_dir.mkdir(parents=True, exist_ok=True)
    else:
        temp_dir = Path(tempfile.mkdtemp(prefix='graph_export_test_'))

    print(f"\n📁 Working directory: {temp_dir}")

    # Paths
    json_path = temp_dir / "cdsagent_graph.json"
    pkl_path = temp_dir / "cdsagent_graph.pkl"
    locagent_golden = Path("tests/fixtures/parity/golden_outputs/graph_locagent.json")
    comparison_report = temp_dir / "comparison_report.json"

    # Step 1: Build CDSAgent graph and export to JSON (placeholder - needs Rust test)
    print("\n" + "=" * 80)
    print("STEP 1: Export CDSAgent graph to JSON")
    print("=" * 80)
    print("\n⚠️  NOTE: This step requires a Rust integration test.")
    print(f"    Expected output: {json_path}")
    print(f"    For now, skipping to Python pipeline test...")

    # For testing Python scripts, create a mock JSON file
    print(f"\n→ Creating mock CDSAgent JSON for pipeline testing...")
    mock_graph = {
        "nodes": [
            {
                "id": "LocAgent::dependency_graph/build_graph.py",
                "kind": "file",
                "display_name": "build_graph.py",
                "file_path": "dependency_graph/build_graph.py",
                "range": None,
                "attributes": {}
            },
            {
                "id": "LocAgent::dependency_graph/build_graph.py::build_graph",
                "kind": "function",
                "display_name": "build_graph",
                "file_path": "dependency_graph/build_graph.py",
                "range": {"start_line": 100, "end_line": 200},
                "attributes": {}
            }
        ],
        "edges": [
            {
                "source": "LocAgent::dependency_graph/build_graph.py",
                "target": "LocAgent::dependency_graph/build_graph.py::build_graph",
                "kind": "contain",
                "alias": None
            }
        ]
    }

    with open(json_path, 'w') as f:
        json.dump(mock_graph, f, indent=2)
    print(f"  ✅ Created mock JSON: {json_path}")

    # Step 2: Convert JSON to LocAgent pickle
    print("\n" + "=" * 80)
    print("STEP 2: Convert CDSAgent JSON to LocAgent pickle")
    print("=" * 80)

    run_command(
        [
            'python3',
            'scripts/export_graph_to_locagent.py',
            '--input', str(json_path),
            '--output', str(pkl_path),
            '--verbose'
        ],
        description="Converting to LocAgent format"
    )

    # Verify pickle was created
    if not pkl_path.exists():
        print(f"❌ ERROR: Pickle file not created: {pkl_path}")
        sys.exit(1)

    # Step 3: Load and inspect pickle
    print("\n" + "=" * 80)
    print("STEP 3: Verify pickle format")
    print("=" * 80)

    with open(pkl_path, 'rb') as f:
        G = pickle.load(f)

    print(f"  ✅ Successfully loaded NetworkX graph")
    print(f"  Nodes: {G.number_of_nodes()}")
    print(f"  Edges: {G.number_of_edges()}")

    # Step 4: Compare with LocAgent golden (if it exists)
    if locagent_golden.exists():
        print("\n" + "=" * 80)
        print("STEP 4: Compare with LocAgent golden baseline")
        print("=" * 80)

        # For this test, we'll skip the comparison since we used a mock graph
        print(f"  ⚠️  Skipping comparison (using mock graph)")
        print(f"      To test comparison, run with a real CDSAgent graph export")
    else:
        print(f"\n⚠️  LocAgent golden baseline not found: {locagent_golden}")

    # Summary
    print("\n" + "=" * 80)
    print("TEST SUMMARY")
    print("=" * 80)

    print("\n✅ Pipeline Components Verified:")
    print(f"  1. JSON format: {json_path}")
    print(f"  2. LocAgent export script: scripts/export_graph_to_locagent.py")
    print(f"  3. Pickle serialization: {pkl_path}")

    print("\n📋 Next Steps:")
    print("  1. Add Rust integration test to export real graph to JSON")
    print("  2. Run export on all 6 test fixtures")
    print("  3. Compare with LocAgent golden baselines")
    print("  4. Identify top 10 entity extraction gaps")

    if not args.keep_temp:
        print(f"\n🗑️  Cleaning up temp directory: {temp_dir}")
        import shutil
        shutil.rmtree(temp_dir)
    else:
        print(f"\n📁 Temporary files kept in: {temp_dir}")

    print("\n✅ Test complete!")


if __name__ == '__main__':
    main()
