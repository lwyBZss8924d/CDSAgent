# Development Process Documentation

**Version**: 1.1  
**Last Updated**: 2025-11-02 UTC
**Purpose**: Entry point for all CDSAgent development workflows, tools, and templates

---

## CONSTITUTION Documents

These foundational documents define the CDSAgent development process and serve as the authoritative reference:

- **[WORKTREE_WORKFLOW.md](.dev/workflows/WORKTREE_WORKFLOW.md)** - Main SOP: Complete Spec-Tasks DEV-COOKING Workflow (Standard Operating Procedure)
- **[RFC-DEV-TOOLS.md](.dev/tools/RFC-DEV-TOOLS.md)** - Dev Toolkit Architecture: Unified development tools design, script reference, and usage patterns
- **[WORK_SESSION_CHECKPOINT_WORKFLOW.md](.dev/workflows/WORK_SESSION_CHECKPOINT_WORKFLOW.md)** - Checkpoint Process: End-of-session review and artifact consistency workflow
- **[WORKLOG-HANDBOOK.md](.dev/workflows/WORKLOG-HANDBOOK.md)** - Session Lifecycle: Worklog management, RAW logs, and checkpoint practices

> **Note**: For detailed script usage and architecture decisions, see [RFC-DEV-TOOLS.md](.dev/tools/RFC-DEV-TOOLS.md) (Dev Toolkit reference).

---

## Quick Navigation

### 🔄 Workflows

- **[Next Task Selection](.dev/workflows/NEXT_TASK_CHECKLIST.md)** - Choose which task to start
- **[Worktree Workflow](.dev/workflows/WORKTREE_WORKFLOW.md)** - Complete task development SOP
- **[Session Initialization](.dev/workflows/SESSION_INITIALIZATION_WORKFLOW.md)** - Start new work sessions
- **[Worklog Handbook](.dev/workflows/WORKLOG-HANDBOOK.md)** - Session lifecycle reference
- **[Checkpoint Workflow](.dev/workflows/WORK_SESSION_CHECKPOINT_WORKFLOW.md)** - End-of-session review
- **[Checkpoint Guides](.dev/workflows/checkpoint/)** - 11 detailed checkpoint chapters

### 🛠️ Dev Toolkit

- **[Development Tools RFC](.dev/tools/RFC-DEV-TOOLS.md)** - **Dev Toolkit Reference**: Architecture, design decisions, and complete script documentation
- **[Session Scripts](.dev/scripts/session/)** - create-session-worklog.sh, create-raw-log.sh
- **[Task Scripts](.dev/scripts/task/)** - create-task-worklog.sh, worktree management
- **[Validation Scripts](.dev/scripts/validation/)** - checkpoint-helper.sh, git-notes-check.sh

> **Dev Toolkit**: See [RFC-DEV-TOOLS.md](.dev/tools/RFC-DEV-TOOLS.md) for authoritative script signatures, usage patterns, exit codes, and troubleshooting.

### 📝 Templates

- **[Template System](.dev/templates/README.md)** - Complete template documentation
- **[Metadata Template](.dev/templates/metadata.template.yaml)** - Task metadata structure
- **[Worklog Templates](.dev/templates/worklogs/)** - Session artifacts (work-summary, commit-log, notes, codereview, RAW log)

---

## Getting Started

### For New Developers

1. **Read**: [WORKTREE_WORKFLOW.md](.dev/workflows/WORKTREE_WORKFLOW.md) - Understand the complete development process
2. **Select Task**: Use [NEXT_TASK_CHECKLIST.md](.dev/workflows/NEXT_TASK_CHECKLIST.md) to choose your first task
3. **Initialize**: Run `./.dev/scripts/task/create-task-worklog.sh T-XX-XX "Task Title" "Your Name"` to set up task artifacts
4. **Start Session**: Use `./.dev/scripts/session/create-session-worklog.sh T-XX-XX NN "Description" "Your Name"` for each work session
5. **Checkpoint**: Follow [WORK_SESSION_CHECKPOINT_WORKFLOW.md](.dev/workflows/WORK_SESSION_CHECKPOINT_WORKFLOW.md) after each session

### For Returning Developers

**Starting a new session**:

```shell
# From worktree
cd ~/dev-space/CDSAgent-T-XX-XX-task-name

# Initialize session (automated)
./.dev/scripts/session/create-session-worklog.sh T-XX-XX 05 "Session Description" "Developer Name"

# Begin development
# ... work through threads ...

# End session - create RAW log
./.dev/scripts/session/create-raw-log.sh T-XX-XX 05 01 07 "Session Description"

# Run checkpoint validation
./.dev/scripts/validation/checkpoint-helper.sh T-XX-XX

# Follow checkpoint workflow
# See: .dev/workflows/WORK_SESSION_CHECKPOINT_WORKFLOW.md
```

---

## Directory Structure

```tree
.dev/
├── README.md                           # This file
│
├── workflows/                          # Development workflows (15 files)
│   ├── NEXT_TASK_CHECKLIST.md          # Task selection guide
│   ├── WORKTREE_WORKFLOW.md            # Complete task development SOP
│   ├── SESSION_INITIALIZATION_WORKFLOW.md  # Session start guide
│   ├── WORK_SESSION_CHECKPOINT_WORKFLOW.md  # Checkpoint index
│   └── checkpoint/                     # Checkpoint guides (11 chapters)
│       ├── 01-overview.md
│       ├── 02-phases-overview.md
│       ├── 03-phase1-review.md
│       ├── 04-phase2-verification.md
│       ├── 05-phase3-update.md
│       ├── 06-phase4-git.md
│       ├── 07-phase5-final.md
│       ├── 08-common-issues.md
│       ├── 09-template.md
│       ├── 10-example.md
│       └── 11-commands.md
│
├── scripts/                            # Development automation (8 scripts)
│   ├── session/                        # Session-level scripts (2)
│   │   ├── create-session-worklog.sh   # Initialize session artifacts (4 files)
│   │   └── create-raw-log.sh           # Create RAW log template (after session)
│   ├── task/                           # Task-level scripts (4)
│   │   ├── create-task-worklog.sh      # Initialize task artifacts (one-time)
│   │   ├── create-daily-worklog.sh     # Create daily worklogs (legacy)
│   │   ├── sync-worktrees.sh           # Sync all worktrees with main
│   │   └── worktree-symlink.sh         # Manage symlinks (create/list/remove)
│   └── validation/                     # Pre-checkpoint validation (2)
│       ├── checkpoint-helper.sh        # Pre-flight checks (5 validations)
│       └── git-notes-check.sh          # Verify git notes (all commits)
│
├── templates/                          # Artifact templates
│   ├── README.md                       # Template documentation
│   ├── metadata.template.yaml          # Task metadata structure
│   └── worklogs/                       # Session artifact templates
│       ├── work-summary.template.md
│       ├── commit-log.template.md
│       ├── notes.template.md
│       ├── codereview.template.md
│       └── raw-session.template.txt
│
└── tools/                              # Development tools
    └── RFC-DEV-TOOLS.md                # Dev Toolkit: Architecture, design, script reference
```

---

## Workflow Integration

### Complete Development Flow

```text
┌─────────────────────────────────────────────────────────────────┐
│                    TASK DEVELOPMENT LIFECYCLE                   │
└─────────────────────────────────────────────────────────────────┘
         │
         ↓
    [1] Select Next Task
         └─→ NEXT_TASK_CHECKLIST.md
         │
         ↓
    [2] Initialize Task Environment
         └─→ WORKTREE_WORKFLOW.md (Phase 1)
         └─→ create-task-worklog.sh
         │
         ↓
┌────────────────────────────────────────────────────────────────┐
│                      SESSION LOOP (Repeats)                    │
├────────────────────────────────────────────────────────────────┤
│   [3] Start New Session                                        │
│        └─→ SESSION_INITIALIZATION_WORKFLOW.md                  │
│        └─→ create-session-worklog.sh                           │
│        │                                                       │
│        ↓                                                       │
│   [4] Development Work (Threads 01-NN)                         │
│        └─→ WORKTREE_WORKFLOW.md (Phase 2)                      │
│        │                                                       │
│        ↓                                                       │
│   [5] End Session - Create RAW Log                             │
│        └─→ create-raw-log.sh                                   │
│        │                                                       │
│        ↓                                                       │
│   [6] Run Checkpoint                                           │
│        └─→ checkpoint-helper.sh (validation)                   │
│        └─→ WORK_SESSION_CHECKPOINT_WORKFLOW.md (5 phases)      │
│        │                                                       │
│        └─→ Repeat or Exit Loop                                 │
└────────────────────────────────────────────────────────────────┘
         │
         ↓
    [7] Complete Task
         └─→ WORKTREE_WORKFLOW.md (Phase 3)
         └─→ Create PR, merge, cleanup
```

---

## Key Concepts

### Hierarchical Work Structure

```text
Task → Phase → Day → Session → Thread
```

- **Task**: High-level work unit (T-02-02-sparse-index) - tracked in TODO.yaml
- **Phase**: Implementation stage (Phase 0: Planning, Phase 1: Upper Index, etc.)
- **Day**: Calendar date (2025-11-02)
- **Session**: Focused work period (1-8 hours) - numbered sequentially: 01, 02, 03...
- **Thread**: Work unit within session (30min-2h) - resets to 01 each session

### Session-Based Development

**CDSAgent uses session-based development, not daily development**:

- Multiple sessions can occur on same day (e.g., Sessions 01-03 on 2025-10-31)
- Sessions numbered sequentially across all days: 01, 02, 03, 04, 05...
- Thread numbers reset to 01 for each new session
- File naming: `{date}-S{NN}-work-summary.md` (session-specific)

### Key Files & Artifacts

**Per Task**:

- `.artifacts/spec-tasks-T-XX-XX/metadata.yaml` - Central task metadata
- `.artifacts/spec-tasks-T-XX-XX/CLAUDE.md` - Task-specific AI guide (optional)
- `.artifacts/spec-tasks-T-XX-XX/WORKLOG-HANDBOOK.md` - Session lifecycle reference (optional)

**Per Session**:

- `{date}-S{NN}-work-summary.md` - Session objectives & accomplishments
- `{date}-S{NN}-commit-log.md` - Git commits with context
- `{date}-S{NN}-notes.md` - Technical notes & decisions
- `{date}-S{NN}-codereview.md` - Testing & code review results (optional)
- `worklogs/raw/WORK-SESSIONS-{NN}-THREADS-{START}-{END}-SUMMARY-{date}.txt` - Complete session narrative for AI handoff

---

## Common Tasks

### Initialize New Task

```shell
cd ~/dev-space/CDSAgent
git checkout main && git pull origin main

# Create worktree (if not exists)
git worktree add .worktrees/T-XX-XX-task-name -b feat/task/T-XX-XX-task-name main

# Create symlinks for all worktrees (from main repo)
./.dev/scripts/task/worktree-symlink.sh create

# Initialize artifacts (from worktree)
cd ~/dev-space/CDSAgent-T-XX-XX-task-name
./.dev/scripts/task/create-task-worklog.sh T-XX-XX-task-name "Task Title" "Your Name"
```

### Start New Session

```shell
cd ~/dev-space/CDSAgent-T-XX-XX-task-name

# Determine next session number (check existing files)
ls .artifacts/spec-tasks-T-XX-XX/worklogs/ | grep -E "S[0-9]{2}" | sort | tail -1

# Create session artifacts
./.dev/scripts/session/create-session-worklog.sh T-XX-XX-task-name 05 "Session Description" "Developer Name"

# Fill out session objectives in work-summary.md
vim .artifacts/spec-tasks-T-XX-XX/worklogs/$(date +%Y-%m-%d)-S05-work-summary.md
```

### Complete Session & Checkpoint

```shell
# After development work completes

# 1. Create RAW log
./.dev/scripts/session/create-raw-log.sh T-XX-XX-task-name 05 01 07 "Session Description"

# 2. Fill out RAW log content
vim .artifacts/spec-tasks-T-XX-XX/worklogs/raw/WORK-SESSIONS-05-THREADS-01-07-SUMMARY-$(date +%Y-%m-%d).txt

# 3. Run pre-checkpoint validation
./.dev/scripts/validation/checkpoint-helper.sh T-XX-XX-task-name

# 4. Follow checkpoint workflow
# See: .dev/workflows/WORK_SESSION_CHECKPOINT_WORKFLOW.md
```

### Verify Git Notes

```shell
# Before pushing commits
./.dev/scripts/validation/git-notes-check.sh

# Add missing notes
git notes add <commit-hash> -m "spec-tasks/T-XX-XX
Day: X
Date: YYYY-MM-DD
Sessions: XX (Threads XX-XX)
Duration: X.Xh
Worklog: .artifacts/spec-tasks-T-XX-XX/worklogs/YYYY-MM-DD-*
Status: ...
Files: N files (±NNN lines)"

# Push notes
git push origin refs/notes/commits
```

---

## Troubleshooting

### Artifacts Not Visible in Worktree

**Problem**: `.artifacts/spec-tasks-T-XX-XX/` exists in main but not in worktree

**Solution**: Always run scripts FROM worktree using relative paths:

```shell
cd ~/dev-space/CDSAgent-T-XX-XX-task-name  # ✅ CORRECT
./.dev/scripts/task/create-task-worklog.sh ...
```

> **See**: [RFC-DEV-TOOLS.md](.dev/tools/RFC-DEV-TOOLS.md) for script usage patterns and path conventions.

### Session Number Confusion

**Problem**: What session number should I use next?

**Solution**: Check existing files:

```shell
ls .artifacts/spec-tasks-T-XX-XX/worklogs/ | grep -E "S[0-9]{2}" | sort | tail -1
# Shows: 2025-11-01-S04-work-summary.md
# Next session: 05
```

### Missing Git Notes

**Problem**: `checkpoint-helper.sh` fails with "commits missing notes"

**Solution**: Run verification and add notes:

```shell
./.dev/scripts/validation/git-notes-check.sh
# Follow instructions to add notes to each commit
```

> **See**: [RFC-DEV-TOOLS.md](.dev/tools/RFC-DEV-TOOLS.md) Appendix B for detailed error solutions and [checkpoint/06-phase4-git.md](.dev/workflows/checkpoint/06-phase4-git.md) for git notes workflow.

---

## Related Documentation

### Business & Technical Docs

Located in `docs/`:

- `docs/api/` - API specifications, error codes
- `docs/parity-validation-methodology.md` - LocAgent parity methodology

### Project Management

Located in `spacs/`:

- `spacs/tasks/0.1.0-mvp/TODO.yaml` - Central task registry
- `spacs/tasks/0.1.0-mvp/README.md` - Milestone overview
- `spacs/prd/` - Product requirements documents
- `spacs/issues/` - Technical specifications
- `spacs/tasks/` - Implementation task specs

---

## Version History

- **v1.1** (2025-11-02): Aligned with RFC-DEV-TOOLS.md v1.1, added CONSTITUTION documents section, fixed script examples, enhanced Dev Toolkit references
- **v1.0** (2025-11-02): Initial .dev/ directory creation with all workflow docs, scripts, and templates

---

**Maintainer**: CDSAgent Tech Lead  
**Feedback**: Create issue at [CDSAgent/issues](https://github.com/lwyBZss8924d/CDSAgent/issues)

---

END OF DEVELOPMENT PROCESS DOCUMENTATION
