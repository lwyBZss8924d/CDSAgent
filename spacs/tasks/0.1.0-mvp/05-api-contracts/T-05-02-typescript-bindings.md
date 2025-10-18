# Task T-05-02: TypeScript Client Bindings & SDK Integration

**Issue**: [API Contracts](../../issues/04-0.1.0-mvp/05-api-contracts.md)

**PRD References**: [PRD-05 §3](../../prd/0.1.0-MVP-PRDs-v0/05-api-specifications.md), [PRD-04 §2.1](../../prd/0.1.0-MVP-PRDs-v0/04-cds-agent-integration.md)

**Owner**: TypeScript Dev 1

**Status**: ☐ Not Started | **Week**: 3

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

- [ ] Client methods `searchEntities`, `traverseGraph`, `retrieveEntity` compile and work.
- [ ] TypeScript types align with PRD-05 schemas; no `any` usage.
- [ ] Unit tests cover success + error scenarios.
- [ ] Client used by agent entrypoint (`main.ts`).

## Dependencies

- Requires schema definition (T-05-01) and service methods (T-02-03).
- Blocks agent hook and prompt tasks.

## Notes

- Consider using `zod` or `io-ts` for runtime validation of responses.
