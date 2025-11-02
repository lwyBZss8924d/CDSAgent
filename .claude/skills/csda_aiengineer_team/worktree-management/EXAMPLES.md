# Worktree Management - Real-World Examples

This document provides practical scenarios demonstrating worktree management operations.

## Table of Contents

- [Scenario 1: Creating and Removing Task Worktrees](#scenario-1-creating-and-removing-task-worktrees)
- [Scenario 2: Parallel Development with Multiple Worktrees](#scenario-2-parallel-development-with-multiple-worktrees)
- [Scenario 3: Recovering from Worktree Issues](#scenario-3-recovering-from-worktree-issues)

---

## Scenario 1: Creating and Removing Task Worktrees

**Context**: Complete lifecycle of a task worktree from creation to cleanup.

### Creating Worktree

```shell
# From main repository
cd ~/dev-space/CDSAgent

# Create worktree for T-02-02
git worktree add .worktrees/T-02-02-sparse-index feat/task/T-02-02-sparse-index

# Output:
# Preparing worktree (new branch 'feat/task/T-02-02-sparse-index')
# HEAD is now at 2a2ad34 ...

# Navigate to worktree
cd .worktrees/T-02-02-sparse-index

# Create symlink
./.dev/scripts/task/worktree-symlink.sh create

# Output:
# ✅ Created symlink: ~/dev-space/CDSAgent-T-02-02-sparse-index
```

### Working in Worktree

```shell
# Use symlink for easy access
cd ~/dev-space/CDSAgent-T-02-02-sparse-index

# Verify you're in worktree
git branch --show-current
# Output: feat/task/T-02-02-sparse-index

# Work on task...
# ... (sessions, commits, PR)
```

### Removing Worktree After Completion

```shell
# After PR merged
cd ~/dev-space/CDSAgent

# Remove worktree
git worktree remove .worktrees/T-02-02-sparse-index

# Output:
# ✅ Worktree removed

# Remove symlink
rm ~/dev-space/CDSAgent-T-02-02-sparse-index

# Delete local branch (already merged)
git branch -d feat/task/T-02-02-sparse-index

# Prune remote-tracking branches
git fetch --prune
```

**Result**: Complete worktree lifecycle managed cleanly.

---

## Scenario 2: Parallel Development with Multiple Worktrees

**Context**: Working on M1 milestone with 3 independent tasks simultaneously.

### Setup Multiple Worktrees

```shell
# From main repository
cd ~/dev-space/CDSAgent

# Create worktrees for T-05-01, T-05-02, T-06-01
git worktree add .worktrees/T-05-01-jsonrpc-schema feat/task/T-05-01
git worktree add .worktrees/T-05-02-typescript-bindings feat/task/T-05-02
git worktree add .worktrees/T-06-01-parity-methodology feat/task/T-06-01

# Create symlinks for all
cd .worktrees/T-05-01-jsonrpc-schema
./.dev/scripts/task/worktree-symlink.sh create

cd ../T-05-02-typescript-bindings
./.dev/scripts/task/worktree-symlink.sh create

cd ../T-06-01-parity-methodology
./.dev/scripts/task/worktree-symlink.sh create

# List all worktrees
cd ~/dev-space/CDSAgent
git worktree list
```

**Output**:

```text
/Users/arthur/dev-space/CDSAgent                       (main)
/Users/arthur/dev-space/CDSAgent/.worktrees/T-05-01    (feat/task/T-05-01)
/Users/arthur/dev-space/CDSAgent/.worktrees/T-05-02    (feat/task/T-05-02)
/Users/arthur/dev-space/CDSAgent/.worktrees/T-06-01    (feat/task/T-06-01)
```

### Parallel Work

```shell
# Terminal 1: Work on T-05-01
cd ~/dev-space/CDSAgent-T-05-01
./.dev/scripts/session/create-session-worklog.sh T-05-01 01 "Schema Design" "Rust Dev 1"
# ... work on schema ...

# Terminal 2: Work on T-06-01 (independent task)
cd ~/dev-space/CDSAgent-T-06-01
./.dev/scripts/session/create-session-worklog.sh T-06-01 01 "Methodology Design" "Rust Lead"
# ... work on parity methodology ...

# Terminal 3: Wait for T-05-01 schema, then start T-05-02
# (T-05-02 depends on T-05-01 schema)
cd ~/dev-space/CDSAgent-T-05-02
# Wait for T-05-01 Session 01 to complete...
./.dev/scripts/session/create-session-worklog.sh T-05-02 01 "TypeScript Bindings" "TS Dev 1"
# ... work on bindings using schema from T-05-01 ...
```

### Cleanup After Milestone

```shell
# After all M1 tasks merged
cd ~/dev-space/CDSAgent

# Remove all M1 worktrees
git worktree remove .worktrees/T-05-01-jsonrpc-schema
git worktree remove .worktrees/T-05-02-typescript-bindings
git worktree remove .worktrees/T-06-01-parity-methodology

# Remove symlinks
rm ~/dev-space/CDSAgent-T-05-01-jsonrpc-schema
rm ~/dev-space/CDSAgent-T-05-02-typescript-bindings
rm ~/dev-space/CDSAgent-T-06-01-parity-methodology

# Delete merged branches
git branch -d feat/task/T-05-01-jsonrpc-schema
git branch -d feat/task/T-05-02-typescript-bindings
git branch -d feat/task/T-06-01-parity-methodology

# Clean up git metadata
git worktree prune
git fetch --prune
```

**Result**: Multiple independent tasks developed in parallel, cleaned up efficiently.

---

## Scenario 3: Recovering from Worktree Issues

**Context**: Various worktree problems and solutions.

### Issue 1: Worktree Exists But Directory Missing

```shell
# Symptom: git worktree list shows worktree, but directory doesn't exist
git worktree list
# Output shows: /path/to/.worktrees/T-03-01 (feat/task/T-03-01)

ls .worktrees/T-03-01
# Output: No such file or directory

# Solution: Prune stale worktree metadata
git worktree prune

# Verify removal
git worktree list
# Output: Worktree no longer listed

# Recreate if needed
git worktree add .worktrees/T-03-01-cli-commands feat/task/T-03-01
```

### Issue 2: Branch Already Checked Out

```shell
# Symptom: Cannot create worktree because branch is checked out elsewhere
git worktree add .worktrees/T-02-02-sparse-index feat/task/T-02-02

# Output:
# fatal: 'feat/task/T-02-02-sparse-index' is already checked out at '/path/to/old-location'

# Solution: Find and remove old worktree
git worktree list | grep "feat/task/T-02-02"
# Output: /path/to/old-location (feat/task/T-02-02-sparse-index)

# Remove old worktree
git worktree remove /path/to/old-location

# Or force remove if has uncommitted changes
git worktree remove --force /path/to/old-location

# Retry creation
git worktree add .worktrees/T-02-02-sparse-index feat/task/T-02-02-sparse-index
# ✅ Success
```

### Issue 3: Broken Symlink

```shell
# Symptom: Symlink exists but points to deleted/moved worktree
cd ~/dev-space/CDSAgent-T-02-02
# Output: No such file or directory

# Find broken symlinks
ls -l ~/dev-space/CDSAgent-*
# Output shows broken symlink with different color

# Solution: Remove broken symlink
rm ~/dev-space/CDSAgent-T-02-02

# Recreate if worktree still exists
cd ~/dev-space/CDSAgent/.worktrees/T-02-02-sparse-index
./.dev/scripts/task/worktree-symlink.sh create

# Or find all broken symlinks and remove
find ~/dev-space/ -type l ! -exec test -e {} \; -delete
```

### Issue 4: Force Remove Worktree with Uncommitted Changes

```shell
# Symptom: Need to remove worktree but has uncommitted changes
git worktree remove .worktrees/T-03-01-cli-commands

# Output:
# fatal: '.worktrees/T-03-01-cli-commands' contains modified or untracked files, use --force to delete it

# Solution 1: Stash and remove
cd .worktrees/T-03-01-cli-commands
git stash push -m "Backup before worktree removal"
cd ~/dev-space/CDSAgent
git worktree remove .worktrees/T-03-01-cli-commands

# Solution 2: Force remove (loses changes)
git worktree remove --force .worktrees/T-03-01-cli-commands
```

### Issue 5: .dev/ Directory Missing in Worktree

```shell
# Symptom: Scripts not found in worktree
cd ~/dev-space/CDSAgent-T-02-02
ls .dev/
# Output: No such file or directory

# Solution: Sync .dev/ from main repository
cd ~/dev-space/CDSAgent
./.dev/scripts/task/sync-worktrees.sh

# Verify
ls ~/dev-space/CDSAgent-T-02-02/.dev/
# Output:
# README.md  scripts/  templates/  workflows/
```

**Result**: All common worktree issues resolved with standard solutions.

---

## Summary

These examples demonstrate:

1. **Creating and Removing Worktrees**: Complete lifecycle from creation to cleanup
2. **Parallel Development**: Multiple worktrees for simultaneous task development
3. **Recovering from Issues**: Common problems and solutions

**Key Takeaways**:

- ✅ **Use symlinks for convenience** - easier navigation to worktrees
- ✅ **Prune stale metadata** - `git worktree prune` after manual deletions
- ✅ **Force remove when needed** - `--force` flag for stuck worktrees
- ✅ **Clean up after merges** - remove worktrees, symlinks, and branches
- ✅ **Sync .dev/ regularly** - keep toolkit consistent across worktrees

**Important⚠️**: Always run `git worktree list` before creating worktrees to avoid "branch already checked out" errors.
**Important⚠️**: Wen Run [worktree-management] job, MUST run (date -u '+%Y-%m-%dT%H:%M:%SZ') get UTC TIME NOW first!!!
