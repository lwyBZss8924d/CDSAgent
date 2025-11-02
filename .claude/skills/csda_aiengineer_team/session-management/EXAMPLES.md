# Session Management - Real-World Examples

This document provides practical scenarios demonstrating session management in action.

## Table of Contents

- [Scenario 1: Starting First Session of a Task](#scenario-1-starting-first-session-of-a-task)
- [Scenario 2: Multiple Sessions Same Day](#scenario-2-multiple-sessions-same-day)
- [Scenario 3: Ending Session and Creating RAW Log](#scenario-3-ending-session-and-creating-raw-log)
- [Scenario 4: Complete Session with Checkpoint](#scenario-4-complete-session-with-checkpoint)
- [Scenario 5: Determining Next Session Number](#scenario-5-determining-next-session-number)
- [Scenario 6: Session Handoff Between AI Agents](#scenario-6-session-handoff-between-ai-agents)

---

## Scenario 1: Starting First Session of a Task

**Context**: Beginning work on T-02-02-sparse-index for the first time.

**Workflow**:

```shell
# 1. Navigate to task worktree
cd ~/dev-space/CDSAgent-T-02-02-sparse-index

# 2. Check if any sessions exist yet
ls .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/ 2>/dev/null

# Output: (empty or directory doesn't exist)
# Conclusion: This is Session 01

# 3. Create first session
./.dev/scripts/session/create-session-worklog.sh T-02-02 01 "Phase 0 Planning & Analysis" "Claude Code Agent"

# Output:
# ✅ Created .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-10-31-S01-work-summary.md
# ✅ Created .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-10-31-S01-notes.md
# ✅ Created .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-10-31-S01-commit-log.md
# Session 01 initialized. Ready to start work.
```

**Result**: Session 01 initialized with 3 worklog files, ready for Thread 01.

**Next Steps**:

```shell
# Start working on threads
# Thread 01: Worktree initialization
# Thread 02: Documentation updates
# Thread 03: Implementation planning

# ... execute threads ...

# After completing threads, create RAW log
./.dev/scripts/session/create-raw-log.sh T-02-02 01 01 03 "Planning complete"
```

---

## Scenario 2: Multiple Sessions Same Day

**Context**: Day 1 of T-02-02, three distinct sessions with natural breakpoints.

**Timeline**:

### Session 01 (07:17-08:30, 1.2h) - Planning

```shell
# Start Session 01
./.dev/scripts/session/create-session-worklog.sh T-02-02 01 "Phase 0 Planning & Analysis" "Claude Code Agent"

# Work threads 01-03
# - Thread 01: Worktree initialization
# - Thread 02: Documentation updates
# - Thread 03: Implementation planning

# End Session 01
./.dev/scripts/session/create-raw-log.sh T-02-02 01 01 03 "Planning complete"
```

**Break** (08:30-10:22, 1.9h): Context switch, meeting, or pause

### Session 02 (10:22-10:55, 0.55h) - Re-analysis

```shell
# Start Session 02 (next sequential number)
./.dev/scripts/session/create-session-worklog.sh T-02-02 02 "Re-analysis & Development Roadmap" "Claude Code Agent"

# Work threads 01-03
# - Thread 01: Spec alignment & gap analysis
# - Thread 02: Parity assets review
# - Thread 03: Implementation readiness checklist

# End Session 02
./.dev/scripts/session/create-raw-log.sh T-02-02 02 01 03 "Research baseline locked"
```

**Break** (10:55-12:02, 1.1h): Lunch or other activities

### Session 03 (12:02-15:17, 3.3h) - Implementation

```shell
# Start Session 03
./.dev/scripts/session/create-session-worklog.sh T-02-02 03 "Phase 1 Upper Index Implementation" "Claude Code Agent"

# Work threads 01-04
# - Thread 01: Phase 1 kickoff
# - Thread 02: NameIndex implementation
# - Thread 03: Validation & benchmarking
# - Thread 04: Coverage hardening

# End Session 03
./.dev/scripts/session/create-raw-log.sh T-02-02 03 01 04 "Phase 1 Upper Index delivered"

# Run checkpoint
./.dev/scripts/validation/checkpoint-helper.sh T-02-02
```

**Result**: Three sessions on same day (01, 02, 03), each with distinct purpose and RAW log.

**Files Created**:

```text
.artifacts/spec-tasks-T-02-02-sparse-index/worklogs/
├── 2025-10-31-S01-S02-work-summary.md    # Combined planning sessions
├── 2025-10-31-S01-S02-notes.md
├── 2025-10-31-S01-S02-commit-log.md
├── 2025-10-31-S03-work-summary.md        # Implementation session
├── 2025-10-31-S03-notes.md
├── 2025-10-31-S03-commit-log.md
└── raw/
    ├── WORK-SESSIONS-01-THREADS-01-03-SUMMARY-2025-10-31.txt
    ├── WORK-SESSIONS-02-THREADS-01-03-SUMMARY-2025-10-31.txt
    └── WORK-SESSIONS-03-THREADS-01-04-SUMMARY-2025-10-31.txt
```

---

## Scenario 3: Ending Session and Creating RAW Log

**Context**: Completed Session 04 with 7 threads, need to create RAW log for handoff.

**Workflow**:

```shell
# 1. Review threads completed
echo "Threads completed:
- Thread 01: Phase 2 planning
- Thread 02: Tokenizer parity analysis
- Thread 03: Tokenizer scaffolding
- Thread 04: Stop-word fixtures prep
- Thread 05: Tantivy analyzer integration
- Thread 06: BM25 index scaffold
- Thread 07: BM25 persistence planning"

# 2. Create RAW log with thread range
./.dev/scripts/session/create-raw-log.sh T-02-02 04 01 07 "Phase 2 Tokenizer + BM25 Scaffold delivered"

# Output:
# ✅ Created .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/raw/WORK-SESSIONS-04-THREADS-01-07-SUMMARY-2025-11-01.txt
# RAW log created. Ready for checkpoint.

# 3. Verify RAW log exists
ls -lh .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/raw/

# Output:
# -rw-r--r--  1 user  staff   45K Nov  1 12:45 WORK-SESSIONS-04-THREADS-01-07-SUMMARY-2025-11-01.txt
```

**RAW Log Content Structure**:

```text
WORK-SESSIONS-04-THREADS-01-07-SUMMARY-2025-11-01.txt
═══════════════════════════════════════════════════════

Session: 04
Task: T-02-02-sparse-index
Date: 2025-11-01
Duration: 01:39-12:45 UTC (3.2h)
Phase: Phase 2 - Custom Tokenizer + BM25 Scaffold
Status: COMPLETE

═══════════════════════════════════════════════════════

## THREAD 01 (01:39-02:04, 25min): Phase 2 Planning & Spec Alignment
... (complete narrative)

## THREAD 02 (02:05-02:13, 8min): Tokenizer Parity Analysis
... (complete narrative)

... (continues through Thread 07) ...

═══════════════════════════════════════════════════════

## SESSION SUMMARY

**Phase Status**: ✅ Phase 2 COMPLETE

**Deliverables**: ... (detailed list)

**Next Steps**: ... (Phase 3 planning)
```

**Result**: RAW log created, ready for checkpoint and AI handoff.

---

## Scenario 4: Complete Session with Checkpoint

**Context**: End-to-end workflow for Session 04 from start to checkpoint.

**Full Workflow**:

### Step 1: Start Session

```shell
# Navigate to worktree
cd ~/dev-space/CDSAgent-T-02-02-sparse-index

# Determine next session number (last was 03)
cat .artifacts/spec-tasks-T-02-02-sparse-index/metadata.yaml | grep -A 5 "sessions:" | grep "id:"
# Output: id: "03"
# Next: 04

# Create session worklogs
./.dev/scripts/session/create-session-worklog.sh T-02-02 04 "Phase 2 Custom Tokenizer + BM25 Scaffold" "Claude Code Agent"

# ✅ Session 04 initialized
```

### Step 2: Work Threads (01-07)

```shell
# Thread 01: Phase 2 planning (25 min)
# ... development work ...

# Thread 02: Tokenizer parity analysis (8 min)
# ... development work ...

# Thread 03: Tokenizer scaffolding (8 min)
# ... development work ...

# Thread 04: Stop-word fixtures (24 min)
# ... development work ...

# Thread 05: Tantivy analyzer integration (43 min)
# ... development work ...

# Thread 06: BM25 index scaffold (50 min)
git add crates/cds-index/src/index/bm25.rs
git commit -m "feat(index): T-02-02 Phase 2 - tokenizer + BM25 scaffold with parity strategy"
git notes add HEAD -m "spec-tasks/T-02-02: Session 04 Thread 06 - BM25 index scaffold"

# Thread 07: BM25 persistence planning (20 min)
# ... development work ...
```

### Step 3: Create RAW Log

```shell
# After completing all threads, create RAW log
./.dev/scripts/session/create-raw-log.sh T-02-02 04 01 07 "Phase 2 Tokenizer + BM25 Scaffold delivered"

# ✅ RAW log created
```

### Step 4: Update Session Worklogs

```shell
# Update work-summary.md with deliverables
# Update notes.md with technical details
# Update commit-log.md with git commits

# Example work-summary.md excerpt:
# ## Deliverables
# - ✅ tokenizer.rs (387 lines) - Custom tokenizer with offset preservation
# - ✅ bm25.rs (+442 lines) - Tantivy backend scaffold
# - ✅ stop_words.rs (180 lines) - NLTK stop-word integration
# - ✅ 12 new tests (7 tokenizer, 2 BM25, 3 fixture)
```

### Step 5: Update metadata.yaml

```shell
# Add Session 04 entry to metadata.yaml sessions array
# Update cumulative metrics (actual_hours, lines_added, etc.)

# Excerpt from metadata.yaml:
# sessions:
#   - id: "04"
#     date: "2025-11-01"
#     phase: "Phase 2"
#     threads:
#       count: 7
#       range: "01-07"
#     duration:
#       hours: 3.2
#     status: completed
```

### Step 6: Run Checkpoint

```shell
# Run checkpoint helper
./.dev/scripts/validation/checkpoint-helper.sh T-02-02

# Interactive prompts:
# Phase 1: Review session work ✅
# Phase 2: Verify artifacts ✅
# Phase 3: Update metadata ✅
# Phase 4: Git operations ✅
# Phase 5: Final validation ✅

# Checkpoint commit created:
# checkpoint(worklog): T-02-02 Day 2 Session 04 complete - Phase 2 Tokenizer + BM25 delivered

# Git note added:
# spec-tasks/T-02-02: Session 04 Checkpoint - Phase 2 delivered
```

### Step 7: Push to Remote

```shell
# Push commits and notes
git push origin feat/task/T-02-02-sparse-index
git push origin refs/notes/commits

# ✅ Session 04 complete and synced
```

**Result**: Session 04 fully documented, checkpointed, and pushed to remote.

---

## Scenario 5: Determining Next Session Number

**Context**: Multiple ways to find the correct next session number.

### Method 1: Check metadata.yaml (Recommended)

```shell
# View sessions array
cat .artifacts/spec-tasks-T-02-02-sparse-index/metadata.yaml | grep -A 50 "sessions:"

# Output:
# sessions:
#   - id: "01"
#     date: "2025-10-31"
#     ...
#   - id: "02"
#     date: "2025-10-31"
#     ...
#   - id: "03"
#     date: "2025-10-31"
#     ...
#   - id: "04"
#     date: "2025-11-01"
#     ...
#
# Last session: 04
# Next session: 05

# One-liner to get last session ID
cat .artifacts/spec-tasks-T-02-02-sparse-index/metadata.yaml | grep -A 50 "sessions:" | grep "id:" | tail -1

# Output: - id: "04"
# Next: 05
```

### Method 2: List Worklog Files

```shell
# List all session worklogs sorted
ls .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/ | grep "^2025" | grep "S0[0-9]" | sort

# Output:
# 2025-10-31-S01-S02-work-summary.md
# 2025-10-31-S01-S02-notes.md
# 2025-10-31-S01-S02-commit-log.md
# 2025-10-31-S03-work-summary.md
# 2025-10-31-S03-notes.md
# 2025-10-31-S03-commit-log.md
# 2025-11-01-S04-work-summary.md
# 2025-11-01-S04-notes.md
# 2025-11-01-S04-commit-log.md

# Last file: ...S04-...
# Next session: 05
```

### Method 3: Count RAW Logs

```shell
# List RAW logs
ls .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/raw/

# Output:
# WORK-SESSIONS-01-THREADS-01-03-SUMMARY-2025-10-31.txt
# WORK-SESSIONS-02-THREADS-01-03-SUMMARY-2025-10-31.txt
# WORK-SESSIONS-03-THREADS-01-04-SUMMARY-2025-10-31.txt
# WORK-SESSIONS-04-THREADS-01-07-SUMMARY-2025-11-01.txt

# Count: 4 RAW logs
# Next session: 05

# One-liner
ls .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/raw/ | wc -l

# Output: 4
# Next: 05
```

### Method 4: Query Git Log

```shell
# Search commit messages for session checkpoints
git log --all --grep="checkpoint(worklog)" --oneline

# Output:
# 9e1774e checkpoint(worklog): T-02-02 Day 2 Session 04 complete
# 7b624c4 checkpoint(worklog): T-02-02 Day 1 Session 03 complete
# ...

# Last checkpoint: Session 04
# Next session: 05
```

**Result**: Multiple methods confirm next session is 05.

---

## Scenario 6: Session Handoff Between AI Agents

**Context**: Claude Code Agent completes Session 04, hands off to human developer for Session 05.

### Agent 1 (Claude Code): Session 04 Completion

```shell
# 1. Complete Session 04 work
# ... (threads 01-07 executed) ...

# 2. Create comprehensive RAW log
./.dev/scripts/session/create-raw-log.sh T-02-02 04 01 07 "Phase 2 complete - handoff to human for Phase 3"

# 3. Update work-summary.md with handoff notes
echo "## Handoff Notes

**Status**: Phase 2 COMPLETE ✅

**Completed**:
- Custom tokenizer (387 lines) with offset preservation matching LocAgent
- BM25 Tantivy backend scaffold (442 lines)
- Stop-word integration (NLTK parity)
- 12 new tests (78/78 passing, 100% pass rate)

**Next Steps for Session 05**:
1. Phase 3: Integrate BM25 with hierarchical search strategy
2. Implement upper→lower tier fallback logic
3. Add parity tests (target: overlap@10 ≥90%)

**Known Issues**:
- BM25 persistence not yet implemented (planned for Thread 07 next session)
- Performance benchmarks pending (Phase 5)

**Context Preserved**:
- RAW log contains complete narrative (7 threads, 3.2h)
- All design decisions documented in notes.md
- Code review notes in codereview.md

**Environment**:
- Branch: feat/task/T-02-02-sparse-index
- Last commit: 414f7f2
- Tests passing: 78/78
- Coverage: ~95%

" >> .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-11-01-S04-work-summary.md

# 4. Run checkpoint
./.dev/scripts/validation/checkpoint-helper.sh T-02-02

# 5. Push everything
git push origin feat/task/T-02-02-sparse-index
git push origin refs/notes/commits

# ✅ Handoff complete - Session 04 artifacts available for next agent
```

### Agent 2 (Human Developer): Session 05 Initialization

```shell
# 1. Pull latest changes
cd ~/dev-space/CDSAgent-T-02-02-sparse-index
git pull origin feat/task/T-02-02-sparse-index
git fetch origin refs/notes/commits:refs/notes/commits

# 2. Read handoff artifacts
cat .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-11-01-S04-work-summary.md
# ... (review handoff notes) ...

# 3. Read RAW log for complete context
cat .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/raw/WORK-SESSIONS-04-THREADS-01-07-SUMMARY-2025-11-01.txt
# ... (understand Session 04 decisions and implementations) ...

# 4. Verify session number (last was 04)
cat .artifacts/spec-tasks-T-02-02-sparse-index/metadata.yaml | grep -A 5 "sessions:" | grep "id:"
# Output: Last session: "04"
# Next: 05

# 5. Initialize Session 05
./.dev/scripts/session/create-session-worklog.sh T-02-02 05 "Phase 3 BM25 Integration & Hierarchical Search" "Human Developer"

# ✅ Session 05 initialized, ready to continue from Session 04
```

### Handoff Verification Checklist

```shell
# Verify all Session 04 artifacts exist
echo "Checking Session 04 artifacts..."

# ✅ Worklogs
[ -f .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-11-01-S04-work-summary.md ] && echo "✅ work-summary.md"
[ -f .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-11-01-S04-notes.md ] && echo "✅ notes.md"
[ -f .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-11-01-S04-commit-log.md ] && echo "✅ commit-log.md"

# ✅ RAW log
[ -f .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/raw/WORK-SESSIONS-04-THREADS-01-07-SUMMARY-2025-11-01.txt ] && echo "✅ RAW log"

# ✅ Metadata
grep -q "id: \"04\"" .artifacts/spec-tasks-T-02-02-sparse-index/metadata.yaml && echo "✅ metadata.yaml updated"

# ✅ Git commits
git log --oneline -1 | grep -q "414f7f2" && echo "✅ Last commit present"

# ✅ Git notes
git notes show 414f7f2 | grep -q "spec-tasks/T-02-02" && echo "✅ Git notes present"

# ✅ Tests
cargo test --package cds-index 2>&1 | grep -q "78 passed" && echo "✅ Tests passing"

echo "Handoff verification complete ✅"
```

**Result**: Seamless handoff between AI agent and human developer with complete context preservation.

---

## Summary

These examples demonstrate:

1. **Starting First Session**: Initialize Session 01 for new task
2. **Multiple Sessions Same Day**: Natural breakpoints enable multiple sessions per day
3. **Ending Session with RAW Log**: Document complete narrative for handoff
4. **Complete Session with Checkpoint**: End-to-end workflow from start to push
5. **Determining Next Session Number**: Multiple methods to find correct session ID
6. **AI Agent Handoff**: Preserve complete context for next agent/developer

**Key Takeaways**:

- ✅ **Session numbering is sequential** - never resets per day
- ✅ **RAW logs enable handoff** - complete narrative for AI agents
- ✅ **Checkpoint before push** - validate artifacts before syncing
- ✅ **Check metadata.yaml** - authoritative source for last session number
- ✅ **Multiple sessions per day is normal** - natural breakpoints improve clarity

**Important⚠️**: Any [session-management] job changed any PMP and docs & metadata .yaml MUST run (date -u '+%Y-%m-%dT%H:%M:%SZ') get UTC TIME NOW first!!!
