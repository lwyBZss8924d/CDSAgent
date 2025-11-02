# Task Initialization - Real-World Examples

This document provides practical scenarios demonstrating task initialization in action.

## Table of Contents

- [Scenario 1: Starting First Task (T-02-01)](#scenario-1-starting-first-task-t-02-01)
- [Scenario 2: Starting Dependent Task (T-02-02)](#scenario-2-starting-dependent-task-t-02-02)
- [Scenario 3: Parallel Task Initialization](#scenario-3-parallel-task-initialization)
- [Scenario 4: Recovering from Failed Initialization](#scenario-4-recovering-from-failed-initialization)

---

## Scenario 1: Starting First Task (T-02-01)

**Context**: Beginning M2 milestone with T-02-01-graph-builder (no prerequisites).

### Step 1: Check TODO.yaml

```shell
# View T-02-01 in PMP
cat spacs/tasks/0.1.0-mvp/TODO.yaml | grep -A 30 "T-02-01"

# Output:
# T-02-01-graph-builder:
#   title: "Graph Builder - AST Parsing & Construction"
#   milestone: M2
#   status: not_started
#   dependencies:
#     requires:
#       - T-06-01-parity-methodology  # ✅ Check if completed
#     blocks:
#       - T-02-02-sparse-index
```

### Step 2: Verify Dependencies

```shell
# Check T-06-01 status
cat spacs/tasks/0.1.0-mvp/TODO.yaml | grep -A 5 "T-06-01"

# Output:
# T-06-01-parity-methodology:
#   status: completed  # ✅ Ready to start T-02-01
```

### Step 3: Create Worktree

```shell
# From main repository
cd ~/dev-space/CDSAgent

# Create worktree
git worktree add .worktrees/T-02-01-graph-builder feat/task/T-02-01-graph-builder

# Output:
# Preparing worktree (new branch 'feat/task/T-02-01-graph-builder')
# HEAD is now at 2a2ad34 feat(parity): T-06-01 Phase 2 complete

# Navigate to worktree
cd .worktrees/T-02-01-graph-builder
```

### Step 4: Create Symlink

```shell
# From worktree
./.dev/scripts/task/worktree-symlink.sh create

# Output:
# ✅ Created symlink: ~/dev-space/CDSAgent-T-02-01-graph-builder -> .worktrees/T-02-01-graph-builder

# Verify
ls -l ~/dev-space/ | grep CDSAgent-T-02-01

# Output:
# lrwxr-xr-x  ... CDSAgent-T-02-01-graph-builder -> .worktrees/T-02-01-graph-builder
```

### Step 5: Initialize Metadata

```shell
# From worktree
./.dev/scripts/task/create-task-worklog.sh T-02-01 "Graph Builder - AST Parsing & Construction"

# Output:
# ✅ Created .artifacts/spec-tasks-T-02-01-graph-builder/metadata.yaml
# ✅ Created .artifacts/spec-tasks-T-02-01-graph-builder/worklogs/
# ✅ Created .artifacts/spec-tasks-T-02-01-graph-builder/worklogs/raw/
# Task T-02-01 initialized. Ready to start Session 01.
```

### Step 6: Update metadata.yaml

```shell
# Edit metadata.yaml
vim .artifacts/spec-tasks-T-02-01-graph-builder/metadata.yaml

# Update fields:
task:
  id: T-02-01-graph-builder
  title: "Graph Builder - AST Parsing & Construction"
  owner: "Rust Dev 1"
  status: in_progress
  start_date: "2025-10-24"

specs:
  prds:
    - spacs/prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md
    - spacs/prd/0.1.0-MVP-PRDs-v0/06-rust-refactoring-plan.md
  issues:
    - spacs/issues/04-0.1.0-mvp/02-index-core/01-graph-build.md
  tasks:
    - spacs/tasks/0.1.0-mvp/02-index-core/T-02-01-graph-builder.md

git:
  worktree: .worktrees/T-02-01-graph-builder
  branch: feat/task/T-02-01-graph-builder
  base_commit: 2a2ad34  # Current HEAD

dependencies:
  requires:
    - T-06-01-parity-methodology  # ✅ Completed
  blocks:
    - T-02-02-sparse-index
    - T-03-01-core-commands
```

### Step 7: Start First Session

```shell
# Create Session 01
./.dev/scripts/session/create-session-worklog.sh T-02-01 01 "Phase 0 Planning & Analysis" "Rust Dev 1"

# ✅ Session 01 initialized, ready to work!
```

**Result**: T-02-01 fully initialized, ready for development.

---

## Scenario 2: Starting Dependent Task (T-02-02)

**Context**: T-02-01 completed, ready to start T-02-02-sparse-index.

### Step 1: Verify Prerequisite Complete

```shell
# Check T-02-01 status in TODO.yaml
cat spacs/tasks/0.1.0-mvp/TODO.yaml | grep -A 5 "T-02-01"

# Output:
# T-02-01-graph-builder:
#   status: completed  # ✅ Prerequisite met
#   completed_date: "2025-10-30"
#   pr: "https://github.com/lwyBZss8924d/CDSAgent/pull/6"
```

### Step 2: Check Blocker List

```shell
# Ensure T-02-02 not in blocked_tasks
cat spacs/tasks/0.1.0-mvp/TODO.yaml | grep -A 10 "blocked_tasks:"

# Output:
# blocked_tasks: []  # ✅ No blockers
```

### Step 3: Wen Task Init Start, Create Worktree for all Tasks SESSIONS Work Space

```shell
# From main repository
cd ~/dev-space/CDSAgent

# Pull latest (includes T-02-01 PR merge)
git pull origin main

# Create worktree
git worktree add .worktrees/T-02-02-sparse-index feat/task/T-02-02-sparse-index

# Navigate
cd .worktrees/T-02-02-sparse-index
```

### Step 4: Create Symlink and Initialize

```shell
# Create symlink
./.dev/scripts/task/worktree-symlink.sh create

# Initialize metadata
./.dev/scripts/task/create-task-worklog.sh T-02-02 "Sparse Index - Name/ID + BM25 Search"

# Update metadata.yaml with specs and dependencies
# ... (similar to Scenario 1)
```

### Step 5: Verify Dependency Chain

```shell
# Check that T-02-01 deliverables are available
ls crates/cds-index/src/graph/

# Output:
# builder/
# parser.rs
# traversal.rs
# mod.rs

# ✅ Graph builder code available (from T-02-01)
```

### Step 6: Start Development

```shell
# Create Session 01
./.dev/scripts/session/create-session-worklog.sh T-02-02 01 "Phase 0 Planning & Analysis" "Claude Code Agent"

# Read T-02-01 deliverables
cat .artifacts/spec-tasks-T-02-01-graph-builder/worklogs/2025-10-30-work-summary.md

# ✅ Ready to build on T-02-01's work
```

**Result**: T-02-02 initialized with dependency on completed T-02-01.

---

## Scenario 3: Parallel Task Initialization

**Context**: M1 milestone with 3 independent tasks (T-05-01, T-05-02, T-06-01).

### Identify Parallel Tasks

```shell
# Check TODO.yaml for tasks with no dependencies
cat spacs/tasks/0.1.0-mvp/TODO.yaml | grep -A 10 "T-05-01"
cat spacs/tasks/0.1.0-mvp/TODO.yaml | grep -A 10 "T-06-01"

# Both have:
# dependencies:
#   requires: []  # ✅ Can start in parallel
```

### Initialize All Tasks

```shell
# From main repository
cd ~/dev-space/CDSAgent

# Create worktrees for all 3 tasks
git worktree add .worktrees/T-05-01-jsonrpc-schema feat/task/T-05-01-jsonrpc-schema
git worktree add .worktrees/T-05-02-typescript-bindings feat/task/T-05-02-typescript-bindings
git worktree add .worktrees/T-06-01-parity-methodology feat/task/T-06-01-parity-methodology

# Create symlinks for all
cd .worktrees/T-05-01-jsonrpc-schema
./.dev/scripts/task/worktree-symlink.sh create

cd ../T-05-02-typescript-bindings
./.dev/scripts/task/worktree-symlink.sh create

cd ../T-06-01-parity-methodology
./.dev/scripts/task/worktree-symlink.sh create

# Initialize metadata for all
cd ~/dev-space/CDSAgent-T-05-01
./.dev/scripts/task/create-task-worklog.sh T-05-01 "JSON-RPC Schema Definition & Validation"

cd ~/dev-space/CDSAgent-T-05-02
./.dev/scripts/task/create-task-worklog.sh T-05-02 "TypeScript Client Types & SDK Bindings"

cd ~/dev-space/CDSAgent-T-06-01
./.dev/scripts/task/create-task-worklog.sh T-06-01 "LocAgent Parity Validation Methodology"
```

### Work in Parallel

```shell
# Terminal 1: T-05-01 (Rust Dev 1)
cd ~/dev-space/CDSAgent-T-05-01
./.dev/scripts/session/create-session-worklog.sh T-05-01 01 "Schema Design" "Rust Dev 1"

# Terminal 2: T-05-02 (TS Dev 1) - BLOCKED until T-05-01 schema exists
# (Will start after T-05-01 Session 01 completes)

# Terminal 3: T-06-01 (Rust Lead)
cd ~/dev-space/CDSAgent-T-06-01
./.dev/scripts/session/create-session-worklog.sh T-06-01 01 "Methodology Design" "Rust Lead"
```

**Result**: Two tasks (T-05-01, T-06-01) run in parallel, T-05-02 waits for T-05-01 schema.

---

## Scenario 4: Recovering from Failed Initialization

**Context**: Initialization failed mid-process, need to clean up and retry.

### Identify Partial Initialization

```shell
# Check what exists
ls .worktrees/ | grep T-03-01

# Output:
# T-03-01-cli-commands  # Worktree exists

ls .artifacts/ | grep T-03-01

# Output:
# (empty)  # Metadata not created

ls -l ~/dev-space/ | grep T-03-01

# Output:
# (empty)  # Symlink not created
```

### Clean Up Partial State

```shell
# Remove incomplete worktree
git worktree remove .worktrees/T-03-01-cli-commands

# Or force remove if needed
git worktree remove --force .worktrees/T-03-01-cli-commands

# Delete branch if it exists
git branch -D feat/task/T-03-01-cli-commands

# Remove any partial artifacts
rm -rf .artifacts/spec-tasks-T-03-01-cli-commands

# Remove broken symlink if exists
rm ~/dev-space/CDSAgent-T-03-01-cli-commands
```

### Retry Initialization

```shell
# From main repository
cd ~/dev-space/CDSAgent

# Verify clean state
git worktree list | grep T-03-01
# (should be empty)

# Create worktree again
git worktree add .worktrees/T-03-01-cli-commands feat/task/T-03-01-cli-commands

# Navigate to worktree
cd .worktrees/T-03-01-cli-commands

# Create symlink
./.dev/scripts/task/worktree-symlink.sh create

# ✅ Symlink created

# Initialize metadata
./.dev/scripts/task/create-task-worklog.sh T-03-01 "CLI Core Commands Implementation"

# ✅ Metadata created

# Verify complete initialization
echo "Checking initialization..."
[ -d .worktrees/T-03-01-cli-commands ] && echo "✅ Worktree exists"
[ -L ~/dev-space/CDSAgent-T-03-01-cli-commands ] && echo "✅ Symlink exists"
[ -f .artifacts/spec-tasks-T-03-01-cli-commands/metadata.yaml ] && echo "✅ Metadata exists"
[ -d .artifacts/spec-tasks-T-03-01-cli-commands/worklogs ] && echo "✅ Worklogs directory exists"

echo "Initialization complete ✅"
```

**Result**: Clean recovery from partial initialization, task fully initialized.

---

## Summary

These examples demonstrate:

1. **Starting First Task**: Initialize task with no dependencies
2. **Starting Dependent Task**: Verify prerequisite completion before initialization
3. **Parallel Task Initialization**: Setup multiple independent tasks simultaneously
4. **Recovering from Failed Initialization**: Clean up and retry after errors

**Key Takeaways**:

- ✅ **Check dependencies first** - verify prerequisites in TODO.yaml
- ✅ **Use symlinks for convenience** - easier navigation to worktrees
- ✅ **Update metadata.yaml thoroughly** - copy specs and dependencies from TODO.yaml
- ✅ **Clean up failed state** - remove partial worktrees/symlinks before retry
- ✅ **Parallel tasks are powerful** - work on multiple independent tasks simultaneously

**Important⚠️**: Wen Run [task-initialization] job changed any PMP and docs & metadata .yaml MUST run (date -u '+%Y-%m-%dT%H:%M:%SZ') get UTC TIME NOW first!!!
