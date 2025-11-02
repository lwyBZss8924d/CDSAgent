# Session Management - Complete Reference

This document provides detailed information about CDSAgent's session management, worklog creation, and session lifecycle.

## Table of Contents

- [Session Lifecycle Deep Dive](#session-lifecycle-deep-dive)
- [Session vs Thread vs Phase](#session-vs-thread-vs-phase)
- [Script Signatures and Parameters](#script-signatures-and-parameters)
- [Session Artifact Structure](#session-artifact-structure)
- [RAW Log Format and Purpose](#raw-log-format-and-purpose)
- [Session Numbering Strategy](#session-numbering-strategy)
- [Integration with Checkpoint Workflow](#integration-with-checkpoint-workflow)
- [Template Expansion](#template-expansion)
- [Metadata Updates](#metadata-updates)
- [Advanced Session Patterns](#advanced-session-patterns)

---

## Session Lifecycle Deep Dive

### What is a Session?

A **session** is a contiguous work period (1-8 hours) focused on specific objectives within a task. Sessions are the fundamental unit of work tracking in CDSAgent.

**Key Characteristics**:

- **Sequential numbering**: 01, 02, 03, 04, 05... across all days
- **Multiple per day**: Normal to have Sessions 01-03 on same day
- **Contains threads**: Each session has 1-N threads (work units)
- **Produces artifacts**: 3 worklogs + 1 RAW log per session

### Session States

1. **Uninitialized**: Task exists but no sessions yet
2. **Initialized**: Session worklogs created, ready for work
3. **In Progress**: Threads being executed (01-NN)
4. **Completed**: RAW log created, threads documented
5. **Checkpointed**: Artifacts committed, metadata updated

### Full Lifecycle Flow

```text
┌─────────────────────────────────────────────────────────────┐
│ Session Lifecycle                                           │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  1. [Uninitialized]                                         │
│       ↓                                                     │
│  create-session-worklog.sh ←── Determines next session #    │
│       ↓                                                     │
│  2. [Initialized]                                           │
│       ↓                                                     │
│  Work Threads 01-NN ←── Development work, commits           │
│       ↓                                                     │
│  3. [In Progress]                                           │
│       ↓                                                     │
│  create-raw-log.sh ←── Documents completed threads          │
│       ↓                                                     │
│  4. [Completed]                                             │
│       ↓                                                     │
│  checkpoint-helper.sh ←── Validates and commits artifacts   │
│       ↓                                                     │
│  5. [Checkpointed] ←── Ready for next session               │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## Session vs Thread vs Phase

Understanding the hierarchy is critical for proper artifact management.

### Hierarchy

```text
Task
  └── Phase (implementation stage, 1-7 days)
        └── Session (work period, 1-8 hours)
              └── Thread (work unit, 30min-2h)
```

### Definitions

**Task**:

- Example: `T-02-02-sparse-index`
- Duration: 4-10 days
- Contains: 1-10 sessions

**Phase** (implementation stage):

- Example: "Phase 2: Custom Tokenizer"
- Duration: 0.5-3 days
- Technical milestones: Planning, Upper Index, Tokenizer, BM25, etc.
- **Not reflected in file naming**

**Session** (work period):

- Example: "Session 04"
- Duration: 1-8 hours
- Contains: 1-10 threads
- **Reflected in file naming**: `2025-11-01-S04-work-summary.md`

**Thread** (work unit):

- Example: "Thread 06"
- Duration: 30min-2h
- Granularity: Single objective (implement X, test Y, fix Z)
- **Reflected in RAW log**: `WORK-SESSIONS-04-THREADS-01-07-SUMMARY-2025-11-01.txt`

### Numbering Rules

| Entity  | Numbering Scope       | Resets When?     | Example         |
|---------|----------------------|------------------|-----------------|
| Phase   | Per task (implicit)  | New task         | Phase 2         |
| Session | Per task (sequential)| Never in task    | 01, 02, 03...   |
| Thread  | Per session          | New session      | 01-07 (Session 04) |

---

## Script Signatures and Parameters

### create-session-worklog.sh

**Purpose**: Initialize a new work session with 3 worklog files.

**Signature**:

```shell
./.dev/scripts/session/create-session-worklog.sh TASK_ID SESSION_NUM "DESCRIPTION" "DEVELOPER"
```

**Parameters**:

| Parameter      | Type   | Example                  | Description                          |
|----------------|--------|--------------------------|--------------------------------------|
| `TASK_ID`      | String | `T-02-02`                | Task identifier (T-XX-XX format)     |
| `SESSION_NUM`  | String | `05`                     | 2-digit session number (01-99)       |
| `DESCRIPTION`  | String | `"Phase 3 BM25 Integration"` | Brief session purpose          |
| `DEVELOPER`    | String | `"Claude Code Agent"`    | AI agent or human developer name     |

**Validation**:

- Task ID must match `T-XX-XX` pattern
- Session number must be 2 digits (01-99)
- Description required (non-empty)
- Developer name required

**Exit Codes**:

- `0`: Success, worklogs created
- `1`: Invalid parameters or task directory not found
- `2`: Session already exists (duplicate)

**Creates**:

```text
.artifacts/spec-tasks-{TASK_ID}/worklogs/
  ├── {date}-S{SESSION_NUM}-work-summary.md
  ├── {date}-S{SESSION_NUM}-notes.md
  └── {date}-S{SESSION_NUM}-commit-log.md
```

**Template Source**:

- `.dev/templates/worklogs/work-summary.template.md`
- `.dev/templates/worklogs/notes.template.md`
- `.dev/templates/worklogs/commit-log.template.md`

### create-raw-log.sh

**Purpose**: Create RAW session log for AI handoff after session completion.

**Signature**:

```shell
./.dev/scripts/session/create-raw-log.sh TASK_ID SESSION_NUM THREAD_START THREAD_END "DESCRIPTION"
```

**Parameters**:

| Parameter       | Type   | Example               | Description                         |
|-----------------|--------|-----------------------|-------------------------------------|
| `TASK_ID`       | String | `T-02-02`             | Task identifier                     |
| `SESSION_NUM`   | String | `04`                  | 2-digit session number              |
| `THREAD_START`  | String | `01`                  | First thread number (2 digits)      |
| `THREAD_END`    | String | `07`                  | Last thread number (2 digits)       |
| `DESCRIPTION`   | String | `"Tokenizer complete"`| Session summary                     |

**Validation**:

- THREAD_END must be >= THREAD_START
- Both thread numbers must be 2 digits (01-99)

**Exit Codes**:

- `0`: Success, RAW log created
- `1`: Invalid parameters or session worklogs not found
- `2`: RAW log already exists (duplicate)

**Creates**:

```text
.artifacts/spec-tasks-{TASK_ID}/worklogs/raw/
  └── WORK-SESSIONS-{SESSION_NUM}-THREADS-{THREAD_START}-{THREAD_END}-SUMMARY-{date}.txt
```

**Template Source**:

- `.dev/templates/worklogs/raw-session.template.txt`

---

## Session Artifact Structure

### Standard Session Artifacts

Every session produces 4 files:

```text
.artifacts/spec-tasks-T-02-02-sparse-index/worklogs/
├── 2025-11-01-S04-work-summary.md      # Deliverables and outcomes
├── 2025-11-01-S04-notes.md             # Technical implementation notes
├── 2025-11-01-S04-commit-log.md        # Git commit history
└── raw/
    └── WORK-SESSIONS-04-THREADS-01-07-SUMMARY-2025-11-01.txt  # AI handoff
```

### Optional Artifacts

**Code Review** (testing/QA phase):

```text
└── 2025-11-01-S04-codereview.md
```

**Benchmark Results** (performance testing):

```text
└── 2025-11-01-S04-benchmarks.md
```

### Artifact Content Purpose

**work-summary.md**:

- Session objectives
- Deliverables completed
- Acceptance criteria status
- Next steps

**notes.md**:

- Technical decisions
- Implementation details
- Challenges encountered
- Lessons learned

**commit-log.md**:

- Git commit hashes
- Commit messages
- Files changed
- Code review notes

**RAW log (.txt)**:

- Complete session narrative
- Thread-by-thread timeline
- Tool usage
- AI agent handoff context

---

## RAW Log Format and Purpose

### What is a RAW Log?

The RAW log is a **complete, unedited transcript** of the session's work, capturing:

- Thread timeline (start/end times)
- Tool invocations and results
- Decision-making process
- Code snippets and outputs
- Agent thought process

### Purpose

1. **AI Agent Handoff**: Next AI agent reads RAW log to understand context
2. **Session Continuity**: Preserve complete narrative across days/weeks
3. **Debugging**: Trace decisions and actions when issues arise
4. **Audit Trail**: Record of development process for compliance

### Format Structure

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

### Objectives
- Review Phase 2 requirements from task spec
- Analyze tokenizer parity with LocAgent
- Plan implementation roadmap

### Actions Taken
[Tool: Read] spacs/tasks/0.1.0-mvp/02-index-core/T-02-02-sparse-index.md
... (complete content)

[Tool: Read] tmp/LocAgent/repo_index/utils/code_process.py
... (tokenizer implementation)

### Decisions Made
- Custom tokenizer required (not NLTK directly)
- Offset preservation critical for parity
- Use Tantivy analyzer for BM25

### Outcome
✅ Phase 2 roadmap defined (4 threads planned)

═══════════════════════════════════════════════════════

## THREAD 02 (02:05-02:13, 8min): Tokenizer Parity Analysis

... (continues for all 7 threads)

═══════════════════════════════════════════════════════

## SESSION SUMMARY

**Phase Status**: ✅ Phase 2 COMPLETE

**Deliverables**:
- tokenizer.rs (387 lines) - Custom tokenizer with offset preservation
- bm25.rs (+442 lines) - Tantivy backend scaffold
- stop_words.rs (180 lines) - NLTK stop-word integration
- export_stop_words.py (176 lines) - Automation script
- 12 new tests (7 tokenizer, 2 BM25, 3 fixture)

**Quality Metrics**:
- Tests: 78/78 passing (100%)
- Coverage: ~95% (maintained from Phase 1)
- Clippy: 5 errors fixed, zero warnings

**Next Steps**:
- Phase 3: BM25 integration + hierarchical search
- Phase 4: Parity validation (overlap@10 target: ≥90%)
- Phase 5: Performance benchmarking

**Commits**:
- 414f7f2: feat(index): T-02-02 Phase 2 - tokenizer + BM25 scaffold

**Git Notes**:
- spec-tasks/T-02-02: Session 04 Thread 06 - BM25 index scaffold

═══════════════════════════════════════════════════════
```

### Naming Convention

Pattern: `WORK-SESSIONS-{NN}-THREADS-{START}-{END}-SUMMARY-{date}.txt`

Examples:

- `WORK-SESSIONS-01-THREADS-01-03-SUMMARY-2025-10-31.txt` (Session 01, Threads 01-03)
- `WORK-SESSIONS-04-THREADS-01-07-SUMMARY-2025-11-01.txt` (Session 04, Threads 01-07)

---

## Session Numbering Strategy

### Sequential Across Days

Sessions are numbered sequentially from task start, **never resetting per day**.

**Example Task Timeline**:

| Date       | Sessions | Description                          |
|------------|----------|--------------------------------------|
| 2025-10-31 | 01-03    | Day 1: Three sessions (planning + implementation) |
| 2025-11-01 | 04       | Day 2: One session (tokenizer)       |
| 2025-11-02 | 05-06    | Day 3: Two sessions (BM25 + parity)  |

**File Naming**:

```text
# Day 1 (2025-10-31)
2025-10-31-S01-work-summary.md
2025-10-31-S02-work-summary.md
2025-10-31-S03-work-summary.md

# Day 2 (2025-11-01)
2025-11-01-S04-work-summary.md

# Day 3 (2025-11-02)
2025-11-02-S05-work-summary.md
2025-11-02-S06-work-summary.md
```

### Determining Next Session Number

**Method 1**: Check metadata.yaml

```shell
# View sessions array
cat .artifacts/spec-tasks-T-XX-XX/metadata.yaml | grep -A 50 "sessions:"

# Output shows last session
sessions:
  - id: "04"
    date: "2025-11-01"
    ...

# Next session: 05
```

**Method 2**: List worklog files

```shell
# List session worklogs
ls .artifacts/spec-tasks-T-XX-XX/worklogs/ | grep "S0[0-9]" | sort | tail -1

# Output: 2025-11-01-S04-work-summary.md
# Next session: 05
```

**Method 3**: Count RAW logs

```shell
# Count RAW logs
ls .artifacts/spec-tasks-T-XX-XX/worklogs/raw/ | wc -l

# Output: 4
# Next session: 05
```

---

## Integration with Checkpoint Workflow

### Checkpoint Workflow Overview

Checkpoint workflow runs at session end to validate and commit artifacts.

**5-Phase Process**:

1. **Phase 1**: Review session work
2. **Phase 2**: Verify artifacts
3. **Phase 3**: Update metadata
4. **Phase 4**: Git operations (notes + checkpoint commit)
5. **Phase 5**: Final validation

### Session Management's Role

*Before Checkpoint*:

1. Session initialized with `create-session-worklog.sh`
2. Threads executed (01-NN)
3. RAW log created with `create-raw-log.sh`

*During Checkpoint*:
4. Checkpoint validates session artifacts exist
5. Metadata.yaml updated with session entry
6. Checkpoint commit includes session worklogs

*After Checkpoint*:
7. Session marked COMPLETE in metadata
8. Git notes pushed to origin
9. Ready for next session

### Checkpoint Artifact Verification

**Required for Checkpoint**:

- ✅ `{date}-S{NN}-work-summary.md` exists
- ✅ `{date}-S{NN}-notes.md` exists
- ✅ `{date}-S{NN}-commit-log.md` exists
- ✅ `worklogs/raw/WORK-SESSIONS-{NN}-*-SUMMARY-{date}.txt` exists
- ✅ `metadata.yaml` sessions array updated
- ✅ All commits have git notes

**Optional**:

- ⚠️ `{date}-S{NN}-codereview.md` (if QA phase)
- ⚠️ `{date}-S{NN}-benchmarks.md` (if performance testing)

---

## Template Expansion

### Template Variables

Session scripts expand templates with variables:

| Variable         | Example                  | Description                    |
|------------------|--------------------------|--------------------------------|
| `{{TASK_ID}}`    | `T-02-02`                | Task identifier                |
| `{{SESSION_NUM}}`| `04`                     | Session number (2 digits)      |
| `{{DATE}}`       | `2025-11-01`             | Current date (YYYY-MM-DD)      |
| `{{DESCRIPTION}}`| `Phase 2 Tokenizer`      | Session description            |
| `{{DEVELOPER}}`  | `Claude Code Agent`      | Developer/AI agent name        |
| `{{THREADS}}`    | `01-07`                  | Thread range                   |

### Template Files

Located in `.dev/templates/worklogs/`:

- `work-summary.template.md`
- `notes.template.md`
- `commit-log.template.md`
- `raw-session.template.txt`

### Expansion Example

**Template** (work-summary.template.md):

```markdown
# Work Summary - {{TASK_ID}} Session {{SESSION_NUM}}

**Date**: {{DATE}}
**Session**: {{SESSION_NUM}}
**Developer**: {{DEVELOPER}}
**Description**: {{DESCRIPTION}}

## Session Objectives
...
```

**Expanded** (2025-11-01-S04-work-summary.md):

```markdown
# Work Summary - T-02-02 Session 04

**Date**: 2025-11-01
**Session**: 04
**Developer**: Claude Code Agent
**Description**: Phase 2 Custom Tokenizer + BM25 Scaffold

## Session Objectives
...
```

---

## Metadata Updates

### metadata.yaml Sessions Array

Each session adds entry to `metadata.yaml`:

```yaml
sessions:
  - id: "04"
    date: "2025-11-01"
    day: 2
    phase: "Phase 2"
    description: "Custom Tokenizer + BM25 Scaffold"
    threads:
      count: 7
      range: "01-07"
    duration:
      start_time: "01:39"
      end_time: "12:45"
      hours: 3.2
    status: completed
    objectives:
      - "Phase 2 planning & spec alignment"
      - "Tokenizer parity analysis & design"
      - "Tokenizer module scaffolding"
      - "Stop-word fixtures & parity harness prep"
      - "Tantivy analyzer integration & offset plumbing"
      - "BM25 index scaffold & search API"
      - "BM25 persistence & benchmark planning"
    commits:
      - "414f7f2"
    raw_log: "./worklogs/raw/WORK-SESSIONS-04-THREADS-01-07-SUMMARY-2025-11-01.txt"
    artifacts:
      work_summary: "./worklogs/2025-11-01-S04-work-summary.md"
      commit_log: "./worklogs/2025-11-01-S04-commit-log.md"
      notes: "./worklogs/2025-11-01-S04-notes.md"
      codereview: "./worklogs/2025-11-01-S04-codereview.md"
    metrics:
      lines_added: 2013
      lines_deleted: 108
      files_modified: 17
      tests_added: 12
      test_pass_rate: 1.0
      total_tests: 78
      coverage_estimate: 0.95
      clippy_errors_fixed: 5
```

### Cumulative Metrics Update

After each session, cumulative metrics updated:

```yaml
metrics:
  estimated_hours: 32
  actual_hours: 8.3  # Cumulative across Sessions 01-04
  lines_added: 4180  # Cumulative
  lines_deleted: 890  # Cumulative
  files_modified: 33  # Cumulative
  tests_added: 17    # Cumulative
  test_pass_rate: 1.0
  test_coverage: 0.972
```

---

## Advanced Session Patterns

### Pattern 1: Multiple Sessions Per Day

**Scenario**: Long development day with natural break points.

**Example** (T-02-02 Day 1):

- **Session 01** (07:17-08:30, 1.2h): Planning & analysis
- **Session 02** (10:22-10:55, 0.55h): Re-analysis & roadmap
- **Session 03** (12:02-15:17, 3.3h): Upper Index implementation

**Benefits**:

- Natural breakpoints for context switching
- Easier to restart after interruptions
- Clearer RAW logs (one per session)

**When to Use**:

- Break after major phase completion
- Context switch required (meeting, break)
- Handoff to different AI agent

### Pattern 2: Combined Session Worklogs

**Scenario**: Sessions 01-02 are planning only, no code commits.

**Approach**: Combine into single set of worklogs:

```text
2025-10-31-S01-S02-work-summary.md   # Combined Sessions 01-02
2025-10-31-S01-S02-notes.md
2025-10-31-S01-S02-commit-log.md
```

**RAW Logs**: Still separate (one per session):

```text
raw/WORK-SESSIONS-01-THREADS-01-03-SUMMARY-2025-10-31.txt
raw/WORK-SESSIONS-02-THREADS-01-03-SUMMARY-2025-10-31.txt
```

**When to Use**:

- Planning sessions without code commits
- Research/analysis phases
- Reduces duplicate boilerplate

### Pattern 3: Session Across Days

**Scenario**: Long-running session paused overnight, resumed next day.

**Approach**: Single session ID, RAW log captures gap:

```text
Session 05 (started 2025-11-02 22:00, paused at 23:30)
  ... pause overnight ...
Session 05 (resumed 2025-11-03 08:00, ended at 10:30)

# RAW log
WORK-SESSIONS-05-THREADS-01-08-SUMMARY-2025-11-02-to-2025-11-03.txt
```

**When to Use**:

- Context preservation critical
- Single logical unit of work
- Rare (prefer completing sessions same day)

---

## Troubleshooting

### Issue: Session Number Confusion

**Problem**: Unsure what session number to use next.

**Solution**:

```shell
# Check metadata.yaml
cat .artifacts/spec-tasks-T-XX-XX/metadata.yaml | grep -A 5 "sessions:" | grep "id:"

# Output:
#   - id: "03"
#   - id: "04"
# Next: 05
```

### Issue: RAW Log Already Exists

**Problem**: `create-raw-log.sh` fails with "RAW log already exists".

**Solution**:

```shell
# Check existing RAW logs
ls .artifacts/spec-tasks-T-XX-XX/worklogs/raw/

# If duplicate session number, use next number
# If truly duplicate, remove old file (after backup)
mv .artifacts/.../raw/WORK-SESSIONS-04-* .artifacts/.../raw/.backup/
```

### Issue: Session Worklogs Missing

**Problem**: Checkpoint validation fails due to missing session worklogs.

**Solution**:

```shell
# Recreate missing worklogs
./.dev/scripts/session/create-session-worklog.sh T-XX-XX NN "Description" "Developer"

# Or manually create from templates
cp .dev/templates/worklogs/work-summary.template.md .artifacts/.../worklogs/2025-11-02-S05-work-summary.md
# Edit file to replace {{variables}}
```

---

## References

- **Primary Workflow**: `.dev/workflows/WORKLOG-HANDBOOK.md`
- **Session Initialization**: `.dev/workflows/SESSION_INITIALIZATION_WORKFLOW.md`
- **Checkpoint Workflow**: `.dev/workflows/WORK_SESSION_CHECKPOINT_WORKFLOW.md`
- **Template Documentation**: `.dev/templates/README.md`
- **Worktree SOP**: `.dev/workflows/WORKTREE_WORKFLOW.md`

**Important⚠️**: Any [session-management] job changed any PMP and docs & metadata .yaml MUST run (date -u '+%Y-%m-%dT%H:%M:%SZ') get UTC TIME NOW first!!!
