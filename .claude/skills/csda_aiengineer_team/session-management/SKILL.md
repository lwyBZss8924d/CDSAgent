---
name: session-management
description: Manages CDSAgent development work sessions including session initialization, RAW log creation, and checkpoint workflows. Use when starting a new work session, completing a work session, ending development threads, creating session documentation, preparing session handoff, or initializing daily worklogs for T-XX-XX tasks. Critical for session numbering, thread tracking, and AI agent continuity.
allowed-tools: Bash, Edit, Read, Write, Grep, Glob, Task, TodoWrite, SlashCommand, WebSearch, WebFetch
---

# CDSAgent Session Management

Manages work session lifecycle for CDSAgent spec-tasks development.

**Important⚠️**: Any [session-management] job changed any PMP and docs & metadata .yaml MUST run (date -u '+%Y-%m-%dT%H:%M:%SZ') get UTC TIME NOW first!!!

## Quick Start

### Starting a New Session (Most Common)

```shell
# From task worktree
./.dev/scripts/session/create-session-worklog.sh T-XX-XX NN "Session Description" "Developer Name"

# Example
./.dev/scripts/session/create-session-worklog.sh T-02-02 05 "Phase 3 BM25 Integration" "Claude Code Agent"
```

**Creates**: work-summary.md, notes.md, commit-log.md for session

### Ending a Session

```shell
# Create RAW log after completing threads
./.dev/scripts/session/create-raw-log.sh T-XX-XX NN START END "Description"

# Example
./.dev/scripts/session/create-raw-log.sh T-02-02 05 01 04 "BM25 integration complete"
```

**Creates**: RAW log file for AI handoff

### Session Numbering Pattern

- Sessions: Sequential across all days (01, 02, 03, 04, 05...)
- Threads: Reset to 01 for each new session
- File naming: `{date}-S{NN}-{type}.md`

**Examples**:

- `2025-11-02-S05-work-summary.md` (Session 05)
- `2025-11-02-S06-notes.md` (Session 06, same day)

## How to Use

### Starting a New Session

When beginning a new work session for a task:

```shell
# Navigate to task worktree
cd ~/dev-space/CDSAgent-T-XX-XX-task-name

# Create session worklog (from worktree, using relative path)
./.dev/scripts/session/create-session-worklog.sh T-XX-XX NN "Session Description" "Developer Name"
```

**Parameters**:

- `T-XX-XX`: Task ID (e.g., T-02-02-sparse-index)
- `NN`: Session number (e.g., 05)
- Description: Brief session purpose
- Developer: Name of developer/AI agent

**Creates**:

- `.artifacts/spec-tasks-T-XX-XX/worklogs/{date}-S{NN}-work-summary.md`
- `.artifacts/spec-tasks-T-XX-XX/worklogs/{date}-S{NN}-notes.md`
- `.artifacts/spec-tasks-T-XX-XX/worklogs/{date}-S{NN}-commit-log.md`

### Ending a Task's-Phase Session

After completing work threads, create RAW log for AI handoff:

```shell
# Create RAW log (from worktree, using relative path)
./.dev/scripts/session/create-raw-log.sh T-XX-XX NN START END "Description"
```

**Parameters**:

- `T-XX-XX`: Task ID
- `NN`: Session number
- `START`: First thread number (e.g., 01)
- `END`: Last thread number (e.g., 07)
- Description: Summary of session work

**Creates**:

- `.artifacts/spec-tasks-T-XX-XX/worklogs/raw/WORK-SESSIONS-{NN}-THREADS-{START}-{END}-SUMMARY-{date}.txt`

### Running Checkpoint

Execute checkpoint workflow to validate session artifacts:

```shell
# From worktree, using relative path
./.dev/scripts/validation/checkpoint-helper.sh T-XX-XX
```

Follow the checkpoint workflow guide in `.dev/workflows/WORK_SESSION_CHECKPOINT_WORKFLOW.md`.

## Session Numbering Rules

- Sessions numbered sequentially: 01, 02, 03, 04, 05...
- Multiple sessions per day are normal
- Thread numbers reset to 01 for each new session
- Example: Same day can have Sessions 01-03 with different thread ranges

## Workflow Integration

1. **Start Session**: Create session worklogs
2. **Work Threads**: Execute development work (Threads 01-NN)
3. **End Session**: Create RAW log
4. **Checkpoint**: Run checkpoint helper
5. **Git Push**: Validate with git-notes-check before push

## Scripts Location

All session scripts located in `.dev/scripts/session/`:

- `create-session-worklog.sh` - Initialize new session
- `create-raw-log.sh` - Create session summary for handoff

Always run scripts FROM worktree using relative paths: `./.dev/scripts/...`

## Best Practices

1. **Session Numbering**: Check `.artifacts/spec-tasks-T-XX-XX/metadata.yaml` for last session number
2. **Thread Tracking**: Track threads in RAW log file name for continuity
3. **Description Clarity**: Use descriptive session names for easy navigation
4. **Immediate RAW Log**: Create RAW log immediately after session ends while context is fresh
5. **Checkpoint Before Push**: Always run checkpoint before committing artifacts

## Related Skills

- **task-initialization**: Create new task worktrees and initial metadata
- **git-workflow-validation**: Validate git notes and checkpoint status
- **template-usage**: Understand worklog template structure

## Exit Codes

- `0`: Success, proceed
- `1`: Failure, fix and retry
- `2`: Warnings, review and decide

## Common Issues

**Session number confusion**: Check metadata.yaml `sessions:` array for last session ID
**Missing artifacts**: Ensure you're in correct worktree directory
**Script not found**: Use relative path `./.dev/scripts/...` from worktree

## References

- Main SOP: `.dev/workflows/WORKTREE_WORKFLOW.md`
- Session Lifecycle: `.dev/workflows/WORKLOG-HANDBOOK.md`
- Checkpoint Guide: `.dev/workflows/WORK_SESSION_CHECKPOINT_WORKFLOW.md`
- Template Docs: `.dev/templates/README.md`
