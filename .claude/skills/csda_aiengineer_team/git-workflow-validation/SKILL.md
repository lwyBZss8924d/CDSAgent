---
name: git-workflow-validation
description: Validates git workflow compliance including git notes verification, checkpoint status, and commit message standards for CDSAgent development. Use when preparing to git push, after completing a checkpoint, reviewing git commit history, validating git notes format, or checking commit message standards. Critical before every git push to ensure all commits have notes.
allowed-tools: Bash, Edit, Read, Write, Grep, Glob, Task, TodoWrite, SlashCommand, WebSearch, WebFetch
---

# CDSAgent Git Workflow Validation

Ensures git workflow compliance for CDSAgent spec-tasks development.

**Important⚠️**: Wen Write [git-workflow-validation] job's commit message and git notes, MUST run (date -u '+%Y-%m-%dT%H:%M:%SZ') get UTC TIME NOW first!!!

## Quick Start

### Before Every Git Push (CRITICAL)

```shell
# From worktree - validate all commits have notes
./scripts/git-notes-check.sh
```

**Exit Codes**:

- `0`: All commits have notes, safe to push
- `1`: Missing notes, fix before push
- `2`: Warnings, review and decide

### git notes commands synopsis

```shell
git notes [list [<object>]]
git notes add [-f] [--allow-empty] [--[no-]separator | --separator=<paragraph-break>] [--[no-]stripspace] [-F <file> | -m <msg> | (-c | -C) <object>] [-e] [<object>]
git notes copy [-f] ( --stdin | <from-object> [<to-object>] )
git notes append [--allow-empty] [--[no-]separator | --separator=<paragraph-break>] [--[no-]stripspace] [-F <file> | -m <msg> | (-c | -C) <object>] [-e] [<object>]
git notes edit [--allow-empty] [<object>] [--[no-]stripspace]
git notes show [<object>]
git notes merge [-v | -q] [-s <strategy> ] <notes-ref>
git notes merge --commit [-v | -q]
git notes merge --abort [-v | -q]
git notes remove [--ignore-missing] [--stdin] [<object>…​]
git notes prune [-n] [-v]
git notes get-ref
```

### Adding Missing Git Notes

```shell
# Add note to commit
git notes add <commit-hash> -m "spec-tasks/T-XX-XX: Session NN Thread MM - description"

# Example
git notes add 414f7f2 -m "spec-tasks/T-02-02: Session 04 Thread 06 - BM25 index scaffold"

# Push notes to origin
git push origin refs/notes/commits
```

**Note Format**: `spec-tasks/T-XX-XX: Session NN Thread MM - brief description`

### Running Checkpoint Validation

```shell
# Run checkpoint helper
./scripts/checkpoint-helper.sh T-XX-XX

# Follow 5-phase workflow:
# Phase 1: Review session work
# Phase 2: Verify artifacts
# Phase 3: Update metadata
# Phase 4: Git operations (notes, commit)
# Phase 5: Final validation
```

## Git Workflow Overview

CDSAgent uses git notes to track task metadata:

- **Purpose**: Link commits to spec-tasks and sessions
- **Format**: `spec-tasks/T-XX-XX: Session NN Thread MM - description`
- **Storage**: Separate `refs/notes/commits` branch
- **Push**: Requires explicit push: `git push origin refs/notes/commits`

## Commit Message Standards

### Conventional Commits Format

```text
<type>(<scope>): <description>

[optional body]
```

**Types**: `feat`, `fix`, `docs`, `chore`, `refactor`, `test`, `checkpoint`, `init`

**Scopes**: Task ID (e.g., `T-02-02`), Component (e.g., `index`, `graph`), Phase (e.g., `worklog`)

**Examples**:

```text
feat(index): T-02-02 Phase 2 - tokenizer + BM25 scaffold
checkpoint(worklog): T-02-02 Session 04 complete
init(session): T-02-02 Day 2 Session 04 kickoff
```

## Validation Checklist

Before `git push`:

- [ ] All commits have git notes
- [ ] Notes follow format: `spec-tasks/T-XX-XX: Session NN Thread MM`
- [ ] Commit messages follow conventional format
- [ ] Checkpoint complete (if end of session)
- [ ] metadata.yaml updated
- [ ] Worklogs created
- [ ] RAW log created (if session end)

## Common Issues

### Missing Git Notes

**Symptom**: `git-notes-check.sh` reports missing notes

**Fix**:

```shell
# Find commits without notes
git log --all --oneline | while read hash msg; do
  git notes show $hash 2>/dev/null || echo "Missing: $hash $msg"
done

# Add notes
git notes add <hash> -m "spec-tasks/T-XX-XX: Session NN Thread MM - description"
```

### Wrong Note Format

**Fix**:

```shell
# Edit existing note
git notes edit <commit-hash>

# Or remove and recreate
git notes remove <commit-hash>
git notes add <commit-hash> -m "correct format"
```

### Notes Not Pushed

**Fix**:

```shell
# Push notes explicitly
git push origin refs/notes/commits

# Verify
git fetch origin refs/notes/commits:refs/notes/commits
git log --show-notes
```

## Best Practices

1. **Add notes immediately** after each commit
2. **Use consistent format** for easy parsing
3. **Reference session and thread** for traceability
4. **Push notes with commits** in same PR
5. **Never skip notes** - required for all task commits

## Bundled Scripts

This skill includes executable validation scripts:

- **scripts/git-notes-check.sh** - Verify all commits have notes
- **scripts/checkpoint-helper.sh** - Guide checkpoint workflow

Run scripts from skill directory or use `./.dev/scripts/validation/` directly from worktree.

## Exit Codes Reference

**git-notes-check.sh**:

- `0`: Success, all commits have notes
- `1`: Failure, missing notes
- `2`: Warnings, review recommended

**checkpoint-helper.sh**:

- `0`: Checkpoint approved, ready to commit
- `1`: Validation failed, fix issues
- `2`: Warnings, review before proceeding

## Related Skills

- **session-management**: Create sessions that generate commits needing notes
- **task-initialization**: Initial commit should have git note
- **worktree-management**: Understand worktree git operations

## Advanced Topics

For detailed information:

- **Complete Checkpoint Workflow**: See [REFERENCE.md](REFERENCE.md)
- **Real-World Scenarios**: See [EXAMPLES.md](EXAMPLES.md)
- **Git Notes Advanced Management**: See [REFERENCE.md](REFERENCE.md#advanced-git-notes-management)

## Integration with .dev/

This skill integrates with CDSAgent dev toolkit:

- **Workflows**: `.dev/workflows/WORK_SESSION_CHECKPOINT_WORKFLOW.md`
- **Scripts**: `.dev/scripts/validation/` (bundled in `scripts/`)
- **Checkpoint Guides**: `.dev/workflows/checkpoint/01-overview.md` through `11-commands.md`

## References

- Checkpoint Workflow: `.dev/workflows/WORK_SESSION_CHECKPOINT_WORKFLOW.md`
- Worktree SOP: `.dev/workflows/WORKTREE_WORKFLOW.md`
- Git Notes Documentation: `https://git-scm.com/docs/git-notes`
