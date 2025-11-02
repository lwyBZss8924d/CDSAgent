---
name: template-usage
description: Guides usage of CDSAgent development templates including metadata.yaml, worklog templates, and session artifacts. Use when creating session worklogs, updating metadata, or understanding template structure and placeholders.
allowed-tools: Bash, Edit, Read, Write, Grep, Glob, Task, TodoWrite, SlashCommand, WebSearch, WebFetch
---

# CDSAgent Template Usage

Comprehensive guide to CDSAgent development templates.

**Important⚠️**: Wen Write [template-usage] job's templates, metadata.yaml, worklog templates, and session artifacts, MUST run (date -u '+%Y-%m-%dT%H:%M:%SZ') get UTC TIME NOW first!!!

## Capabilities

- Understand metadata.yaml structure
- Use worklog templates for sessions
- Create session-specific artifacts
- Follow template conventions
- Validate template placeholders

## Template Overview

CDSAgent uses structured templates for consistency:

1. **metadata.template.yaml** - Task metadata structure
2. **work-summary.template.md** - Session work summary
3. **commit-log.template.md** - Session commit log
4. **notes.template.md** - Session technical notes
5. **codereview.template.md** - Session code review
6. **raw-session.template.txt** - RAW log for AI handoff

## Template Locations

All templates in `.dev/templates/`:

```tree
.dev/templates/
├── README.md                           # Template documentation
├── metadata.template.yaml              # Task metadata
└── worklogs/
    ├── work-summary.template.md        # Session summary
    ├── commit-log.template.md          # Git commits
    ├── notes.template.md               # Technical notes
    ├── codereview.template.md          # Code review
    └── raw-session.template.txt        # RAW log
```

## How to Use Templates

### 1. Task Metadata (metadata.yaml)

**Location**: `.artifacts/spec-tasks-T-XX-XX/metadata.yaml`

**Created by**: `create-task-worklog.sh`

**Structure**:

```yaml
task:
  id: T-XX-XX-task-name
  title: "Task Title"
  owner: "Developer Name"
  status: in_progress | completed
  start_date: "YYYY-MM-DD"
  last_updated: "YYYY-MM-DDTHH:MM:SSZ"
  estimated_hours: 32
  actual_hours: 8.3

specs:
  prds:
    - spacs/prd/path/to/prd.md
  issues:
    - spacs/issues/path/to/issue.md
  tasks:
    - spacs/tasks/path/to/task.md

git:
  worktree: .worktrees/T-XX-XX-task-name
  branch: feat/task/T-XX-XX-task-name
  base_commit: <hash>
  commits:
    - hash: "abc123"
      message: "feat(index): implementation"
      date: "YYYY-MM-DD"
      files_changed: 10
      notes: "Session NN Thread MM - description"

deliverables:
  - path/to/deliverable.rs
  - path/to/test.rs

acceptance_criteria:
  - criterion: "Description of criterion"
    status: completed | in_progress | not_started
    notes: "Implementation notes"

dependencies:
  requires:
    - T-YY-YY-prerequisite
  blocks:
    - T-ZZ-ZZ-dependent

sessions:
  - id: "01"
    date: "YYYY-MM-DD"
    day: 1
    phase: "Phase 0"
    description: "Session description"
    threads:
      count: 3
      range: "01-03"
    duration:
      start_time: "HH:MM"
      end_time: "HH:MM"
      hours: 1.2
    status: completed
    objectives:
      - "Objective 1"
      - "Objective 2"
    commits:
      - "commit-hash"
    raw_log: "./worklogs/raw/WORK-SESSIONS-01-..."
    artifacts:
      work_summary: "./worklogs/YYYY-MM-DD-S01-work-summary.md"
      commit_log: "./worklogs/YYYY-MM-DD-S01-commit-log.md"
      notes: "./worklogs/YYYY-MM-DD-S01-notes.md"
      codereview: null
    metrics:
      lines_added: 986
      lines_deleted: 0
      files_modified: 6
      tests_added: 0
      test_pass_rate: null

metrics:
  estimated_hours: 32
  actual_hours: 8.3
  lines_added: 4180
  lines_deleted: 890
  files_modified: 33
  tests_added: 17
  test_pass_rate: 1.0
  test_coverage: 0.972
```

### 2. Session Worklogs

**Created by**: `create-session-worklog.sh T-XX-XX NN "Description" "Developer"`

**Naming Pattern**: `{date}-S{NN}-{type}.md`

#### Work Summary Template

**File**: `{date}-S{NN}-work-summary.md`

**Sections**:

- Session Overview (ID, date, phase, description)
- Objectives
- Work Completed (by thread)
- Deliverables
- Metrics (time, lines, files, tests)
- Next Steps

#### Commit Log Template

**File**: `{date}-S{NN}-commit-log.md`

**Format**:

```markdown
# Commit Log - T-XX-XX Session NN

## Session Info
- Date: YYYY-MM-DD
- Session: NN
- Phase: Phase N
- Threads: 01-MM

## Commits

### Thread 01: [Description]
- **Hash**: `abc123`
- **Message**: feat(scope): description
- **Files**: 5 modified (+100/-20)
- **Notes**: spec-tasks/T-XX-XX: Session NN Thread 01 - description

### Thread 02: [Description]
...
```

#### Technical Notes Template

**File**: `{date}-S{NN}-notes.md`

**Sections**:

- Session Context
- Technical Decisions
- Implementation Details
- Code Patterns
- Issues Encountered
- Solutions Applied
- References

#### Code Review Template (Optional)

**File**: `{date}-S{NN}-codereview.md`

**Sections**:

- Review Scope
- Code Quality
- Test Coverage
- Performance Considerations
- Security Considerations
- Recommendations

### 3. RAW Session Log

**Created by**: `create-raw-log.sh T-XX-XX NN START END "Description"`

**File**: `worklogs/raw/WORK-SESSIONS-{NN}-THREADS-{START}-{END}-SUMMARY-{date}.txt`

**Purpose**: Complete session narrative for AI handoff

**Format**:

```text
CDSAgent Task T-XX-XX Session NN Summary
========================================

SESSION METADATA
- Session ID: NN
- Date: YYYY-MM-DD
- Phase: Phase N
- Threads: START-END
- Duration: HH:MM-HH:MM (X.Xh total)
- Developer: Name

THREAD BREAKDOWN

Thread 01 (HH:MM-HH:MM, XXmin): [Description]
[Complete narrative of thread work]

Thread 02 (HH:MM-HH:MM, XXmin): [Description]
[Complete narrative of thread work]

...

SESSION DELIVERABLES
[List of concrete outputs]

METRICS
[Session statistics]

NEXT STEPS
[Follow-up actions]
```

## Template Placeholders

Common placeholders in templates:

- `{TASK_ID}` → T-XX-XX-task-name
- `{SESSION_ID}` → NN (e.g., 05)
- `{DATE}` → YYYY-MM-DD
- `{PHASE}` → Phase 0, Phase 1, etc.
- `{DESCRIPTION}` → Brief description
- `{DEVELOPER}` → Developer/AI name
- `{START_TIME}` → HH:MM
- `{END_TIME}` → HH:MM
- `{DURATION}` → X.Xh
- `{THREADS_COUNT}` → Number of threads
- `{THREADS_RANGE}` → 01-NN

## Template Naming Conventions

### Session-Specific Files

**Pattern**: `{date}-S{NN}-{type}.md`

Examples:

- `2025-11-02-S05-work-summary.md` - Session 05 on Nov 2
- `2025-11-02-S05-notes.md`
- `2025-11-02-S05-commit-log.md`

### RAW Log Files

**Pattern**: `WORK-SESSIONS-{NN}-THREADS-{START}-{END}-SUMMARY-{date}.txt`

Examples:

- `WORK-SESSIONS-01-THREADS-01-03-SUMMARY-2025-10-31.txt`
- `WORK-SESSIONS-04-THREADS-01-07-SUMMARY-2025-11-01.txt`

## Template Validation

Check template compliance:

```shell
# Verify metadata.yaml structure
grep -A 5 "task:" .artifacts/spec-tasks-T-XX-XX/metadata.yaml

# Check session artifacts exist
ls .artifacts/spec-tasks-T-XX-XX/worklogs/{date}-S{NN}-*

# Validate RAW log naming
ls .artifacts/spec-tasks-T-XX-XX/worklogs/raw/WORK-SESSIONS-*
```

## Best Practices

1. **Session Numbers**: Sequential across all days (01, 02, 03...)
2. **Thread Numbers**: Reset to 01 for each new session
3. **File Naming**: Always use session-specific pattern
4. **Metadata Updates**: Update after each session
5. **RAW Logs**: Create immediately after session ends
6. **Completeness**: Fill all template sections

## Common Template Issues

### Wrong Session Numbering

**Problem**: Session numbers not sequential

**Fix**: Check `metadata.yaml` `sessions:` array for last session ID

### Missing Placeholders

**Problem**: Template placeholders not replaced

**Fix**: Manually replace `{PLACEHOLDER}` with actual values

### Inconsistent Naming

**Problem**: Files don't follow session-specific pattern

**Fix**: Rename to `{date}-S{NN}-{type}.md` format

### Incomplete metadata.yaml

**Problem**: Missing required fields

**Fix**: Compare with `.dev/templates/metadata.template.yaml`

## Scripts That Use Templates

- `create-session-worklog.sh` → Uses work-summary, commit-log, notes templates
- `create-raw-log.sh` → Uses raw-session template
- `create-task-worklog.sh` → Uses metadata template

## Related Skills

- **session-management**: Creates session artifacts from templates
- **task-initialization**: Creates initial metadata from template
- **git-workflow-validation**: Validates template-generated artifacts

## Template Customization

Templates can be customized per task by creating task-specific overrides:

```text
.artifacts/spec-tasks-T-XX-XX/
├── metadata.yaml              # From template
├── CLAUDE.md                  # Task-specific guide (optional)
└── worklogs/
    └── {session-files}        # From templates
```

## Exit Codes

Template scripts follow standard exit codes:

- `0`: Template applied successfully
- `1`: Template application failed
- `2`: Warnings, review template output

## References

- Template Documentation: `.dev/templates/README.md`
- Metadata Template: `.dev/templates/metadata.template.yaml`
- Worklog Templates: `.dev/templates/worklogs/`
- Session Lifecycle: `.dev/workflows/WORKLOG-HANDBOOK.md`
- Main SOP: `.dev/workflows/WORKTREE_WORKFLOW.md`
