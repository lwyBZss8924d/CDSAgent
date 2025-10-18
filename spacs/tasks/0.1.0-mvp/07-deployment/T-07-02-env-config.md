# Task T-07-02: Environment Configuration Templates

**Issue**: [Sub-Issue 07.02 – Environment Config](../../issues/04-0.1.0-mvp/07-deployment/03-env-config.md)

**PRD References**: [PRD-07 §2.2](../../prd/0.1.0-MVP-PRDs-v0/07-deployment-operations.md)

**Owners**: DevOps Lead

**Status**: ☐ Not Started | **Week**: 8

---

## Objective

Standardize environment variable management across local, staging, and production setups, providing templates and validation.

## Deliverables

- `config/.env.example` (root-level aggregate)
- Service-specific `.env` templates (index service, CLI, agent)
- Validation logic in `cds-index/src/config.rs` and `cds-agent/src/agent-config.ts`
- Documentation `docs/deployment/env-config.md`

## Implementation Steps

1. Catalogue all env vars (paths, ports, API keys, feature flags).
2. Create `.env.example` with comments, defaults, and security notes.
3. Implement config loaders that fail fast with descriptive errors when variables missing.
4. Provide `just env-check` script to verify setups.

## Acceptance Criteria

- [ ] Running services with missing env vars yields clear validation errors.
- [ ] Templates cover dev/staging/prod scenarios (and mention secrets handling).
- [ ] Documentation explains relationship between `.env` files and docker-compose overrides.
- [ ] CI job ensures `.env.example` stays in sync with loaders.

## Dependencies

- **Prerequisite**: [T-07-01 Docker Compose](T-07-01-docker-compose.md).
- **Blocks**: [T-07-03 Monitoring](T-07-03-monitoring.md), [T-07-04 Documentation](T-07-04-docs.md).

## Notes

- Make `.env` optional but recommended; support fallback to CLI flags for secure environments.
