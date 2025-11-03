# CDSAgent Development Scripts

This directory contains automation scripts for development, testing, and parity validation.

## Quick Reference

### Parity Baseline Extraction

```shell
./scripts/swe-lite check        # Verify environment
./scripts/swe-lite select       # Select 5 SWE-bench Lite instances
./scripts/swe-lite fetch        # Clone repositories
./scripts/swe-lite baseline all # Extract all baselines
```

See [`./scripts/swe-lite help`](./swe-lite) for full documentation.

## Available Scripts

### `swe-lite` - Parity Baseline Extraction CLI ⭐ NEW

Unified CLI for extracting LocAgent parity baselines from SWE-bench Lite repositories.

**Usage:**

```shell
# Complete workflow
./scripts/swe-lite check        # Verify environment
./scripts/swe-lite select       # Select instances → samples.yaml
./scripts/swe-lite fetch        # Clone repos → .artifacts/tmp/
./scripts/swe-lite baseline all # Extract all baselines

# Individual baseline types
./scripts/swe-lite baseline graph      # Graph (nodes + edges)
./scripts/swe-lite baseline search     # BM25 search (50 queries)
./scripts/swe-lite baseline traverse   # Graph traversal (10 scenarios)
./scripts/swe-lite baseline perf       # Performance metrics
```

**Features:**

- Uses `uv` for Python dependency management
- Extracts baselines from LocAgent + 5 SWE-bench Lite instances
- No vendored source code (repos in .artifacts/tmp/)
- Automated dependency installation
- Color-coded output

**Documentation:** [Section 10 - Baseline Extraction CLI](../docs/parity-validation-methodology.md#10-baseline-extraction-cli)

---

### Underlying Python Scripts

The `swe-lite` CLI wraps these Python scripts (for advanced usage):

| Script | Purpose |
|--------|---------|
| `select-swe-bench-instances.py` | Select diverse instances from SWE-bench Lite dataset |
| `fetch-swe-bench-lite.py` | Clone repos at specific commits |
| `extract-parity-baseline.py` | Extract graph with full nodes/edges |
| `extract-search-baseline.py` | Extract BM25 search results (50 queries) |
| `extract-traverse-baseline.py` | Extract graph traversal results (10 scenarios) |
| `benchmark-performance.py` | Measure build/search/traverse timing + memory |

All scripts use `uv run` for dependency management and set `PYTHONPATH` to include LocAgent.

---

## Output Locations

| Artifact | Location | Tracked in Git? |
|----------|----------|-----------------|
| SWE-bench instance metadata | `tests/fixtures/parity/swe-bench-lite/samples.yaml` | ✅ Yes |
| Cloned repositories | `.artifacts/tmp/swe-bench-lite/<id>/` | ❌ No (gitignored) |
| Graph baselines | `tests/fixtures/parity/golden_outputs/graph_*.json` | ✅ Yes |
| Search baselines | `tests/fixtures/parity/golden_outputs/search_queries.jsonl` | ✅ Yes |
| Traversal baselines | `tests/fixtures/parity/golden_outputs/traverse_samples.jsonl` | ✅ Yes |
| Performance baselines | `tests/fixtures/parity/golden_outputs/performance_baselines.json` | ✅ Yes |

---

## Contributing

When adding new scripts:

1. Follow the naming convention: `kebab-case.sh`
2. Add execute permissions: `chmod +x scripts/new-script.sh`
3. Include help/usage documentation
4. Update this README with script description
5. Test on macOS and Linux if possible

---

## Smoke Testing on External Repositories

To verify the generic BM25 pipeline on arbitrary repositories, run the ignored smoke test with a comma-separated list of repo roots:

```
SMOKE_REPO_PATHS="/path/to/django,/path/to/scikit-learn" cargo test -p cds-index smoke_sparse_index_builds_for_external_repos -- --ignored
```

The test simply builds the graph and sparse index for each repository and fails if any step cannot complete.
