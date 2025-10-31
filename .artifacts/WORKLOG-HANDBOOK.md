# Worklog & Checkpoint Management Handbook

**Version**: 1.0
**Date**: 2025-10-31
**Task**: T-02-02-sparse-index
**Purpose**: Guide for maintaining accurate work records and avoiding common mistakes

---

## Table of Contents

1. [Hierarchical Structure](#hierarchical-structure)
2. [Work Session Lifecycle](#work-session-lifecycle)
3. [RAW Log Management](#raw-log-management)
4. [Metadata Updates](#metadata-updates)
5. [Checkpoint Workflow](#checkpoint-workflow)
6. [Common Mistakes](#common-mistakes)
7. [Quick Reference](#quick-reference)

---

## Hierarchical Structure

### Correct Understanding

```text
Task (T-02-02-sparse-index)
└── Implementation Phase (technical stage, used as Checkpoint boundary)
    └── Day (recorded by date, YYYY-MM-DD)
        └── Session (work period, 1-8h, corresponds to ONE RAW log file)
            └── Thread (continuous work unit, 30min-2h, numbered sequentially WITHIN session)
```

### Key Definitions

| Term | Definition | Example |
|------|------------|---------|
| **Task** | Top-level work item | T-02-02-sparse-index |
| **Phase** | Technical implementation stage | Phase 1: Upper Index, Phase 2: Tokenizer |
| **Day** | Calendar date | 2025-10-31 |
| **Session** | Work period with ONE RAW log | Session 01 (07:17-08:30 UTC, 1.2h) |
| **Thread** | Work unit within session | Thread 01, 02, 03 (numbers reset per session) |

### CRITICAL: Thread Numbering

**❌ WRONG**: Thread numbers span multiple sessions

```text
Session 01: Threads 01-03
Session 02: Threads 04-06  # ❌ WRONG! Don't continue numbering!
```

**✅ CORRECT**: Thread numbers reset for each session

```text
Session 01: Threads 01-03
Session 02: Threads 01-03  # ✅ CORRECT! Reset to 01!
```

---

## Work Session Lifecycle

### Phase 1: Session Starts

**When**: Beginning of focused work period

**Actions**:

1. Start working (coding, researching, planning)
2. Make commits as you work
3. **DO NOT** create RAW log yet (will create when session completes)

**Artifacts**: None yet (working in progress)

---

### Phase 2: During Session

**When**: Active work in progress

**Actions**:

1. Continue working through multiple threads
2. Make commits with conventional commit messages
3. Mentally track which thread you're in (based on context switches)

**Thread Transitions**:

- New thread when: switching tasks, taking break, changing focus
- Thread duration: 30min - 2h typically

**Artifacts**: Git commits only

---

### Phase 3: Session Completes

**When**: End of work period (EOD, before break, phase complete)

**Actions**:

1. Review all commits made during session
2. Identify thread boundaries (by time and context)
3. **CREATE RAW log file** with all thread summaries
4. Run Checkpoint Workflow (Phase 4 git operations)

**Artifacts Created**:

- `WORK-SESSIONS-{NN}-THREADS-{START}-{END}-SUMMARY-{YYYY-MM-DD}.txt`
- Updated `metadata.yaml`
- Updated `YYYY-MM-DD-work-summary.md`
- Updated `YYYY-MM-DD-commit-log.md`
- Updated `YYYY-MM-DD-notes.md`
- Git notes on all commits

---

## RAW Log Management

### Naming Convention

**Format**: `WORK-SESSIONS-{NN}-THREADS-{START}-{END}-SUMMARY-{YYYY-MM-DD}.txt`

**Example**: `WORK-SESSIONS-01-THREADS-01-03-SUMMARY-2025-10-31.txt`

Where:

- `{NN}`: Session number (01, 02, 03, ...)
- `{START}`: First thread number in this session
- `{END}`: Last thread number in this session
- `{YYYY-MM-DD}`: Date of session

### When to Create

**✅ Create RAW log**: AFTER session completes (not before!)

**Timing**:

1. Finish all work for the session
2. Commit all code changes
3. Review git history
4. Create RAW log with complete thread summaries
5. Run checkpoint workflow

**❌ DO NOT**:

- Create RAW log before session starts (no work done yet!)
- Create RAW log during active work (incomplete information)
- Create separate RAW logs for each thread (ONE per session!)

### Content Structure

```markdown
================================================================================
WORK SESSION {NN} - THREADS {START}-{END} SUMMARY
================================================================================

Task: T-02-02-sparse-index - Sparse Index - Name/ID + BM25 Search
Date: YYYY-MM-DD
Session: {NN} (Day {X} - Session description)
Threads: {START}-{END}
Duration: HH:MM-HH:MM UTC (X.Xh total)
Status: ✅ SESSION COMPLETE

Session Overview:
- Thread 01: Brief description
- Thread 02: Brief description
- Thread 03: Brief description

================================================================================
THREAD 01: DETAILED TITLE
================================================================================
Time: HH:MM-HH:MM UTC (X.Xh)

Objective:
- What this thread aimed to accomplish

Actions Completed:
✅ Action 1
✅ Action 2
✅ Action 3

Work Completed: (if applicable)
1. Section 1 (+N lines):
   - Detail 1
   - Detail 2

2. Section 2 (+N lines):
   - Detail 1
   - Detail 2

Key Decisions:
- Decision 1 with rationale
- Decision 2 with rationale

Files Modified: (if applicable)
Total: N files, +NNN insertions, -NN deletions

Git Operations:
- Commit hash1: commit message
- Commit hash2: commit message
- Git notes added to commits
- Pushed to remote with notes

Checkpoint Workflow: (if checkpoint performed)
✅ Phase 4.1: Code commit created
✅ Phase 4.2: Git notes added
✅ Phase 4.3: metadata.yaml updated
✅ Phase 4.4: Artifacts staged
✅ Phase 4.5: Checkpoint commit created
✅ Phase 4.6: Pushed commits + notes

--------------------------------------------------------------------------------

[Repeat for Thread 02, 03, etc.]

================================================================================
SESSION {NN} SUMMARY
================================================================================

Total Duration: X.X hours (HH:MM-HH:MM UTC)
Threads Completed: N
Commits Created: N

Commits:
1. hash1 - message1
2. hash2 - message2

Git Notes: ✅ All commits have git notes
Remote: ✅ Synchronized

Artifacts Updated:
- metadata.yaml: Task metadata with commit tracking
- git-refs.txt: Git workflow reference
- YYYY-MM-DD-work-summary.md: Daily progress summary
- YYYY-MM-DD-commit-log.md: Git commit details
- YYYY-MM-DD-notes.md: Technical notes and plans

Metrics Snapshot:
- Actual hours: X.X (Session {NN} complete)
- Commits: N
- Lines added: NNN
- Lines deleted: NN
- Files modified: N
- Tests added: N

================================================================================
END OF WORK SESSION {NN}
================================================================================
```

---

## Metadata Updates

### File: `metadata.yaml`

**Update Frequency**: After each session completes (during checkpoint)

**Key Sections to Update**:

#### 1. Task Hours

```yaml
task:
  actual_hours: 1.2  # Update after each session
```

**Calculation**: Sum of all completed sessions

- Session 01: 1.2h → `actual_hours: 1.2`
- Session 02: 2.5h → `actual_hours: 3.7` (1.2 + 2.5)
- Session 03: 1.0h → `actual_hours: 4.7` (3.7 + 1.0)

#### 2. Git Commits

```yaml
git:
  commits:
    - hash: "4f834f6"
      message: "docs(milestone): update documentation"
      date: "2025-10-31"
      files_changed: 5
      notes: "Session details"
```

**When to Add**:

- Add commit entry for each code commit (not checkpoint commits)
- Include git notes content
- Track files changed

#### 3. Work Sessions

```yaml
notes: |
  Implementation Phases (technical stages):
  - Phase 1: Upper Index - Name/ID HashMap (Days 1-2)
  - Phase 2: Custom Tokenizer (Day 3)
  ...

  Work Sessions (by day):
  - Day 1 Session 01: [Phase 0] Planning & Analysis (Threads 01-03, 1.2h) ✅ COMPLETE
    * Thread 01: Worktree initialization (0.05h)
    * Thread 02: Documentation updates for M2 milestone (0.15h)
    * Thread 03: Comprehensive tasks initial analysis, planning & implementation roadmap (0.75h, 986 lines)
    * RAW: WORK-SESSIONS-01-THREADS-01-03-SUMMARY-2025-10-31.txt
    * Commits: 4f834f6, 8abc915, d281dcc, d36ea10, 37cf911, 628724b
    * Duration: 07:17-08:30 UTC (1.2h total)
    * Status: ✅ COMPLETE - All planning and setup finished

  - Day 1 Session 02: [Phase 0] re-analysis & planning for development
    * Phase 0: Reading and re-Analysis & re-Planning DEEPRESEARCH PRDs and Issues and Tasks for development
    * Thread ...
    * RAW: WORK-SESSIONS-02-THREADS-01-XX-SUMMARY-2025-10-31.txt (will be created when Thread 01 begins)
    * Status: NOT STARTED

  - Day 1 Session 03: [Phase 1] Upper Index Implementation (Threads 01-XX)
    * Phase--Session: Executing Phase 1 (Upper Index) implementation
    * Thread 01: NameIndex design & exact match
    * Thread 02: Prefix match & type filtering
    * Thread 03: Graph integration & benchmarks
    * Expected: ~4-6 hours, name_index.rs + tests
    * RAW: WORK-SESSIONS-03-THREADS-01-XX-SUMMARY-2025-10-31.txt (will be created when Thread 01 begins)
    * Status: NOT STARTED
```

**Structure**:

- List sessions chronologically
- Mark completed sessions with ✅ COMPLETE
- Include thread breakdown
- Link to RAW log file
- List all commits
- Show duration and status

---

## Checkpoint Workflow

### When to Run

**Required**:

- ✅ End of each session (after RAW log created)
- ✅ Before major push to remote
- ✅ After significant milestone (phase complete)

**Optional**:

- Mid-session for long sessions (>4h)
- Before long break

### Workflow Steps

Based on `docs/WORK_SESSION_CHECKPOINT_WORKFLOW.md`:

#### Phase 1: Review & Data Collection

1. Review raw action logs (git log, git diff)
2. Collect session metrics (commits, files, lines)
3. Identify all threads in session

#### Phase 2: Consistency Verification

1. Create consistency matrix (git vs. documented)
2. Verify commit hashes match
3. Check files changed match
4. Validate git notes exist

#### Phase 3: Update Artifacts

1. Update `metadata.yaml` (hours, commits, sessions)
2. Update `YYYY-MM-DD-work-summary.md`
3. Update `YYYY-MM-DD-commit-log.md`
4. Update `YYYY-MM-DD-notes.md`
5. Create RAW log file

#### Phase 4: Git Operations

1. Stage artifact files
2. Create checkpoint commit
3. Add git notes to checkpoint commit
4. Push commits to remote
5. **Push git notes** to remote (`git push origin refs/notes/commits`)

#### Phase 5: Final Verification

1. Run `./scripts/git-notes-check.sh`
2. Run `./scripts/checkpoint-helper.sh`
3. Verify 100% consistency

---

## Common Mistakes

### Mistake 1: Thread Numbering Across Sessions

**❌ WRONG**:

```yaml
Session 01: Threads 01-03
Session 02: Threads 04-06  # ❌ Continues numbering
```

**✅ CORRECT**:

```yaml
Session 01: Threads 01-03
Session 02: Threads 01-03  # ✅ Resets to 01
```

**Why**: Threads are session-scoped, not task-scoped

---

### Mistake 2: Creating RAW Log Before Session Starts

**❌ WRONG**:

```bash
# Before starting work
touch WORK-SESSIONS-02-THREADS-01-XX-SUMMARY-2025-10-31.txt  # ❌ Too early!
# Start coding...
```

**✅ CORRECT**:

```bash
# Do all the work first
# Make commits
# Session completes
# THEN create RAW log with complete info
cat > WORK-SESSIONS-02-THREADS-01-03-SUMMARY-2025-10-31.txt <<EOF  # ✅ After work!
...
EOF
```

**Why**: RAW log documents completed work, not planned work

---

### Mistake 3: Separate RAW Logs Per Thread

**❌ WRONG**:

```text
WORK-SESSIONS-02-THREAD-01-SUMMARY-2025-10-31.txt  # ❌ One per thread
WORK-SESSIONS-02-THREAD-02-SUMMARY-2025-10-31.txt
WORK-SESSIONS-02-THREAD-03-SUMMARY-2025-10-31.txt
```

**✅ CORRECT**:

```text
WORK-SESSIONS-02-THREADS-01-03-SUMMARY-2025-10-31.txt  # ✅ One per session
```

**Why**: Session is the checkpoint boundary, ONE RAW log per session

---

### Mistake 4: Forgetting to Update actual_hours

**❌ WRONG**:

```yaml
# After Session 01 (1.2h) and Session 02 (2.5h)
actual_hours: 1.2  # ❌ Forgot to add Session 02!
```

**✅ CORRECT**:

```yaml
# After Session 01 (1.2h) and Session 02 (2.5h)
actual_hours: 3.7  # ✅ 1.2 + 2.5 = 3.7
```

**Fix**: Always sum all completed sessions

---

### Mistake 5: Missing Git Notes

**❌ WRONG**:

```shell
git commit -m "feat(index): implement NameIndex"
git push  # ❌ No git notes added!
```

**✅ CORRECT**:

```shell
git commit -m "feat(index): implement NameIndex"
git notes add <hash> -m "..."  # ✅ Add git notes
git push origin <branch>
git push origin refs/notes/commits  # ✅ Push notes too!
```

**Verify**: Run `./scripts/git-notes-check.sh`

---

### Mistake 6: Splitting One Session Into Multiple

**❌ WRONG** (User's original understanding):

```yaml
Session 01: Threads 01-02 (init + docs)  # ❌ Split incorrectly
Session 02: Thread 01 (planning)         # ❌ Should be Session 01 Thread 03
```

**✅ CORRECT**:

```yaml
Session 01: Threads 01-03 (init + docs + planning)  # ✅ All in one session
  * Thread 01: init (0.05h)
  * Thread 02: docs (0.15h)
  * Thread 03: planning (0.75h)
  * Total: 1.2h (one continuous work period)
```

**Rule**: Session = continuous work period with no major break. If work continues without significant pause, it's same session.

---

## Quick Reference

### Session Lifecycle Checklist

- [ ] **Start session**: Begin work, make commits
- [ ] **During session**: Work through threads, track context switches
- [ ] **End session**: Review git log, identify threads
- [ ] **Create RAW log**: Write WORK-SESSIONS-{NN}-THREADS-{START}-{END}-SUMMARY-{DATE}.txt
- [ ] **Update metadata.yaml**: Add session entry, update hours, add commits
- [ ] **Update daily worklogs**: work-summary.md, commit-log.md, notes.md
- [ ] **Create checkpoint commit**: Stage artifacts, commit, add git notes
- [ ] **Push to remote**: `git push origin <branch>` + `git push origin refs/notes/commits`
- [ ] **Verify**: Run checkpoint-helper.sh and git-notes-check.sh

### RAW Log Naming Template

```text
WORK-SESSIONS-{NN}-THREADS-{START}-{END}-SUMMARY-{YYYY-MM-DD}.txt

Examples:
WORK-SESSIONS-01-THREADS-01-03-SUMMARY-2025-10-31.txt
WORK-SESSIONS-02-THREADS-01-05-SUMMARY-2025-10-31.txt
WORK-SESSIONS-03-THREADS-01-02-SUMMARY-2025-11-01.txt (next day)
```

### Metadata Session Entry Template

```yaml
- Day {X} Session {NN}: [Phase {N}] Description (Threads 01-XX, Xh) STATUS
  * Phase--Session: Phase description
  * Thread 01: Work description (Xh)
  * Thread 02: Work description (Xh)
  * Thread 03: Work description (Xh)
  * RAW: WORK-SESSIONS-{NN}-THREADS-01-XX-SUMMARY-{DATE}.txt
  * Commits: hash1, hash2, hash3
  * Duration: HH:MM-HH:MM UTC (Xh total)
  * Status: ✅ COMPLETE / NOT STARTED
```

### Git Notes Template

```text
spec-tasks/T-02-02-sparse-index
Day: 1
Date: 2025-10-31
Sessions: 01 (Threads 01-03)
Duration: 1.2h
Worklog: .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-10-31-*
Status: Session summary or commit description
Files: N files (±NNN lines)
```

---

## Files Relationships

```tree
.artifacts/
├── WORKLOG-HANDBOOK.md                     # This handbook (shared across all tasks)
└── spec-tasks-T-02-02-sparse-index/
    ├── CLAUDE.md                           # AI assistant quick reference for this task
    ├── metadata.yaml                       # Central task metadata
    ├── git-refs.txt                        # Git workflow reference
    └── worklogs/
        ├── YYYY-MM-DD-work-summary.md      # Daily summary (for PR review)
        ├── YYYY-MM-DD-commit-log.md        # Commit details (for PR review)
        ├── YYYY-MM-DD-notes.md             # Technical notes (for PR review)
        └── raw/
            ├── WORK-SESSIONS-01-THREADS-01-03-SUMMARY-2025-10-31.txt  # Session 01 RAW
            ├── WORK-SESSIONS-02-THREADS-01-XX-SUMMARY-2025-10-31.txt  # Session 02 RAW
            └── WORK-SESSIONS-03-THREADS-01-XX-SUMMARY-2025-11-01.txt  # Session 03 RAW
```

**Purpose of Each**:

- `WORKLOG-HANDBOOK.md`: Shared workflow guide for all tasks (read first!)
- `CLAUDE.md`: Task-specific quick reference for AI assistants
- `metadata.yaml`: Source of truth for task progress
- `YYYY-MM-DD-*.md`: PR documentation (daily view for reviewers)
- `raw/*.txt`: Session context for AI handoff (detailed view for continuity)

---

## Version History

- **v1.0** (2025-10-31): Initial handbook created after Session 01 mistakes
  - Clarified thread numbering (reset per session)
  - Documented RAW log creation timing
  - Listed common mistakes with fixes
  - Provided templates and checklists

---

**Maintainer**: Claude Code Agent + Rust Dev 2
**Status**: Active - Update as workflow evolves

---

END OF HANDBOOK
