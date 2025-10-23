# Task T-05-02: TypeScript Client Bindings & SDK Integration

**Issue**: [API Contracts](../../issues/04-0.1.0-mvp/05-api-contracts.md)

**PRD References**: [PRD-05 §3](../../prd/0.1.0-MVP-PRDs-v0/05-api-specifications.md), [PRD-04 §2.1](../../prd/0.1.0-MVP-PRDs-v0/04-cds-agent-integration.md)

**Owner**: TypeScript Dev 1

**Status**: ✅ Completed (2025-10-23) | **Week**: 1

**PR Link**: [PR #5](https://github.com/lwyBZss8924d/CDSAgent/pull/5)

---

## Objective

Generate strongly typed client methods for the JSON-RPC API so the agent can call index service methods with compile-time safety.

## Deliverables

- `cds-agent/src/client/jsonrpc.ts`
- Type definitions `cds-agent/src/types/api.ts`
- Unit tests `cds-agent/tests/jsonrpc-client.test.ts`

## Implementation Steps

1. Convert JSON schema into TypeScript types (manual or using `quicktype`/`zod`).
2. Implement client wrapper with fetch + retry/backoff and error handling.
3. Ensure client integrates with agent hooks (respect env vars, logging).
4. Write unit tests verifying happy path and error responses.

## Acceptance Criteria

- [x] Client methods `searchEntities`, `traverseGraph`, `retrieveEntity`, `rebuildIndex` compile and work with typed wrappers.
- [x] TypeScript types align with PRD-05 schemas; no `any` usage (validated via Zod schemas).
- [x] Unit tests cover success + error scenarios (11 Bun tests across happy/error/retry cases).
- [x] Client integrated with agent entrypoint (`main.ts`) via configuration-driven bootstrap.

## Completion Summary (2025-10-23)

- Implemented `JSONRPCClient` with retry/backoff, error mapping, and logging hooks (`cds-agent/src/client/jsonrpc.ts`).
- Authored manual Zod-backed schemas and exported type definitions (`cds-agent/src/types/api.ts`).
- Added configuration helper for service URL, timeouts, retries, and logging (`cds-agent/src/utils/config.ts`).
- Updated entrypoint to consume configuration and expose the shared client (`cds-agent/src/main.ts`).
- Wrote 11 Bun unit tests covering all API methods, error codes, snippet modes, and retry behaviour (`cds-agent/tests/jsonrpc-client.test.ts`).
- Documented usage, configuration, and error handling in `cds-agent/README.md` and `.env.example`.

## Dependencies

- Requires schema definition (T-05-01) and service methods (T-02-03).
- Blocks agent hook and prompt tasks.

## Notes

- Consider using `zod` or `io-ts` for runtime validation of responses.
