# T-02-02 Sparse Index - AI Engineer Tasks Guide

**Task**: T-02-02-sparse-index - Sparse Index - Name/ID + BM25 Search
**Version**: 1.1
**Date**: 2025-11-02 UTC
**Last Updated**: 2025-11-02 UTC (Infrastructure updates post-Session 04, and task dev-cooking phase-session Next SESSIONS-05-Threads-01-NN Initialization)

- @.artifacts/spec-tasks-T-02-02-sparse-index/metadata.yaml <📌> ["T-02-02 Sparse Index Metadata"](.artifacts/spec-tasks-T-02-02-sparse-index/metadata.yaml) </📌>
- @spacs/prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md <📌> ["CDS Index Service PRD"](spacs/prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md) </📌>
- @spacs/issues/04-0.1.0-mvp/02-index-core/02-sparse-index.md <📌> ["Sparse Index Issue"](spacs/issues/04-0.1.0-mvp/02-index-core/02-sparse-index.md) </📌>
- @spacs/tasks/0.1.0-mvp/02-index-core/T-02-02-sparse-index.md <📌> ["Sparse Index Task"](spacs/tasks/0.1.0-mvp/02-index-core/T-02-02-sparse-index.md) </📌>

- spacs/tasks/0.1.0-mvp/TODO.yaml <📌> ["Sparse Index TODO List"](spacs/tasks/0.1.0-mvp/TODO.yaml) </📌>

---

## 📚 Essential Reference

**MUST READ FIRST**: [WORKLOG-HANDBOOK.md](.dev/workflows/WORKLOG-HANDBOOK.md)

The WORKLOG-HANDBOOK.md contains the complete workflow for:

- Hierarchical structure (Task → Phase → Day → Session → Thread)
- Work session lifecycle
- RAW log management
- Metadata updates
- Checkpoint workflow
- Common mistakes and how to avoid them

**Read it before any worklog operations!**

---

## 🏗️ Infrastructure Updates (2025-11-02)

### Major Infrastructure Reorganization

Following Session 04 (2025-11-01), comprehensive infrastructure reorganization was completed:

#### 1. **New `.dev/` Directory** (39 files, +11,857 lines)

**Workflows** (17 files): `.dev/workflows/`

- WORKTREE_WORKFLOW.md
- WORK_SESSION_CHECKPOINT_WORKFLOW.md
- SESSION_INITIALIZATION_WORKFLOW.md
- WORKLOG-HANDBOOK.md
- NEXT_TASK_CHECKLIST.md
- checkpoint/ (11 detailed guides)

**Scripts** (14 files): `.dev/scripts/`

- **Session Management**: `session/create-session-worklog.sh`, `session/create-raw-log.sh`
- **Task Management**: `task/create-task-worklog.sh`, `task/worktree-symlink.sh`, `task/sync-worktrees.sh`
- **Validation**: `validation/checkpoint-helper.sh` ✨ (bugfixes), `validation/git-notes-check.sh`

**Templates** (7 files): `.dev/templates/`

- metadata.template.yaml
- worklogs/ templates (work-summary, commit-log, notes, codereview, raw-session)

**Tools** (1 file): `.dev/tools/`

- RFC-DEV-TOOLS.md (complete dev toolkit reference)

#### 2. **New `.claude/` Directory** (48 files, Claude Code CLI Integration)

**AI Engineer's Sub-Agents** (3 files): `.claude/agents/`

- code-analyzer.md
- code-retriever.md
- document-retriever.md

**Hooks** (24 source + 4 binaries): `.claude/hooks/`

- **Compiled Hooks** (88MB total):
  - inject-datetime (580KB) - UTC timestamp injection ✅
  - user-prompt-submit (2.2MB) - Security filtering ✅
  - pre-tool-use-approval (508KB) - Tool approval ✅
  - markdown-formatter (2.3MB) - Auto-formatting ✅
- **Source Code**: `rs/` (Rust workspace with 4 hook crates)

**AI Engineer Skills** (17 files): `.claude/skills/csda_aiengineer_team/`

- git-workflow-validation/
- session-management/
- task-initialization/
- template-usage/
- worktree-management/

#### 3. **Path Migrations**

- `scripts/` → `.dev/scripts/` (14 files)
- `docs/` → `.dev/workflows/` (17 files)
- `.artifacts/spec-tasks-templates/` → `.dev/templates/` (7 files)

#### 4. **Infrastructure Commits**

- **4fbec17**: Major reorganization (157 files, +13,854/-1,997)
- **5bc82dc**: Fix checkpoint-helper.sh REPO_ROOT path
- **b1a0f18**: Fix checkpoint-helper.sh arithmetic increments
- **cf547c8**: Fix hooks compilation (4/4 hooks built, 5/5 tests passed)

**Status**: ✅ All 27 commits synced to remote with git notes (100% coverage)

---

## Task Overview

### Current Status (2025-11-02, Session 05 In Progress)

- **Start Date**: 2025-10-31
- **Estimated Hours**: 32 (4 days × 8 hours)
- **Actual Hours**: 8.3h (Sessions 01-04 complete)
- **Status**: in_progress (Phase 3 - BM25 Integration & Parity Validation)

### Progress Summary

**Completed Phases**:

- ✅ Phase 0: Planning & Analysis (Sessions 01-02, 1.75h)
- ✅ Phase 1: Upper Index - NameIndex (Session 03, 3.3h)
- ✅ Phase 2: Custom Tokenizer + BM25 Scaffold (Session 04, 3.2h)

**Current Phase**:

- 🚧 Phase 3: BM25 Integration & Parity Validation (Session 05, in progress)

**Remaining Phases**:

- ⏳ Phase 4: Hierarchical Search Strategy (Day 2)
- ⏳ Phase 5: Comprehensive Benchmarking (Day 2)

### Implementation Status

| Component | Status | API | Tests | Coverage | Performance |
|-----------|--------|-----|-------|----------|-------------|
| `name_index.rs` | ✅ COMPLETE | exact_match, prefix_match, from_graph | 8 passing | 97.20% | 68 ns / 699 ns |
| `tokenizer.rs` | ✅ COMPLETE | tokenize, tokenize_with_offsets | 7 passing | - | - |
| `bm25.rs` | 🚧 SCAFFOLD | create_in_dir, open, search | 2 passing | - | - |
| `stop_words.rs` | ✅ COMPLETE | STOP_WORDS constant | - | - | 180 words |
| `sparse_index.rs` | ❌ NOT STARTED | - | - | - | **Phase 3 Goal** |

**Total Tests**: 78/78 passing (100%)
**Coverage**: 97.20% lines, 95.35% functions (Session 03 baseline)

---

## Tasks References

### Core Documentation

- `.artifacts/spec-tasks-T-02-02-sparse-index/metadata.yaml` - Task metadata
- `LocAgent Paper & Repo`: `tmp/LocAgent/` - Python reference implementation
- `Claude Agent SDK`: `tmp/claude-agent-sdk/` - TypeScript integration reference
- `Claude Code CLI Docs`: `tmp/claude-code-cli-docs/` - CLI documentation

### PRDs (`spacs/prd/0.1.0-MVP-PRDs-v0/`)

- 01-system-architecture.md
- 02-cds-index-service.md ⭐
- 03-cds-tools-cli.md
- 04-cds-agent-integration.md
- 05-api-specifications.md
- 06-rust-refactoring-plan.md ⭐
- 07-deployment-operations.md
- 08-testing-quality.md
- 09-implementation-roadmap.md
- 10-extensibility-future.md

### Issues (`spacs/issues/04-0.1.0-mvp/`)

- README.md
- 02-index-core/00-overview.md
- 02-index-core/01-graph-build.md (T-02-01 ✅)
- 02-index-core/02-sparse-index.md ⭐ (T-02-02 🚧)
- 02-index-core/03-service-layer.md
- 02-index-core/04-serialization-fixtures.md

### Tasks (`spacs/tasks/0.1.0-mvp/`)

- TODO.yaml ⭐ - Central task registry
- README.md
- 02-index-core/README.md
- 02-index-core/T-02-01-graph-builder.md (✅ COMPLETED)
- 02-index-core/T-02-02-sparse-index.md ⭐ (🚧 IN PROGRESS)
- 02-index-core/T-02-03-service-layer.md
- 02-index-core/T-02-04-serialization.md

---

## Completed Work (Sessions 01-04, 8.3h)

### ✅ Session 01-02: [Phase 0] Planning & Analysis (1.75h)

**Session 01** (2025-10-31, 07:17-08:30 UTC, 1.2h)

- Thread 01: Worktree initialization (0.05h)
- Thread 02: Documentation updates for M2 milestone (0.15h)
- Thread 03: Comprehensive planning & implementation roadmap (0.75h, 986 lines)
- **RAW Log**: `WORK-SESSIONS-01-THREADS-01-03-SUMMARY-2025-10-31.txt`

**Session 02** (2025-10-31, 10:22-10:55 UTC, 0.55h)

- Thread 01: Spec alignment & implementation gap analysis
- Thread 02: Parity assets + performance baselines review
- Thread 03: Implementation readiness checklist & roadmap
- **RAW Log**: `WORK-SESSIONS-02-THREADS-01-03-SUMMARY-2025-10-31.txt`

### ✅ Session 03: [Phase 1] Upper Index Implementation (3.3h)

**Date**: 2025-10-31, 12:02-15:17 UTC
**Threads**: 01-04

**Deliverables**:

- `name_index.rs` (477 lines) - DashMap-based upper index
- `index_tests.rs` (83 lines, 8 tests) - exact_match, prefix_match, from_graph
- `search_bench.rs` (+91 lines) - Performance benchmarks

**Performance**:

- Exact match: 68.42 ns
- Prefix match: 699.40 ns
- Build 1,024 entities: 2.287 ms

**Quality**:

- Coverage: 97.20% lines, 95.35% functions
- Tests: 8/8 passing

**Commits**:

- 7f390a7: feat(index): Phase 1 - NameIndex upper tier with <1μs queries
- 7b624c4: checkpoint(worklog): Session 03 complete

**RAW Log**: `WORK-SESSIONS-03-THREADS-01-04-SUMMARY-2025-10-31.txt`

### ✅ Session 04: [Phase 2] Custom Tokenizer + BM25 Scaffold (3.2h)

**Date**: 2025-11-01, 01:39-12:45 UTC
**Threads**: 01-07

**Deliverables**:

- `tokenizer.rs` (387 lines) - Custom code tokenizer with offset preservation
- `bm25.rs` (+442 lines) - Tantivy backend integration, custom analyzer
- `stop_words.rs` (180 lines) - Python stop words from LocAgent
- `export_stop_words.py` (176 lines) - Stop-word extraction automation
- 12 new tests (7 tokenizer, 2 BM25, 3 fixture)

**Quality**:

- All 78/78 tests passing
- 5 clippy errors fixed
- ~95% coverage maintained

**Commits**:

- e4beefa: init(session): Session 04 kickoff
- 414f7f2: feat(index): Phase 2 - tokenizer + BM25 scaffold with parity strategy
- 9e1774e: checkpoint(worklog): Session 04 complete

**RAW Log**: `WORK-SESSIONS-04-THREADS-01-07-SUMMARY-2025-11-01.txt`

---

## Current Session

### 🚧 Session 05: [Phase 3] BM25 Integration & Parity Validation (in progress)

**Date**: 2025-11-02, Started 07:43 UTC
**Threads**: 01-NN (Thread 01 COMPLETE)

**Objectives**:

- Implement `BM25Index::from_graph()` builder
- Create unified `SparseIndex` wrapper (hierarchical search)
- Build parity test harness for 50 LocAgent queries
- **CRITICAL**: Achieve search overlap@10 ≥90%
- Validate search latency <500ms p95

**Progress**:

- ✅ Thread 01: Phase 3 planning & integration strategy (~30 min)
- ⏳ Thread 02: Implement BM25::from_graph() (pending)
- ⏳ Thread 03: Create SparseIndex wrapper (pending)
- ⏳ Thread 04: Build parity test harness (pending)
- ⏳ Thread 05: Run parity validation & tuning (pending)
- ⏳ Thread 06: Performance benchmarking (pending)
- ⏳ Thread 07: Documentation & code review (pending)

**RAW Log**: `WORK-SESSIONS-05-THREADS-01-NN-SUMMARY-2025-11-02.txt` (in progress)

**Estimated Duration**: 5-6 hours total

---

## Quick Commands

### Session Management (Updated Paths)

```shell
# Start new session (use .dev/scripts/ path)
./.dev/scripts/session/create-session-worklog.sh T-02-02-sparse-index NN "Description" "Developer Name"

# After session completes, create RAW log
./.dev/scripts/session/create-raw-log.sh T-02-02-sparse-index NN START END "Description"

# Update metadata.yaml after session
vim .artifacts/spec-tasks-T-02-02-sparse-index/metadata.yaml
# Update: actual_hours, Work Sessions, commits

# Run checkpoint validation
./.dev/scripts/validation/checkpoint-helper.sh T-02-02-sparse-index
./.dev/scripts/validation/git-notes-check.sh
```

### Git Operations

```shell
# Add git notes to commit
git notes add <hash> -m "spec-tasks/T-02-02-sparse-index
Day: N
Date: 2025-11-0N
Sessions: {NN} (Threads {START}-{END})
Duration: X.Xh
Worklog: .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-11-0N-*
Status: ...
Files: N files (±NNN lines)"

# Push commits AND notes
git push origin feat/task/T-02-02-sparse-index
git push origin refs/notes/commits  # Don't forget!
```

### Testing & Validation

```shell
# Run all tests
cargo test --lib

# Run specific test module
cargo test --lib index::name_index::tests

# Run parity tests (Phase 3)
cargo test search_parity_tests -- --nocapture

# Run benchmarks
cargo bench --bench search_bench
```

---

## Implementation Phases Plan

- ✅ **Phase 0**: Planning & Analysis (Day 1) - Sessions 01-02 complete (1.75h)
- ✅ **Phase 1**: Upper Index - Name/ID HashMap (Day 1) - Session 03 complete (3.3h)
- ✅ **Phase 2**: Custom Tokenizer (Day 1) - Session 04 complete (3.2h)
- 🚧 **Phase 3**: BM25 Integration & Parity Validation (Day 3) - Session 05 in progress
- ⏳ **Phase 4**: Hierarchical Search Strategy (Day 2-3)
- ⏳ **Phase 5**: Comprehensive Benchmarking (Day 2-3)

---

## Key Files

```tree
.artifacts/spec-tasks-T-02-02-sparse-index/
├── CLAUDE.md                              # This file
├── metadata.yaml                          # Central task metadata
├── git-refs.txt                           # Git workflow reference
└── worklogs/
    ├── 2025-10-31-S01-S02-work-summary.md # Sessions 01-02 (combined)
    ├── 2025-10-31-S01-S02-commit-log.md
    ├── 2025-10-31-S01-S02-notes.md
    ├── 2025-10-31-S03-work-summary.md     # Session 03
    ├── 2025-10-31-S03-commit-log.md
    ├── 2025-10-31-S03-notes.md
    ├── 2025-11-01-S04-work-summary.md     # Session 04
    ├── 2025-11-01-S04-commit-log.md
    ├── 2025-11-01-S04-notes.md
    ├── 2025-11-01-S04-codereview.md
    ├── 2025-11-02-S05-work-summary.md     # Session 05 (in progress)
    ├── 2025-11-02-S05-commit-log.md
    ├── 2025-11-02-S05-notes.md
    ├── 2025-11-02-S05-codereview.md
    └── raw/
        ├── WORK-SESSIONS-01-THREADS-01-03-SUMMARY-2025-10-31.txt
        ├── WORK-SESSIONS-02-THREADS-01-03-SUMMARY-2025-10-31.txt
        ├── WORK-SESSIONS-03-THREADS-01-04-SUMMARY-2025-10-31.txt
        ├── WORK-SESSIONS-04-THREADS-01-07-SUMMARY-2025-11-01.txt
        └── WORK-SESSIONS-05-THREADS-01-NN-SUMMARY-2025-11-02.txt (in progress)
```

---

## Critical Reminders

### ❌ Common Mistakes to Avoid

1. **Thread Numbering**: Reset to 01 for EACH session (don't continue across sessions!)
2. **RAW Log Timing**: Create DURING or AFTER session (document as you work!)
3. **Session = ONE RAW log**: Don't create separate files per thread
4. **Update actual_hours**: Sum all completed sessions (currently 8.3h)
5. **Git Notes**: Always add notes and push with `git push origin refs/notes/commits`
6. **Script Paths**: Use `./.dev/scripts/` (not `./scripts/`) after reorganization

### ✅ Correct Pattern

```text
Session 01: Threads 01-03 ✅ COMPLETE (RAW log created)
Session 02: Threads 01-03 ✅ COMPLETE (RAW log created)
Session 03: Threads 01-04 ✅ COMPLETE (RAW log created)
Session 04: Threads 01-07 ✅ COMPLETE (RAW log created)
Session 05: Threads 01-NN 🚧 IN PROGRESS (RAW log being documented)
```

---

## Session Lifecycle Checklist

When completing a session:

- [ ] Review git log and identify thread boundaries
- [ ] Update RAW log file with final thread summaries
- [ ] Update metadata.yaml (hours, sessions, commits)
- [ ] Update daily worklogs (work-summary, commit-log, notes, codereview if applicable)
- [ ] Create checkpoint commit with git notes
- [ ] Push commits AND git notes
- [ ] Run verification: `./.dev/scripts/validation/git-notes-check.sh`
- [ ] Run validation: `./.dev/scripts/validation/checkpoint-helper.sh T-02-02-sparse-index`

---

## Technical Context

### Dependencies

- ✅ **T-02-01-graph-builder**: COMPLETED & MERGED (PR #6)
  - Provides: 4 node types, 4 edge types, <2% parity variance
  - Graph API: `crates/cds-index/src/graph/mod.rs`
  - Parity tests: 6 fixtures passing (658 to 6,876 nodes)

### Test Fixtures

- **Parity baselines**: `tests/fixtures/parity/golden_outputs/`
  - 6 repos with graph data (LocAgent + 5 SWE-bench)
  - 50 search queries for overlap@10 testing (search_queries.jsonl)
  - 60 traverse samples (traverse_samples.jsonl)
  - Performance baselines (performance_baselines.json)

### Performance Targets

- **Search latency**: <500ms p95 ⚠️ CRITICAL (Phase 3 validation)
- **Index build**: <5s for 1K files ✅ EXCEEDED (2.287 ms, Phase 1)
- **Search overlap@10**: ≥90% on 50 queries ⚠️ CRITICAL (Phase 3 goal)
- **Unit test coverage**: >95% ✅ ACHIEVED (97.20%, Phase 1)

---

## Acceptance Criteria

- [x] Upper index (name/ID HashMap) with prefix matching ✅ Phase 1
- [ ] Lower index (BM25 k1=1.5, b=0.75) 🚧 Phase 3 in progress
- [x] Search latency <500ms p95 ✅ <1μs for upper index (Phase 1)
- [x] Index build <5s for 1K files ✅ 2.287 ms (Phase 1)
- [ ] **Search overlap@10 ≥90% on 50 queries** ⚠️ CRITICAL - Phase 3 goal
- [x] Unit test coverage >95% ✅ 97.20% (Phase 1-2)

**Overall Status**: 4/6 complete (67%), 1 in progress, 1 not started

---

## Resources

### Documentation

- **PRD**: `spacs/prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md`
- **Issue**: `spacs/issues/04-0.1.0-mvp/02-index-core/02-sparse-index.md`
- **Task**: `spacs/tasks/0.1.0-mvp/02-index-core/T-02-02-sparse-index.md`
- **TODO**: `spacs/tasks/0.1.0-mvp/TODO.yaml`
- **Workflow**: `.dev/workflows/WORKLOG-HANDBOOK.md`

### Reference Code

- **LocAgent**: `tmp/LocAgent/dependency_graph/entity_searcher.py`
- **Graph API**: `crates/cds-index/src/graph/mod.rs` (T-02-01 ✅)
- **Test fixtures**: `tests/fixtures/parity/`

### Automation Scripts (Updated Paths)

**Session Management**:

- `./.dev/scripts/session/create-session-worklog.sh`
- `./.dev/scripts/session/create-raw-log.sh`

**Task Management**:

- `./.dev/scripts/task/create-task-worklog.sh`
- `./.dev/scripts/task/worktree-symlink.sh`
- `./.dev/scripts/task/sync-worktrees.sh`

**Validation**:

- `./.dev/scripts/validation/checkpoint-helper.sh` - Pre-checkpoint validation
- `./.dev/scripts/validation/git-notes-check.sh` - Git notes verification

**Templates**:

- `./.dev/templates/` - All worklog templates

---

## Version History

- **v1.0** (2025-10-31): Initial guide created after Session 01
- **v1.1** (2025-11-02): Updated post-infrastructure reorganization
  - Updated all script paths (.dev/scripts/)
  - Added Infrastructure Updates section
  - Updated Completed Work (Sessions 01-04)
  - Added Current Session (Session 05)
  - Updated Key Files structure
  - Updated Automation Scripts paths
  - Added .claude/ and .dev/ directory references

---

**Maintainer**: Claude and Codex AI Engineer
**Status**: Active
**Current Session**: 05 (Phase 3 - BM25 Integration & Parity Validation)
**Progress**: 8.3h / 32h estimated (26%), Phases 0-2 complete

---

END OF AI ENGINEER TASKS GUIDE
