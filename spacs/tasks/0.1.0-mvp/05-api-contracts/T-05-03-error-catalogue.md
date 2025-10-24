# Task T-05-03: Error Code Catalogue & Documentation

**Issue**: [API Contracts](../../issues/04-0.1.0-mvp/05-api-contracts.md)

**PRD References**: [PRD-05 §4](../../prd/0.1.0-MVP-PRDs-v0/05-api-specifications.md)

**Owner**: PM / Technical Writer

**Status**: ✅ Completed (2025-10-25) | **Week**: 1

---

## Objective

Document all JSON-RPC error codes, their meaning, and recommended remediation steps for CLI and agent consumers.

## Deliverables

- `docs/api/error-codes.md`
- Updates to `README.md` (API section)
- Error mapping table for CLI/agent (e.g., `errorCode → exit code/message`)

## Implementation Steps

1. List error codes defined in service (`jsonrpc.rs`) and categorize by severity.
2. Provide human-readable messages and remediation tips (e.g., missing entity, traversal failure).
3. Update CLI and agent code to map error codes to friendly outputs.
4. Review docs with engineering to ensure accuracy.

## Acceptance Criteria

- [x] Every error code defined in PRD-05 documented with description and remediation.
- [x] CLI and agent reference documentation updated to link to error catalogue.
- [x] Process for adding new error codes established (checklist in docs).

## Dependencies

- Requires T-05-01 to finalize code values.
- Blocks release notes and support docs.

## Notes

- Keep error catalogue bilingual-ready if translating documentation later.
