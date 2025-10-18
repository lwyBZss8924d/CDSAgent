# Task T-02-02: Sparse Index (Name/ID + BM25 Search)

**Issue**: [Sub-Issue 02.02 – Sparse Index](../../issues/04-0.1.0-mvp/02-index-core/02-sparse-index.md)

**PRD References**: [PRD-02 §2.1, §3.2](../../prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md), [PRD-03 §2.1](../../prd/0.1.0-MVP-PRDs-v0/03-cds-tools-cli.md)

**Owners**: Rust Dev 2

**Status**: ☐ Not Started | **Week**: 2-3

---

## Objective

Provide hierarchical sparse search that mirrors LocAgent’s upper dictionary (name/ID lookup) and lower BM25 index, enabling fast keyword → entity discovery for both CLI and agent.

## Deliverables

- `crates/cds-index/src/index/name_index.rs`
- `crates/cds-index/src/index/bm25.rs`
- Tokenizer utilities (`crates/cds-index/src/index/tokenizer.rs`)
- Benchmarks (`crates/cds-index/benches/search_bench.rs`)
- Unit tests (`crates/cds-index/tests/index_tests.rs`)

## Implementation Steps

1. **Upper index**: build `DashMap<String, Vec<EntityId>>` with prefix search and entity-type filters.
2. **Tokenizer**: implement camelCase/snake_case splitting + stop-word filtering aligned with LocAgent.
3. **BM25**: integrate Tantivy; index per-entity code bodies with k1=1.5, b=0.75 parameters.
4. **Query pipeline**: combine dictionary hits with BM25 fallback, deduplicate, sort by score.
5. **Benchmarks**: ensure search latency p95 <500 ms on 10 K-file repo snapshot.

## Acceptance Criteria

- [ ] Exact match, prefix match, and case-insensitive lookup supported.
- [ ] BM25 fallback triggered when upper index < threshold (default 5 results).
- [ ] Tokenization and scoring parity within ±5 % of LocAgent outputs on reference repos.
- [ ] Benchmarks recorded and stored in `crates/cds-index/benches/README.md`.

## Dependencies

- **Prerequisite**: [T-02-01 Graph Builder](T-02-01-graph-builder.md) (needs node metadata).
- **Blocks**: [T-02-03 Service Layer](T-02-03-service-layer.md), [T-03-01 Core Commands](../03-cli-tools/T-03-01-core-commands.md).

## Notes

- Plan for future sharding (v0.2.0 incremental updates). Keep index directory layout configurable via `GRAPH_INDEX_DIR` & `BM25_INDEX_DIR`.
