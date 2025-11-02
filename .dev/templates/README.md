# CDSAgent Development Templates

**Location**: `.dev/templates/`
**Purpose**: Templates for task initialization and session-based development tracking
**Version**: 2.0 (Session-based workflow)
**Date**: 2025-11-02 UTC

---

## Table of Contents

1. [Overview](#overview)
2. [Template Files](#template-files)
3. [Session vs. Day Terminology](#session-vs-day-terminology)
4. [Usage](#usage)
5. [Template Placeholders](#template-placeholders)
6. [File Naming Conventions](#file-naming-conventions)
7. [Automation Scripts](#automation-scripts)
8. [Example Structures](#example-structures)
9. [Migration from Daily to Session-Based](#migration-from-daily-to-session-based)

---

## Overview

This directory contains templates for **session-based development tracking**. Key concepts:

- **Task**: High-level work unit (e.g., T-02-02-sparse-index)
- **Phase**: Implementation stage (Phase 1, Phase 2, etc.)
- **Day**: Calendar date (YYYY-MM-DD)
- **Session**: Focused work period (1-8h) - can be multiple per day
- **Thread**: Work unit within session (30min-2h) - numbered 01, 02, 03, etc.

### Why Session-Based?

Development doesn't always fit into "daily" boundaries. For example, T-02-02 had:

- **Day 1 (2025-10-31)**: Sessions 01, 02, 03 (three distinct work periods)
- **Day 2 (2025-11-01)**: Session 04 (one work period)

Session-based tracking provides:

- ✅ Better granularity for multi-session days
- ✅ Thread numbering resets per session (cleaner organization)
- ✅ RAW logs capture complete session context
- ✅ Easier to resume work after breaks

---

## Template Files

### `.dev/templates/metadata.template.yaml`

**Purpose**: Task-level metadata with structured session tracking

**Used for**: Task initialization (one-time per task)

**Key sections**:

- `task`: ID, title, status, priority, milestone
- `specs`: PRD/Issue/Task file references
- `git`: Worktree, branch, commits, PR
- `deliverables`: Expected outputs
- `acceptance_criteria`: Status tracking
- `sessions`: **NEW** - Structured session tracking (replaces free-form notes)
- `metrics`: Code statistics
- `comments`: Change log

**See**: Enhanced with `sessions:` field (v2.0)

---

### `.dev/templates/worklogs/` (5 templates)

#### 1. `work-summary.template.md`

**Purpose**: Session objectives, completed work, decisions, next steps

**Used for**: Every session (session-specific file: `{date}-S{NN}-work-summary.md`)

**Key sections**:

- Session Objectives (checkbox list)
- Work Completed (by thread)
- Code Changes
- Key Decisions Made
- Challenges & Solutions
- Next Steps
- Acceptance Criteria Progress
- Session Statistics

#### 2. `commit-log.template.md`

**Purpose**: Git commit tracking with context

**Used for**: Every session (session-specific file: `{date}-S{NN}-commit-log.md`)

**Key sections**:

- Commits Made Today
- Commit details (hash, message, files changed, diff summary)
- Git Commands Used
- Branch Status
- References (issue, task spec, PRs)

#### 3. `notes.template.md`

**Purpose**: Technical notes, research, decisions, learnings

**Used for**: Every session (session-specific file: `{date}-S{NN}-notes.md`)

**Key sections**:

- Technical Notes (architecture decisions, implementation details)
- Research & Learning
- Code Review Notes
- Testing Notes
- Performance Notes
- Integration Notes
- TODO / Follow-up
- References

#### 4. `codereview.template.md` ✨ NEW

**Purpose**: Testing results, linting fixes, quality metrics

**Used for**: Sessions with testing/code review phase (optional)

**Session-specific file**: `{date}-S{NN}-codereview.md`

**Key sections**:

- Testing Phase Results (total tests, pass rate)
- Linting & Code Quality (clippy errors, warnings)
- Detailed Fix Diffs (error messages, diffs, rationale)
- Quality Metrics (coverage, pass rate, performance)
- Summary & Next Steps

**When to create**: After testing phase or when fixing linting errors

#### 5. `raw-session.template.txt` ✨ NEW

**Purpose**: Complete session narrative for AI handoff

**Used for**: After each session completes (not before!)

**File location**: `worklogs/raw/WORK-SESSIONS-{NN}-THREADS-{START}-{END}-SUMMARY-{DATE}.txt`

**Key sections**:

- Session header (task, date, session, threads, duration, status)
- Session overview (thread list with objectives)
- Thread-by-thread detailed narrative (objectives, actions, decisions, code changes)
- Session summary (commits, metrics, artifacts updated)

**Timing**: Created **AFTER session completes**, not before or during

---

## Session vs. Day Terminology

**Glossary**:

- **Day**: Calendar date (YYYY-MM-DD)
  - Example: `2025-11-02`
  - Can have multiple sessions

- **Session**: Focused work period (1-8h)
  - Example: Session 05 (05 = session number, padded to 2 digits)
  - Numbered sequentially across all days: 01, 02, 03, 04, 05, ...
  - Multiple sessions can occur on same day

- **Thread**: Work unit within session (30min-2h)
  - Example: Thread 01, 02, 03 within Session 05
  - Numbering **resets to 01** for each new session
  - Session 04 has Threads 01-07, Session 05 has Threads 01-03 (resets)

- **Phase**: Implementation stage
  - Example: Phase 2 (Implementation)
  - Orthogonal to sessions (multiple sessions can be in same phase)

**Example Timeline**:

```text
Task: T-02-02-sparse-index
  Phase 1: Planning
    Day 2025-10-31:
      Session 01: Threads 01-03 (1.2h) ← Planning
      Session 02: Threads 01-03 (1.5h) ← Design
      Session 03: Threads 01-04 (1.2h) ← Prep
  Phase 2: Implementation
    Day 2025-11-01:
      Session 04: Threads 01-07 (3.2h) ← Tokenizer + BM25
    Day 2025-11-02:
      Session 05: Threads 01-03 (2.0h) ← Testing (planned)
```

---

## Usage

### Task Initialization (One-Time)

**When**: Starting a brand new task for the first time

**Script**: `.dev/scripts/task/create-task-worklog.sh`

```shell
# From main repo
cd ~/dev-space/CDSAgent

# Run task initialization script
./.dev/scripts/task/create-task-worklog.sh \
  T-XX-XX-task-name "Task Title from Spec" "Developer Name"

# Creates:
# - .artifacts/spec-tasks-T-XX-XX-task-name/
# - .artifacts/spec-tasks-T-XX-XX-task-name/metadata.yaml (from template)
# - .artifacts/spec-tasks-T-XX-XX-task-name/worklogs/ (empty directory)
# - .artifacts/spec-tasks-T-XX-XX-task-name/git-refs.txt
```

**Result**: Task structure initialized, ready for first session

---

### Session Initialization (Every Session)

**When**: Beginning of each work session (multiple times per task)

**Script**: `.dev/scripts/session/create-session-worklog.sh` ✨ NEW

```shell
# From task worktree
cd ~/dev-space/CDSAgent-T-XX-XX-task-name

# Determine next session number
ls .artifacts/spec-tasks-T-XX-XX/worklogs/ | grep -E "S[0-9]{2}" | sort | tail -1
# Example output: 2025-11-01-S04-work-summary.md
# Next session: 05

# Run session initialization (RECOMMENDED - ~2 minutes)
./.dev/scripts/session/create-session-worklog.sh \
  T-XX-XX-task-name 05 "Phase 2 Testing" "Developer Name"

# Creates 4 files:
# - {date}-S05-work-summary.md
# - {date}-S05-commit-log.md
# - {date}-S05-notes.md
# - {date}-S05-codereview.md

# All placeholders auto-replaced:
# - {DATE} → 2025-11-02
# - {SESSION} → 05
# - {TASK_ID} → T-XX-XX-task-name
# - {TASK_TITLE} → (read from metadata.yaml)
# - {DEVELOPER_NAME} → Developer Name
# - {BRANCH_NAME} → (auto-detected from git)
```

**Time**: ~2 minutes (automated) vs. ~10 minutes (manual)

**See**: [SESSION_INITIALIZATION_WORKFLOW.md](../workflows/SESSION_INITIALIZATION_WORKFLOW.md) for detailed guide

---

### RAW Log Creation (After Session)

**When**: After session completes (not before!)

**Script**: `.dev/scripts/session/create-raw-log.sh` ✨ NEW

```shell
# From task worktree
cd ~/dev-space/CDSAgent-T-XX-XX-task-name

# Run after session completes
./.dev/scripts/session/create-raw-log.sh \
  T-XX-XX-task-name 05 01 03 "Phase 2 Testing"
# Args: TASK_ID SESSION_NUM THREAD_START THREAD_END DESCRIPTION

# Creates:
# worklogs/raw/WORK-SESSIONS-05-THREADS-01-03-SUMMARY-2025-11-02.txt

# Then: Fill out session narrative in RAW log
```

**Timing**: Always AFTER session completes, never before or during

**See**: [WORKLOG-HANDBOOK.md](../../.artifacts/WORKLOG-HANDBOOK.md) for RAW log management

---

## Template Placeholders

### Common Placeholders (All Templates)

| Placeholder | Description | Example | Auto-Filled by Script |
|-------------|-------------|---------|----------------------|
| `{DATE}` | Current date | 2025-11-02 | ✅ Yes |
| `{TASK_ID}` | Task identifier | T-02-02-sparse-index | ✅ Yes |
| `{TASK_TITLE}` | Full task name | Sparse Index - Name/ID + BM25 Search | ✅ Yes (from metadata.yaml) |
| `{DEVELOPER_NAME}` | Developer name | Rust Dev 2 | ✅ Yes (from arg) |
| `{BRANCH_NAME}` | Git branch | feat/task/T-02-02-sparse-index | ✅ Yes (auto-detect) |

### Session-Specific Placeholders

| Placeholder | Description | Example | Auto-Filled by Script |
|-------------|-------------|---------|----------------------|
| `{SESSION}` | Session number (padded) | 05 | ✅ Yes |
| `{DESCRIPTION}` | Session description | Phase 2 Testing | ✅ Yes (from arg) |
| `{THREAD_START}` | First thread number | 01 | ✅ Yes (RAW log only) |
| `{THREAD_END}` | Last thread number | 03 | ✅ Yes (RAW log only) |

### Metadata-Specific Placeholders

| Placeholder | Description | Example | Manual Fill |
|-------------|-------------|---------|-------------|
| `{MILESTONE_ID}` | Milestone | M2 | ✅ Manual |
| `{PRD_FILE}` | PRD filename | 02-cds-index-service | ✅ Manual |
| `{ISSUE_FILE}` | Issue filename | 02-index-core | ✅ Manual |
| `{PHASE}` | Current phase | Phase 2 | ✅ Manual |
| `{DAY}` | Day number | 2 | ✅ Manual |

---

## File Naming Conventions

### Session-Specific Files (Most Common)

**Pattern**: `{date}-S{NN}-{type}.md`

**Examples**:

```text
2025-11-02-S05-work-summary.md
2025-11-02-S05-commit-log.md
2025-11-02-S05-notes.md
2025-11-02-S05-codereview.md
```

**Session Number Padding**:

- Always 2 digits: `S01`, `S02`, ..., `S10`, `S11`
- NOT: `S1`, `S2`

**Rationale**: Session numbers are sequential across all days, not reset per day

---

### RAW Log Files

**Pattern**: `WORK-SESSIONS-{NN}-THREADS-{START}-{END}-SUMMARY-{date}.txt`

**Location**: `worklogs/raw/`

**Examples**:

```text
worklogs/raw/WORK-SESSIONS-01-THREADS-01-03-SUMMARY-2025-10-31.txt
worklogs/raw/WORK-SESSIONS-04-THREADS-01-07-SUMMARY-2025-11-01.txt
worklogs/raw/WORK-SESSIONS-05-THREADS-01-03-SUMMARY-2025-11-02.txt
```

**Number Padding**:

- Session: 2 digits (`01`, `05`, `10`)
- Threads: 2 digits (`01`, `03`, `07`)

---

### Combined Session Files (Legacy Pattern)

**Pattern**: `{date}-S{NN}-S{MM}-{type}.md`

**Used when**: Combining multiple short sessions into one file

**Examples**:

```text
2025-10-31-S01-S02-work-summary.md  ← Sessions 01 and 02 combined
```

**When to use**: Only if both sessions are very short (< 30 min each)

**Next session after combined**: Session 03 (not 02)

---

## Automation Scripts

### 1. `.dev/scripts/task/create-task-worklog.sh`

**Purpose**: One-time task initialization

**Usage**:

```shell
./.dev/scripts/task/create-task-worklog.sh <TASK_ID> "<TASK_TITLE>" "<DEVELOPER_NAME>"
```

**Example**:

```shell
./.dev/scripts/task/create-task-worklog.sh \
  T-02-02-sparse-index \
  "Sparse Index - Name/ID + BM25 Search" \
  "Rust Dev 2"
```

**Creates**:

- Task directory: `.artifacts/spec-tasks-T-02-02-sparse-index/`
- `metadata.yaml` (from template)
- `worklogs/` (empty)
- `git-refs.txt`

**Run from**: Main repository (`~/dev-space/CDSAgent`)

---

### 2. `.dev/scripts/session/create-session-worklog.sh` ✨ NEW

**Purpose**: Session initialization (every work session)

**Usage**:

```shell
./.dev/scripts/session/create-session-worklog.sh \
  <TASK_ID> <SESSION_NUM> "<DESCRIPTION>" "<DEVELOPER_NAME>"
```

**Example**:

```shell
./.dev/scripts/session/create-session-worklog.sh \
  T-02-02-sparse-index 05 "Phase 2 Testing & Benchmarks" "Rust Dev 2"
```

**Creates** (4 files):

- `2025-11-02-S05-work-summary.md`
- `2025-11-02-S05-commit-log.md`
- `2025-11-02-S05-notes.md`
- `2025-11-02-S05-codereview.md`

**Auto-fills**:

- Date, session number, task ID, task title, developer name, branch name

**Run from**: Task worktree (`~/dev-space/CDSAgent-T-XX-XX`)

**Time**: ~2 minutes vs. ~10 minutes manual

---

### 3. `.dev/scripts/session/create-raw-log.sh` ✨ NEW

**Purpose**: RAW log creation (after session completes)

**Usage**:

```shell
./.dev/scripts/session/create-raw-log.sh \
  <TASK_ID> <SESSION_NUM> <THREAD_START> <THREAD_END> "<DESCRIPTION>"
```

**Example**:

```shell
./.dev/scripts/session/create-raw-log.sh \
  T-02-02-sparse-index 05 01 03 "Phase 2 Testing"
```

**Creates**:

- `worklogs/raw/WORK-SESSIONS-05-THREADS-01-03-SUMMARY-2025-11-02.txt`

**Run from**: Task worktree

**Timing**: AFTER session completes (not before!)

---

### 4. `.dev/scripts/task/create-daily-worklog.sh` (Legacy - Daily Pattern)

**Purpose**: Daily worklog creation (old pattern)

**Usage**:

```shell
./.dev/scripts/task/create-daily-worklog.sh <TASK_ID> [DATE]
```

**Creates** (old pattern):

- `{date}-work-summary.md` (no session number)
- `{date}-commit-log.md`
- `{date}-notes.md`

**Status**: ⚠️ Legacy - Use `create-session-worklog.sh` instead for session-based workflow

---

## Example Structures

### Example 1: Task with 4 Sessions Across 2 Days

```tree
.artifacts/spec-tasks-T-02-02-sparse-index/
├── metadata.yaml                        # Task metadata
├── git-refs.txt                         # Git references
├── CLAUDE.md                            # Task-specific AI guide
├── WORKLOG-HANDBOOK.md                  # Session lifecycle guide
└── worklogs/
    ├── 2025-10-31-S01-S02-work-summary.md   # Day 1, Sessions 01-02 combined
    ├── 2025-10-31-S01-S02-commit-log.md
    ├── 2025-10-31-S01-S02-notes.md
    ├── 2025-10-31-S03-work-summary.md       # Day 1, Session 03
    ├── 2025-10-31-S03-commit-log.md
    ├── 2025-10-31-S03-notes.md
    ├── 2025-11-01-S04-work-summary.md       # Day 2, Session 04
    ├── 2025-11-01-S04-commit-log.md
    ├── 2025-11-01-S04-notes.md
    ├── 2025-11-01-S04-codereview.md         # ✨ NEW: Code review phase
    └── raw/                                 # ✨ NEW: RAW logs
        ├── WORK-SESSIONS-01-THREADS-01-03-SUMMARY-2025-10-31.txt
        ├── WORK-SESSIONS-02-THREADS-01-03-SUMMARY-2025-10-31.txt
        ├── WORK-SESSIONS-03-THREADS-01-04-SUMMARY-2025-10-31.txt
        └── WORK-SESSIONS-04-THREADS-01-07-SUMMARY-2025-11-01.txt
```

### Example 2: Typical Session Files

```markdown
# 2025-11-02-S05-work-summary.md

## Session Objectives

- [ ] Run comprehensive BM25 search tests
- [ ] Benchmark search performance
- [ ] Compare with LocAgent baseline

## Work Completed

### Thread 01 (09:00-10:30 UTC, 1.5h): BM25 Testing
- Implemented 12 BM25 search test cases
- Verified ranking algorithm correctness
- Fixed 2 edge cases in tokenization

### Thread 02 (10:45-12:00 UTC, 1.25h): Performance Benchmarks
- Added criterion benchmarks for search
- Latency p95: 234ms (target <500ms ✅)
- Throughput: 4.2 queries/sec

### Thread 03 (13:00-14:30 UTC, 1.5h): Parity Validation
- Compared results with LocAgent baseline
- Overlap@10: 94% (target ≥90% ✅)
- Documented 3 minor variance cases

## Session Statistics

- Duration: 4.25h (Threads 01-03)
- Commits: 3 (a1b2c3d, e4f5g6h, i7j8k9l)
- Tests Added: 12
- Pass Rate: 100%
```

---

## Migration from Daily to Session-Based

### Old Pattern (Daily)

```text
worklogs/
├── 2025-10-31-work-summary.md      # ❌ One file per day
├── 2025-10-31-commit-log.md
├── 2025-10-31-notes.md
├── 2025-11-01-work-summary.md
├── 2025-11-01-commit-log.md
└── 2025-11-01-notes.md
```

**Problem**: If you have 3 sessions on 2025-10-31, how do you track them separately?

---

### New Pattern (Session-Based)

```text
worklogs/
├── 2025-10-31-S01-work-summary.md  # ✅ Session 01
├── 2025-10-31-S02-work-summary.md  # ✅ Session 02
├── 2025-10-31-S03-work-summary.md  # ✅ Session 03
├── 2025-11-01-S04-work-summary.md  # ✅ Session 04
└── raw/                            # ✅ RAW logs for AI handoff
    ├── WORK-SESSIONS-01-THREADS-01-03-SUMMARY-2025-10-31.txt
    ├── WORK-SESSIONS-02-THREADS-01-03-SUMMARY-2025-10-31.txt
    ├── WORK-SESSIONS-03-THREADS-01-04-SUMMARY-2025-10-31.txt
    └── WORK-SESSIONS-04-THREADS-01-07-SUMMARY-2025-11-01.txt
```

**Benefit**: Each session tracked separately with proper context

---

## Best Practices

### Session Initialization

- ✅ **Always use automated script** (`create-session-worklog.sh`)
- ✅ **Determine session number first** (ls existing S{NN} files)
- ✅ **Fill out objectives immediately** (before coding)
- ✅ **Create codereview.md** only when testing/fixing

### During Session

- ✅ **Update files continuously** (don't wait until EOD)
- ✅ **Document decisions as made** (in notes.md)
- ✅ **Track commits** (in commit-log.md after each commit)
- ✅ **Update work-summary.md** (mark objectives complete)

### After Session

- ✅ **Create RAW log** (always AFTER, not before)
- ✅ **Run checkpoint workflow** (update metadata, git notes)
- ✅ **Commit all artifacts** (including worklogs)

---

## Related Documentation

- **[SESSION_INITIALIZATION_WORKFLOW.md](../workflows/SESSION_INITIALIZATION_WORKFLOW.md)** - Complete session initialization guide
- **[WORKLOG-HANDBOOK.md](../../.artifacts/WORKLOG-HANDBOOK.md)** - Session lifecycle and RAW log management
- **[WORKTREE_WORKFLOW.md](../../docs/WORKTREE_WORKFLOW.md)** - Overall task development lifecycle
- **[WORK_SESSION_CHECKPOINT_WORKFLOW.md](../../docs/WORK_SESSION_CHECKPOINT_WORKFLOW.md)** - End-of-session checkpoint process
- **[TODO.yaml](../../spacs/tasks/0.1.0-mvp/TODO.yaml)** - Central task registry
- **[RFC-DEV-TOOLS.md](../../docs/RFC-DEV-TOOLS.md)** - Development tools architecture

---

**Version History**:

- **v2.0** (2025-11-02): Complete rewrite for session-based workflow with RAW logs and codereview.md
- **v1.0** (2025-10-19): Initial version (daily-based workflow)

**Maintainer**: CDSAgent Tech Lead

---

END OF README
