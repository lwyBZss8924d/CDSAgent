# LocAgent Parity Test Fixtures

This directory contains golden outputs and test data for validating CDSAgent's Rust implementation maintains algorithmic fidelity with LocAgent's Python codebase.

## Directory Structure

```tree
tests/fixtures/parity/
├── README.md                # This file
├── locagent_version.txt     # LocAgent baseline version tracking
├── locagent_repo/           # LocAgent's own codebase (ground truth)
├── sample_repos/            # Sample repos from SWE-bench Lite
│   ├── repo1_name/
│   ├── repo2_name/
│   ├── repo3_name/
│   ├── repo4_name/
│   └── repo5_name/
└── golden_outputs/          # Expected outputs from LocAgent
    ├── graph_locagent.json          # LocAgent's graph structure
    ├── graph_sample_repos.json      # Sample repos graph structures
    ├── search_queries.jsonl         # 50 queries + expected top-10 results
    ├── traverse_samples.jsonl       # 10 traversal cases + expected outputs
    └── performance_baselines.json   # Performance metrics (time, memory)
```

## Purpose

These fixtures serve three critical purposes:

1. **Algorithm Validation**: Ensure Rust refactoring produces identical results to Python
2. **Regression Prevention**: Catch unintended deviations during development
3. **Performance Benchmarking**: Validate 2-5x performance improvements over Python

## Regenerating Golden Outputs

### Prerequisites

(1) **LocAgent Environment Setup**

```bash
# Navigate to LocAgent reference implementation
cd tmp/LocAgent

# Setup Python environment (if not already done)
conda create -n locagent python=3.12 -y
conda activate locagent
pip install -r requirements.txt

# Set environment variables
export PYTHONPATH="$PYTHONPATH:$(pwd)"
export GRAPH_INDEX_DIR="tests/fixtures/parity/golden_outputs/graph_index"
export BM25_INDEX_DIR="tests/fixtures/parity/golden_outputs/bm25_index"
```

(2) **Sample Repository Selection** (from SWE-bench Lite)

Select 5 small-to-medium Python repositories with diverse characteristics:

- **Criteria**:
  - Size: 50-500 Python files
  - Complexity: Mix of simple and complex class hierarchies
  - Dependencies: Various import patterns
  - Coverage: Different coding styles (Django, Flask, scientific, CLI tools)

- **Recommended Repos** (examples from SWE-bench Lite):
  - `pytest-dev/pytest` (testing framework)
  - `pallets/flask` (web framework)
  - `django/django` (web framework - subset)
  - `requests/requests` (HTTP library)
  - `scikit-learn/scikit-learn` (ML library - subset)

### Extraction Steps

#### 1. Extract Graph Structure Baseline

```bash
# For LocAgent's own codebase
cd tmp/LocAgent
python dependency_graph/build_graph.py \
  --repo_path . \
  --output ../../tests/fixtures/parity/golden_outputs/graph_locagent.json

# For each sample repo
for repo in tests/fixtures/parity/sample_repos/*/; do
    repo_name=$(basename "$repo")
    python dependency_graph/build_graph.py \
      --repo_path "$repo" \
      --output "../../tests/fixtures/parity/golden_outputs/graph_${repo_name}.json"
done
```

**Expected Output Format** (JSON):

```json
{
  "repository": "LocAgent",
  "total_nodes": 1234,
  "total_edges": 5678,
  "node_counts_by_type": {
    "directory": 50,
    "file": 200,
    "class": 300,
    "function": 684
  },
  "edge_counts_by_type": {
    "contain": 2000,
    "import": 1500,
    "invoke": 1800,
    "inherit": 378
  },
  "nodes": [...],  // Full node list with qualified names
  "edges": [...]   // Full edge list
}
```

#### 2. Extract Search Query Baselines

Create 50 diverse search queries covering:

- **Exact name matches** (10 queries): `"User"`, `"parse_file"`, etc.
- **Partial matches** (10 queries): `"sanitize"`, `"validate"`, etc.
- **Multi-word queries** (10 queries): `"authentication middleware"`, `"database connection"`, etc.
- **camelCase/snake_case** (10 queries): `"getUserData"`, `"parse_ast_node"`, etc.
- **Edge cases** (10 queries): Single char `"x"`, numbers `"404"`, special chars `"@property"`, etc.

```bash
# Run search queries on LocAgent repo
python plugins/location_tools/locationtools.py search_entities \
  --repo_path . \
  --queries_file ../../tests/fixtures/parity/search_queries.txt \
  --limit 10 \
  --output ../../tests/fixtures/parity/golden_outputs/search_queries.jsonl
```

**Expected Output Format** (JSONL - one per query):

```jsonl
{"query": "User", "top_10_results": [{"id": "...", "name": "...", "score": 0.95}, ...], "used_upper_index": true, "used_bm25": false}
{"query": "sanitize input", "top_10_results": [...], "used_upper_index": false, "used_bm25": true}
...
```

#### 3. Extract Traversal Baselines

Create 10 traversal scenarios:

- **Containment traversal** (2): Directory → Files → Classes → Functions
- **Import traversal** (2): Module → Imported modules (depth=1, depth=2)
- **Invoke traversal** (2): Function → Called functions (depth=1, depth=2)
- **Inherit traversal** (2): Class → Parent classes, child classes
- **Mixed relation** (2): Multi-hop with all relations

```bash
# Run traversal scenarios
python plugins/location_tools/locationtools.py traverse_graph \
  --repo_path . \
  --scenarios_file ../../tests/fixtures/parity/traverse_scenarios.txt \
  --output ../../tests/fixtures/parity/golden_outputs/traverse_samples.jsonl
```

**Expected Output Format** (JSONL - one per scenario):

```jsonl
{"scenario": "contain_depth_2", "start_entities": ["src/"], "depth": 2, "relations": ["contain"], "result_nodes": [...], "result_edges": [...]}
...
```

#### 4. Extract Performance Baselines

```bash
# Benchmark index building
time python dependency_graph/build_graph.py \
  --repo_path tests/fixtures/parity/sample_repos/pytest \
  --output /dev/null 2>&1 | tee ../../tests/fixtures/parity/golden_outputs/perf_index_build.log

# Benchmark search queries
time python plugins/location_tools/locationtools.py search_entities \
  --repo_path . \
  --query "User" \
  --iterations 100 2>&1 | tee ../../tests/fixtures/parity/golden_outputs/perf_search.log

# Benchmark traversal
time python plugins/location_tools/locationtools.py traverse_graph \
  --repo_path . \
  --start "src/" \
  --depth 2 \
  --iterations 100 2>&1 | tee ../../tests/fixtures/parity/golden_outputs/perf_traverse.log
```

**Expected Performance Baselines** (from PRD-06 §5.3):

| Metric | LocAgent (Python) | CDSAgent (Rust) Target |
|--------|------------------|----------------------|
| Index 1K files | ~5s | <3s (1.6x faster) |
| Search query | ~200ms | <100ms (2x faster) |
| Traverse 2-hop | ~500ms | <200ms (2.5x faster) |
| Memory (10K files) | ~3GB | <2GB |

### Validation Thresholds

When comparing CDSAgent output to these baselines:

1. **Graph Structure**:
   - Node count variance: ≤2%
   - Edge count variance: ≤2%
   - Qualified name format: Exact match

2. **Search Results**:
   - Top-10 overlap: ≥90% (at least 9/10 results match)
   - Score tolerance: ±0.01 (floating-point precision)
   - Rank order: Exact match for top-5

3. **Traversal**:
   - Node set: Exact match (100%)
   - Edge set: Exact match (100%)
   - Order: Not required to match (BFS implementation details)

4. **Performance**:
   - Index build time: ≤60% of Python baseline
   - Search latency: ≤50% of Python baseline
   - Memory usage: ≤67% of Python baseline

## Usage in Tests

### Automated Parity Checks

Use the `scripts/parity-check.sh` automation:

```bash
# Run all parity checks
./scripts/parity-check.sh --all

# Run specific checks
./scripts/parity-check.sh --graph
./scripts/parity-check.sh --search
./scripts/parity-check.sh --traverse
./scripts/parity-check.sh --performance
```

### Manual Validation

```bash
# Compare graph structure
diff <(jq -S . tests/fixtures/parity/golden_outputs/graph_locagent.json) \
     <(jq -S . target/debug/cds_graph_output.json)

# Validate search results
cargo test --test search_parity_tests -- --nocapture

# Validate traversal outputs
cargo test --test traverse_parity_tests -- --nocapture
```

## Maintenance

### When to Update Baselines

1. **LocAgent Version Update**: When upgrading to a new LocAgent version
2. **Algorithm Change**: When intentionally changing an algorithm (document in Issue-06)
3. **Bug Fix**: If LocAgent baseline was incorrect, update with justification

### Update Procedure

1. Document reason for update in `CHANGELOG.md`
2. Re-run extraction steps (see above)
3. Update `locagent_version.txt` with new version
4. Create PR with updated baselines and rationale
5. Require approval from Rust Lead + Tech Lead

## Related Documentation

- **Parity Methodology**: `docs/parity-validation-methodology.md`
- **Automation Script**: `scripts/parity-check.sh`
- **Issue Tracking**: `spacs/issues/04-0.1.0-mvp/06-refactor-parity.md`
- **PRD Reference**: `spacs/prd/0.1.0-MVP-PRDs-v0/06-rust-refactoring-plan.md`
- **Task Spec**: `spacs/tasks/0.1.0-mvp/06-refactor-parity/T-06-01-parity-methodology.md`

---

**Created**: 2025-10-20
**Last Updated**: 2025-10-20
**Maintained By**: Rust Lead + All Rust Developers
