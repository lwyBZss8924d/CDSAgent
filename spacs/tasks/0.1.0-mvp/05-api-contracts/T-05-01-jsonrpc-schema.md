# Task T-05-01: JSON-RPC Schema Definition & Validation

**Issue**: [API Contracts](../../issues/04-0.1.0-mvp/05-api-contracts.md)

**PRD References**: [PRD-05 §2-4](../../prd/0.1.0-MVP-PRDs-v0/05-api-specifications.md), [PRD-02 §4.1](../../prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md)

**Owners**: Rust Dev 1 (service) & TS Dev 1 (schema tooling)

**Status**: ✅ Completed | **Week**: 1 (2025-10-19)

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

- [x] Schema file published and referenced from docs.
- [x] Service contract tests fail if response deviates from schema.
- [x] CLI integration test validates JSON output via schema.
- [x] Schema versioning plan recorded (for future breaking changes).

## Dependencies

- Requires T-02-03 (service methods) to exist.
- Blocks TypeScript bindings, CLI formatter tests.

## Notes

- Consider using `schemars` to derive schema from `serde` structs for consistency.

---

## commit message

```text
Implement comprehensive JSON-RPC API schema definition and validation for
CDS-Index Service, establishing contract-first development for v0.1.0.

## Schema Definition

- docs/api/jsonrpc-schema.json: Complete JSON Schema (Draft 7) for all 4
  JSON-RPC methods with proper type definitions
- Snippet fields made optional based on snippet_mode parameter:
  - 'fold' mode: only fold field present
  - 'preview' mode: fold + preview fields
  - 'full' mode: all three fields (fold, preview, full)
- All entity types, relation types, and error codes properly enumerated
- Custom error codes documented (-32001 to -32004)

## Schema Validation Tests

- crates/cds-index/tests/service_contract_tests.rs: 25 passing tests
- Validates JSON-RPC response format (jsonrpc, id, result/error fields)
- Validates entity schema with jsonschema crate
- Tests snippet field variations (fold-only, fold+preview, full)
- Tests error format compliance with JSON-RPC 2.0 spec
- Validates backward compatibility (new optional fields ignored by old clients)
- Schema embedded at compile time via include_str! for reliability

## Documentation

- docs/api/README.md: Complete API documentation with examples
- docs/api/error-codes.md: Error code catalogue with recovery actions
- docs/api/versioning.md: API versioning strategy and compatibility policy
- Test fixtures in tests/fixtures/api/ for validation examples
- README.md updated with API documentation links

## Worklog & Tracking

- Initialized task worklog in .artifacts/spec-tasks-T-05-01-jsonrpc-schema/
- Created daily worklog template for 2025-10-19
- Tracked task metadata (owner, dependencies, acceptance criteria)

## Acceptance Criteria Met

- [x] Schema file published at docs/api/jsonrpc-schema.json
- [x] Service contract tests validate responses against schema
- [x] Test fixtures demonstrate schema compliance
- [x] Schema versioning plan recorded (v0.1.0)
- [x] Error codes documented with examples
- [x] All 25 tests pass with schema validation

## Technical Details

- Used jsonschema 0.18 for Draft 7 validation
- Inline schema for entity validation (avoids $ref resolution issues)
- Timestamps aligned to 2025-10-19 (task start date)
- Fixed relative paths in documentation links

Blocks: T-02-03 (Service Layer), T-05-02 (TypeScript Bindings)
Related: PRD-05 §2-4, Issue-05
```
