# Task T-07-03: Monitoring & Health Checks

**Issue**: [Sub-Issue 07.03 – Monitoring](../../issues/04-0.1.0-mvp/07-deployment/04-monitoring.md)

**PRD References**: [PRD-07 §2.3, §4.3](../../prd/0.1.0-MVP-PRDs-v0/07-deployment-operations.md)

**Owners**: DevOps Lead

**Status**: ☐ Not Started | **Week**: 9

---

## Objective

Provide observability for CDSAgent services including readiness probes, Prometheus metrics, and alerting guidance.

## Deliverables

- `/health` and `/metrics` endpoints in index service
- Docker health-check definitions
- Prometheus scrape config (`deployment/monitoring/prometheus.yml`)
- Alert runbook (`docs/deployment/monitoring.md`)

## Implementation Steps

1. Add health-check handler returning service + index status (graph loaded, BM25 ready).
2. Expose counters/gauges (request counts, latency, graph size) via `metrics` crate.
3. Configure docker-compose health checks and document `curl` usage.
4. Draft Prometheus/Grafana setup instructions.

## Acceptance Criteria

- [ ] `/health` returns 200 OK with JSON payload (status, uptime, index stats).
- [ ] `/metrics` exposes Prometheus data; manual scrape works.
- [ ] Docker health checks mark container unhealthy on failure.
- [ ] Monitoring docs tested by fresh setup (QA or PM).

## Dependencies

- **Prerequisite**: [T-07-01](T-07-01-docker-compose.md), [T-07-02](T-07-02-env-config.md).
- **Blocks**: [T-07-04 Documentation](T-07-04-docs.md), release readiness checklist.

## Notes

- Optional: integrate tracing subscriber with OpenTelemetry exporter for future use.
