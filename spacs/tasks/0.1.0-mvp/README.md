# CDSAgent 0.1.0-MVP Task Tracking

This directory tracks implementation tasks for the CDSAgent 0.1.0-MVP release. Each task corresponds to an issue in `spacs/issues/04-0.1.0-mvp/` and references specific PRD sections.

## Task Organization

Tasks are organized by work stream, mirroring the issue structure:

```tree
spacs/tasks/0.1.0-mvp/
├── README.md                    # This file
├── 01-architecture/             # High-level architecture tasks
├── 02-index-core/               # Graph & BM25 index implementation tasks
├── 03-cli-tools/                # CLI commands implementation tasks
├── 04-agent-integration/        # Agent orchestration layer tasks
├── 05-api-contracts/            # Service API definition tasks
├── 06-refactor-parity/          # LocAgent parity validation tasks
├── 07-deployment/               # Docker deployment tasks
├── 08-testing/                  # Testing & quality assurance tasks
├── 09-roadmap/                  # Timeline coordination tasks
└── 10-extensibility/            # Future-proofing tasks
```

## Task Workflow

### 1. Task States

Each task follows this lifecycle:

- **☐ Not Started**: Task not yet assigned or begun
- **🔨 In Progress**: Currently being worked on
- **✓ Completed**: Finished and verified
- **⏸ Blocked**: Waiting on dependencies or external input

### 2. Task File Format

Each task file includes:

- **Task ID**: Unique identifier (e.g., T-02-01-001)
- **Title**: Brief description
- **Owner**: Assigned developer
- **Issue Reference**: Link to `spacs/issues/04-0.1.0-mvp/`
- **PRD Reference**: Link to specific PRD section
- **Dependencies**: Prerequisite tasks
- **Acceptance Criteria**: Definition of done
- **Status**: Current progress

### 3. Dependency Flow

Follow this order to respect critical path dependencies:

```text
M0: Foundation Setup (Week 0) - COMPLETED ✅
    ↓
M1: API Contracts & Parity (Week 1)
    ├─ T-05-01: JSON-RPC Schema (P0, 3 days)
    ├─ T-06-01: Parity Methodology (P0, 2 days) [parallel]
    ├─ T-05-02: TypeScript Bindings (P0, 2 days)
    └─ T-05-03: Error Catalogue (P1, 1 day)
    ↓
M2: Core Indexing Prototype (Week 2-3)
    ├─ T-02-01: Graph Builder (P0, 5 days) [requires T-06-01]
    ├─ T-02-02: Sparse Index (P0, 4 days) [requires T-02-01]
    └─ T-02-04: Serialization (P1, 2 days) [requires T-02-01, T-02-02]
    ↓
M3: Service & CLI Alpha (Week 3-5)
    ├─ T-02-03: Service Layer (P0, 4 days) [requires T-02-01, T-02-02, T-05-01]
    ├─ T-03-01: Core Commands (P0, 3 days) [requires T-02-03]
    └─ T-03-02: Output Format (P1, 2 days) [requires T-03-01]
    ↓
M4: Agent Integration Beta (Week 5-7)
    ├─ T-04-01: Agent SDK (P0, 2 days) [requires T-03-01, T-05-02]
    ├─ T-04-02: Prompt Design (P0, 2 days) [requires T-04-01]
    ├─ T-04-03: Hooks (P0, 3 days) [requires T-04-01]
    └─ T-04-04: Sample Transcripts (P1, 2 days) [requires T-04-02, T-04-03]
    ↓
M5: Production RC (Week 8-10)
    ├─ T-07-01: Docker Compose (P1, 2 days)
    ├─ T-08-03: Parity Validation (P0, 3 days)
    └─ T-08-04: Benchmark Testing (P0, 2 days)
```

## Quick Reference

### Milestone M0: Foundation Setup (Week 0) ✅ COMPLETED

**Status**: Completed 2025-10-19

**Deliverables**:

- [x] Git worktree infrastructure (7 task branches)
- [x] PRD documentation suite (10 PRDs)
- [x] Task specifications (30 tasks)
- [x] Development environment setup

**PR**: [#2 - Complete development environment setup](https://github.com/lwyBZss8924d/CDSAgent/pull/2)

---

### Milestone M1: API Contracts & Parity (Week 1)

**Target**: 2025-10-26 (1 week from start)
**Status**: 🏗 Ready to Start

**Critical Path Tasks**:

- [ ] **T-05-01**: JSON-RPC Schema Definition (3 days, P0) ← START HERE
- [ ] **T-06-01**: LocAgent Parity Methodology (2 days, P0) [parallel with T-05-01]
- [ ] **T-05-02**: TypeScript Client Bindings (2 days, P0) [after T-05-01]
- [ ] **T-05-03**: Error Code Catalogue (1 day, P1) [after T-05-01]

**Success Criteria**:

- JSON-RPC API fully specified and validated
- LocAgent parity baseline established (3 sample repos)
- TypeScript types auto-generated from Rust
- All error codes documented

---

### Milestone M2: Core Indexing Prototype (Week 2-3)

**Target**: 2025-11-09
**Status**: ⏸ Blocked (waiting for M1)

**Critical Path Tasks**:

- [ ] **T-02-01**: Graph Builder - AST Parsing (5 days, P0)
- [ ] **T-02-02**: Sparse Index - BM25 + Name (4 days, P0) [parallel after day 3]
- [ ] **T-02-04**: Serialization (2 days, P1) [after T-02-01, T-02-02]

**Success Criteria**:

- Graph parity ≤2% variance from LocAgent
- Search latency <500ms p95
- Index build <5s for 1K files
- Unit test coverage >80%

---

### Milestone M3: Service & CLI Alpha (Week 3-5)

**Target**: 2025-11-23
**Status**: ⏸ Blocked (waiting for M2)

**Critical Path Tasks**:

- [ ] **T-02-03**: Service Layer - JSON-RPC Server (4 days, P0)
- [ ] **T-03-01**: CLI Core Commands (3 days, P0)
- [ ] **T-03-02**: Output Formatting (2 days, P1)

**Success Criteria**:

- JSON-RPC service exposes all 4 endpoints
- CLI can query service (search, traverse, retrieve)
- Integration tests pass
- Service contract tests validate schema compliance

---

### Milestone M4: Agent Integration Beta (Week 5-7)

**Target**: 2025-12-07
**Status**: ⏸ Blocked (waiting for M3)

**Critical Path Tasks**:

- [ ] **T-04-01**: Agent SDK Bootstrap (2 days, P0)
- [ ] **T-04-02**: Prompt Design (2 days, P0)
- [ ] **T-04-03**: Hooks Implementation (3 days, P0)
- [ ] **T-04-04**: Sample Transcripts (2 days, P1)
- [ ] **T-08-01**: Unit Tests (continuous, P1)
- [ ] **T-08-02**: Integration Tests (3 days, P0)

**Success Criteria**:

- Agent executes multi-step code localization
- PreToolUse/PostToolUse hooks functional
- 5+ sample transcripts documented
- Integration tests pass

---

### Milestone M5: Production Release Candidate (Week 8-10)

**Target**: 2025-12-31
**Status**: ⏸ Blocked (waiting for M4)

**Critical Path Tasks**:

- [ ] **T-07-01**: Docker Compose Setup (2 days, P1)
- [ ] **T-07-02**: Environment Configuration (1 day, P1)
- [ ] **T-07-03**: Monitoring & Health Checks (2 days, P1)
- [ ] **T-08-03**: Parity Validation (3 days, P0)
- [ ] **T-08-04**: Benchmark Testing (2 days, P0)
- [ ] **T-07-04**: Deployment Documentation (1 day, P1)

**Success Criteria**:

- LocAgent parity >80% accuracy on SWE-bench Lite
- 2-5x performance improvement over Python
- Docker Compose deployment works
- All documentation complete

---

## Task Metadata & Tracking

### TODO.yaml - Central Task Registry

All tasks are tracked in **[TODO.yaml](./TODO.yaml)** with complete metadata:

- Task specifications (PRDs, Issues, Tasks)
- Git worktree information
- Dependencies and blockers
- Acceptance criteria
- Worklog paths
- Status tracking

**Usage:**

```bash
# View all task metadata
cat spacs/tasks/0.1.0-mvp/TODO.yaml

# Check current milestone
yq '.milestones.M1' spacs/tasks/0.1.0-mvp/TODO.yaml

# List active tasks
yq '.workflows.active_tasks' spacs/tasks/0.1.0-mvp/TODO.yaml
```

### Worklog Structure

Each task maintains a worklog in `.artifacts/spec-tasks-{TASK_ID}/worklogs/`:

```tree
.artifacts/spec-tasks-T-05-01/
├── worklogs/
│   ├── 2025-10-19-work-summary.md
│   ├── 2025-10-19-commit-log.md
│   └── 2025-10-19-notes.md
├── metadata.yaml
└── git-refs.txt
```

**Worklog Metadata:**

- PRD references
- Issue references
- Task specifications
- Worktree branch
- Git commits
- PR links
- Status updates
- Comments

---

## Related Documents

- **TODO.yaml**: `spacs/tasks/0.1.0-mvp/TODO.yaml` - Central task registry with metadata
- **Issues**: `spacs/issues/04-0.1.0-mvp/` - Detailed issue breakdown
- **PRDs**: `spacs/prd/0.1.0-MVP-PRDs-v0/` - Requirements documentation
- **Backlog**: `spacs/plan/0.1.0-mvp-backlog.md` - Master backlog plan
- **Architecture**: `spacs/issues/02-CDSAgent-Tech-Architecture-Plan.md` - System design
- **Development Status**: `DEVELOPMENT_STATUS.md` - Current project state
- **Worktree Workflow**: `docs/WORKTREE_WORKFLOW.md` - Git worktree guide

## Task Tracking Tools

### CLI Usage

```bash
# List all tasks for a component
ls spacs/tasks/0.1.0-mvp/02-index-core/

# View specific task
cat spacs/tasks/0.1.0-mvp/02-index-core/T-02-01-graph-builder.md

# Track progress (example with grep)
grep -r "Status:" spacs/tasks/0.1.0-mvp/ | grep "In Progress"
```

### Integration with Issue Tracker

Each task references its parent issue. When starting a task:

1. Read the task file for context
2. Review the parent issue for detailed acceptance criteria
3. Consult the PRD section for requirements
4. Update task status when complete

---

**Last Updated**: 2025-10-19
**Status**: Active Development
