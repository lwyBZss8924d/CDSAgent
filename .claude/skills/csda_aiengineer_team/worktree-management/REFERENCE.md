# Worktree Management - Complete Reference

This document provides detailed information about CDSAgent's git worktree management, advanced operations, and troubleshooting.

## Table of Contents

- [Worktree Fundamentals](#worktree-fundamentals)
- [Advanced Worktree Operations](#advanced-worktree-operations)
- [Symlink Management](#symlink-management)
- [Worktree Synchronization](#worktree-synchronization)
- [Multi-Worktree Workflows](#multi-worktree-workflows)
- [Worktree Cleanup](#worktree-cleanup)
- [Troubleshooting](#troubleshooting)

---

## Worktree Fundamentals

### What is a Git Worktree?

A **git worktree** is an additional working directory linked to the same repository, allowing multiple branches to be checked out simultaneously.

**Benefits**:

- ✅ **Parallel Development**: Work on multiple tasks without branch switching
- ✅ **Clean State**: No stashing required
- ✅ **Isolated Environments**: Each worktree has independent working directory
- ✅ **Shared History**: All worktrees share same `.git` repository

### Worktree vs Clone

| Aspect | Worktree | Clone |
|--------|----------|-------|
| Disk Usage | Shared `.git` (minimal) | Full repository copy |
| Sync | Instant (same .git) | Requires push/pull |
| Setup Time | <1 second | Depends on repo size |
| Use Case | Multiple local branches | Different remotes/forks |

### CDSAgent Worktree Structure

```text
~/dev-space/CDSAgent/                    # Main repository
  ├── .git/                              # Shared git database
  │   └── worktrees/                     # Worktree metadata
  │       ├── T-02-01-graph-builder/
  │       └── T-02-02-sparse-index/
  ├── .worktrees/                        # Worktree directories
  │   ├── T-02-01-graph-builder/         # Task 1 worktree
  │   │   ├── .git                       # Link to main .git
  │   │   ├── .artifacts/                # Task 1 artifacts
  │   │   └── crates/                    # Task 1 code
  │   └── T-02-02-sparse-index/          # Task 2 worktree
  │       ├── .git                       # Link to main .git
  │       ├── .artifacts/                # Task 2 artifacts
  │       └── crates/                    # Task 2 code
  └── main branch files...

~/dev-space/                             # Symlinks (shortcuts)
  ├── CDSAgent-T-02-01-graph-builder -> CDSAgent/.worktrees/T-02-01-graph-builder
  └── CDSAgent-T-02-02-sparse-index -> CDSAgent/.worktrees/T-02-02-sparse-index
```

---

## Advanced Worktree Operations

### Creating Worktrees

**Basic Creation**:

```shell
# From main repository
git worktree add .worktrees/T-XX-XX-task-name feat/task/T-XX-XX-task-name
```

**Create with Existing Branch**:

```shell
# If branch already exists
git worktree add .worktrees/T-XX-XX-task-name -b feat/task/T-XX-XX-task-name

# From remote branch
git worktree add .worktrees/T-XX-XX-task-name origin/feat/task/T-XX-XX-task-name
```

**Create from Specific Commit**:

```shell
# Checkout worktree at specific commit
git worktree add .worktrees/T-XX-XX-task-name -b feat/task/T-XX-XX <commit-hash>
```

**Detached HEAD Worktree** (rare):

```shell
# Create worktree in detached HEAD state
git worktree add --detach .worktrees/experiment <commit-hash>
```

### Listing Worktrees

**Basic List**:

```shell
git worktree list
```

**Output**:

```text
/Users/arthur/dev-space/CDSAgent                       (main)
/Users/arthur/dev-space/CDSAgent/.worktrees/T-02-01    (feat/task/T-02-01)
/Users/arthur/dev-space/CDSAgent/.worktrees/T-02-02    (feat/task/T-02-02)
```

**Verbose List** (with commit hashes):

```shell
git worktree list -v
```

**Output**:

```text
/Users/arthur/dev-space/CDSAgent                       2a2ad34 (main)
/Users/arthur/dev-space/CDSAgent/.worktrees/T-02-01    72f9db5 (feat/task/T-02-01)
/Users/arthur/dev-space/CDSAgent/.worktrees/T-02-02    414f7f2 (feat/task/T-02-02)
```

**Porcelain Format** (machine-readable):

```shell
git worktree list --porcelain
```

**Output**:

```text
worktree /Users/arthur/dev-space/CDSAgent
HEAD 2a2ad34
branch refs/heads/main

worktree /Users/arthur/dev-space/CDSAgent/.worktrees/T-02-01-graph-builder
HEAD 72f9db5
branch refs/heads/feat/task/T-02-01-graph-builder
```

### Moving Worktrees

**Rename/Move Worktree**:

```shell
# Move worktree to new location
git worktree move .worktrees/T-02-02-sparse-index .worktrees/T-02-02-new-name

# Update symlink after move
rm ~/dev-space/CDSAgent-T-02-02-sparse-index
ln -s ~/dev-space/CDSAgent/.worktrees/T-02-02-new-name ~/dev-space/CDSAgent-T-02-02-new-name
```

### Locking Worktrees

**Lock Worktree** (prevent removal):

```shell
# Lock worktree with reason
git worktree lock .worktrees/T-02-02-sparse-index --reason "Active development"

# Verify locked
git worktree list
# Shows: locked (Active development)
```

**Unlock Worktree**:

```shell
git worktree unlock .worktrees/T-02-02-sparse-index
```

**Use Case**: Lock worktrees on network drives to prevent accidental removal.

### Pruning Worktrees

**Prune Stale References**:

```shell
# Remove stale worktree references (metadata only)
git worktree prune

# Dry-run (show what would be pruned)
git worktree prune --dry-run
```

**When to Prune**:

- After manually deleting worktree directory
- After moving repository
- Cleaning up git metadata

---

## Symlink Management

### Creating Symlinks

**Manual Creation**:

```shell
# From worktree directory
cd .worktrees/T-XX-XX-task-name

# Create symlink in ~/dev-space/
ln -s $(pwd) ~/dev-space/CDSAgent-T-XX-XX-task-name
```

**Using Script**:

```shell
# From worktree
./.dev/scripts/task/worktree-symlink.sh create

# Creates: ~/dev-space/CDSAgent-{TASK_ID} -> .worktrees/{TASK_ID}
```

### Verifying Symlinks

**Check Symlink Target**:

```shell
# List symlink with target
ls -l ~/dev-space/CDSAgent-T-XX-XX

# Output:
# CDSAgent-T-XX-XX -> /Users/arthur/dev-space/CDSAgent/.worktrees/T-XX-XX
```

**Test Symlink**:

```shell
# Follow symlink
cd ~/dev-space/CDSAgent-T-XX-XX

# Verify you're in worktree
pwd
# Output: /Users/arthur/dev-space/CDSAgent/.worktrees/T-XX-XX-task-name
```

### Removing Symlinks

**Manual Removal**:

```shell
rm ~/dev-space/CDSAgent-T-XX-XX
```

**Using Script**:

```shell
# From worktree
./.dev/scripts/task/worktree-symlink.sh remove
```

**Batch Removal**:

```shell
# Remove all CDSAgent symlinks
rm ~/dev-space/CDSAgent-*

# Or selectively
rm ~/dev-space/CDSAgent-T-02-*
```

---

## Worktree Synchronization

### Syncing .dev/ Directory

**Purpose**: Keep `.dev/` toolkit consistent across all worktrees.

**Manual Sync**:

```shell
# From main repository
cd ~/dev-space/CDSAgent

# Copy .dev/ to all worktrees
for worktree in .worktrees/*/; do
  rsync -av .dev/ "$worktree/.dev/"
done
```

**Using Script**:

```shell
# From main repository
./.dev/scripts/task/sync-worktrees.sh

# Or dry-run
./.dev/scripts/task/sync-worktrees.sh --dry-run
```

### Syncing Artifact Templates

**Copy Templates to Worktrees**:

```shell
# Sync .artifacts/ metadata templates
for worktree in .worktrees/*/; do
  mkdir -p "$worktree/.artifacts/"
  rsync -av .artifacts/.gitkeep "$worktree/.artifacts/"
done
```

### Pulling Updates in All Worktrees

**Update All Worktrees**:

```shell
# Update each worktree with latest main
for worktree in .worktrees/*/; do
  echo "Updating $(basename $worktree)..."
  cd "$worktree"
  git pull origin main
  cd -
done
```

---

## Multi-Worktree Workflows

### Parallel Task Development

**Setup**:

```shell
# Create multiple worktrees for independent tasks
git worktree add .worktrees/T-05-01-jsonrpc-schema feat/task/T-05-01
git worktree add .worktrees/T-06-01-parity-methodology feat/task/T-06-01

# Work in parallel
# Terminal 1
cd ~/dev-space/CDSAgent-T-05-01

# Terminal 2
cd ~/dev-space/CDSAgent-T-06-01
```

### Code Review Worktree

**Setup**:

```shell
# Create temporary worktree for PR review
git fetch origin pull/6/head:pr-6-review
git worktree add .worktrees/review-pr-6 pr-6-review

# Review code
cd .worktrees/review-pr-6
cargo test
cargo clippy

# Remove after review
cd ~/dev-space/CDSAgent
git worktree remove .worktrees/review-pr-6
git branch -D pr-6-review
```

### Experimental Feature Worktree

**Setup**:

```shell
# Create experimental branch from task worktree
cd ~/dev-space/CDSAgent-T-02-02-sparse-index
git branch feat/task/T-02-02-experiment

# Create experimental worktree
cd ~/dev-space/CDSAgent
git worktree add .worktrees/T-02-02-experiment feat/task/T-02-02-experiment

# Work on experiment
cd .worktrees/T-02-02-experiment

# If successful, merge back to main task
cd ~/dev-space/CDSAgent-T-02-02-sparse-index
git merge feat/task/T-02-02-experiment

# If unsuccessful, discard
cd ~/dev-space/CDSAgent
git worktree remove .worktrees/T-02-02-experiment
git branch -D feat/task/T-02-02-experiment
```

---

## Worktree Cleanup

### Removing Completed Task Worktrees

**After PR Merged**:

```shell
# 1. Verify PR merged
gh pr view 6 --json state
# Output: {"state": "MERGED"}

# 2. Remove worktree
git worktree remove .worktrees/T-02-01-graph-builder

# 3. Remove symlink
rm ~/dev-space/CDSAgent-T-02-01-graph-builder

# 4. Delete local branch (optional, already merged)
git branch -d feat/task/T-02-01-graph-builder

# 5. Prune remote-tracking branches
git fetch --prune
```

### Force Removal (Uncommitted Changes)

**Scenario**: Worktree has uncommitted changes but needs removal.

**Force Remove**:

```shell
# Remove worktree ignoring uncommitted changes
git worktree remove --force .worktrees/T-XX-XX-task-name

# Or if worktree directory deleted manually
git worktree prune
```

### Batch Cleanup (All Completed Tasks)

**List Completed Tasks**:

```shell
# Find merged branches
git branch --merged main | grep "feat/task/"

# Output:
#   feat/task/T-05-01-jsonrpc-schema
#   feat/task/T-05-02-typescript-bindings
#   feat/task/T-06-01-parity-methodology
```

**Remove All Merged Worktrees**:

```shell
# For each merged task branch
for branch in $(git branch --merged main | grep "feat/task/"); do
  task_id=$(echo $branch | sed 's|feat/task/||')
  echo "Removing worktree for $task_id..."

  # Remove worktree
  git worktree remove .worktrees/$task_id 2>/dev/null || true

  # Remove symlink
  rm ~/dev-space/CDSAgent-$task_id 2>/dev/null || true

  # Delete branch
  git branch -d $branch
done

# Prune stale worktree metadata
git worktree prune
```

---

## Troubleshooting

### Issue: "Worktree already exists"

**Problem**: Creating worktree fails with "worktree already exists".

**Solution**:

```shell
# Check if worktree registered
git worktree list | grep T-XX-XX

# If listed but directory missing, prune
git worktree prune

# If directory exists, remove it
git worktree remove .worktrees/T-XX-XX

# Or force remove
git worktree remove --force .worktrees/T-XX-XX
```

### Issue: "Branch already checked out"

**Problem**: Cannot create worktree because branch is checked out elsewhere.

**Solution**:

```shell
# Find where branch is checked out
git worktree list | grep "feat/task/T-XX-XX"

# Output shows existing worktree location
# Remove old worktree first
git worktree remove <old-worktree-path>

# Then create new worktree
git worktree add .worktrees/T-XX-XX feat/task/T-XX-XX
```

### Issue: Broken Symlink

**Problem**: Symlink points to non-existent directory.

**Solution**:

```shell
# Find broken symlinks
find ~/dev-space/ -type l ! -exec test -e {} \; -print

# Output:
# /Users/arthur/dev-space/CDSAgent-T-XX-XX

# Remove broken symlink
rm ~/dev-space/CDSAgent-T-XX-XX

# Recreate if worktree still exists
cd .worktrees/T-XX-XX
./.dev/scripts/task/worktree-symlink.sh create
```

### Issue: ".dev/ Directory Missing"

**Problem**: Worktree doesn't have `.dev/` directory.

**Solution**:

```shell
# Sync .dev/ from main repository
cd ~/dev-space/CDSAgent
./.dev/scripts/task/sync-worktrees.sh

# Or manual sync to specific worktree
rsync -av .dev/ .worktrees/T-XX-XX/.dev/
```

### Issue: "Cannot remove current worktree"

**Problem**: Trying to remove worktree you're currently in.

**Solution**:

```shell
# Exit worktree first
cd ~/dev-space/CDSAgent  # Main repository

# Then remove
git worktree remove .worktrees/T-XX-XX
```

---

## References

- **Primary Workflow**: `.dev/workflows/WORKTREE_WORKFLOW.md`
- **Task Initialization**: `.dev/workflows/NEXT_TASK_CHECKLIST.md`
- **Git Worktree Documentation**: `https://git-scm.com/docs/git-worktree`

**Important⚠️**: Always run `git worktree list` before creating worktrees to avoid "branch already checked out" errors.
**Important⚠️**: Wen Run [worktree-management] job, MUST run (date -u '+%Y-%m-%dT%H:%M:%SZ') get UTC TIME NOW first!!!
