# Session Initialization Workflow

**Version**: 1.0
**Date**: 2025-11-02
**Purpose**: Guide for starting new work sessions with proper artifact initialization
**Audience**: All CDSAgent Development Team Members

---

## Table of Contents

1. [Overview](#overview)
2. [When to Use This Workflow](#when-to-use-this-workflow)
3. [Prerequisites](#prerequisites)
4. [Session Initialization Steps](#session-initialization-steps)
5. [File Naming Conventions](#file-naming-conventions)
6. [Automated vs Manual Initialization](#automated-vs-manual-initialization)
7. [Integration with Other Workflows](#integration-with-other-workflows)
8. [Troubleshooting](#troubleshooting)
9. [Example Walkthrough](#example-walkthrough)

---

## Overview

A **work session** is a focused period of development (typically 1-8 hours) during which you work on one or more threads toward a specific objective. Sessions are tracked independently, even when multiple sessions occur on the same calendar day.

### Key Concepts

- **Task**: High-level work unit (e.g., T-02-02-sparse-index)
- **Phase**: Implementation stage (Phase 1, Phase 2, etc.)
- **Day**: Calendar date (YYYY-MM-DD)
- **Session**: Work period within a day (can be multiple per day)
- **Thread**: Work unit within a session (typically 30min-2h)

### Session Hierarchy

```text
Task (T-02-02)
  └─ Phase 1: Planning
  └─ Phase 2: Implementation
      └─ Day 1 (2025-11-01)
          └─ Session 01 (1.2h)
              ├─ Thread 01: Setup (0.3h)
              ├─ Thread 02: Analysis (0.5h)
              └─ Thread 03: Planning (0.4h)
          └─ Session 02 (2.1h)
              ├─ Thread 01: Implementation (1.0h)
              ├─ Thread 02: Testing (0.6h)
              └─ Thread 03: Fixes (0.5h)
      └─ Day 2 (2025-11-02)
          └─ Session 03 (3.5h)
              ├─ Thread 01: Development (1.5h)
              ├─ Thread 02: Testing (1.0h)
              └─ Thread 03: Code Review (1.0h)
```

---

## When to Use This Workflow

Use this workflow at the **start of each new work session**:

### Required Session Initialization

✅ **Beginning of work day**

- First session of the day
- Fresh start after overnight break

✅ **After significant context switch**

- Returning after multi-hour break
- Switching from different task

✅ **Starting new phase**

- Moving from Phase 1 (Planning) to Phase 2 (Implementation)
- Phase transition within same day

✅ **After previous session complete**

- Previous session's RAW log finalized
- Checkpoint workflow completed

### NOT Required

❌ **Between threads within same session**

- Thread 01 → Thread 02: No new session
- Just continue updating existing session files

❌ **Short breaks (< 1 hour)**

- Coffee break, lunch
- Resume same session

❌ **Task initialization**

- Use `create-task-worklog.sh` instead (one-time setup)

---

## Prerequisites

Before starting session initialization:

### 1. Task Already Initialized

```shell
# Verify task artifacts exist
ls .artifacts/spec-tasks-T-XX-XX/

# Should show:
# - metadata.yaml
# - git-refs.txt
# - worklogs/
# - CLAUDE.md (optional)
```

If missing, run:

```shell
./.dev/scripts/task/create-task-worklog.sh T-XX-XX "Task Title" "Your Name"
```

### 2. Template System Ready

```shell
# Verify templates exist
ls .dev/templates/worklogs/

# Should show:
# - work-summary.template.md
# - commit-log.template.md
# - notes.template.md
# - codereview.template.md
# - raw-session.template.txt
```

If missing, copy from `.artifacts/spec-tasks-templates/` or create from examples.

### 3. Previous Session Complete (if applicable)

- [ ] Previous session's RAW log finalized
- [ ] Previous session's checkpoint completed
- [ ] All artifacts committed to git

---

## Session Initialization Steps

### Method A: Automated (Recommended - ~2 minutes)

**Step-1**: Determine Session Number

```shell
# Check existing sessions
ls .artifacts/spec-tasks-T-XX-XX/worklogs/ | grep -E "S[0-9]{2}"

# Example output:
# 2025-10-31-S01-S02-work-summary.md
# 2025-10-31-S03-work-summary.md
# 2025-11-01-S04-work-summary.md

# Next session: 05
```

**Step-2**: Run Session Initialization Script

```shell
cd ~/dev-space/CDSAgent-T-XX-XX-task-name

# Run script
./.dev/scripts/session/create-session-worklog.sh \
  T-XX-XX-task-name 05 "Phase 2 Testing" "Your Name"

# Output:
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
#    Session Worklog Initialization
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
#
# Task:        T-XX-XX-task-name
# Session:     05
# Date:        2025-11-02
# Description: Phase 2 Testing
# Developer:   Your Name
#
# ✓ Created: 2025-11-02-S05-work-summary.md
# ✓ Created: 2025-11-02-S05-commit-log.md
# ✓ Created: 2025-11-02-S05-notes.md
# ✓ Created: 2025-11-02-S05-codereview.md
```

**Step-3**: Verify Files Created

```shell
ls .artifacts/spec-tasks-T-XX-XX/worklogs/ | grep S05

# Should show:
# 2025-11-02-S05-work-summary.md
# 2025-11-02-S05-commit-log.md
# 2025-11-02-S05-notes.md
# 2025-11-02-S05-codereview.md
```

**Step-4**: Fill Out Session Objectives

```shell
# Open work-summary.md
vim .artifacts/spec-tasks-T-XX-XX/worklogs/2025-11-02-S05-work-summary.md

# Fill out "Session Objectives" section:
## Session Objectives

- [ ] Implement BM25 ranking algorithm
- [ ] Add unit tests for tokenizer
- [ ] Benchmark search performance
```

**Step-5**: Begin Development (Thread 01)

Start coding! Update files continuously during session.

---

### Method B: Manual (Fallback - ~10 minutes)

If scripts unavailable, manually create session files:

**Step-1**: Determine Session Number and Date

```shell
SESSION_NUM=05
DATE=$(date +%Y-%m-%d)  # 2025-11-02
```

**Step-2**: Copy Templates

```shell
WORKLOG_DIR=".artifacts/spec-tasks-T-XX-XX/worklogs"
TEMPLATE_DIR=".dev/templates/worklogs"

# Copy 4 templates
cp ${TEMPLATE_DIR}/work-summary.template.md \
   ${WORKLOG_DIR}/${DATE}-S${SESSION_NUM}-work-summary.md

cp ${TEMPLATE_DIR}/commit-log.template.md \
   ${WORKLOG_DIR}/${DATE}-S${SESSION_NUM}-commit-log.md

cp ${TEMPLATE_DIR}/notes.template.md \
   ${WORKLOG_DIR}/${DATE}-S${SESSION_NUM}-notes.md

cp ${TEMPLATE_DIR}/codereview.template.md \
   ${WORKLOG_DIR}/${DATE}-S${SESSION_NUM}-codereview.md
```

**Step-3**: Replace Placeholders

Edit each file and replace:

- `{DATE}` → `2025-11-02`
- `{SESSION}` → `05`
- `{TASK_ID}` → `T-XX-XX-task-name`
- `{TASK_TITLE}` → Task title from metadata.yaml
- `{DEVELOPER_NAME}` → Your name
- `{DESCRIPTION}` → Session description

---

## File Naming Conventions

### Session-Specific Files

**Pattern**: `{date}-S{NN}-{type}.md`

```text
2025-11-02-S05-work-summary.md
2025-11-02-S05-commit-log.md
2025-11-02-S05-notes.md
2025-11-02-S05-codereview.md  # Optional (create if testing/fixes phase)
```

**Session Number Padding**:

- Always 2 digits: `S01`, `S02`, ..., `S10`, `S11`
- NOT: `S1`, `S2`

### RAW Log Files (Created AFTER Session)

**Pattern**: `WORK-SESSIONS-{NN}-THREADS-{START}-{END}-SUMMARY-{date}.txt`

```text
worklogs/raw/WORK-SESSIONS-05-THREADS-01-03-SUMMARY-2025-11-02.txt
```

**Important**: RAW log is created **AFTER session completes**, not before!

---

## Automated vs Manual Initialization

### Automated Initialization (Recommended)

**Pros**:

- ✅ Fast (~2 minutes vs. ~10 minutes manual)
- ✅ Consistent file naming (no typos)
- ✅ Auto-populates task title, branch name
- ✅ Validates session number
- ✅ Creates all 4 files in one command

**Cons**:

- ❌ Requires scripts available
- ❌ Less flexible (standard template only)

### Manual Initialization

**Pros**:

- ✅ Works when scripts unavailable
- ✅ More flexible customization
- ✅ Can create partial set (e.g., skip codereview.md)

**Cons**:

- ❌ Slow (~10 minutes)
- ❌ Error-prone (typos in file naming)
- ❌ Manual placeholder replacement

---

## Integration with Other Workflows

### Relationship to WORKTREE_WORKFLOW.md

Session initialization is **Phase 2: Daily Development - Start-of-session routine**.

```text
Task Development Lifecycle (WORKTREE_WORKFLOW)
  ├─ Phase 1: Worktree Environment Preparation (one-time)
  ├─ Phase 2: Daily Development
  │    ├─ Start-of-session:  ← SESSION_INITIALIZATION_WORKFLOW
  │    │    1. Create session files (this workflow)
  │    │    2. Fill out objectives
  │    │    3. Begin Thread 01
  │    ├─ During session:
  │    │    - Work through threads
  │    │    - Update files continuously
  │    └─ End-of-session:    ← WORK_SESSION_CHECKPOINT_WORKFLOW
  │         1. Create RAW log
  │         2. Run checkpoint
  └─ Phase 3: Task Completion
```

### Relationship to WORKLOG-HANDBOOK.md

Session initialization is **Phase 1: Session Starts** in WORKLOG-HANDBOOK.

```text
Work Session Lifecycle (WORKLOG-HANDBOOK)
  ├─ Phase 1: Session Starts       ← This workflow documents this
  │    1. Create session files
  │    2. Fill out objectives
  │    3. Begin Thread 01
  ├─ Phase 2: During Session
  │    - Thread transitions
  │    - Continuous file updates
  ├─ Phase 3: Session Completes
  │    - Create RAW log
  │    - Run checkpoint
  └─ Phase 4: Checkpoint Complete
```

### Relationship to WORK_SESSION_CHECKPOINT_WORKFLOW.md

Session initialization is the **prerequisite** for checkpoint workflow.

```text
Session Lifecycle:
  Session Init (this doc) → Development → Testing → Code Review → Checkpoint
       ↓                       ↓             ↓            ↓             ↓
  Create files           Update files   Test code    codereview.md  Update all
```

---

## Troubleshooting

### Issue: Script Not Found

**Symptoms**:

```shell
$ ./.dev/scripts/session/create-session-worklog.sh
-bash: ./.dev/scripts/session/create-session-worklog.sh: No such file or directory
```

**Solution**:

```shell
# Check if scripts exist
ls .dev/scripts/session/

# If missing, use absolute path from main repo
/Users/arthur/dev-space/CDSAgent/.dev/scripts/session/create-session-worklog.sh \
  T-XX-XX 05 "Description" "Developer"

# Or fall back to manual initialization
```

### Issue: Templates Not Found

**Symptoms**:

```shell
⚠ Template not found: work-summary.template.md (skipping)
```

**Solution**:

```shell
# Check template location
ls .dev/templates/worklogs/

# If missing, check old location
ls .artifacts/spec-tasks-templates/worklogs/

# Script will auto-fallback to old location
```

### Issue: Session Number Confusion

**Symptoms**: Not sure what session number to use

**Solution**:

```shell
# List existing sessions
ls .artifacts/spec-tasks-T-XX-XX/worklogs/ | grep -E "S[0-9]{2}" | sort | tail -5

# Output example:
# 2025-10-31-S01-S02-work-summary.md
# 2025-10-31-S03-work-summary.md
# 2025-11-01-S04-work-summary.md
# 2025-11-01-S04-codereview.md

# Last session: S04
# Next session: S05
```

**Note on Combined Sessions**:

- `S01-S02` means Sessions 01 and 02 were combined in one file
- This is OK for very short sessions
- Next session after `S01-S02` is `S03` (not `S02`)

### Issue: File Already Exists

**Symptoms**:

```shell
⚠ File already exists: 2025-11-02-S05-work-summary.md (skipping)
```

**Solution**:

```shell
# If you want to recreate:
rm .artifacts/spec-tasks-T-XX-XX/worklogs/2025-11-02-S05-*.md

# Then re-run script

# Or manually edit existing files
```

---

## Example Walkthrough

### Scenario: Starting Session 05 on T-02-02-sparse-index

**Context**:

- Task: T-02-02-sparse-index (Sparse Index - Name/ID + BM25 Search)
- Phase: Phase 2 (Implementation)
- Previous sessions: S01-S04 complete
- Date: 2025-11-02
- Developer: Rust Dev 2

**Step-by-Step**:

```shell
# 1. Navigate to task worktree
cd ~/dev-space/CDSAgent-T-02-02-sparse-index

# 2. Verify previous session complete
ls .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/raw/
# Should show: WORK-SESSIONS-04-THREADS-01-07-SUMMARY-2025-11-01.txt

# 3. Determine next session number
ls .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/ | grep -E "S[0-9]{2}" | sort | tail -1
# Output: 2025-11-01-S04-work-summary.md
# Next session: 05

# 4. Run initialization script
./.dev/scripts/session/create-session-worklog.sh \
  T-02-02-sparse-index 05 "Phase 2: BM25 Testing & Benchmarks" "Rust Dev 2"

# Output:
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
#    Session Worklog Initialization
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
#
# Task:        T-02-02-sparse-index
# Session:     05
# Date:        2025-11-02
# Description: Phase 2: BM25 Testing & Benchmarks
# Developer:   Rust Dev 2
#
# ✓ Created: 2025-11-02-S05-work-summary.md
# ✓ Created: 2025-11-02-S05-commit-log.md
# ✓ Created: 2025-11-02-S05-notes.md
# ✓ Created: 2025-11-02-S05-codereview.md

# 5. Verify files created
ls .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-11-02-S05-*
# Output:
# 2025-11-02-S05-work-summary.md
# 2025-11-02-S05-commit-log.md
# 2025-11-02-S05-notes.md
# 2025-11-02-S05-codereview.md

# 6. Fill out session objectives
vim .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-11-02-S05-work-summary.md

# Edit "Session Objectives" section:
## Session Objectives

- [ ] Run comprehensive BM25 search tests
- [ ] Benchmark search performance (latency, throughput)
- [ ] Compare results with LocAgent parity baseline
- [ ] Fix any failing tests or performance issues
- [ ] Document test results in codereview.md

# 7. Open IDE and begin development
code .

# 8. Start Thread 01
# Begin coding, testing, documenting...
```

**Result**:

- Session 05 initialized in ~2 minutes
- All 4 worklog files ready
- Objectives documented
- Ready to start Thread 01

---

## Quick Reference Commands

```shell
# Automated initialization (recommended)
./.dev/scripts/session/create-session-worklog.sh \
  T-XX-XX <session-num> "<description>" "<developer>"

# Create RAW log AFTER session completes
./.dev/scripts/session/create-raw-log.sh \
  T-XX-XX <session-num> <thread-start> <thread-end> "<description>"

# List existing sessions
ls .artifacts/spec-tasks-T-XX-XX/worklogs/ | grep -E "S[0-9]{2}" | sort

# Verify task initialized
ls .artifacts/spec-tasks-T-XX-XX/metadata.yaml

# Check templates available
ls .dev/templates/worklogs/
```

---

## Related Documents

- **WORKTREE_WORKFLOW.md** - Overall task development lifecycle
- **WORKLOG-HANDBOOK.md** - Session lifecycle and RAW log management
- **WORK_SESSION_CHECKPOINT_WORKFLOW.md** - End-of-session checkpoint process
- **NEXT_TASK_CHECKLIST.md** - Task selection and preparation
- **RFC-DEV-TOOLS.md** - Development tools architecture

---

**Version History**:

- **v1.0** (2025-11-02): Initial version documenting session initialization workflow

**Maintainer**: CDSAgent Tech Lead

---

END OF WORKFLOW
