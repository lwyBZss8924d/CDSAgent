# Task T-03-01: Core Commands (search, traverse, retrieve)

**Issue**: [Sub-Issue 03.01 – Command Implementation](../../issues/04-0.1.0-mvp/03-cli-tools/01-command-impl.md)

**PRD References**: [PRD-03 §2.1](../../prd/0.1.0-MVP-PRDs-v0/03-cds-tools-cli.md), [PRD-05 §2](../../prd/0.1.0-MVP-PRDs-v0/05-api-specifications.md)

**Owners**: Rust Dev 2

**Status**: ☐ Not Started | **Week**: 4

---

## Objective

Implement CLI subcommands that wrap JSON-RPC calls to the index service, providing consistent UX for keyword search, graph traversal, and entity retrieval.

## Deliverables

- `crates/cds-tools/src/main.rs` (clap CLI entrypoint)
- `crates/cds-tools/src/commands/{search,traverse,retrieve}.rs`
- `crates/cds-tools/src/client/rpc_client.rs`
- Smoke tests under `crates/cds-tools/tests/cli_smoke_tests.rs`

## Implementation Steps

1. Define `clap` CLI structure with subcommands and shared flags (limit, output format).
2. Implement RPC client with retry/backoff and JSON parsing safeguards.
3. Wire command output to formatters (JSON/Text/Tree) with streaming support for large payloads.
4. Provide helpful exit codes and error messages for network failures or malformed responses.

## Acceptance Criteria

- [ ] `cds search`, `cds traverse`, `cds retrieve` compile and successfully call running index service.
- [ ] Commands respect global flags (e.g., `--format`, `--limit`, `--entity-type`).
- [ ] CLI handles service errors gracefully and returns non-zero exit codes with context.
- [ ] Smoke tests cover happy path + failure scenarios.

## Dependencies

- **Prerequisite**: [T-02-03 Service Layer](../02-index-core/T-02-03-service-layer.md).
- **Blocks**: [T-04-01 SDK Bootstrap](../04-agent-integration/T-04-01-sdk-bootstrap.md), [T-03-02 Output Formatting](T-03-02-output-format.md).

## Notes

- Follow LocAgent CLI semantics where possible to aid parity comparisons.
- Log command usage (with opt-out) to support telemetry once enabled.
