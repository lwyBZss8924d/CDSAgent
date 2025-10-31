# Spec-Tasks DEV-COOKING Workflow - Standard Operating Procedure (SOP)

**Version**: 1.2
**Last Updated**: 2025-10-23
**Audience**: All CDSAgent Development Team Members

---

## Table of Contents

1. [Overview](#overview)
2. [Quick Start Guide](#quick-start-guide)
3. [Task Development Lifecycle](#task-development-lifecycle)
4. [Worklog Management](#worklog-management)
5. [Git Worktree Operations](#git-worktree-operations)
6. [Best Practices](#best-practices)
7. [Troubleshooting](#troubleshooting)
8. [Reference](#reference)

---

## Overview

This SOP defines the standard workflow for implementing Spec-Tasks in CDSAgent using git worktrees, worklog tracking, and the centralized TODO.yaml system.

### What is a Spec-Task?

A **Spec-Task** is a fully-specified development task with:

- **Task ID**: Unique identifier (e.g., `T-05-01-jsonrpc-schema`)
- **Specifications**: Linked PRDs, Issues, and Task documents
- **Git Worktree**: Isolated branch for development
- **Worklog**: Daily progress tracking
- **Metadata**: Tracked in TODO.yaml

### Workflow Benefits

- ✅ **Parallel Development**: Work on multiple tasks simultaneously
- ✅ **Isolated Environments**: Each worktree is a separate working directory
- ✅ **IDE-Friendly**: Use symlinks for easier navigation
- ✅ **Progress Tracking**: Daily worklogs and centralized metadata
- ✅ **No Context Switching**: Keep different branches checked out at once
- ✅ **Clean History**: Each task branch remains focused and organized
- ✅ **Audit Trail**: Complete development history per task

---

## QUICK DECISION FLOW

**Question: Which task should I start next?**

```text
START
  ↓
┌─────────────────────────────────────────┐
│ 1. Check Current Milestone              │
│    • spacs/tasks/0.1.0-mvp/README.md    │
│    • Current phase: M1, M2, M3, etc.?   │
└──────────────┬──────────────────────────┘
               ↓
┌─────────────────────────────────────────┐
│ 2. List Candidate Tasks                 │
│    • Status: "not_started"              │
│    • In current milestone               │
│    • yq '.milestones.MX.critical_path'  │
└──────────────┬──────────────────────────┘
               ↓
┌─────────────────────────────────────────┐
│ 3. Check Dependencies                   │
│    For each candidate:                  │
│    • yq '.tasks.T-XX-XX.dependencies'   │
│    • All 'requires' tasks MERGED?       │
└──────────────┬──────────────────────────┘
               ↓
┌─────────────────────────────────────────┐
│ 4. Apply Priority Filter                │
│    • P0 (critical) > P1 > P2            │
│    • On critical_path?                  │
│    • Deadline urgency?                  │
└──────────────┬──────────────────────────┘
               ↓
┌─────────────────────────────────────────┐
│ 5. Verify Prerequisites                 │
│    • gh pr list --state merged          │
│    • Sync main: git pull origin main    │
└──────────────┬──────────────────────────┘
               ↓
            SELECT TASK ✅
               ↓
       Go to "Task Development Lifecycle"
```

**Quick Commands:**

```shell
# Check current milestone
cat spacs/tasks/0.1.0-mvp/README.md | grep -A 10 "Milestone M"

# List ready tasks
yq '.tasks | to_entries | map(select(.value.status == "not_started"))' \
  spacs/tasks/0.1.0-mvp/TODO.yaml

# Check specific task dependencies
yq '.tasks.T-XX-XX.dependencies' spacs/tasks/0.1.0-mvp/TODO.yaml

# Verify PRs merged
gh pr list --state merged --limit 10
```

---

## Project Structure

### Repository Layout

```tree
CDSAgent/                                   # Main repository
├── .worktrees/                             # Git worktrees (gitignored)
│   ├── T-05-01-jsonrpc-schema/             # Task-specific worktree
│   ├── T-02-01-graph-builder/
│   └── ...
│
├── .artifacts/                             # Worklog & metadata
│   ├── spec-tasks-templates/               # Worklog templates
│   │   ├── README.md
│   │   ├── metadata.template.yaml
│   │   └── worklogs/
│   │       ├── work-summary.template.md
│   │       ├── commit-log.template.md
│   │       └── notes.template.md
│   │
│   └── spec-tasks-{TASK_ID}/               # Task worklogs
│       ├── metadata.yaml
│       ├── git-refs.txt
│       └── worklogs/
│           ├── YYYY-MM-DD-work-summary.md
│           ├── YYYY-MM-DD-commit-log.md
│           └── YYYY-MM-DD-notes.md
│
├── spacs/                                  # Specifications
│   ├── prd/0.1.0-MVP-PRDs-v0/              # PRD documents
│   ├── issues/04-0.1.0-mvp/                # Issue breakdown
│   └── tasks/0.1.0-mvp/
│       ├── README.md                       # Milestone overview
│       ├── TODO.yaml                       # Central task registry ⭐
│       └── {category}/
│           ├── README.md
│           └── T-XX-XX-{name}.md           # Task specification
│
├── scripts/                                # Automation
│   ├── worktree-symlink.sh                 # Worktree symlinks
│   ├── sync-worktrees.sh                   # Sync with main
│   ├── create-task-worklog.sh              # Init task worklog
│   └── create-daily-worklog.sh             # Daily worklog
│
└── docs/
    └── WORKTREE_WORKFLOW.md                # This document

# IDE-friendly symlinks (created by scripts)
~/dev-space/CDSAgent-T-05-01-jsonrpc-schema -> CDSAgent/.worktrees/T-05-01-jsonrpc-schema
~/dev-space/CDSAgent-T-02-01-graph-builder  -> CDSAgent/.worktrees/T-02-01-graph-builder
```

### Key Files

| File/Directory | Purpose |
|----------------|---------|
| `spacs/tasks/0.1.0-mvp/TODO.yaml` | **Central task registry** - All task metadata |
| `.artifacts/spec-tasks-{ID}/` | **Task worklogs** - Development history |
| `.worktrees/{TASK_ID}/` | **Git worktree** - Isolated development environment |
| `~/dev-space/CDSAgent-{ID}` | **Symlink** - IDE-friendly access |

---

## Quick Start Guide

### Prerequisites

- Git worktree infrastructure set up (already done in M0)
- Familiarity with git and your IDE
- Access to TODO.yaml and task specifications

### 5-Minute Quick Start

Example: Starting T-05-01 (JSON-RPC Schema)

```shell
# Step 1: Find your task in TODO.yaml
cat spacs/tasks/0.1.0-mvp/TODO.yaml | grep -A 20 "T-05-01"

# Step 2: Initialize task worklog
./scripts/create-task-worklog.sh T-05-01-jsonrpc-schema \
  "JSON-RPC Schema Definition & Validation" \
  "Your Name"

# Step 3: Create today's worklog
./scripts/create-daily-worklog.sh T-05-01-jsonrpc-schema

# Step 4: Navigate to task worktree
cd ~/dev-space/CDSAgent-T-05-01-jsonrpc-schema

# Step 5: Read task specification
cat spacs/tasks/0.1.0-mvp/05-api-contracts/T-05-01-jsonrpc-schema.md

# Step 6: Start coding!
code .  # or your preferred IDE
```

**That's it!** You're now ready to develop. Continue reading for the full workflow.

---

## Next Task Selection Procedure

**Purpose**: Systematically identify which Spec-Task to begin next based on dependencies, priorities, and milestone targets.

### Step 1: Review Current Milestone Status

**Check milestone completion**:

```shell
# View current milestone status
cat spacs/tasks/0.1.0-mvp/README.md | grep -A 30 "Milestone M"

# Check TODO.yaml milestone tracking
yq '.milestones' spacs/tasks/0.1.0-mvp/TODO.yaml
```

**Key questions**:

- Which milestone is currently active? (M0, M1, M2, M3, M4, M5)
- What is the target completion date?
- How many critical-path tasks remain?
- What is the completion percentage?

### Step 2: Query TODO.yaml for Candidate Tasks

**List all not-started tasks in current milestone**:

```shell
# Example: List M1 tasks
yq '.tasks | to_entries |
    map(select(.value.milestone == "M1" and .value.status == "not_started")) |
    .[].key' \
  spacs/tasks/0.1.0-mvp/TODO.yaml
```

**Filter by critical path**:

```shell
# Check critical-path tasks for current milestone
yq '.milestones.M1.critical_path' spacs/tasks/0.1.0-mvp/TODO.yaml
```

### Step 3: Verify Dependencies

**For each candidate task, check dependencies**:

```shell
# Check what a task requires
yq '.tasks.T-XX-XX.dependencies.requires' spacs/tasks/0.1.0-mvp/TODO.yaml

# Check what tasks this would unblock
yq '.tasks.T-XX-XX.dependencies.blocks' spacs/tasks/0.1.0-mvp/TODO.yaml
```

**⚠️ CRITICAL RULE**: All `requires` tasks must be **MERGED** (not just completed)

**Verify prerequisites merged**:

```shell
# Check recent merged PRs
gh pr list --state merged --limit 10

# Check specific task PR status
gh pr view <PR-number>
```

### Step 4: Apply Priority Filter

**Priority order**:

1. **P0 (Critical)** - Blocks other work, milestone deadlines
2. **P1 (Important)** - Core functionality, integration work
3. **P2 (Nice-to-have)** - Documentation, polish, extensions

**Additional considerations**:

- **Critical path**: Tasks on `milestone.critical_path` list
- **Deadline urgency**: Days until `milestone.target_date`
- **Parallel ready**: Tasks with no dependencies (can work in parallel with others)

**Example decision matrix**:

| Task | Priority | On Critical Path? | Dependencies Met? | Deadline | Score |
|------|----------|-------------------|-------------------|----------|-------|
| T-05-02 | P0 | Yes | ✅ T-05-01 merged | 4 days | ⭐⭐⭐ |
| T-06-01 | P0 | Yes | ✅ No deps | 4 days | ⭐⭐⭐ |
| T-05-03 | P1 | No | ✅ T-05-01 merged | 6 days | ⭐⭐ |
| T-02-01 | P0 | Yes | ❌ T-06-01 pending | 14 days | ⏸ BLOCKED |

### Step 5: Final Verification Checklist

**Before starting the selected task:**

- [ ] **Specs exist**: Task file in `spacs/tasks/0.1.0-mvp/` exists
- [ ] **Worktree ready**: Check if worktree already created (from M0)
- [ ] **Dependencies merged**: All `requires` tasks have merged PRs
- [ ] **Main synced**: Latest `git pull origin main` completed
- [ ] **Time available**: Sufficient time before milestone deadline
- [ ] **Resources available**: Dependencies (LocAgent, test data) accessible

### Common Scenarios

#### Scenario 1: Starting Fresh Milestone

**Situation**: Just completed M1, starting M2

**Steps**:

1. Update TODO.yaml: Mark M1 as `completed`
2. Review M2 critical-path tasks
3. Select first task with no dependencies (typically graph/index foundations)
4. Verify baseline data ready (if parity task)

#### Scenario 2: Parallel Development

**Situation**: Multiple developers, want to work in parallel

**Steps**:

1. Identify tasks with no mutual dependencies
2. Coordinate with team on Slack/GitHub
3. Each developer takes different critical-path task
4. Example: T-05-01 (API) + T-06-01 (Parity) can run in parallel

#### Scenario 3: Blocked by Dependencies

**Situation**: Want to start T-02-01, but T-06-01 not complete

**Steps**:

1. Check `TODO.yaml` for alternative tasks with dependencies met
2. Consider working on P1 tasks (docs, tests, tooling)
3. Update TODO.yaml: Set blocked task status to `blocked`
4. Add comment explaining blocker

**Example**:

```yaml
T-02-01-graph-builder:
  status: blocked
  comments:
    - date: "2025-10-23"
      author: "Developer"
      text: "Blocked on T-06-01 parity methodology - waiting for baseline data"
```

### DECISION FLOWCHART QUICK REFERENCE

```text
┌──────────────────────────────────┐
│   Review Milestone Status        │
└─────────┬────────────────────────┘
          ↓
┌──────────────────────────────────┐
│   List Not-Started Tasks         │
│   (current milestone)            │
└─────────┬────────────────────────┘
          ↓
┌──────────────────────────────────┐
│   Check Dependencies             │
│   All 'requires' merged?         │
└─────────┬────────────────────────┘
          ↓
    YES ┌─┴─┐ NO
        │   │
        ↓   └──→ Try other task or BLOCK
┌──────────────────────────────────┐
│   Apply Priority Filter          │
│   P0 > P1 > P2                   │
└─────────┬────────────────────────┘
          ↓
┌──────────────────────────────────┐
│   Verify Prerequisites           │
│   Main synced? Specs ready?      │
└─────────┬────────────────────────┘
          ↓
       SELECT ✅
          ↓
   Go to "Worktree Preparation"
```

---

## Task Development Lifecycle

### Phase 0: Task Assignment

**Before starting development:**

(1) **Check TODO.yaml** for task details:

```shell
# View task metadata
yq '.tasks.T-05-01-jsonrpc-schema' spacs/tasks/0.1.0-mvp/TODO.yaml
```

(2) **Verify dependencies** are complete:

```shell
# Check if prerequisite tasks are done
yq '.tasks.T-05-01-jsonrpc-schema.dependencies.requires' \
   spacs/tasks/0.1.0-mvp/TODO.yaml
```

(3) **Read specifications**:

- PRD: Product requirements
- Issue: Technical breakdown
- Task: Implementation details

```shell
# Example for T-05-01
cat spacs/prd/0.1.0-MVP-PRDs-v0/05-api-specifications.md
cat spacs/issues/04-0.1.0-mvp/05-api-contracts.md
cat spacs/tasks/0.1.0-mvp/05-api-contracts/T-05-01-jsonrpc-schema.md
```

### Phase 1: Worktree Environment Preparation

**⚠️ CRITICAL STEPS - Follow in order!**

#### Step 1: Sync Main Branch FIRST

> **ALWAYS sync main BEFORE creating worktree to get latest merged changes**

```shell
# 1. Navigate to main repository
cd ~/dev-space/CDSAgent

# 2. Switch to main
git checkout main

# 3. Pull latest changes (includes all merged PRs)
git pull origin main

# 4. Verify latest commit (should match GitHub main branch)
git rev-parse HEAD
git log -1 --oneline
```

#### Step 2: Create Worktree from Synced Main

```shell
# Create worktree branch from latest main
git worktree add .worktrees/T-XX-XX-task-name \
  -b feat/task/T-XX-XX-task-name main

# Example:
git worktree add .worktrees/T-05-02-typescript-bindings \
  -b feat/task/T-05-02-typescript-bindings main
```

#### Step 3: Create IDE-Friendly Symlink

```shell
# Create symlink for easier navigation
./scripts/worktree-symlink.sh create T-XX-XX-task-name

# Or manually:
ln -s $(pwd)/.worktrees/T-XX-XX-task-name \
  ~/dev-space/CDSAgent-T-XX-XX-task-name

# Verify symlink
ls -la ~/dev-space/CDSAgent-T-XX-XX-task-name
```

#### Step 4: Initialize Artifacts FROM WORKTREE

> **⚠️ CRITICAL: Worktree file system isolation issue**

**Problem**: Worktrees have **independent file snapshots** (different inodes). If you run `create-task-worklog.sh` from main repository AFTER creating the worktree, the artifacts will be created in main's `.artifacts/` directory but will NOT be visible in the worktree.

**Solution**: ALWAYS run artifact initialization FROM the worktree itself

```shell
# ✅ CORRECT: Run FROM worktree
cd ~/dev-space/CDSAgent-T-XX-XX-task-name

# Run script using absolute path
/Users/arthur/dev-space/CDSAgent/scripts/create-task-worklog.sh \
  T-XX-XX-task-name "Task Title" "Your Name"

# Verify artifacts created in worktree
ls -la .artifacts/spec-tasks-T-XX-XX-task-name/

# ❌ WRONG: Running from main after worktree creation
# cd ~/dev-space/CDSAgent  # DON'T DO THIS
# ./scripts/create-task-worklog.sh ...  # Artifacts won't appear in worktree!
```

**Why this happens**:

- Worktree snapshots main's file tree at creation time (inode: 310602118)
- Main repository continues on different file tree (inode: 313565709)
- Files created in main AFTER worktree creation don't exist in worktree's snapshot
- Solution: Create artifacts IN the worktree's file tree directly

#### Step 5: Troubleshooting - If Artifacts Missing

> **Symptoms**: `.artifacts/spec-tasks-T-XX-XX/` exists in main but not in worktree

**Three solutions** (in order of preference):

##### Option 1: Re-run from worktree (Recommended)

```shell
cd ~/dev-space/CDSAgent-T-XX-XX-task-name
/Users/arthur/dev-space/CDSAgent/scripts/create-task-worklog.sh \
  T-XX-XX-task-name "Task Title" "Your Name"
```

##### Option 2: Copy artifacts from main to worktree

```shell
# Copy from main's artifacts to worktree
cp -r /Users/arthur/dev-space/CDSAgent/.artifacts/spec-tasks-T-XX-XX-task-name \
  ~/dev-space/CDSAgent-T-XX-XX-task-name/.artifacts/

# Verify
ls -la ~/dev-space/CDSAgent-T-XX-XX-task-name/.artifacts/spec-tasks-T-XX-XX-task-name
```

##### Option 3: Delete and recreate worktree (Nuclear option)

```shell
# Remove worktree
cd ~/dev-space/CDSAgent
git worktree remove .worktrees/T-XX-XX-task-name
rm ~/dev-space/CDSAgent-T-XX-XX-task-name

# Recreate worktree
git worktree add .worktrees/T-XX-XX-task-name \
  -b feat/task/T-XX-XX-task-name main

# Create symlink
./scripts/worktree-symlink.sh create T-XX-XX-task-name

# Initialize artifacts FROM worktree
cd ~/dev-space/CDSAgent-T-XX-XX-task-name
/Users/arthur/dev-space/CDSAgent/scripts/create-task-worklog.sh \
  T-XX-XX-task-name "Task Title" "Your Name"
```

#### Step 6: Update Task-Specific CLAUDE.md (Optional but Recommended)

**Purpose**: Add task-specific development guidance for AI-assisted development

```shell
cd ~/dev-space/CDSAgent-T-XX-XX-task-name

# Edit CLAUDE.md to add task context
vim CLAUDE.md
```

**What to add**:

- Current task objective and deliverables
- Task-specific commands (type generation, testing, etc.)
- Implementation checklist (5-7 steps)
- API reference quick links
- Testing strategy

**See example**: `.worktrees/T-05-02-typescript-bindings/CLAUDE.md` (commit 0b4181d)

#### Step 7: Create Daily Worklog

```shell
# Navigate to worktree
cd ~/dev-space/CDSAgent-T-XX-XX-task-name

# Create today's worklog
/Users/arthur/dev-space/CDSAgent/scripts/create-daily-worklog.sh T-XX-XX-task-name

# Verify worklogs created
ls -la .artifacts/spec-tasks-T-XX-XX-task-name/worklogs/
```

#### Step 8: Verify Environment Ready

```shell
# Check branch
git branch --show-current  # Should show: feat/task/T-XX-XX-task-name

# Check artifacts exist
ls -la .artifacts/spec-tasks-T-XX-XX-task-name/metadata.yaml

# Read task specification
cat spacs/tasks/0.1.0-mvp/{category}/T-XX-XX-task-name.md

# Ready to code!
code .  # Or your preferred IDE
```

### Phase 2: Daily Development

**Start-of-day routine:**

```shell
# 1. Create today's worklog
cd ~/dev-space/CDSAgent
./scripts/create-daily-worklog.sh T-05-01-jsonrpc-schema

# 2. Navigate to task worktree
cd ~/dev-space/CDSAgent-T-05-01-jsonrpc-schema

# 3. Sync with main (if needed)
git fetch origin main
git rebase origin/main  # or: git merge origin/main

# 4. Fill out "Today's Objectives" in work-summary.md
DATE=$(date +%Y-%m-%d)
vim .artifacts/spec-tasks-T-05-01-jsonrpc-schema/worklogs/${DATE}-work-summary.md
```

**During development:**

```shell
# 1. Write code
# 2. Add tests
# 3. Run tests frequently
cargo test  # Rust
bun test    # TypeScript

# 4. Commit frequently with meaningful messages
git add path/to/files
git commit -m "feat(api): implement JSON-RPC request validation"

# 5. Update commit-log.md after each commit
DATE=$(date +%Y-%m-%d)
vim .artifacts/spec-tasks-T-05-01-jsonrpc-schema/worklogs/${DATE}-commit-log.md
```

**End-of-day routine:**

```shell
# 1. Push commits
git push origin feat/task/T-05-01-jsonrpc-schema

# 2. Fill out work-summary.md
DATE=$(date +%Y-%m-%d)
vim .artifacts/spec-tasks-T-05-01-jsonrpc-schema/worklogs/${DATE}-work-summary.md
# - Mark completed objectives
# - Document key decisions
# - Note blockers
# - List tomorrow's tasks

# 3. Update task metadata
vim .artifacts/spec-tasks-T-05-01-jsonrpc-schema/metadata.yaml
# - Add today's commits
# - Update metrics (hours, lines of code)
# - Update acceptance criteria status

# 4. Update TODO.yaml (optional, can batch update)
vim spacs/tasks/0.1.0-mvp/TODO.yaml
# - Update task status
# - Add git commits
```

### Phase 3: Task Completion

**When ready to submit:**

```shell
# 1. Run full test suite
cargo test --all
cargo clippy --all-targets
cargo build --release

# 2. Verify acceptance criteria
cat spacs/tasks/0.1.0-mvp/05-api-contracts/T-05-01-jsonrpc-schema.md
# Check all criteria in task spec

# 3. Update metadata.yaml
vim .artifacts/spec-tasks-T-05-01-jsonrpc-schema/metadata.yaml
# - Set status: "completed"
# - Record completion date
# - Final metrics

# 4. Push all commits
git push origin feat/task/T-05-01-jsonrpc-schema

# 5. Create Pull Request
gh pr create \
  --title "feat(api): T-05-01 - JSON-RPC Schema Definition & Validation" \
  --body "$(cat <<'EOF'
## Summary
Implements T-05-01: JSON-RPC Schema Definition & Validation

## Changes
- Added JSON-RPC 2.0 schema definitions for 4 endpoints
- Implemented schema validation layer
- Added contract tests
- Created test fixtures

## Acceptance Criteria
- [x] Schema file published: docs/api/jsonrpc-schema.json
- [x] Service contract tests validate responses
- [x] CLI integration test validates JSON output
- [x] Schema versioning plan documented

## Related
- Task: spacs/tasks/0.1.0-mvp/05-api-contracts/T-05-01-jsonrpc-schema.md
- Issue: spacs/issues/04-0.1.0-mvp/05-api-contracts.md
- PRD: spacs/prd/0.1.0-MVP-PRDs-v0/05-api-specifications.md

## Test Plan
- All unit tests pass
- Contract validation tests pass
- Schema validated with jsonschemavalidator.net

## Worklog
See: .artifacts/spec-tasks-T-05-01-jsonrpc-schema/worklogs/
EOF
)" \
  --base main

# 6. Update TODO.yaml with PR info
vim spacs/tasks/0.1.0-mvp/TODO.yaml
# - Add PR number and URL
# - Update status to "in_review"
```

### Phase 4: Post-Merge Cleanup

**After PR is merged:**

```shell
# 1. Switch to main repository
cd ~/dev-space/CDSAgent
git checkout main
git pull origin main

# 2. Update all other worktrees
./scripts/sync-worktrees.sh

# 3. Archive task worklog (optional)
# Worklogs are already committed in main repo under .artifacts/

# 4. Clean up completed worktree (optional)
git worktree remove .worktrees/T-05-01-jsonrpc-schema
rm ~/dev-space/CDSAgent-T-05-01-jsonrpc-schema  # Remove symlink

# 5. Update TODO.yaml
vim spacs/tasks/0.1.0-mvp/TODO.yaml
# - Set status: "completed"
# - Record merge date
# - Move task to completed_tasks list
```

---

## Worklog Management

### Daily Worklog Files

Each day of development requires three worklog files:

1. **work-summary.md** - High-level progress
2. **commit-log.md** - Git commit details
3. **notes.md** - Technical notes and decisions

### Work Summary Template

**Purpose**: Track daily objectives, accomplishments, and blockers

**Fill out**:

- **Morning**: List today's objectives
- **End of day**: Mark completed items, document blockers

**Key sections**:

- Today's Objectives (checkbox list)
- Work Completed (features, bug fixes, tests)
- Code Changes (files modified)
- Key Decisions (with rationale)
- Challenges & Solutions
- Next Steps
- Acceptance Criteria Progress

### Commit Log Template

**Purpose**: Document all git commits with context

**Fill out**:

- **After each commit** or **end of day**

**Key sections**:

- Commit hash and message
- Files changed count
- Diff summary
- Context notes (why this change)

### Development Notes Template

**Purpose**: Capture technical decisions, research, and learnings

**Fill out**:

- **As you work** - Don't wait until EOD

**Key sections**:

- Architecture Decisions
- Implementation Details
- Research & Learning
- Code Review Notes
- Testing Notes
- Performance Notes
- TODO / Follow-up

### Metadata.yaml

**Purpose**: Central metadata for the task

**Update frequency**:

- **Daily**: Add new commits, update hours
- **Weekly**: Update acceptance criteria status
- **At completion**: Final metrics and status

**Key fields**:

```yaml
task:
  status: not_started | in_progress | blocked | completed

git:
  commits:
    - hash: "abc1234"
      message: "feat(api): implement schema"
      date: "2025-10-19"

metrics:
  actual_hours: 16
  lines_added: 450
  lines_deleted: 20
  test_coverage: 0.85

acceptance_criteria:
  - criterion: "Schema file published"
    status: completed
    notes: "Published to docs/api/"
```

---

## Git Worktree Operations

### Viewing Worktrees

```shell
# List all worktrees with status
./scripts/worktree-symlink.sh list

# Or use git directly
git worktree list

# Check symlinks
ls -la ~/dev-space/CDSAgent-T-*
```

### Syncing Worktrees with Main

```shell
# Sync all worktrees at once (automated)
./scripts/sync-worktrees.sh

# Or sync individual worktree
cd ~/dev-space/CDSAgent-T-05-01-jsonrpc-schema
git fetch origin main
git rebase origin/main  # or: git merge origin/main
```

### Switching Between Tasks

**No need to switch!** Open multiple terminals/IDE windows:

```shell
# Terminal/IDE 1
cd ~/dev-space/CDSAgent-T-05-01-jsonrpc-schema
code .

# Terminal/IDE 2
cd ~/dev-space/CDSAgent-T-02-01-graph-builder
code .
```

### Creating New Worktrees

**Rarely needed** - All task worktrees were created in M0. But if needed:

```shell
# Create new worktree
git worktree add .worktrees/T-XX-XX-task-name \
  -b feat/task/T-XX-XX-task-name main

# Create symlink
ln -s $(pwd)/.worktrees/T-XX-XX-task-name \
  ~/dev-space/CDSAgent-T-XX-XX-task-name

# Or use helper
./scripts/worktree-symlink.sh create
```

---

## Best Practices

### Commit Message Format

**Convention**: Conventional Commits

```text
<type>(<scope>): <subject>

[optional body]

[optional footer]
```

**Types**:

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `test`: Tests
- `refactor`: Code restructuring
- `chore`: Maintenance
- `perf`: Performance improvement

**Examples**:

```shell
# Good commits
git commit -m "feat(api): implement JSON-RPC request validation"
git commit -m "test(index): add BM25 search benchmark tests"
git commit -m "docs(api): document error code catalogue"
git commit -m "fix(graph): resolve Python import cycle detection"

# With body
git commit -m "feat(graph): implement Python AST parser

- Add tree-sitter Python integration
- Extract 4 entity types (directory, file, class, function)
- Build graph with 4 edge types
- Unit tests with >80% coverage

Closes T-02-01"
```

### Code Quality Standards

**Before committing**:

```shell
# Format code
cargo fmt --all

# Run linter
cargo clippy --all-targets --all-features

# Run tests
cargo test --all

# Check coverage (aim for >80%)
cargo tarpaulin --out Html
```

**Test requirements**:

- Unit tests for all new functions
- Integration tests for API endpoints
- Property-based tests for critical paths
- Benchmark tests for performance-sensitive code

### Documentation Standards

**Update these when adding features**:

- Code comments (especially public APIs)
- README.md (if CLI commands change)
- API documentation (if endpoints change)
- Task worklog (always)

### Dependency Management

**Check dependencies before starting**:

```shell
# View task dependencies
yq '.tasks.T-XX-XX.dependencies' spacs/tasks/0.1.0-mvp/TODO.yaml

# Wait for prerequisite tasks to complete
# Or coordinate with parallel developers
```

**Example dependency chain**:

```text
T-05-01 (API Schema)
  ↓ blocks
T-02-03 (Service Layer)
  ↓ blocks
T-03-01 (CLI Commands)
  ↓ blocks
T-04-01 (Agent SDK)
```

### Time Management

**Estimate vs. Actual tracking**:

```yaml
# In metadata.yaml
metrics:
  estimated_hours: 24  # From TODO.yaml
  actual_hours: 0      # Update daily

# Daily update
actual_hours: 8   # After day 1
actual_hours: 16  # After day 2
actual_hours: 22  # After day 3
```

**If running over estimate**:

- Document in work-summary.md
- Inform tech lead
- Adjust future estimates

---

## Troubleshooting

### Common Issues

#### Issue: Worktree out of sync with main

**Symptoms**: Tests fail in worktree but pass in main

**Solution**:

```shell
cd ~/dev-space/CDSAgent-T-XX-XX
git fetch origin main
git rebase origin/main

# If conflicts:
# 1. Resolve conflicts
# 2. git add .
# 3. git rebase --continue
```

#### Issue: Cannot switch to main branch

**Error**: "Your local changes would be overwritten"

**Solution**:

```shell
# In worktree, commit or stash changes
git stash
# or
git commit -m "wip: save progress"

# Then switch
git checkout main
```

#### Issue: Symlink broken

**Symptoms**: `cd ~/dev-space/CDSAgent-T-XX-XX` fails

**Solution**:

```shell
# Remove all symlinks
./scripts/worktree-symlink.sh remove

# Recreate
./scripts/worktree-symlink.sh create
```

#### Issue: Forgot to create daily worklog

**Solution**:

```shell
# Create worklog for specific date
./scripts/create-daily-worklog.sh T-XX-XX YYYY-MM-DD

# Example for yesterday
./scripts/create-daily-worklog.sh T-05-01 2025-10-18
```

#### Issue: Artifacts not visible in worktree

**Symptoms**: `.artifacts/spec-tasks-T-XX-XX/` exists in main but not in worktree

**Root cause**: Worktree file system isolation - worktrees snapshot main's file tree at creation time. Files created in main AFTER worktree creation don't exist in the worktree's independent file system.

**Solution**:

See detailed explanation in [Phase 1: Step 4 - Initialize Artifacts FROM WORKTREE](#step-4-initialize-artifacts-from-worktree)

**Quick fix**:

```shell
# Option 1: Re-run from worktree (Recommended)
cd ~/dev-space/CDSAgent-T-XX-XX-task-name
/Users/arthur/dev-space/CDSAgent/scripts/create-task-worklog.sh \
  T-XX-XX-task-name "Task Title" "Your Name"

# Option 2: Copy from main
cp -r /Users/arthur/dev-space/CDSAgent/.artifacts/spec-tasks-T-XX-XX-task-name \
  ~/dev-space/CDSAgent-T-XX-XX-task-name/.artifacts/

# Verify
ls -la ~/dev-space/CDSAgent-T-XX-XX-task-name/.artifacts/spec-tasks-T-XX-XX-task-name/
```

**Prevention**: Always run `create-task-worklog.sh` FROM the worktree, not from main.

#### Issue: Accidentally committed to main

**Solution**:

```shell
# DON'T PANIC!

# Option 1: Move commits to task branch
git checkout feat/task/T-XX-XX
git cherry-pick <commit-hash>

git checkout main
git reset --hard origin/main

# Option 2: Create new task branch from main
git checkout -b feat/task/T-XX-XX-fix main~1
# Then reset main
git checkout main
git reset --hard origin/main
```

### Preventing Common Errors

**NEW**: Automated tools to catch errors before they happen!

#### Prevention 1: Sed Separator Issue (FIXED)

**Problem**: Task titles containing "/" (e.g., "Name/ID + BM25") caused sed to fail

**Solution**: `create-task-worklog.sh` and `create-daily-worklog.sh` now use `|` separator

**Status**: ✅ Fixed in all scripts

#### Prevention 2: Worktree Context Validation

**Problem**: Running scripts from main instead of worktree creates artifacts in wrong place

**Solution**: `create-daily-worklog.sh` now warns if not run from worktree

```shell
cd ~/dev-space/CDSAgent  # Wrong directory
./scripts/create-daily-worklog.sh T-XX-XX

# Output:
# ⚠ Warning: Not running from worktree!
# Recommendation: Navigate to worktree first:
#   cd ~/dev-space/CDSAgent-T-XX-XX
```

**Best Practice**: Always run worktree scripts FROM the worktree!

#### Prevention 3: Git Notes Verification

**Problem**: Forgetting to add git notes before checkpoint

**Solution**: Use automated verification script

```shell
# Before checkpoint, verify all commits have notes
./scripts/git-notes-check.sh

# Example output (failure):
# ✗ Commits missing notes (2):
#   - a3f4d89: feat(index): implement BM25
#   - 7e2c1b0: test(index): add BM25 tests
```

**Best Practice**: Run `git-notes-check.sh` before every checkpoint!

#### Prevention 4: Pre-Checkpoint Validation

**Problem**: Missing artifacts, inconsistent metadata, uncommitted changes

**Solution**: Use comprehensive checkpoint helper

```shell
# Before checkpoint, run comprehensive validation
./scripts/checkpoint-helper.sh T-XX-XX

# Checks:
# ✓ Git status clean or only artifacts
# ✓ All commits have git notes
# ✓ Daily worklogs exist
# ✓ Metadata consistency
# ✓ Artifact completeness
```

**Best Practice**: Run `checkpoint-helper.sh` before Phase 4 of checkpoint workflow!

**Quick Reference Table**:

| Problem | Prevention Tool | When to Use |
|---------|----------------|-------------|
| Sed error with "/" in title | Built-in fix | Automatic |
| Wrong directory context | Worktree validation | Every script run |
| Missing git notes | `git-notes-check.sh` | Before checkpoint |
| Incomplete artifacts | `checkpoint-helper.sh` | Before Phase 4 |

---

### Getting Help

1. **Check documentation**:
   - This SOP
   - Task specification
   - TODO.yaml
   - PRD/Issue docs

2. **Ask team**:
   - Create issue in GitHub
   - Slack channel
   - Tech lead review

3. **Update TODO.yaml**:

```yaml
task:
  status: blocked
comments:
  - date: "2025-10-19"
    author: "Developer"
    text: "Blocked on X, waiting for Y"
```

---

## Reference

### Task States

| State | Description | Next Action |
|-------|-------------|-------------|
| `not_started` | Task not begun | Initialize worklog, start coding |
| `in_progress` | Currently working | Daily worklogs, regular commits |
| `blocked` | Waiting on dependency | Document blocker, coordinate |
| `in_review` | PR submitted | Address review comments |
| `completed` | PR merged | Clean up worktree, update metadata |

### Milestone Status

Check current milestone:

```shell
# View active milestone
cat spacs/tasks/0.1.0-mvp/README.md | grep -A 10 "Milestone M1"

# View TODO.yaml milestones
yq '.milestones' spacs/tasks/0.1.0-mvp/TODO.yaml
```

### Current Task Branches

| Task ID | Branch | Status | Owner |
|---------|--------|--------|-------|
| T-05-01 | feat/task/T-05-01-jsonrpc-schema | Ready | Rust Dev 1 + TS Dev 1 |
| T-02-01 | feat/task/T-02-01-graph-builder | Pending | Rust Dev 1 |
| T-02-02 | feat/task/T-02-02-sparse-index | Pending | Rust Dev 2 |
| T-02-03 | feat/task/T-02-03-service-layer | Pending | Rust Dev 1 |
| T-03-01 | feat/task/T-03-01-cli-commands | Pending | Rust Dev 2 |
| T-04-01 | feat/task/T-04-01-agent-sdk | Pending | TS Dev 1 |
| T-04-02 | feat/task/T-04-02-prompt-design | Pending | TS Dev 1 |

### Quick Commands Reference

```shell
# Task management
./scripts/create-task-worklog.sh <ID> "<TITLE>" "<NAME>"
./scripts/create-daily-worklog.sh <ID> [DATE]
./scripts/sync-worktrees.sh

# Worktree operations
./scripts/worktree-symlink.sh list
./scripts/worktree-symlink.sh create
./scripts/worktree-symlink.sh remove

# Git operations
git worktree list
git worktree add <path> -b <branch> main
git worktree remove <path>

# Metadata queries
yq '.tasks.<TASK_ID>' spacs/tasks/0.1.0-mvp/TODO.yaml
cat .artifacts/spec-tasks-<ID>/metadata.yaml

# Development
cd ~/dev-space/CDSAgent-<TASK_ID>
cargo test --all
cargo clippy --all-targets
gh pr create --title "..." --body "..." --base main
```

### Resources

- **Specifications**:
  - [TODO.yaml](../spacs/tasks/0.1.0-mvp/TODO.yaml) - Central task registry
  - [Task README](../spacs/tasks/0.1.0-mvp/README.md) - Milestone overview
  - [PRDs](../spacs/prd/0.1.0-MVP-PRDs-v0/) - Product requirements
  - [Issues](../spacs/issues/04-0.1.0-mvp/) - Technical breakdown

- **Templates**:
  - [Worklog Templates](../.artifacts/spec-tasks-templates/) - Daily logging

- **Scripts**:
  - [create-task-worklog.sh](../scripts/create-task-worklog.sh)
  - [create-daily-worklog.sh](../scripts/create-daily-worklog.sh)
  - [sync-worktrees.sh](../scripts/sync-worktrees.sh)
  - [worktree-symlink.sh](../scripts/worktree-symlink.sh)

- **External**:
  - [Git Worktree Documentation](https://git-scm.com/docs/git-worktree)
  - [Conventional Commits](https://www.conventionalcommits.org/)
  - [LocAgent Paper](https://arxiv.org/html/2503.09089v2)

---

## Appendix: Example Workflow

### Complete Example: T-05-01 Development

Day 1: Setup & Initial Implementation

```shell
# Morning - Initialize
./scripts/create-task-worklog.sh T-05-01-jsonrpc-schema \
  "JSON-RPC Schema Definition" "Rust Dev 1"
./scripts/create-daily-worklog.sh T-05-01
cd ~/dev-space/CDSAgent-T-05-01-jsonrpc-schema

# Read specs
cat spacs/tasks/0.1.0-mvp/05-api-contracts/T-05-01-jsonrpc-schema.md

# Code
mkdir -p docs/api
vim docs/api/jsonrpc-schema.json
# ... implement schema ...

# Test & commit
cargo test
git add docs/api/
git commit -m "feat(api): add JSON-RPC 2.0 schema definitions"
git push origin feat/task/T-05-01

# EOD - Update worklog
vim .artifacts/spec-tasks-T-05-01/worklogs/2025-10-19-work-summary.md
vim .artifacts/spec-tasks-T-05-01/worklogs/2025-10-19-commit-log.md
```

Day 2: Validation Layer

```shell
# Morning
./scripts/create-daily-worklog.sh T-05-01
cd ~/dev-space/CDSAgent-T-05-01-jsonrpc-schema

# Code
vim crates/cds-index/src/service/jsonrpc.rs
# ... implement validation ...

# Commits
git add crates/
git commit -m "feat(api): implement schema validation layer"
git push

# EOD - Update worklog
# ... fill out daily logs ...
```

Day 3: Tests & Completion

```shell
# Morning
./scripts/create-daily-worklog.sh T-05-01
cd ~/dev-space/CDSAgent-T-05-01-jsonrpc-schema

# Add tests
vim crates/cds-index/tests/service_contract_tests.rs
cargo test --all

# Final commit
git add crates/
git commit -m "test(api): add schema validation contract tests"
git push

# Create PR
gh pr create --title "feat(api): T-05-01 - JSON-RPC Schema" --base main

# Update metadata
vim .artifacts/spec-tasks-T-05-01/metadata.yaml
# Set status: completed, add PR link

# EOD - Final worklog
# ... document completion ...
```

---

**Version History**:

- **v1.2** (2025-10-23): Enhanced workflow with next task selection and worktree preparation procedures
  - Added "Quick Decision Flowchart" for task selection
  - Added "Next Task Selection Procedure" (5-step methodology)
  - Expanded "Phase 1: Worktree Environment Preparation" (8 steps)
  - Documented worktree file system isolation issue and solutions
  - Added task-specific CLAUDE.md update guidance
  - Enhanced troubleshooting section with artifacts visibility issue
- **v1.1** (2025-10-19): Complete SOP with TODO.yaml and worklog system
- **v1.0** (2025-10-19): Initial SOP based on git worktrees

**Maintainer**: CDSAgent Tech Lead

**Feedback**: Create issue at [CDSAgent/issues](https://github.com/lwyBZss8924d/CDSAgent/issues)

---

END OF SOP
