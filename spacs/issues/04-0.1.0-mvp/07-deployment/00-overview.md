# Issue-07: Deployment & Operations - Local Daemon & Docker Production Setup

**Priority**: P1 (Critical Path - Production Readiness)
**Status**: ☐ Not Started
**Owner**: DevOps Lead
**PRD Reference**: [PRD-07: Deployment & Operations](../../../prd/0.1.0-MVP-PRDs-v0/07-deployment-operations.md)

---

## Overview

Deployment & Operations provides production-ready deployment options including local daemon configuration (systemd/launchd) and Docker-based orchestration with environment configuration, health monitoring, and comprehensive documentation. This component ensures CDSAgent can be reliably deployed and operated in production environments.

## Objective

Deliver a multi-deployment solution that:

- Supports local daemon mode with systemd (Linux) and launchd (macOS)
- Provides Docker Compose orchestration for Index Service + CLI + Agent
- Implements environment-based configuration with .env templates
- Includes health checks and basic monitoring (Prometheus metrics)
- Documents deployment procedures and troubleshooting guides
- Supports single-machine deployment with future k8s extensibility

## Dependencies

- **Requires**: CDS-Index Service ([02-index-core/](../02-index-core/)), CDS-Tools CLI ([03-cli-tools/](../03-cli-tools/)), CDS-Agent ([04-agent-integration/](../04-agent-integration/))
- **Coordinates With**: Testing & Quality ([08-testing/](../08-testing/))
- **Timing**: Phase 4 (Weeks 8-10)

---

## Sub-Issues Breakdown

### 1. [Local Daemon Configuration](01-local-daemon.md) - **P1, Week 8**

**Owner**: DevOps Lead
**Scope**: systemd/launchd configuration, daemon management

- systemd service unit for Linux (Type=notify, auto-restart)
- launchd property list for macOS (user-mode, KeepAlive)
- Installation scripts for both platforms
- Health check integration with systemd notifications
- Configuration defaults aligned with PRD-04 hooks

**Acceptance**:

- [ ] systemd service starts automatically on boot (Linux)
- [ ] launchd service runs in user mode (macOS)
- [ ] Service restarts automatically on failure
- [ ] Health checks integrated with systemd Type=notify
- [ ] Configuration defaults match PRD-04 FR-HOOK-1

---

### 2. [Docker Compose Setup](02-docker-compose.md) - **P1, Week 8**

**Owner**: DevOps Lead
**Scope**: Multi-service orchestration, volume management

- Dockerfiles for Index Service (Rust), CLI (Rust), Agent (TypeScript/Bun)
- docker-compose.yml with service definitions, networking, and volumes
- Build stages and layer caching optimization
- Volume persistence for graph index and BM25 index

**Acceptance**:

- [ ] All services start with `docker-compose up`
- [ ] Health checks pass for Index Service
- [ ] CLI container can invoke Index Service endpoints
- [ ] Agent container can invoke CLI commands

---

### 3. [Environment Configuration](03-env-config.md) - **P1, Week 8**

**Owner**: DevOps Lead
**Scope**: .env templates, secrets management

- .env.example with all required variables
- Configuration validation on startup
- Support for dev/staging/prod profiles
- API key management for Claude/OpenAI

**Acceptance**:

- [ ] .env.example documents all variables
- [ ] Services fail-fast if required env vars missing
- [ ] Secrets not committed to repo (gitignore)
- [ ] Environment switching via ENV_PROFILE

---

### 4. [Monitoring & Health Checks](04-monitoring.md) - **P1, Week 9**

**Owner**: DevOps Lead
**Scope**: Prometheus metrics, health endpoints

- Health check endpoints for Index Service (/health)
- Prometheus metrics (request counts, latency, index size)
- Basic alerts for service availability
- Log aggregation recommendations (optional)

**Acceptance**:

- [ ] /health endpoint returns 200 when service ready
- [ ] Prometheus metrics exposed on /metrics
- [ ] Docker health checks restart failed services
- [ ] Sample Grafana dashboard (optional)

---

### 5. [Deployment Documentation](05-docs.md) - **P1, Week 10**

**Owner**: DevOps Lead + Technical Writer
**Scope**: Setup guides, troubleshooting, runbooks

- Quick start guide (clone → configure → deploy)
- Troubleshooting common issues (port conflicts, volume permissions)
- Production deployment checklist
- API key setup instructions

**Acceptance**:

- [ ] README.md with quick start steps
- [ ] TROUBLESHOOTING.md with common errors
- [ ] PRODUCTION.md with production checklist
- [ ] API key setup documented

---

## Deployment Project Structure

```tree
deployment/
├── systemd/                     # Linux daemon configuration
│   ├── cds-index.service        # systemd unit file
│   └── README.md                # Installation instructions
├── launchd/                     # macOS daemon configuration
│   ├── com.cdsagent.index.plist # launchd property list
│   └── README.md                # Installation instructions
├── docker/
│   ├── index-service/
│   │   └── Dockerfile           # Rust multi-stage build
│   ├── cli-tools/
│   │   └── Dockerfile           # Rust with runtime dependencies
│   ├── agent/
│   │   └── Dockerfile           # Bun + TypeScript
│   └── docker-compose.yml       # Orchestration
├── config/
│   ├── .env.example             # Template with all vars
│   ├── config.toml.example      # Daemon config template
│   ├── prometheus.yml           # Metrics scraping config
│   └── grafana/
│       └── dashboards/          # Sample dashboards (optional)
├── scripts/
│   ├── install-daemon-linux.sh  # systemd installation
│   ├── install-daemon-macos.sh  # launchd installation
│   ├── daemon-ctl.sh            # Unified daemon control
│   ├── health-check.sh          # Manual health validation
│   ├── build-all.sh             # Build all Docker images
│   └── deploy.sh                # Wrapper for docker-compose
├── docs/
│   ├── README.md                # Quick start guide
│   ├── TROUBLESHOOTING.md       # Common issues
│   └── PRODUCTION.md            # Production checklist
└── volumes/                     # Mounted volumes (gitignored)
    ├── graph_index/
    ├── bm25_index/
    └── agent_logs/
```

---

## Acceptance Criteria Summary (from PRD-07 §8)

### Must-Have (v0.1.0 MVP)

- [ ] Local daemon mode functional on Linux (systemd) and macOS (launchd)
- [ ] Daemon restarts automatically on failure
- [ ] Configuration defaults aligned with PRD-04 hooks
- [ ] Docker Compose orchestrates all 3 services
- [ ] .env.example documents all configuration
- [ ] Health checks restart failed services
- [ ] Prometheus metrics exposed for Index Service
- [ ] Quick start guide enables new users to deploy in <10 minutes
- [ ] Troubleshooting guide covers 5+ common issues

### Should-Have (v0.2.0)

- [ ] Kubernetes manifests (deployment, service, ingress)
- [ ] Horizontal Pod Autoscaler for Index Service
- [ ] Centralized logging (ELK/Loki)
- [ ] Automated backup/restore for indexes

---

## Performance Targets (from PRD-07 §3.1)

| Metric | Target | Validation Method |
|--------|--------|-------------------|
| Service startup time | <30s | docker-compose up timing |
| Health check response | <100ms | curl /health benchmark |
| Container build time | <5min | CI/CD pipeline timing |
| Volume mount overhead | <2% | Index query performance comparison |

---

## Dependencies & Coordination

### Internal Dependencies

- Local Daemon (01) and Docker Compose (02) run in parallel
- Environment Config (03) supports both daemon and Docker deployments
- Monitoring (04) must complete before Deployment Docs (05)
- Deployment Docs (05) requires all components functional

### External Coordination

- **PRD-02 (Index Service)**: Service must expose /health endpoint
- **PRD-03 (CLI Tools)**: CLI must work inside Docker container
- **PRD-04 (Agent)**: Agent must read env vars from .env
- **PRD-08 (Testing)**: Integration tests validate Docker deployment

---

## Implementation Phases

### Phase 4, Week 8: Deployment Foundation

- [ ] Sub-issue 01: Local daemon configuration (systemd/launchd)
- [ ] Sub-issue 02: Docker Compose setup
- [ ] Sub-issue 03: Environment configuration
- [ ] Milestone: Services can run as daemon OR in Docker

### Phase 4, Week 9-10: Monitoring & Documentation

- [ ] Sub-issue 04: Monitoring and health checks
- [ ] Sub-issue 05: Deployment documentation
- [ ] Milestone: Production-ready deployment with monitoring

---

## Testing Strategy

### Integration Tests (see [../08-testing/02-integration.md](../08-testing/02-integration.md))

- [ ] Docker Compose brings up all services successfully
- [ ] Agent can invoke CLI which queries Index Service
- [ ] Health checks correctly detect service failures
- [ ] Environment variables properly injected

### Smoke Tests

- [ ] Quick start guide can be followed by new user
- [ ] Prometheus metrics scraped successfully
- [ ] Volume persistence survives container restart

---

## Open Questions & Risks

### 1. Index Persistence

**Question**: How to handle large index files (>10GB) in Docker volumes?
**Mitigation**: Document volume mount best practices, recommend bind mounts for production
**Escalation**: If performance issues, consider external object storage (S3) in v0.2.0

### 2. Multi-Node Deployment

**Decision**: Single-machine deployment only in v0.1.0, k8s in v0.2.0 (per PRD-07 FR-DOCKER-1)
**Rationale**: Simplifies initial deployment, most users will run on single server
**Risk**: May need to refactor for distributed deployment later

### 3. Secret Management

**Question**: How to securely manage API keys in production?
**Mitigation**: Document use of Docker secrets or external secret managers (Vault)
**Tracking**: Placeholder support in .env.example, full implementation in v0.2.0

---

## Related Issues

- **Sub-Issues**: [01-local-daemon.md](01-local-daemon.md), [02-docker-compose.md](02-docker-compose.md), [03-env-config.md](03-env-config.md), [04-monitoring.md](04-monitoring.md), [05-docs.md](05-docs.md)
- **Depends On**: [02-index-core/](../02-index-core/), [03-cli-tools/](../03-cli-tools/), [04-agent-integration/](../04-agent-integration/)
- **Tests**: [08-testing/02-integration.md](../08-testing/02-integration.md)

---

## Next Steps

1. [ ] Create deployment/ directory structure with systemd/launchd subdirs (Week 8, Day 1)
2. [ ] Write systemd unit and launchd plist files (Week 8, Day 1-2)
3. [ ] Write Dockerfiles for all 3 services (Week 8, Day 2-3)
4. [ ] Begin Sub-issue 01: Local daemon configuration (Week 8, Day 2)
5. [ ] Begin Sub-issue 02: Docker Compose setup (Week 8, parallel)
6. [ ] Begin Sub-issue 03: Environment configuration (Week 8, parallel)
7. [ ] Validate daemon mode AND docker-compose both work

---

**Status Updates**:

- *2025-10-19*: Issue created, awaiting service completion
