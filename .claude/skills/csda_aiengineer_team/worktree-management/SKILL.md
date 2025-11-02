---
name: worktree-management
description: Manages git worktrees for CDSAgent spec-tasks including creation, symlinks, cleanup, and synchronization. Use when managing multiple task worktrees, fixing worktree issues, or understanding worktree architecture.
allowed-tools: Bash, Edit, Read, Write, Grep, Glob, Task, TodoWrite, SlashCommand, WebSearch, WebFetch
---

# CDSAgent Worktree Management

Complete worktree lifecycle management for CDSAgent development.

**Important⚠️**: Always run `git worktree list` before creating worktrees to avoid "branch already checked out" errors.
**Important⚠️**: Wen Run [worktree-management] job, MUST run (date -u '+%Y-%m-%dT%H:%M:%SZ') get UTC TIME NOW first!!!

## Capabilities

- Create and remove task worktrees
- Manage symlinks for convenient access
- Synchronize artifacts across worktrees
- Fix broken worktree states
- Understand worktree isolation

## Worktree Architecture Overview

Each task gets dedicated worktree:

- **Isolation**: Separate working directory per task
- **Parallel**: Work on multiple tasks simultaneously
- **Branch**: Each worktree has own branch
- **Artifacts**: `.artifacts/` content synced across worktrees via symlinks

```text
Main Repo (~arthur/dev-space/CDSAgent)
├── .worktrees/
│   ├── T-02-01-graph-builder/
│   │   ├── .dev/                    # Symlink to main .dev/
│   │   ├── .artifacts/              # Synced via symlink
│   │   └── <task-files>
│   ├── T-02-02-sparse-index/
│   │   ├── .dev/                    # Symlink to main .dev/
│   │   ├── .artifacts/              # Synced via symlink
│   │   └── <task-files>
│   └── ...
└── .dev/                            # Source for all .dev/ symlinks

Convenience Symlinks (~arthur/dev-space/)
├── CDSAgent-T-02-01 -> CDSAgent/.worktrees/T-02-01-graph-builder
├── CDSAgent-T-02-02 -> CDSAgent/.worktrees/T-02-02-sparse-index
└── ...
```

## How to Use

### Creating Worktree

From main repository:

```shell
# Create new worktree
git worktree add .worktrees/T-XX-XX-task-name feat/task/T-XX-XX-task-name

# Navigate to worktree
cd .worktrees/T-XX-XX-task-name

# Create convenience symlink
./.dev/scripts/task/worktree-symlink.sh create
```

### Managing Symlinks

**Create Symlink**:

```shell
# From worktree
./.dev/scripts/task/worktree-symlink.sh create

# Creates: ~/dev-space/CDSAgent-T-XX-XX -> .worktrees/T-XX-XX
```

**Remove Symlink**:

```shell
./.dev/scripts/task/worktree-symlink.sh remove
```

**List Symlinks**:

```shell
ls -l ~/dev-space/ | grep "CDSAgent-T-"
```

### Removing Worktree

When task is complete and merged:

```shell
# From main repo
git worktree remove .worktrees/T-XX-XX-task-name

# Remove symlink (if not auto-removed)
rm ~/dev-space/CDSAgent-T-XX-XX-task-name

# Clean worktree list
git worktree prune
```

### Syncing Artifacts

Synchronize `.artifacts/` across all worktrees:

```shell
# From any worktree or main repo
./.dev/scripts/task/sync-worktrees.sh
```

**Syncs**:

- Creates symlinks: `.artifacts/` → main repo `.artifacts/`
- Updates all worktrees with shared artifacts
- Preserves task-specific artifacts

### Listing Worktrees

```shell
# Show all worktrees
git worktree list

# Sample output:
# /Users/arthur/dev-space/CDSAgent                  abc1234 [main]
# /Users/arthur/dev-space/CDSAgent/.worktrees/T-02-01  def5678 [feat/task/T-02-01-graph-builder]
# /Users/arthur/dev-space/CDSAgent/.worktrees/T-02-02  ghi9012 [feat/task/T-02-02-sparse-index]
```

## Worktree Best Practices

1. **One worktree per task** - Never mix tasks in same worktree
2. **Use symlinks** - Convenience access via ~/dev-space/CDSAgent-T-XX-XX
3. **Sync before switch** - Run `sync-worktrees.sh` before switching tasks
4. **Clean when done** - Remove worktree after task merged
5. **Run scripts from worktree** - Always use relative paths: `./.dev/scripts/...`

## Worktree Isolation Benefits

- **Parallel Development**: Multiple tasks simultaneously
- **Clean Context**: Each task has isolated workspace
- **Branch Safety**: No accidental commits to wrong branch
- **Fast Switching**: `cd ~/dev-space/CDSAgent-T-XX-XX` instant switch
- **Artifact Sharing**: `.artifacts/` synced across all worktrees

## Common Worktree Issues

### Worktree Locked

**Symptom**: Cannot create/remove worktree

**Fix**:

```shell
# Unlock worktree
git worktree unlock .worktrees/T-XX-XX-task-name

# Or force remove
git worktree remove --force .worktrees/T-XX-XX-task-name
```

### Broken Symlink

**Symptom**: `~/dev-space/CDSAgent-T-XX-XX` points to non-existent directory

**Fix**:

```shell
# Remove broken symlink
rm ~/dev-space/CDSAgent-T-XX-XX-task-name

# Recreate from worktree
cd .worktrees/T-XX-XX-task-name
./.dev/scripts/task/worktree-symlink.sh create
```

### .dev/ Not Accessible

**Symptom**: `.dev/scripts/` not found in worktree

**Fix**:

```shell
# Check .dev/ is symlink
ls -la .dev/

# Recreate symlink
rm -rf .dev
ln -s ../../.dev .dev
```

### Artifacts Out of Sync

**Symptom**: Changes to `.artifacts/` not visible in other worktrees

**Fix**:

```shell
# Sync all worktrees
./.dev/scripts/task/sync-worktrees.sh

# Verify symlink
ls -la .artifacts/
```

## Worktree Directory Structure

```text
.worktrees/T-XX-XX-task-name/
├── .git                           # Worktree-specific git
├── .dev/                          # → Symlink to main .dev/
├── .artifacts/                    # → Symlink to main .artifacts/
├── crates/                        # Task code changes
├── spacs/                         # Shared specs
├── tests/                         # Shared tests
└── <other-shared-files>
```

## Scripts Location

Worktree management scripts in `.dev/scripts/task/`:

- `worktree-symlink.sh` - Manage convenience symlinks
- `sync-worktrees.sh` - Synchronize artifacts
- `create-task-worklog.sh` - Initialize task (creates worktree metadata)

## Advanced Operations

### Move Worktree

```shell
# Move worktree to new location
git worktree move .worktrees/T-XX-XX-old .worktrees/T-XX-XX-new

# Update symlink
cd .worktrees/T-XX-XX-new
./.dev/scripts/task/worktree-symlink.sh create
```

### Repair Worktree

```shell
# Repair corrupted worktree
git worktree repair

# Or repair specific worktree
git worktree repair .worktrees/T-XX-XX-task-name
```

### Detached HEAD in Worktree

```shell
# Check worktree status
git branch

# Checkout correct branch
git checkout feat/task/T-XX-XX-task-name
```

## Integration with Other Skills

- **task-initialization**: Creates initial worktree
- **session-management**: Sessions run in task worktree
- **git-workflow-validation**: Validates git state in worktree

## Worktree Lifecycle

1. **Create**: `git worktree add` + `worktree-symlink.sh create`
2. **Develop**: Multiple sessions in worktree
3. **Sync**: `sync-worktrees.sh` as needed
4. **Complete**: Task merged to main
5. **Remove**: `git worktree remove` + remove symlink
6. **Prune**: `git worktree prune` cleanup

## Exit Codes

- `0`: Operation successful
- `1`: Operation failed, check error message
- `2`: Warnings, review before proceeding

## References

- Main SOP: `.dev/workflows/WORKTREE_WORKFLOW.md`
- Task Initialization: `.dev/workflows/NEXT_TASK_CHECKLIST.md`
- Git Worktree Docs: `https://git-scm.com/docs/git-worktree`
- Symlink Management: `.dev/scripts/task/worktree-symlink.sh --help`
