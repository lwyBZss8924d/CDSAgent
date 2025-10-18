# PRD-09: Implementation Roadmap

**Version:** 1.0 (Round 1 - Concise)
**Date:** 2025-10-18
**Status:** Draft
**Parent:** PRD-01 System Architecture

---

## 1. Document Overview

### 1.1 Purpose

Define phased implementation plan for CDSAgent, including milestones, dependencies, resource allocation, and success criteria for each development phase.

### 1.2 Timeline Overview

- **Phase 1**: Foundational Spike (Weeks 1-2) - Core graph + basic CLI
- **Phase 2**: Service + SDK Integration (Weeks 3-5) - APIs + Claude Agent
- **Phase 3**: Agentization & Hooks (Weeks 6-7) - End-to-end agent workflows
- **Phase 4**: Polyglot & Optimization (Weeks 8-10) - Multi-language + performance

**Total Duration**: ~10 weeks (2.5 months)

---

## 2. Phase 1: Foundational Spike (Weeks 1-2)

### 2.1 Goals

- Validate core architecture assumptions
- Build minimal working graph indexer
- Implement basic search functionality
- Establish development workflow

### 2.2 Deliverables

| Deliverable | Owner | Status |
|------------|-------|--------|
| Rust workspace setup | Rust Lead | ☐ |
| `cds_graph` crate (Python parsing only) | Rust Dev 1 | ☐ |
| Tree-sitter integration | Rust Dev 1 | ☐ |
| `cds_sparse_index` crate (name/ID index) | Rust Dev 2 | ☐ |
| Basic `cds search` CLI command | Rust Dev 2 | ☐ |
| Unit tests (>70% coverage) | All Devs | ☐ |

### 2.3 Tasks

#### Week 1: Project Setup & Graph Foundation

**Day 1-2**: Infrastructure

- [ ] Create Rust workspace with crate structure
- [ ] Set up CI/CD (GitHub Actions: test, lint, benchmark)
- [ ] Configure development tools (rustfmt, clippy)
- [ ] Copy LocAgent tree-sitter queries to project

**Day 3-5**: Core Graph Building

- [ ] Implement AST parser for Python (tree-sitter bindings)
- [ ] Build entity extractor (classes, functions)
- [ ] Implement graph data structures (nodes, edges)
- [ ] Create graph builder (walk repo, parse files, build graph)
- [ ] Unit tests: Parse LocAgent repo, verify entity counts

#### Week 2: Search & CLI Prototype

**Day 1-3**: Name/ID Index

- [ ] Implement upper index (HashMap-based name lookup)
- [ ] Build tokenizer (camelCase/snake_case splitting)
- [ ] Exact match search
- [ ] Unit tests: Search by name

**Day 4-5**: CLI Prototype

- [ ] Set up `clap` CLI framework
- [ ] Implement `cds search` command
- [ ] JSON output formatter
- [ ] Integration test: End-to-end search on test repo

### 2.4 Success Criteria

- [ ] Can index LocAgent repository (~150 files)
- [ ] `cds search` returns results matching entity names
- [ ] Performance: Index build <10s, search <500ms
- [ ] Unit test coverage ≥70%

### 2.5 Risks & Mitigations

| Risk | Mitigation |
|------|-----------|
| Tree-sitter binding issues | Test early with small Python files |
| Graph structure mismatch | Compare node/edge counts with LocAgent |

---

## 3. Phase 2: Service + SDK Integration (Weeks 3-5)

### 3.1 Goals

- Complete BM25 indexing (lower index)
- Implement graph traversal
- Stand up CDS-Index Service (JSON-RPC)
- Integrate with Claude Agent SDK

### 3.2 Deliverables

| Deliverable | Owner | Status |
|------------|-------|--------|
| BM25 index (tantivy or custom) | Rust Dev 2 | ☐ |
| Hierarchical search (upper + lower) | Rust Dev 2 | ☐ |
| `cds_traversal` crate (BFS) | Rust Dev 1 | ☐ |
| `cds traverse`, `cds retrieve` commands | Rust Dev 1 | ☐ |
| CDS-Index Service (JSON-RPC) | Rust Dev 3 | ☐ |
| TypeScript agent with bash tool | TS Dev 1 | ☐ |
| System prompt with CoT | TS Dev 1 | ☐ |

### 3.3 Tasks

#### Week 3: BM25 & Traversal

**Day 1-3**: BM25 Implementation

- [ ] Integrate `tantivy` crate
- [ ] Build inverted index from code content
- [ ] Implement BM25 search
- [ ] Benchmark: Compare search results with LocAgent

**Day 4-5**: Graph Traversal

- [ ] Implement BFS algorithm
- [ ] Add filters (relation type, entity type, depth)
- [ ] Tree formatting (LocAgent-style output)
- [ ] Unit tests: Traversal on sample graphs

#### Week 4: CLI Completeness

**Day 1-2**: Remaining Commands

- [ ] `cds traverse` command with all options
- [ ] `cds retrieve` command
- [ ] Fold/preview/full snippet formatting

**Day 3-5**: Service Layer

- [ ] Set up JSON-RPC server (`axum` or `jsonrpc`)
- [ ] Implement RPC methods (search, traverse, retrieve)
- [ ] Health and metrics endpoints
- [ ] Integration test: CLI calls service

#### Week 5: Claude Agent Integration

**Day 1-2**: Agent Setup

- [ ] Install `@anthropic-ai/claude-agent-sdk`
- [ ] Create TypeScript project structure
- [ ] Implement agent initialization
- [ ] Configure bash tool with allowed commands

**Day 3-4**: System Prompt & Hooks

- [ ] Write system prompt (LocAgent CoT steps)
- [ ] Implement PreToolUse hook (inject index paths)
- [ ] Implement PostToolUse hook (compress outputs)
- [ ] Test: Agent can call `cds search`

**Day 5**: E2E Test

- [ ] Run full agent workflow on sample issue
- [ ] Verify tool calls and final answer
- [ ] Debug and iterate

### 3.4 Success Criteria

- [ ] Hierarchical search matches LocAgent top-10 results (50 queries)
- [ ] Traverse produces correct subgraphs
- [ ] Claude agent can execute multi-step code search
- [ ] Performance: Search <500ms, traverse (2-hop) <1s

### 3.5 Risks & Mitigations

| Risk | Mitigation |
|------|-----------|
| BM25 accuracy lower than LocAgent | Validate tokenization, try custom BM25 |
| Agent tool calls fail | Add detailed error messages, logging |

---

## 4. Phase 3: Agentization & Hooks (Weeks 6-7)

### 4.1 Goals

- Implement `code-retrievaler` subagent
- Add all hooks (Pre/Post/SubagentStop)
- Enable hybrid retrieval (cds + rg + ast-grep)
- Validate on SWE-bench Lite samples

### 4.2 Deliverables

| Deliverable | Owner | Status |
|------------|-------|--------|
| `code-retrievaler` subagent config | TS Dev 1 | ☐ |
| All hooks implemented | TS Dev 1 | ☐ |
| Hybrid retrieval examples | TS Dev 2 | ☐ |
| `cds combo` command (YAML plans) | Rust Dev 2 | ☐ |
| SWE-bench Lite validation (10 samples) | QA Lead | ☐ |

### 4.3 Tasks

#### Week 6: Subagent & Hooks

**Day 1-2**: Subagent Configuration

- [ ] Define subagent YAML spec
- [ ] Implement subagent invocation from main agent
- [ ] Restrict bash commands (allow-list)

**Day 3-4**: Hooks Enhancement

- [ ] SubagentStop hook with session logging
- [ ] PostToolUse: Compress large JSON outputs
- [ ] PreToolUse: Validate and inject context

**Day 5**: Hybrid Workflows

- [ ] Document cds + rg + ast-grep examples
- [ ] Test piping workflows (CLI integration)

#### Week 7: Validation & Refinement

**Day 1-3**: SWE-bench Lite Testing

- [ ] Select 10 representative instances
- [ ] Run agent on each instance
- [ ] Compare locations with ground truth
- [ ] Compute Acc@5, NDCG@5

**Day 4-5**: Iteration

- [ ] Fix issues identified in testing
- [ ] Tune prompts for better accuracy
- [ ] Optimize tool call sequences

### 4.4 Success Criteria

- [ ] Subagent successfully delegates to bash tool
- [ ] Hooks reduce context usage by >30%
- [ ] SWE-bench Lite (10 samples): Acc@5 ≥75%

---

## 5. Phase 4: Polyglot & Optimization (Weeks 8-10)

### 5.1 Goals

- Add TypeScript and Java language support
- Optimize for performance (parallelization, mmap)
- Containerize and document deployment
- Full SWE-bench Lite validation (300 instances)

### 5.2 Deliverables

| Deliverable | Owner | Status |
|------------|-------|--------|
| TypeScript parser | Rust Dev 1 | ☐ |
| Java parser | Rust Dev 1 | ☐ |
| Parallelized indexing (rayon) | Rust Dev 3 | ☐ |
| Dockerfiles & compose | DevOps | ☐ |
| SWE-bench Lite full validation | QA Lead | ☐ |
| Documentation (README, guides) | Tech Writer | ☐ |

### 5.3 Tasks

#### Week 8: Multi-Language Support

**Day 1-2**: TypeScript Parser

- [ ] Add tree-sitter-typescript grammar
- [ ] Implement TypeScript entity extractor
- [ ] Test on sample TS repos

**Day 3-4**: Java Parser

- [ ] Add tree-sitter-java grammar
- [ ] Implement Java entity extractor
- [ ] Test on sample Java repos

**Day 5**: Language Detection

- [ ] Auto-detect language from file extension
- [ ] Configure per-repo language settings

#### Week 9: Performance Optimization

**Day 1-2**: Parallelization

- [ ] Parallelize file parsing with `rayon`
- [ ] Parallelize BM25 indexing
- [ ] Benchmark: Compare before/after

**Day 3-4**: Memory Optimization

- [ ] Implement mmap for large indices
- [ ] String interning for file paths
- [ ] Profile and reduce allocations

**Day 5**: Benchmarking

- [ ] Full performance benchmarking
- [ ] Validate targets (Index <5s/1K files, Search <500ms)

#### Week 10: Deployment & Validation

**Day 1-2**: Containerization

- [ ] Create Dockerfile for cds-indexd
- [ ] Docker Compose setup
- [ ] Kubernetes manifests (optional)

**Day 3-5**: Full SWE-bench Lite Validation

- [ ] Index all SWE-bench Lite repos
- [ ] Run agent on 300 instances
- [ ] Compute metrics: File Acc@5, Func Acc@10, NDCG@5
- [ ] Compare with LocAgent published results

### 5.4 Success Criteria

- [ ] TypeScript and Java support functional
- [ ] Performance targets met or exceeded
- [ ] SWE-bench Lite: File Acc@5 ≥80%, NDCG@5 ≥0.70
- [ ] Docker deployment tested and documented

---

## 6. Post-v1.0 Roadmap (Optional)

### 6.1 Future Enhancements (v1.1+)

| Feature | Priority | Effort | Target Version |
|---------|----------|--------|----------------|
| Vector/semantic search | Medium | 3 weeks | v1.1 |
| gRPC service (replace JSON-RPC) | Medium | 2 weeks | v1.1 |
| Multi-repo federated search | Low | 4 weeks | v1.2 |
| Fine-tuning on agent trajectories | High | 6 weeks | v1.2 |
| Web UI for index exploration | Low | 5 weeks | v2.0 |
| OpenAI Codex SDK adapter | Medium | 2 weeks | v1.1 |

### 6.2 Research Extensions

- Fine-tune Claude/Qwen on successful trajectories (LocAgent §5.2)
- Explore LLM-guided index compression
- Continuous learning from agent feedback

---

## 7. Resource Allocation

### 7.1 Team Structure

| Role | Count | Responsibility |
|------|-------|----------------|
| **Rust Lead** | 1 | Architecture, code review, performance |
| **Rust Developers** | 3 | Crates implementation, testing |
| **TypeScript Developer** | 1 | Agent integration, hooks |
| **QA Lead** | 1 | Testing, benchmark validation |
| **DevOps** | 0.5 | CI/CD, containerization |
| **Tech Writer** | 0.5 | Documentation |

**Total**: ~6.5 FTE

### 7.2 Time Allocation by Phase

| Phase | Duration | FTE-weeks |
|-------|----------|-----------|
| Phase 1 | 2 weeks | 12 |
| Phase 2 | 3 weeks | 18 |
| Phase 3 | 2 weeks | 12 |
| Phase 4 | 3 weeks | 18 |
| **Total** | **10 weeks** | **60 FTE-weeks** |

---

## 8. Milestones & Release Plan

### 8.1 Milestone Definitions

| Milestone | Criteria | Date |
|-----------|----------|------|
| **M1: Prototype** | Phase 1 complete, basic search works | Week 2 |
| **M2: Alpha** | Phase 2 complete, agent integration functional | Week 5 |
| **M3: Beta** | Phase 3 complete, SWE-bench samples validated | Week 7 |
| **M4: v1.0 RC** | Phase 4 complete, full validation done | Week 10 |
| **M5: v1.0 Release** | Bug fixes, documentation finalized | Week 12 |

### 8.2 Release Checklist (v1.0)

- [ ] All acceptance criteria met (PRD-08)
- [ ] SWE-bench Lite: Acc@5 ≥80%
- [ ] Performance benchmarks passed
- [ ] Documentation complete (README, API docs)
- [ ] Docker images published
- [ ] Security audit passed
- [ ] 5 beta testers validated product
- [ ] Release notes written

---

## 9. Risk Management

### 9.1 Critical Path Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|-----------|
| BM25 accuracy gap vs LocAgent | Medium | High | Early validation, custom BM25 if needed |
| Claude API rate limits | Low | Medium | Caching, local testing with mocks |
| Tree-sitter compatibility issues | Low | High | Test all grammars early |
| Team bandwidth shortage | Medium | High | Prioritize core features, defer nice-to-haves |

### 9.2 Schedule Risks

| Risk | Impact | Mitigation |
|------|--------|-----------|
| Phase 2 runs long (SDK complexity) | +1 week | Buffer in Phase 4, reduce polyglot scope |
| SWE-bench validation uncovers gaps | +2 weeks | Continuous validation from Week 5 |

---

## 10. Communication Plan

### 10.1 Weekly Sync

- **When**: Every Monday, 10 AM
- **Attendees**: Full team
- **Agenda**: Progress updates, blockers, next week's plan

### 10.2 Milestone Reviews

- **When**: End of each phase
- **Attendees**: Team + stakeholders
- **Agenda**: Demo, metrics review, decision on go/no-go for next phase

### 10.3 Status Reporting

- **Weekly**: Update Linear/Jira with completed tasks
- **Bi-weekly**: Stakeholder email with progress summary

---

## 11. Success Metrics

### 11.1 Quantitative

| Metric | Target | Actual (TBD) |
|--------|--------|--------------|
| SWE-bench File Acc@5 | ≥80% | ___ |
| SWE-bench NDCG@5 | ≥0.70 | ___ |
| Search latency (p95) | <500ms | ___ |
| Index build time (1K files) | <5s | ___ |
| Test coverage | ≥80% | ___ |

### 11.2 Qualitative

- [ ] Code quality: Passes clippy, rustfmt
- [ ] Usability: 5 beta testers report "easy to use"
- [ ] Documentation: Complete and clear (stakeholder approval)

---

**Status**: Ready for execution. Requires team onboarding and kickoff meeting.

**Next Steps**:

1. Assemble team and assign roles
2. Set up development environment
3. Schedule Phase 1 kickoff
4. Begin Week 1 tasks
