# Tasks: API Specifications & Schema Validation

**Work Stream**: Issue-05: API Contracts
**Issue Reference**: [../../issues/04-0.1.0-mvp/05-api-contracts.md](../../issues/04-0.1.0-mvp/05-api-contracts.md)
**PRD Reference**: [PRD-05: API Specifications](../../prd/0.1.0-MVP-PRDs-v0/05-api-specifications.md)

## Task Overview

| Task ID | Title | Owner | Status | Week |
|---------|-------|-------|--------|------|
| T-05-01 | JSON-RPC Schema Definition & Validation | Rust Dev 1 + TS Dev 1 | ☐ Not Started | W3 |
| T-05-02 | TypeScript Client Types & SDK Bindings | TS Dev 1 | ☐ Not Started | W3 |
| T-05-03 | Error Code Catalogue & Docs | PM/Writer | ☐ Not Started | W4 |

## Task Details

### T-05-01: JSON-RPC Schema Definition

- **Deliverables**: `docs/api/jsonrpc-schema.json`, `crates/cds-index/src/service/jsonrpc.rs` tests, schema validation script.
- **Acceptance**: All service responses conform to schema; CLI/agent tests use schema validation; documented in PRD-05 alignment.

### T-05-02: TypeScript Client Bindings

- **Deliverables**: `cds-agent/src/client/jsonrpc.ts`, generated typings, unit tests.
- **Acceptance**: Agent can call service with typed responses; handles error codes defined in schema.

### T-05-03: Error Code Catalogue

- **Deliverables**: `docs/api/error-codes.md`, updates to README.
- **Acceptance**: Every error code mapped to description, remediation, and HTTP mapping (for future REST proxy).

## Dependencies

- Requires graph/search implementations for realistic payload examples.
- Blocks service integration tests and CLI formatter validation.

## Notes

- Coordinate with PRD-02/04 owners to keep schemas up to date when new fields are added.
- Track schema versioning for future backward compatibility (v0.1.0 → v0.2.0).
