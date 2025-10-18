# Task T-05-01: JSON-RPC Schema Definition & Validation

**Issue**: [API Contracts](../../issues/04-0.1.0-mvp/05-api-contracts.md)

**PRD References**: [PRD-05 §2-4](../../prd/0.1.0-MVP-PRDs-v0/05-api-specifications.md), [PRD-02 §4.1](../../prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md)

**Owners**: Rust Dev 1 (service) & TS Dev 1 (schema tooling)

**Status**: ☐ Not Started | **Week**: 3

---

## Objective

Formalize JSON-RPC request/response schemas for `search_entities`, `traverse_graph`, and `retrieve_entity`, and enforce validation in the service and clients.

## Deliverables

- `docs/api/jsonrpc-schema.json` (JSON Schema draft)
- Schema validation tests in `crates/cds-index/tests/service_contract_tests.rs`
- CLI/agent schema fixtures `tests/fixtures/api/`

## Implementation Steps

1. Capture response examples from PRD-05 and convert to JSON Schema definitions (using `serde_json::Value`).
2. Wire schema validation into service tests (load schema, validate response before assertions).
3. Provide schema bundle to TypeScript client for runtime validation (optional but recommended).
4. Document versioning (`v0.1.0`) and update README.

## Acceptance Criteria

- [ ] Schema file published and referenced from docs.
- [ ] Service contract tests fail if response deviates from schema.
- [ ] CLI integration test validates JSON output via schema.
- [ ] Schema versioning plan recorded (for future breaking changes).

## Dependencies

- Requires T-02-03 (service methods) to exist.
- Blocks TypeScript bindings, CLI formatter tests.

## Notes

- Consider using `schemars` to derive schema from `serde` structs for consistency.
