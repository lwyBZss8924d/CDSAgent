# Smoke Build Caching Plan

Slow smoke builds (django ~4 min, scikit-learn ~2 min) dominate the Session-05 runtime.
This note outlines the caching/batching plan so we can keep running those repos in CI
without burning the entire budget.

## Goals

1. Reuse graph + sparse index artifacts between smoke runs unless the source repo or
   index code changed.
2. Allow per-repo overrides (e.g., force rebuild for flaky repos).
3. Integrate with the existing `smoke_multi_repo` test so contributors get quick local
   feedback before CI.

## Proposed Workflow

| Step | Action |
| ---- | ------ |
| 1 | Compute a cache key per repo: `{repo_name}:{git_sha_of_repo}:{git_sha_of_index}` |
| 2 | Before invoking `GraphBuilder`, look for `~/.cdsagent-smoke-cache/{cache_key}` |
| 3 | If present, reuse the serialized graph + sparse index (just load into `SparseIndex::from_dir`) |
| 4 | If missing/stale, rebuild graph/index once, then store them under the cache key |
| 5 | Expose `SMOKE_FORCE_REBUILD=repo1,repo2` to bypass the cache when debugging |

## Implementation Notes

- **Cache layout**: `~/.cdsagent-smoke-cache/{repo}/{cache_key}/graph.tar.zst` and
  `.../sparse_index/`. Using tar+zstd keeps copies small and portable.
- **Hashing strategy**: combine (
    1. repo HEAD commit via `git -C repo rev-parse HEAD`,
    2. the sparse-index crate hash via `git rev-parse HEAD -- crates/cds-index`, and
    3. the GraphBuilder config hash (so different settings get unique caches).
- **Integration point**: wrap `GraphBuilder::build()` and `SparseIndex::from_graph()` in
  a helper (e.g., `tests/support/cache.rs`) that coordinates reads/writes.
- **CI**: teach the GitHub workflow to persist `~/.cdsagent-smoke-cache` between jobs so
  nightly runs only rebuild when code changes.
- **Telemetry**: extend `smoke_multi_repo` output with `cache_hit`, `cache_miss`, and
  `rebuild_seconds` so we can monitor cache health.

## Next Steps

1. Land the helper module + CLI switches in a follow-up PR.
2. Populate the cache with the current six smoke repos on the nightly builder.
3. After cache hit-rate is healthy, re-enable the slow repos in CI by default.
