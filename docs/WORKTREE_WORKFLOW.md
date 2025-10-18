# Git Worktree Development Workflow

This document describes the git worktree workflow for parallel task development in CDSAgent.

## Overview

CDSAgent uses git worktrees to enable parallel development on multiple tasks without switching branches or maintaining multiple repository clones.

### Benefits

- **Parallel Development**: Work on multiple tasks simultaneously
- **Isolated Environments**: Each worktree is a separate working directory
- **IDE-Friendly**: Use symlinks for easier navigation
- **No Context Switching**: Keep different branches checked out at once
- **Clean History**: Each task branch remains focused and organized

## Directory Structure

```
CDSAgent/                              # Main repository (feat/dev-environment-setup)
├── .worktrees/                        # Worktree storage (gitignored)
│   ├── T-02-01-graph-builder/         # Task branch worktree
│   ├── T-02-02-sparse-index/
│   ├── T-02-03-service-layer/
│   ├── T-03-01-cli-commands/
│   ├── T-04-01-agent-sdk/
│   ├── T-04-02-prompt-design/
│   └── T-05-01-jsonrpc-schema/
└── scripts/worktree-symlink.sh       # Symlink manager

# IDE-friendly symlinks in ~/dev-space/
~/dev-space/CDSAgent-T-02-01-graph-builder -> CDSAgent/.worktrees/T-02-01-graph-builder
~/dev-space/CDSAgent-T-02-02-sparse-index  -> CDSAgent/.worktrees/T-02-02-sparse-index
...
```

## Quick Start

### 1. List Existing Worktrees

```bash
# View all worktrees and their branches
./scripts/worktree-symlink.sh list

# Or use git directly
git worktree list
```

### 2. Start Working on a Task

```bash
# Navigate to the task worktree via symlink
cd ~/dev-space/CDSAgent-T-05-01-jsonrpc-schema

# Or directly
cd .worktrees/T-05-01-jsonrpc-schema

# Verify you're on the correct branch
git branch --show-current  # Should show: feat/task/T-05-01-jsonrpc-schema
```

### 3. Make Changes and Commit

```bash
# Normal git workflow
git add .
git commit -m "feat(api): implement JSON-RPC schema validation"
git push -u origin feat/task/T-05-01-jsonrpc-schema
```

### 4. Create Pull Request

```bash
# Using GitHub CLI (recommended)
gh pr create --title "feat(api): T-05-01 - JSON-RPC Schema Definition" \
  --body "Implements task T-05-01: JSON-RPC schema with validation" \
  --base main

# Or push and create PR via GitHub web UI
```

### 5. After Merge: Clean Up

```bash
# Switch to main worktree
cd ~/dev-space/CDSAgent

# Update main branch
git checkout main
git pull origin main

# Remove the worktree
git worktree remove .worktrees/T-05-01-jsonrpc-schema

# Remove symlink
./scripts/worktree-symlink.sh remove
```

## Creating New Task Worktrees

### Manual Method

```bash
# From main repository
cd ~/dev-space/CDSAgent

# Create new worktree branched from main
git worktree add .worktrees/T-XX-XX-task-name -b feat/task/T-XX-XX-task-name main

# Create symlink
ln -s $(pwd)/.worktrees/T-XX-XX-task-name ~/dev-space/CDSAgent-T-XX-XX-task-name

# Verify
git worktree list
```

### Using Helper Script

```bash
# Create symlinks for all existing worktrees
./scripts/worktree-symlink.sh create

# List all worktrees and symlinks
./scripts/worktree-symlink.sh list

# Remove all symlinks (keeps worktrees)
./scripts/worktree-symlink.sh remove
```

## Workflow Best Practices

### Branch Naming Convention

```
feat/task/T-XX-XX-short-description
```

Examples:
- `feat/task/T-05-01-jsonrpc-schema`
- `feat/task/T-02-01-graph-builder`
- `feat/task/T-04-02-prompt-design`

### Commit Message Format

```
<type>(<scope>): <subject>

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `test`: Adding tests
- `refactor`: Code refactoring
- `chore`: Maintenance tasks

Examples:
```bash
git commit -m "feat(graph): implement Python AST parser with tree-sitter"
git commit -m "test(index): add BM25 search benchmark tests"
git commit -m "docs(api): document JSON-RPC error codes"
```

### Task Development Lifecycle

1. **Start Task**
   - Navigate to worktree: `cd ~/dev-space/CDSAgent-T-XX-XX`
   - Read task spec: `cat spacs/tasks/0.1.0-mvp/.../T-XX-XX.md`
   - Review dependencies and acceptance criteria

2. **Implement**
   - Write code following task requirements
   - Add unit tests (target >80% coverage)
   - Run local tests: `cargo test` or `bun test`
   - Run lints: `cargo clippy --all-targets`

3. **Validate**
   - Check task acceptance criteria
   - Run integration tests if applicable
   - Compare with LocAgent baseline (for core tasks)
   - Build: `cargo build --release`

4. **Submit**
   - Commit with descriptive message
   - Push to origin: `git push -u origin feat/task/T-XX-XX`
   - Create PR referencing task ID
   - Request review from tech lead

5. **Merge & Cleanup**
   - Address review comments
   - Merge to main via GitHub
   - Delete remote branch (GitHub auto-delete)
   - Remove worktree: `git worktree remove .worktrees/T-XX-XX`
   - Update main: `git checkout main && git pull`

## IDE Integration

### VSCode

Open worktree directly in VSCode:

```bash
# From command line
code ~/dev-space/CDSAgent-T-05-01-jsonrpc-schema

# Or use VSCode "Open Folder" and navigate to symlink
```

### Cursor / Other IDEs

Symlinks work transparently with all IDEs. Simply open the symlinked directory:

```
~/dev-space/CDSAgent-T-05-01-jsonrpc-schema
```

The IDE will treat it as a normal directory, with full git integration.

## Advanced Operations

### Switching Between Tasks

```bash
# No need to switch! Just open different terminal tabs
cd ~/dev-space/CDSAgent-T-02-01-graph-builder  # Terminal 1
cd ~/dev-space/CDSAgent-T-05-01-jsonrpc-schema  # Terminal 2
```

### Sharing Code Between Worktrees

```bash
# From worktree A, reference shared code in main
cd ~/dev-space/CDSAgent-T-02-01-graph-builder

# Cherry-pick commit from another branch
git cherry-pick <commit-hash>

# Or rebase on main to get latest changes
git fetch origin main
git rebase origin/main
```

### Syncing with Main Branch

```bash
# Update main in primary repo
cd ~/dev-space/CDSAgent
git checkout main
git pull origin main

# Update task branch with latest main
cd ~/dev-space/CDSAgent-T-XX-XX
git fetch origin main
git rebase origin/main  # or: git merge origin/main
```

### Pruning Merged Worktrees

```bash
# After PR is merged, clean up
git worktree list | grep T- | while read -r path commit branch; do
  if git branch -r --merged main | grep -q "${branch#*]}"; then
    echo "Removing merged worktree: $path"
    git worktree remove "$path"
  fi
done

# Remove all stale symlinks
./scripts/worktree-symlink.sh remove
./scripts/worktree-symlink.sh create  # Recreate for remaining worktrees
```

## Troubleshooting

### Error: "worktree already exists"

```bash
# List worktrees to find the conflict
git worktree list

# Remove the existing worktree
git worktree remove .worktrees/T-XX-XX

# Or remove with force if it has uncommitted changes
git worktree remove --force .worktrees/T-XX-XX
```

### Error: "branch already exists"

```bash
# Delete the branch if no longer needed
git branch -D feat/task/T-XX-XX

# Or use a different branch name
git worktree add .worktrees/T-XX-XX-v2 -b feat/task/T-XX-XX-v2 main
```

### Symlink Broken After Repository Move

```bash
# Remove all symlinks
./scripts/worktree-symlink.sh remove

# Recreate symlinks
./scripts/worktree-symlink.sh create
```

### Worktree Out of Sync with Main

```bash
cd ~/dev-space/CDSAgent-T-XX-XX

# Fetch latest main
git fetch origin main

# Option 1: Rebase (clean history)
git rebase origin/main

# Option 2: Merge (preserves all commits)
git merge origin/main

# If conflicts, resolve and continue
git add .
git rebase --continue  # or: git merge --continue
```

## Task Dependency Management

Some tasks have dependencies on others. Check the task README files:

```bash
# Example: T-02-03 depends on T-05-01
cat spacs/tasks/0.1.0-mvp/02-index-core/README.md

# Wait for dependent task PR to merge before starting
# Or coordinate with parallel developer
```

## Reference: Current Task Branches

| Task ID | Branch | Status | Owner |
|---------|--------|--------|-------|
| T-05-01 | feat/task/T-05-01-jsonrpc-schema | Active | Rust Dev 1 + TS Dev 1 |
| T-02-01 | feat/task/T-02-01-graph-builder | Pending | Rust Dev 1 |
| T-02-02 | feat/task/T-02-02-sparse-index | Pending | Rust Dev 2 |
| T-02-03 | feat/task/T-02-03-service-layer | Pending | Rust Dev 1 |
| T-03-01 | feat/task/T-03-01-cli-commands | Pending | Rust Dev 2 |
| T-04-01 | feat/task/T-04-01-agent-sdk | Pending | TS Dev 1 |
| T-04-02 | feat/task/T-04-02-prompt-design | Pending | TS Dev 1 |

## Resources

- [Git Worktree Documentation](https://git-scm.com/docs/git-worktree)
- [Task Specifications](../spacs/tasks/0.1.0-mvp/)
- [Issue Breakdown](../spacs/issues/04-0.1.0-mvp/)
- [PRD Documentation](../spacs/prd/0.1.0-MVP-PRDs-v0/)

---

**Last Updated**: 2025-10-19
**Maintainer**: CDSAgent Development Team
