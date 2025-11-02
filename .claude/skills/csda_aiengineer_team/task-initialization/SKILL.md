---
name: task-initialization
description: Initializes new CDSAgent spec-tasks including worktree creation, metadata setup, and initial documentation structure. Use when starting work on a new task from the TODO.yaml PMP or transitioning to a new development task.
allowed-tools: Bash, Edit, Read, Write, Grep, Glob, Task, TodoWrite, SlashCommand, WebSearch, WebFetch
---

# CDSAgent Task Initialization

Complete task initialization workflow for CDSAgent spec-tasks development.

**Important⚠️**: Wen Run [task-initialization] job changed any PMP and docs & metadata .yaml MUST run (date -u '+%Y-%m-%dT%H:%M:%SZ') get UTC TIME NOW first!!!

## Capabilities

- Create dedicated git worktrees for tasks
- Initialize task metadata.yaml from template
- Setup symlinks for convenient access
- Create initial worklog structure
- Validate task dependencies and prerequisites

## Task Structure Overview

Each task follows hierarchical specification flow:

```text
PRDs (spacs/prd/)           → What to build
    ↓
Issues (spacs/issues/)      → How to build
    ↓
Tasks (spacs/tasks/)        → Concrete work
    ↓
metadata.yaml (.artifacts/) → In-progress tracking
```

## How to Use

### 1. Select Task from PMP

Review the Project Management Plan:

```shell
# View task list
cat spacs/tasks/0.1.0-mvp/TODO.yaml

# Check task dependencies
grep -A 5 "T-XX-XX" spacs/tasks/0.1.0-mvp/TODO.yaml
```

Verify:

- Task `status: not_started` or `status: in_progress`
- Dependencies completed (check `requires:` field)
- No blockers in `blocked_tasks:` list

### 2. Create Task Worktree

From main repository:

```shell
# Create worktree
git worktree add .worktrees/T-XX-XX-task-name feat/task/T-XX-XX-task-name

# Navigate to worktree
cd .worktrees/T-XX-XX-task-name
```

**Naming Convention**:

- Directory: `T-XX-XX-task-name` (lowercase, hyphenated)
- Branch: `feat/task/T-XX-XX-task-name`

### 3. Create Symlink

For convenient access:

```shell
# From worktree directory
./.dev/scripts/task/worktree-symlink.sh create

# This creates: ~/dev-space/CDSAgent-T-XX-XX-task-name -> .worktrees/T-XX-XX-task-name
```

### 4. Initialize Task Metadata

```shell
# From worktree
./.dev/scripts/task/create-task-worklog.sh T-XX-XX "Task Title"
```

**Creates**:

- `.artifacts/spec-tasks-T-XX-XX/metadata.yaml` (from template)
- `.artifacts/spec-tasks-T-XX-XX/worklogs/` (directory)
- `.artifacts/spec-tasks-T-XX-XX/CLAUDE.md` (optional task-specific guide)

### 5. Update Metadata

Edit `.artifacts/spec-tasks-T-XX-XX/metadata.yaml`:

```yaml
task:
  id: T-XX-XX-task-name
  title: "Task Title from TODO.yaml"
  owner: "Developer Name"
  status: in_progress
  start_date: "2025-11-02"

specs:
  prds:
    - spacs/prd/0.1.0-MVP-PRDs-v0/XX-component.md
  issues:
    - spacs/issues/04-0.1.0-mvp/XX-component/XX-issue.md
  tasks:
    - spacs/tasks/0.1.0-mvp/XX-component/T-XX-XX-task-name.md

git:
  worktree: .worktrees/T-XX-XX-task-name
  branch: feat/task/T-XX-XX-task-name
  base_commit: <current-commit-hash>

dependencies:
  requires:
    - T-YY-YY-prerequisite  # From TODO.yaml
  blocks:
    - T-ZZ-ZZ-dependent  # From TODO.yaml
```

### 6. Verify Setup

Check initialization:

```shell
# Verify worktree
git worktree list | grep T-XX-XX

# Verify symlink
ls -l ~/dev-space/ | grep CDSAgent-T-XX-XX

# Verify artifacts
ls -la .artifacts/spec-tasks-T-XX-XX/

# Verify .dev/ access
ls -la .dev/
```

## Initialization Checklist

- [ ] Task selected from TODO.yaml
- [ ] Dependencies verified as completed
- [ ] Worktree created at `.worktrees/T-XX-XX-task-name`
- [ ] Branch created: `feat/task/T-XX-XX-task-name`
- [ ] Symlink created: `~/dev-space/CDSAgent-T-XX-XX-task-name`
- [ ] metadata.yaml initialized with correct specs
- [ ] .dev/ directory accessible (symlink or scripts present)
- [ ] First session ready to start

## Task Naming Convention

**Format**: `T-{COMPONENT}-{NUMBER}-{description}`

Examples:

- `T-02-01-graph-builder` (Component 02, Task 01)
- `T-05-02-typescript-bindings` (Component 05, Task 02)

**Components**:

- `02`: Index Core
- `03`: CLI Tools
- `04`: Agent Integration
- `05`: API Contracts
- `06`: Parity Validation
- `07`: Deployment
- `08`: Testing

## Directory Structure After Init

```text
.worktrees/T-XX-XX-task-name/
├── .dev/                                  # Workflows, scripts, templates
├── .artifacts/
│   └── spec-tasks-T-XX-XX/
│       ├── metadata.yaml
│       ├── CLAUDE.md                       # Task-specific guide
│       └── worklogs/
│           └── raw/
├── spacs/                                  # Specs (PRDs, Issues, Tasks)
├── crates/                                 # Rust code
└── <other-files>

~/dev-space/CDSAgent-T-XX-XX → .worktrees/T-XX-XX-task-name  # Symlink
```

## Scripts Location

Task initialization scripts in `.dev/scripts/task/`:

- `create-task-worklog.sh` - Initialize task metadata
- `worktree-symlink.sh` - Manage worktree symlinks
- `sync-worktrees.sh` - Sync artifacts across worktrees

## Common Issues

**Worktree already exists**: `git worktree remove` old worktree first
**Symlink broken**: Re-run `worktree-symlink.sh create`
**metadata.yaml missing fields**: Check template at `.dev/templates/metadata.template.yaml`
**Wrong dependencies**: Cross-reference with `spacs/tasks/0.1.0-mvp/TODO.yaml`

## Related Skills

- **session-management**: Start first session after initialization
- **worktree-management**: Advanced worktree operations
- **template-usage**: Understand metadata.yaml structure

## Exit Codes

- `0`: Success, ready to start first session
- `1`: Failure, fix errors
- `2`: Warnings, verify before proceeding

## Next Steps

After task initialization:

1. **Start First Session**: Use `session-management` skill
2. **Read Task Spec**: Review `spacs/tasks/0.1.0-mvp/XX-component/T-XX-XX-task-name.md`
3. **Review Dependencies**: Check prerequisite tasks completed
4. **Plan Implementation**: Create Phase 0 analysis if needed

## References

- Next Task Guide: `.dev/workflows/NEXT_TASK_CHECKLIST.md`
- Main SOP: `.dev/workflows/WORKTREE_WORKFLOW.md`
- Template Documentation: `.dev/templates/README.md`
- PMP: `spacs/tasks/0.1.0-mvp/TODO.yaml`
