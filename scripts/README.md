# CDSAgent Development Scripts

This directory contains automation scripts for development, testing, and parity validation.

## Quick Reference

### Parity Baseline Extraction

```bash
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

```bash
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

### `worktree-symlink.sh`

Manages git worktree symlinks for parallel task development.

**Usage:**

```bash
# Create symlinks for all worktrees in ~/dev-space/
./scripts/worktree-symlink.sh create

# List all worktrees and their symlinks
./scripts/worktree-symlink.sh list

# Remove all CDSAgent worktree symlinks
./scripts/worktree-symlink.sh remove

# Show help
./scripts/worktree-symlink.sh help
```

**Features:**

- Automatically creates symlinks for all task worktrees
- Symlinks pattern: `~/dev-space/CDSAgent-T-XX-XX-task-name`
- Makes worktrees easier to access in IDEs
- Color-coded output for better visibility

**Examples:**

```bash
# After creating new worktrees
git worktree add .worktrees/T-08-01-unit-tests -b feat/task/T-08-01-unit-tests main
./scripts/worktree-symlink.sh create

# Access via symlink in IDE
code ~/dev-space/CDSAgent-T-08-01-unit-tests
```

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

### Task Management Scripts

| Script | Purpose |
|--------|---------|
| `create-task-worklog.sh` | Initialize task worklog structure |
| `create-daily-worklog.sh` | Create daily worklog entries |
| `sync-worktrees.sh` | Sync all worktrees with main branch |

**Documentation:** [Worktree Workflow SOP](../docs/WORKTREE_WORKFLOW.md)

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
| Task worklogs | `.artifacts/spec-tasks-T-XX-XX/worklogs/` | ✅ Yes |

---

## Future Scripts

Planned development scripts:

- `test-runner.sh` - Run tests across all worktrees
- `setup-dev.sh` - One-command development environment setup

## Contributing

When adding new scripts:

1. Follow the naming convention: `kebab-case.sh`
2. Add execute permissions: `chmod +x scripts/new-script.sh`
3. Include help/usage documentation
4. Update this README with script description
5. Test on macOS and Linux if possible

---

**See Also:**

- [Worktree Workflow Documentation](../docs/WORKTREE_WORKFLOW.md)
- [Development Guide](../README.md#development)
