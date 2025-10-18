# Tasks: Deployment & Operations - Docker-First Production Setup

**Work Stream**: Issue-07: Deployment & Operations
**Issue Reference**: [../../issues/04-0.1.0-mvp/07-deployment/](../../issues/04-0.1.0-mvp/07-deployment/)
**PRD Reference**: [PRD-07: Deployment & Operations](../../prd/0.1.0-MVP-PRDs-v0/07-deployment-operations.md)

## Task Overview

| Task ID | Title | Owner | Status | Week |
|---------|-------|-------|--------|------|
| T-07-01 | Docker Compose - Multi-service orchestration | DevOps Lead | ☐ Not Started | W8 |
| T-07-02 | Environment Config - .env templates & validation | DevOps Lead | ☐ Not Started | W8 |
| T-07-03 | Monitoring - Health checks & Prometheus metrics | DevOps Lead | ☐ Not Started | W9 |
| T-07-04 | Documentation - Setup guides & troubleshooting | DevOps Lead + Writer | ☐ Not Started | W10 |

## Dependencies

- **Prerequisite**: All services (T-02, T-03, T-04) must be functional
- **Enables**: Production deployment and operations

## Task Details

### T-07-01: Docker Compose

**File**: `T-07-01-docker-compose.md`
**Issue**: [Sub-Issue 07.01](../../issues/04-0.1.0-mvp/07-deployment/01-docker-compose.md)
**PRD**: PRD-07 §2.1, §4.1

**Scope**:

- Dockerfiles for Index Service, CLI, Agent
- docker-compose.yml with networking and volumes
- Build optimization (layer caching)

**Deliverables**:

- `docker/index-service/Dockerfile`
- `docker/cli-tools/Dockerfile`
- `docker/agent/Dockerfile`
- `docker-compose.yml`

---

### T-07-02: Environment Configuration

**File**: `T-07-02-env-config.md`
**Issue**: [Sub-Issue 07.02](../../issues/04-0.1.0-mvp/07-deployment/02-env-config.md)
**PRD**: PRD-07 §2.2, §4.2

**Scope**:

- .env.example with all required variables
- Configuration validation on startup
- Support for dev/staging/prod profiles

**Deliverables**:

- `config/.env.example`
- Validation logic in service bootstraps

---

### T-07-03: Monitoring & Health Checks

**File**: `T-07-03-monitoring.md`
**Issue**: [Sub-Issue 07.03](../../issues/04-0.1.0-mvp/07-deployment/03-monitoring.md)
**PRD**: PRD-07 §2.3, §4.3

**Scope**:

- Health check endpoints (/health)
- Prometheus metrics (/metrics)
- Docker health checks in compose file

**Deliverables**:

- Health endpoint in Index Service
- Prometheus configuration
- Alert rules

---

### T-07-04: Deployment Documentation

**File**: `T-07-04-docs.md`
**Issue**: [Sub-Issue 07.04](../../issues/04-0.1.0-mvp/07-deployment/04-docs.md)
**PRD**: PRD-07 §2.4, §5

**Scope**:

- Quick start guide (<10 minutes)
- Troubleshooting guide (5+ common issues)
- Production deployment checklist
- API key setup instructions

**Deliverables**:

- `deployment/README.md`
- `deployment/TROUBLESHOOTING.md`
- `deployment/PRODUCTION.md`
- `deployment/docs/API_KEYS.md`

---

## Phase Milestones

### Week 8: Docker Foundation

- [ ] T-07-01: All services start with docker-compose up
- [ ] T-07-02: Environment variables properly configured

**Validation**: Agent can run code localization task in Docker

### Week 9-10: Production Ready

- [ ] T-07-03: Health checks and metrics functional
- [ ] T-07-04: Documentation tested by new user

**Validation**: User can deploy from scratch in <10 minutes

---

## Quick Links

- [Issue-07 Overview](../../issues/04-0.1.0-mvp/07-deployment/00-overview.md)
- [PRD-07: Deployment & Operations](../../prd/0.1.0-MVP-PRDs-v0/07-deployment-operations.md)
- [Integration Tests](../08-testing/README.md#t-08-02-integration-tests)
