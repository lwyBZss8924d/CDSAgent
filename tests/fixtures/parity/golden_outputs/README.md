# LocAgent Parity Validation Baselines

This directory contains golden output baselines extracted from LocAgent (Python) for parity validation of CDSAgent (Rust).

## Baseline Files

### ✅ Complete Baselines

#### Graph Structure Baselines (6/6 repos)
- `graph_locagent.json` - LocAgent codebase (658 nodes: 86 classes, 478 functions)
- `graph_django__django-10914.json` - Django @ e7fd69d (6,876 nodes: 1,598 classes, 4,176 functions)
- `graph_scikit-learn__scikit-learn-10297.json` - Scikit-learn @ b90661d (6,613 nodes: 556 classes, 5,468 functions)
- `graph_matplotlib__matplotlib-18869.json` - Matplotlib @ b7d0591 (1,304 nodes: 121 classes, 617 functions)
- `graph_pytest-dev__pytest-11143.json` - Pytest @ 6995257 (5,991 nodes: 648 classes, 5,030 functions)
- `graph_psf__requests-1963.json` - Requests @ 110048f (752 nodes: 118 classes, 548 functions)

**Schema**: Each graph JSON contains:
```json
{
  "nodes": [{"id": "file.py:ClassName", "type": "class|function|file|directory", "name": "...", "file": "...", "line": 123}],
  "edges": [{"source": "...", "target": "...", "type": "contains|imports|invokes|inherits"}],
  "metadata": {"repo": "...", "commit": "...", "node_counts": {...}, "edge_counts": {...}}
}
```

#### Traverse Baselines (60/60 scenarios)
- `traverse_samples.jsonl` - 10 traversal scenarios × 6 repos
  - Callees 1-hop (5 scenarios)
  - Subclasses (3 scenarios)
  - Imports (2 scenarios)

**Schema**: Each line is a JSON object:
```json
{
  "scenario": "callees_1hop_function_1",
  "start_entity": "file.py:function_name",
  "direction": "downstream",
  "edge_types": ["invokes"],
  "max_depth": 1,
  "total_results": 3,
  "results": [...],
  "graph_text": "digraph {...}",
  "repo": "LocAgent"
}
```

### ⚠️ Partial Baselines (Known Limitation)

#### Search Baselines (1/6 repos - LocAgent only)
- `search_queries.jsonl` - BM25 search results for 50 queries
  - ✅ LocAgent: 50/50 queries
  - ❌ SWE-bench repos: Failed due to llama-index limitation (see below)

#### Performance Baselines (1/6 repos - LocAgent only)
- `performance_baselines.json` - Timing and memory metrics
  - ✅ LocAgent: Graph build, search, traverse timings
  - ❌ SWE-bench repos: Failed due to same llama-index limitation

## Known Limitations

### Search & Performance Baseline Extraction Issue

**Problem**: `llama-index` (v0.11.22) `SimpleDirectoryReader` with `required_exts=['.py']` validates file existence at the directory root level before recursing. For SWE-bench repos where Python code lives in subdirectories (e.g., `django/django/`, `sklearn/sklearn/`), this validation fails with:

```
ValueError: No files found in /path/to/repo.
```

**Impact**:
- Search and performance baselines could only be extracted for LocAgent (which has `.py` files at root level)
- SWE-bench repo baselines are missing from `search_queries.jsonl` and `performance_baselines.json`

**Why This Doesn't Block CDSAgent Development**:
1. **Graph baselines (most critical)**: ✅ Complete for all 6 repos
2. **Traverse baselines (important)**: ✅ Complete for all 60 scenarios
3. **Search baselines (secondary)**: Only needed for BM25 parity validation, which CDSAgent will implement independently in Rust using `tantivy` or similar, without llama-index dependencies
4. **Performance baselines (supplementary)**: CDSAgent will generate its own benchmarks during T-08-04

**Root Cause**:
- This is a limitation of LocAgent's dependency (`llama-index`), not a design issue
- LocAgent's `plugins/location_tools/retriever/bm25_retriever.py` uses `SimpleDirectoryReader` directly

**Workaround Attempts**:
1. ❌ Package directory detection - llama-index validates before accepting subdirectory paths
2. ❌ Dummy file creation - llama-index caches directory listing or excludes dotfiles
3. ✅ **Accepted limitation** - Document and proceed; CDSAgent Rust implementation will not have this issue

**Future Resolution**:
- When CDSAgent implements BM25 search in Rust (T-02-02), it will use standard filesystem traversal (`walkdir`) without llama-index limitations
- Search and performance parity can be validated directly against live SWE-bench repos using CDSAgent's implementation

## Usage

### For T-02-01 (Graph Builder)
Compare CDSAgent graph output against `graph_*.json` baselines:
```bash
# After implementing CDSAgent graph builder
./scripts/parity-check.sh graph django__django-10914
# Expects: Node count variance ≤2%, edge types match
```

### For T-02-02 (Sparse Index)
CDSAgent will implement BM25 independently:
```bash
# CDSAgent BM25 search implementation
cds-tools search "query" --repo django__django-10914
# Compare results with LocAgent search (run live, not from baseline)
```

### For T-08-03 (Parity Validation)
Primary validation uses graph + traverse baselines:
```bash
./scripts/parity-check.sh validate-all
# Graph structure: ✅ Use baselines
# Traverse patterns: ✅ Use traverse_samples.jsonl
# Search overlap: ⚠️ Run live comparison (both systems)
```

## Regeneration

To regenerate baselines (if LocAgent updates or methodology changes):

```bash
# Fetch SWE-bench repos
export HF_TOKEN="your_token"
./scripts/swe-lite fetch

# Extract all baseline types
./scripts/swe-lite baseline graph   # ✅ Works for all repos
./scripts/swe-lite baseline traverse # ✅ Works for all repos
./scripts/swe-lite baseline search  # ⚠️ LocAgent only
./scripts/swe-lite baseline perf    # ⚠️ LocAgent only
```

## References

- Methodology: `docs/parity-validation-methodology.md`
- Task specification: `spacs/tasks/0.1.0-mvp/06-refactor-parity/T-06-01-parity-methodology.md`
- Sample selection: `tests/fixtures/parity/swe-bench-lite/samples.yaml`
- LocAgent source: `tmp/LocAgent/`

---

**Last Updated**: 2025-10-24
**LocAgent Version**: Commit ca8b167 (2024-03-15)
**Graph Version**: v2.3
**SWE-bench Lite**: 5 samples selected for diversity
