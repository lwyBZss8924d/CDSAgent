# Task T-07-01: Docker Compose Orchestration

**Issue**: [Sub-Issue 07.01 – Docker Compose](../../issues/04-0.1.0-mvp/07-deployment/02-docker-compose.md)

**PRD References**: [PRD-07 §2.1, §4.1](../../prd/0.1.0-MVP-PRDs-v0/07-deployment-operations.md)

**Owners**: DevOps Lead

**Status**: ☐ Not Started | **Week**: 8

---

## Objective

Containerize Index Service, CLI, and Agent, and provide a docker-compose environment for local and staging deployments.

## Deliverables

- Dockerfiles (`docker/index-service/Dockerfile`, `docker/cli-tools/Dockerfile`, `docker/agent/Dockerfile`)
- `docker-compose.yml`
- Build script `scripts/docker/build-all.sh`
- Deployment script `scripts/docker/deploy.sh`

## Implementation Steps

1. Create multi-stage Dockerfiles optimizing for size and reproducibility.
2. Define compose services with shared network, volumes, and health checks.
3. Verify interoperability: CLI container can reach index-service, agent can call CLI.
4. Document usage (`docs/deployment/docker.md`).

## Acceptance Criteria

- [ ] `docker compose up -d` starts all services successfully.
- [ ] Health checks pass and services become healthy within 60 seconds.
- [ ] Data directories (`graph_index`, `bm25_index`, `agent_logs`) persisted via volumes.
- [ ] Documentation includes setup, teardown, and troubleshooting guidance.

## Dependencies

- **Prerequisite**: Completion of T-02, T-03, T-04 deliverables.
- **Blocks**: [T-07-02 Env Config](T-07-02-env-config.md), [T-07-03 Monitoring](T-07-03-monitoring.md).

## Notes

- Plan for future image publication (GitHub Packages or OCI registry) but keep v0.1.0 local-only.
