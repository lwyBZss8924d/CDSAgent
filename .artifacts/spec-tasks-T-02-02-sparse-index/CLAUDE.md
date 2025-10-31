# T-02-02 Sparse Index - AI Engineer Guide

**Task**: T-02-02-sparse-index - Sparse Index - Name/ID + BM25 Search
**Version**: 1.0
**Date**: 2025-10-31

- @.artifacts/spec-tasks-T-02-02-sparse-index/metadata.yaml <📌> ["T-02-02 Sparse Index Metadata"](.artifacts/spec-tasks-T-02-02-sparse-index/metadata.yaml) </📌>
- @spacs/prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md <📌> ["CDS Index Service PRD"](spacs/prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md) </📌>
- @spacs/issues/04-0.1.0-mvp/02-index-core/02-sparse-index.md <📌> ["Sparse Index Issue"](spacs/issues/04-0.1.0-mvp/02-index-core/02-sparse-index.md) </📌>
- @spacs/tasks/0.1.0-mvp/02-index-core/T-02-02-sparse-index.md <📌> ["Sparse Index Task"](spacs/tasks/0.1.0-mvp/02-index-core/T-02-02-sparse-index.md) </📌>
- @spacs/tasks/0.1.0-mvp/TODO.yaml <📌> ["Sparse Index TODO List"](spacs/tasks/0.1.0-mvp/TODO.yaml) </📌>
- @.artifacts/spec-tasks-T-02-02-sparse-index/WORKLOG-HANDBOOK.md <📌> ["T-02-02 Sparse Index WORKLOG-HANDBOOK"](.artifacts/spec-tasks-T-02-02-sparse-index/WORKLOG-HANDBOOK.md) </📌>

---

## 📚 Essential Reference

**MUST READ FIRST**: [WORKLOG-HANDBOOK.md](../../WORKLOG-HANDBOOK.md)

The WORKLOG-HANDBOOK.md contains the complete workflow for:

- Hierarchical structure (Task → Phase → Day → Session → Thread)
- Work session lifecycle
- RAW log management
- Metadata updates
- Checkpoint workflow
- Common mistakes and how to avoid them

**Read it before any worklog operations!**

---

## Task Overview

### Current Status

- **Start Date**: 2025-10-31
- **Estimated Hours**: 32 (4 days × 8 hours)
- **Actual Hours**: 1.2h (Session 01 complete)
- **Status**: in_progress

## Tasks references

- .artifacts/spec-tasks-T-02-02-sparse-index/metadata.yaml

- LocAgent Paper's and Repo reference: (tmp/LocAgent/)
- Claude Agent SDK reference: (tmp/claude-agent-sdk/)
- Claude Code CLI reference: (tmp/claude-code-cli-docs/)

### (spacs/prd/0.1.0-MVP-PRDs-v0/)

- spacs/prd/0.1.0-MVP-PRDs-v0/01-system-architecture.md
- spacs/prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md
- spacs/prd/0.1.0-MVP-PRDs-v0/03-cds-tools-cli.md
- spacs/prd/0.1.0-MVP-PRDs-v0/04-cds-agent-integration.md
- spacs/prd/0.1.0-MVP-PRDs-v0/05-api-specifications.md
- spacs/prd/0.1.0-MVP-PRDs-v0/06-rust-refactoring-plan.md
- spacs/prd/0.1.0-MVP-PRDs-v0/07-deployment-operations.md
- spacs/prd/0.1.0-MVP-PRDs-v0/08-testing-quality.md
- spacs/prd/0.1.0-MVP-PRDs-v0/09-implementation-roadmap.md
- spacs/prd/0.1.0-MVP-PRDs-v0/10-extensibility-future.md

### (spacs/issues/04-0.1.0-mvp/)

- spacs/issues/04-0.1.0-mvp/README.md
- spacs/issues/04-0.1.0-mvp/02-index-core/00-overview.md
- spacs/issues/04-0.1.0-mvp/02-index-core/01-graph-build.md
- spacs/issues/04-0.1.0-mvp/02-index-core/02-sparse-index.md
- spacs/issues/04-0.1.0-mvp/02-index-core/03-service-layer.md
- spacs/issues/04-0.1.0-mvp/02-index-core/04-serialization-fixtures.md

### (spacs/tasks/0.1.0-mvp/)

- spacs/tasks/0.1.0-mvp/TODO.yaml
- spacs/tasks/0.1.0-mvp/README.md
- spacs/tasks/0.1.0-mvp/02-index-core/README.md
- spacs/tasks/0.1.0-mvp/02-index-core/T-02-01-graph-builder.md
- spacs/tasks/0.1.0-mvp/02-index-core/T-02-02-sparse-index.md
- spacs/tasks/0.1.0-mvp/02-index-core/T-02-03-service-layer.md
- spacs/tasks/0.1.0-mvp/02-index-core/T-02-04-serialization.md

### Completed Work

✅ **Session 01**: [Phase 0] Planning & Analysis (1.2h)

- Thread 01: Worktree initialization (0.05h)
- Thread 02: Documentation updates (0.15h)
- Thread 03: Comprehensive planning (0.75h, 986 lines)

### Next Sessions

📝 **Session 02**: [Phase 0] Deep Research (TBD)

- Read 23 PRD/Issue/Task documents
- Re-analysis and detailed planning

🚧 **Session 03**: [Phase 1] Upper Index Implementation (4-6h)

- NameIndex design & exact match
- Prefix match & type filtering
- Graph integration & benchmarks

---

## Quick Commands

### Session Management

```shell
# When Session completes, create RAW log:
# DO NOT create before session starts!
cd ~/dev-space/CDSAgent-T-02-02-sparse-index/.artifacts/spec-tasks-T-02-02-sparse-index/worklogs/raw
cat > WORK-SESSIONS-{NN}-THREADS-{START}-{END}-SUMMARY-2025-10-31.txt <<EOF
...
EOF

# Update metadata.yaml after session
vim .artifacts/spec-tasks-T-02-02-sparse-index/metadata.yaml
# Update: actual_hours, Work Sessions, commits

# Run checkpoint
./scripts/checkpoint-helper.sh T-02-02-sparse-index
./scripts/git-notes-check.sh
```

### Git Operations

```shell
# Add git notes to commit
git notes add <hash> -m "spec-tasks/T-02-02-sparse-index
Day: 1
Date: 2025-10-31
Sessions: {NN} (Threads {START}-{END})
Duration: X.Xh
Worklog: .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-10-31-*
Status: ...
Files: N files (±NNN lines)"

# Push commits AND notes
git push origin feat/task/T-02-02-sparse-index
git push origin refs/notes/commits  # Don't forget!
```

---

## Implementation Phases Plan

- **Phase 0**: Planning & Analysis (Day 1) ✅ Session 01 complete
- **Phase 1**: Upper Index - Name/ID HashMap (Day 1)
- **Phase 2**: Custom Tokenizer (Day 1)
- **Phase 3**: BM25 Lower Index - Tantivy or custom (Day 1)
- **Phase 4**: Hierarchical Search Strategy (Day 2)
- **Phase 5**: Parity & Benchmarking (Day 2)

---

## Key Files

```tree
.artifacts/spec-tasks-T-02-02-sparse-index/
├── CLAUDE.md                          # This file
├── metadata.yaml                      # Central task metadata
├── git-refs.txt                       # Git workflow reference
└── worklogs/
    ├── 2025-10-31-work-summary.md     # Daily summary
    ├── 2025-10-31-commit-log.md       # Commit log
    ├── 2025-10-31-notes.md            # Technical notes
    └── raw/
        └── WORK-SESSIONS-01-THREADS-01-03-SUMMARY-2025-10-31.txt
```

---

## Critical Reminders

### ❌ Common Mistakes to Avoid

1. **Thread Numbering**: Reset to 01 for EACH session (don't continue across sessions!)
2. **RAW Log Timing**: Create AFTER session completes (not before!)
3. **Session = ONE RAW log**: Don't create separate files per thread
4. **Update actual_hours**: Sum all completed sessions (currently 1.2h)
5. **Git Notes**: Always add notes and push with `git push origin refs/notes/commits`

### ✅ Correct Pattern

```text
Session 01: Threads 01-03 ✅ COMPLETE
Session 02: Threads 01-XX (resets to 01!)
Session 03: Threads 01-XX (resets to 01!)
```

---

## Session Lifecycle Checklist

When completing a session:

- [ ] Review git log and identify thread boundaries
- [ ] Create RAW log file with all thread summaries
- [ ] Update metadata.yaml (hours, sessions, commits)
- [ ] Update daily worklogs (work-summary, commit-log, notes)
- [ ] Create checkpoint commit with git notes
- [ ] Push commits AND git notes
- [ ] Run verification: `./scripts/git-notes-check.sh`
- [ ] Run validation: `./scripts/checkpoint-helper.sh T-02-02-sparse-index`

---

## Technical Context

### Dependencies

- ✅ **T-02-01-graph-builder**: COMPLETED & MERGED (PR #6)
  - Provides: 4 node types, 4 edge types, <2% parity variance
  - Graph API: `crates/cds-index/src/graph/mod.rs`

### Test Fixtures

- Parity baselines: `tests/fixtures/parity/golden_outputs/`
  - 6 repos with graph data
  - 50 search queries for overlap@10 testing

### Performance Targets

- Search latency: <500ms p95
- Index build: <5s for 1K files
- Search overlap@10: ≥90% on 50 queries
- Unit test coverage: >95%

---

## Acceptance Criteria

- [ ] Upper index (name/ID HashMap) with prefix matching
- [ ] Lower index (BM25 k1=1.5, b=0.75)
- [ ] Search latency <500ms p95
- [ ] Index build <5s for 1K files
- [ ] Search overlap@10 ≥90% on 50 queries
- [ ] Unit test coverage >95%

---

## Resources

### Documentation

- **PRD**: `spacs/prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md`
- **Issue**: `spacs/issues/04-0.1.0-mvp/02-index-core/02-sparse-index.md`
- **Task**: `spacs/tasks/0.1.0-mvp/02-index-core/T-02-02-sparse-index.md`
- **TODO**: `spacs/tasks/0.1.0-mvp/TODO.yaml`

### Reference Code

- **LocAgent**: `tmp/LocAgent/dependency_graph/entity_searcher.py`
- **Graph API**: `crates/cds-index/src/graph/mod.rs`
- **Test fixtures**: `tests/fixtures/parity/`

### Automation Scripts

- `./scripts/checkpoint-helper.sh` - Pre-checkpoint validation
- `./scripts/git-notes-check.sh` - Git notes verification
- `./scripts/create-daily-worklog.sh` - Daily worklog creation

---

## Version History

- **v1.0** (2025-10-31): Initial guide created after Session 01

---

**Maintainer**: Cladue and Codex AI Engineer
**Status**: Active

---

END OF AI ENGINEER GUIDE
