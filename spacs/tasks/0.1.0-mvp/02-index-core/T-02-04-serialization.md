# Task T-02-04: Serialization & Test Fixtures

**Issue**: [Sub-Issue 02.04 – Serialization Fixtures](../../issues/04-0.1.0-mvp/02-index-core/04-serialization-fixtures.md)

**PRD References**: [PRD-02 §4.4](../../prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md), [PRD-08 §2.1](../../prd/0.1.0-MVP-PRDs-v0/08-testing-quality.md)

**Owners**: Rust Dev 2

**Status**: ☐ Not Started | **Week**: 4

---

## Objective

Persist graph and index artifacts to disk and provide reusable fixtures for integration tests, ensuring reproducibility across CLI, agent, and deployment workflows.

## Deliverables

- `crates/cds-index/src/persistence/mod.rs` (serialization APIs)
- Storage layout docs (`docs/index-storage.md`)
- Test fixtures under `tests/fixtures/{graphs,bm25}`
- Serialization integration tests (`crates/cds-index/tests/serialization_tests.rs`)

## Implementation Steps

1. Define `GraphSnapshot`/`IndexSnapshot` structs with Serde (JSON for metadata, bincode for payloads).
2. Implement `save_graph/load_graph` and equivalent for BM25 index directories.
3. Generate fixtures from sample repos and add checksum validation.
4. Wire persistence into service bootstrap and CLI `cds init` command.

## Acceptance Criteria

- [ ] Graph and BM25 indices persist to directories defined by `GRAPH_INDEX_DIR`/`BM25_INDEX_DIR`.
- [ ] Loading snapshots reproduces node/edge counts and index stats exactly.
- [ ] Fixtures available for tests and documented in `docs/index-storage.md`.
- [ ] Serialization error handling produces actionable messages and exit codes.

## Dependencies

- **Prerequisite**: [T-02-01](T-02-01-graph-builder.md), [T-02-02](T-02-02-sparse-index.md).
- **Blocks**: [T-03-03 Integration Tests](../03-cli-tools/T-03-03-integration-tests.md), [T-07-02 Env Config](../07-deployment/T-07-02-env-config.md).

## Notes

- Plan ahead for incremental update support (v0.2.0); keep snapshot format extensible with version metadata.
