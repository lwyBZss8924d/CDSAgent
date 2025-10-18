# Issue-09: Implementation Roadmap & Milestone Tracking

**Priority**: P0 (Critical Path - Planning Foundation)
**Status**: ☐ Not Started
**Owner**: Project Manager / Tech Lead
**PRD Reference**: [PRD-09: Implementation Roadmap](../../prd/0.1.0-MVP-PRDs-v0/09-implementation-roadmap.md)

---

## Overview

Track the 4-phase, 10-week implementation plan for CDSAgent 0.1.0-MVP, ensuring milestone delivery, resource allocation, and risk management across all work streams.

## Objective

Provide a living roadmap document that:

- Tracks progress against PRD-09's phased plan
- Monitors milestone criteria and exit gates
- Coordinates cross-team dependencies
- Escalates schedule/resource risks early

## Dependencies

- **Input**: All other issues (synthesis of work across streams)
- **Blocked By**: None (planning document)
- **Blocks**: None (provides guidance)

---

## Phase Breakdown

### Phase 1: Foundational Spike (Weeks 1-2)

**Goal**: Validate core architecture, build minimal working graph indexer

**Milestones**:

- [ ] M1: Prototype (Week 2)
  - [ ] Can index LocAgent repository (~150 files)
  - [ ] `cds search` returns name-based results
  - [ ] Performance: Index <10s, search <500ms
  - [ ] Unit test coverage ≥70%

**Key Deliverables** (from PRD-09 §2.2):

- [ ] Rust workspace setup with CI/CD
- [ ] `cds_graph` crate (Python parsing only)
- [ ] Tree-sitter integration
- [ ] `cds_sparse_index` crate (name/ID index)
- [ ] Basic `cds search` CLI command
- [ ] Unit tests (>70% coverage)

**Risks & Mitigations**:

- Tree-sitter binding issues → Test early with small Python files
- Graph structure mismatch → Compare node/edge counts with LocAgent

**Exit Criteria**:

- [ ] All Phase 1 deliverables complete
- [ ] M1 milestone criteria met
- [ ] Demo to stakeholders successful
- [ ] Go/no-go decision for Phase 2

---

### Phase 2: Service + SDK Integration (Weeks 3-5)

**Goal**: Complete BM25 indexing, graph traversal, stand up JSON-RPC service, integrate Claude Agent SDK

**Milestones**:

- [ ] M2: Alpha (Week 5)
  - [ ] Hierarchical search matches LocAgent top-10 (50 queries)
  - [ ] Traverse produces correct subgraphs
  - [ ] Claude agent can execute multi-step code search
  - [ ] Performance: Search <500ms, traverse (2-hop) <1s

**Key Deliverables** (from PRD-09 §3.2):

- [ ] BM25 index (tantivy or custom)
- [ ] Hierarchical search (upper + lower)
- [ ] `cds_traversal` crate (BFS)
- [ ] `cds traverse`, `cds retrieve` commands
- [ ] CDS-Index Service (JSON-RPC)
- [ ] TypeScript agent with bash tool
- [ ] System prompt with CoT steps

**Risks & Mitigations**:

- BM25 accuracy lower than LocAgent → Validate tokenization, try custom BM25
- Agent tool calls fail → Add detailed error messages, logging

**Exit Criteria**:

- [ ] All Phase 2 deliverables complete
- [ ] M2 milestone criteria met
- [ ] Agent can complete sample code search task
- [ ] Go/no-go decision for Phase 3

---

### Phase 3: Agentization & Hooks (Weeks 6-7)

**Goal**: Implement subagent, add all hooks, enable hybrid retrieval, validate on SWE-bench samples

**Milestones**:

- [ ] M3: Beta (Week 7)
  - [ ] Subagent successfully delegates to bash tool
  - [ ] Hooks reduce context usage by >30%
  - [ ] SWE-bench Lite (10 samples): Acc@5 ≥75%

**Key Deliverables** (from PRD-09 §4.2):

- [ ] `code-retrievaler` subagent config
- [ ] All hooks implemented (Pre/Post/SubagentStop)
- [ ] Hybrid retrieval examples (cds + rg + ast-grep)
- [ ] `cds combo` command (YAML plans) - developer-only in v0.1.0
- [ ] SWE-bench Lite validation (10 samples)

**Exit Criteria**:

- [ ] All Phase 3 deliverables complete
- [ ] M3 milestone criteria met
- [ ] Accuracy targets on sample set achieved
- [ ] Go/no-go decision for Phase 4

---

### Phase 4: Polyglot & Optimization (Weeks 8-10)

**Goal**: Add TypeScript/Java support (deferred to v0.2.0), optimize performance, containerize, full SWE-bench validation

**Note**: Multi-language support postponed to v0.2.0 based on PRD-01 §4.2 language roadmap. Phase 4 focuses on:

- Python-only optimization and hardening
- Full SWE-bench Lite validation (300 instances)
- Deployment readiness

**Milestones**:

- [ ] M4: v1.0 RC (Week 10)
  - [ ] Performance targets met (Index <5s/1K files, Search <500ms)
  - [ ] SWE-bench Lite: File Acc@5 ≥80%, NDCG@5 ≥0.70
  - [ ] Docker deployment tested and documented

**Key Deliverables** (from PRD-09 §5.2, adjusted):

- [ ] ~~TypeScript parser~~ (deferred to v0.2.0)
- [ ] ~~Java parser~~ (deferred to v0.2.0)
- [ ] Parallelized indexing (rayon)
- [ ] Memory optimization (mmap, string interning)
- [ ] Dockerfiles & docker-compose
- [ ] SWE-bench Lite full validation (300 instances)
- [ ] Documentation (README, API docs, deployment guide)

**Exit Criteria**:

- [ ] All Phase 4 deliverables complete
- [ ] M4 milestone criteria met
- [ ] SWE-bench Lite targets achieved (≥80% Acc@5)
- [ ] Beta testers validate product (5 users)
- [ ] Go/no-go decision for v1.0 release

---

### Final Release: v1.0 (Weeks 11-12)

**Milestone**:

- [ ] M5: v1.0 Release
  - [ ] All acceptance criteria met (PRD-08)
  - [ ] Security audit passed
  - [ ] Release notes written
  - [ ] Docker images published

---

## Success Metrics (from PRD-09 §11)

### Quantitative Targets

| Metric | Target | Actual | Status |
|--------|--------|--------|--------|
| SWE-bench File Acc@5 | ≥80% | ___ | ☐ |
| SWE-bench NDCG@5 | ≥0.70 | ___ | ☐ |
| Search latency (p95) | <500ms | ___ | ☐ |
| Index build (1K files) | <5s | ___ | ☐ |
| Test coverage | ≥80% | ___ | ☐ |

### Qualitative Targets

- [ ] Code quality: Passes clippy, rustfmt
- [ ] Usability: 5 beta testers report "easy to use"
- [ ] Documentation: Complete and clear (stakeholder approval)

---

## Resource Allocation (from PRD-09 §7)

### Team Structure

- **Rust Lead**: 1 FTE (Architecture, code review, performance)
- **Rust Developers**: 3 FTE (Crates implementation, testing)
- **TypeScript Developer**: 1 FTE (Agent integration, hooks)
- **QA Lead**: 1 FTE (Testing, benchmark validation)
- **DevOps**: 0.5 FTE (CI/CD, containerization)
- **Tech Writer**: 0.5 FTE (Documentation)

**Total**: ~6.5 FTE

### Time Allocation by Phase

| Phase | Duration | FTE-weeks | Key Focus |
|-------|----------|-----------|-----------|
| Phase 1 | 2 weeks | 12 | Graph indexing foundation |
| Phase 2 | 3 weeks | 18 | Service + SDK integration |
| Phase 3 | 2 weeks | 12 | Agent workflows + validation |
| Phase 4 | 3 weeks | 18 | Optimization + deployment |
| **Total** | **10 weeks** | **60 FTE-weeks** | |

---

## Risk Management

### Critical Path Risks (from PRD-09 §9)

| Risk | Probability | Impact | Mitigation | Status |
|------|-------------|--------|-----------|--------|
| BM25 accuracy gap vs LocAgent | Medium | High | Early validation, custom BM25 if needed | ☐ |
| Claude API rate limits | Low | Medium | Caching, local testing with mocks | ☐ |
| Tree-sitter compatibility issues | Low | High | Test all grammars early | ☐ |
| Team bandwidth shortage | Medium | High | Prioritize core features, defer nice-to-haves | ☐ |

### Schedule Risks

| Risk | Impact | Mitigation | Status |
|------|--------|-----------|--------|
| Phase 2 runs long (SDK complexity) | +1 week | Buffer in Phase 4, reduce polyglot scope | ☐ |
| SWE-bench validation uncovers gaps | +2 weeks | Continuous validation from Week 5 | ☐ |

---

## Communication & Reporting

### Weekly Sync

- **When**: Every Monday, 10 AM
- **Attendees**: Full team
- **Agenda**: Progress updates, blockers, next week's plan

### Milestone Reviews

- **When**: End of each phase
- **Attendees**: Team + stakeholders
- **Agenda**: Demo, metrics review, go/no-go decision

### Status Reporting

- **Weekly**: Update Linear/Jira with completed tasks
- **Bi-weekly**: Stakeholder email with progress summary

---

## Acceptance Criteria (v1.0)

### Must-Pass (from PRD-09 §8.4)

- [ ] All Phase 1-4 deliverables complete
- [ ] All 5 milestones (M1-M5) met
- [ ] SWE-bench Lite: File Acc@5 ≥80%
- [ ] Performance targets achieved
- [ ] Security audit passed
- [ ] Beta testing complete (5 users)
- [ ] Documentation finalized
- [ ] Docker images published

---

## Related Issues

- [01-architecture-and-roadmap.md](01-architecture-and-roadmap.md) - System architecture synthesis
- [02-index-core/](02-index-core/) - Phase 1-2 core work
- [03-cli-tools/](03-cli-tools/) - Phase 2 CLI implementation
- [04-agent-integration/](04-agent-integration/) - Phase 2-3 agent work
- [07-deployment/](07-deployment/) - Phase 4 deployment
- [08-testing/](08-testing/) - Continuous validation

---

## Next Steps

1. [ ] Assemble team and assign roles (Rust Lead, Devs, QA, etc.)
2. [ ] Set up development environment (Rust workspace, CI/CD)
3. [ ] Schedule Phase 1 kickoff meeting
4. [ ] Begin Week 1 tasks (Infrastructure + Core Graph Building)
5. [ ] Establish weekly sync cadence

---

**Status Updates**:

- *2025-10-19*: Roadmap issue created, Phase 1 planning underway
