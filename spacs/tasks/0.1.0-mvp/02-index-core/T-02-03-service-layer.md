# Task T-02-03: Service Layer (JSON-RPC Endpoints)

**Issue**: [Sub-Issue 02.03 – Service Layer](../../issues/04-0.1.0-mvp/02-index-core/03-service-layer.md)

**PRD References**: [PRD-02 §4.1](../../prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md), [PRD-05 §2-4](../../prd/0.1.0-MVP-PRDs-v0/05-api-specifications.md)

**Owners**: Rust Dev 1

**Status**: ☐ Not Started | **Week**: 3-4

---

## Objective

Expose graph and search capabilities through a JSON-RPC 2.0 HTTP service (`cds-indexd`) so CLI and agent clients can query indices with low latency.

## Deliverables

- `crates/cds-index/src/service/server.rs`
- `crates/cds-index/src/service/handlers.rs`
- `crates/cds-index/src/service/jsonrpc.rs`
- Health & metrics endpoints (`/health`, `/metrics`)
- Integration tests (`crates/cds-index/tests/service_contract_tests.rs`)

## Implementation Steps

1. **Server bootstrap**: set up Axum router, configuration loading, structured logging.
2. **JSON-RPC implementation**: request parsing, error codes, method dispatch for `search_entities`, `traverse_graph`, `retrieve_entity`.
3. **Health/metrics**: add readiness probes and Prometheus-friendly metrics scaffold.
4. **Testing**: contract tests verifying schema compliance and latency targets.

## Acceptance Criteria

- [ ] Server starts with default `127.0.0.1:9001` using env overrides (`CDS_HOST`, `CDS_PORT`).
- [ ] All JSON-RPC methods return responses matching PRD-05 schemas.
- [ ] Health endpoint returns 200 OK; metrics endpoint exposes basic counters.
- [ ] Integration tests cover success + error scenarios (invalid params, missing entity, unknown method).

## Dependencies

- **Prerequisite**: [T-02-01](T-02-01-graph-builder.md), [T-02-02](T-02-02-sparse-index.md) (need in-memory graph/searcher).
- **Blocks**: [T-03-01 Core Commands](../03-cli-tools/T-03-01-core-commands.md), [T-04-01 SDK Bootstrap](../04-agent-integration/T-04-01-sdk-bootstrap.md).

## Notes

- For v0.1.0 keep service single-threaded with shared `Arc<AppState>`; document future gRPC upgrade in v0.2.0 backlog.
