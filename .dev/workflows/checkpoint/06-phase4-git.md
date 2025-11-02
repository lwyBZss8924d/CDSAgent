# Phase 4: Git Operations

**Part of**: [Work Session Checkpoint Workflow](./../WORK_SESSION_CHECKPOINT_WORKFLOW.md)

---

**Objective**: Commit code changes, add git notes, commit artifact updates, and push to remote

**Time Estimate**: 5-10 minutes

**Mode**: Git operations (code commits + checkpoint commits)

---

## Overview

Phase 4 handles all git operations in the correct sequence:

1. **Code Commit** (if applicable) - Commit day's code changes
2. **Git Notes** - Add task tracking metadata to code commit
3. **Update Artifacts** - Update metadata.yaml with new commit hash
4. **Checkpoint Commit** - Commit artifact updates
5. **Push Everything** - Push commits AND git notes to remote

⚠️ **CRITICAL**: Git notes must be pushed separately with `git push origin refs/notes/commits`

### Pre-Flight Check (Recommended)

**NEW**: Before starting Phase 4, run the checkpoint helper script to verify readiness:

```shell
# Run comprehensive pre-checkpoint checks
./.dev/scripts/validation/checkpoint-helper.sh [task_id]

# Example
./.dev/scripts/validation/checkpoint-helper.sh T-02-02-sparse-index
```

This automated tool checks:

- ✅ Git status (clean or only artifact changes)
- ✅ All commits have git notes
- ✅ Daily worklogs exist
- ✅ Metadata has no PENDING fields
- ✅ Commit count matches between git and metadata

**Output example**:

```text
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   Checkpoint Helper - Pre-flight Checks
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[1/5] Checking git status...
  ✓ Only artifact changes (expected)

[2/5] Checking git notes...
  ✓ All 3 commits have git notes

[3/5] Checking daily worklogs...
  ✓ Today's worklogs exist

[4/5] Checking metadata consistency...
  ✓ No PENDING fields in metadata
  ✓ Commit count consistent (3 in metadata, 3 in git)

[5/5] Checking artifact completeness...
  ✓ All required artifacts exist

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✓ Ready for checkpoint!
```

If any checks fail, the script will provide specific actions to fix them before proceeding.

---

## Step 4.1: Create Code Commit (If Applicable)

**When**: If there are uncommitted code changes from the work session

**Check for code changes**:

```shell
# Check status
git status

# View code file changes
git diff --stat
```

**If code changes exist, create commit**:

```shell
# Stage code files only
git add crates/cds-index/src/...
git add crates/cds-index/tests/...

# Create commit with documented message
git commit -m "$(cat <<'EOF'
[type]([scope]): [subject]

[body with implementation details]

[parity results / test results]

Files changed: X code files, +XXX/-XXX lines
EOF
)"
```

**Verify commit created**:

```shell
# Get commit hash
git log -1 --oneline
# Example output: 52c2b7e fix(parity): align SWE fixtures with TYPE_CHECKING import semantics
```

**Checklist**:

- [ ] Code files staged and committed
- [ ] Commit message follows conventional commits format
- [ ] Commit hash noted for next step

**If no code changes**: Skip to Step 4.3

---

## Step 4.2: Add Git Notes to Code Commit ⭐

**Purpose**: Attach task tracking metadata to code commit for workflow automation

**Why Git Notes**:

- Tracks day/session/duration without cluttering commit history
- Links code commits to worklog artifacts
- Enables automated task progress tracking
- Preserves development timeline metadata

**⚠️ CRITICAL**: This is a **required step**, not optional!

### Standard Git Notes Format

```shell
git notes add <commit-hash> -m "spec-tasks/T-XX-XX-task-name
Day: X
Date: YYYY-MM-DD
Sessions: X-XX to X-XX (HH:MM-HH:MM UTC)
Duration: Xh
Worklog: .artifacts/spec-tasks-T-XX-XX/worklogs/YYYY-MM-DD-*
Parity: [parity status or N/A]
Status: [one-line summary of work completed]
Files: X code files (+XXX/-XXX lines)"
```

### Real Example

```shell
git notes add 52c2b7e -m "spec-tasks/T-02-01-graph-builder
Day: 5
Date: 2025-10-29
Sessions: 3-03 to 3-07 (13:30-18:30 UTC)
Duration: 5h
Worklog: .artifacts/spec-tasks-T-02-01-graph-builder/worklogs/2025-10-29-*
Parity: All fixtures ≤2% (imports/inherits 0%, invokes +1.29%)
Status: TYPE_CHECKING & scoped imports resolved
Files: 7 code files (+1,072/-140 lines)"
```

### Verify Git Notes Added

```shell
# Show notes for specific commit
git notes show 52c2b7e

# List all commits with notes
git notes list
```

**Expected Output**:

```text
spec-tasks/T-02-01-graph-builder
Day: 5
Date: 2025-10-29
Sessions: 3-03 to 3-07 (13:30-18:30 UTC)
Duration: 5h
Worklog: .artifacts/spec-tasks-T-02-01-graph-builder/worklogs/2025-10-29-*
Parity: All fixtures ≤2% (imports/inherits 0%, invokes +1.29%)
Status: TYPE_CHECKING & scoped imports resolved
Files: 7 code files (+1,072/-140 lines)
```

**Checklist**:

- [ ] Git notes added to code commit
- [ ] Format follows standard template
- [ ] Day, Date, Sessions filled correctly
- [ ] Parity/Status summarized accurately
- [ ] Verified with `git notes show <hash>`

### Automated Verification

**NEW**: Use `git-notes-check.sh` to verify all commits have notes:

```shell
# Verify git notes on all commits
./.dev/scripts/validation/git-notes-check.sh

# Or specify custom base commit
./.dev/scripts/validation/git-notes-check.sh <base-commit-hash>
```

**Output example (success)**:

```text
Git Notes Verification
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Branch: feat/task/T-02-02-sparse-index
Base:   origin/main

Checking 3 commit(s)...

✓ 52c2b7e - Notes present
✓ a3f4d89 - Notes present
✓ 7e2c1b0 - Notes present

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✓ All commits have valid git notes!
```

**Output example (failure)**:

```text
✗ Git notes check FAILED

Commits missing notes (2):
  - a3f4d89: feat(index): implement BM25 scoring
  - 7e2c1b0: test(index): add BM25 unit tests

How to fix:
1. Add git notes to each commit:
   git notes add -m "spec-tasks/T-XX-XX-task-name
   ...
```

This automated check ensures you don't forget the git notes step before pushing!

---

## Step 4.3: Update metadata.yaml with Commit Hash

**Purpose**: Record code commit in task metadata for tracking

**Edit metadata.yaml**:

```shell
vim .artifacts/spec-tasks-T-XX-XX/metadata.yaml
```

**Add new commit entry**:

```yaml
git:
  commits:
    # ... previous commits ...
    - hash: "52c2b7e"
      message: "fix(parity): align SWE fixtures with TYPE_CHECKING import semantics"
      date: "2025-10-29"
      files_changed: 7
      notes: "Day 5 (Sessions 3-03 to 3-07): Scoped TYPE_CHECKING & SWE parity improvements. ..."
```

**Update cumulative metrics**:

```yaml
metrics:
  lines_added: 8265   # Previous + new commit
  lines_deleted: 295   # Previous + new commit
  files_modified: 62   # Total unique files in branch
```

**Checklist**:

- [ ] Commit hash added to git_commits section
- [ ] Commit message, date, files_changed filled
- [ ] Cumulative metrics updated
- [ ] Saved changes

---

## Step 4.4: Stage Artifact Changes

**⚠️ IMPORTANT**: Only stage artifact files, not code files (already committed)

```shell
# Stage updated artifacts
git add .artifacts/spec-tasks-T-XX-XX/metadata.yaml
git add .artifacts/spec-tasks-T-XX-XX/worklogs/*.md
git add .artifacts/spec-tasks-T-XX-XX/worklogs/raw/*.txt

# Or stage all artifacts at once
git add .artifacts/spec-tasks-T-XX-XX/

# Verify staged files (should only be artifacts)
git diff --cached --stat
```

**Expected Output**:

```text
 .artifacts/spec-tasks-T-02-01/metadata.yaml                     |  8 +++++---
 .artifacts/spec-tasks-T-02-01/worklogs/2025-10-29-commit-log.md |  2 ++
 .artifacts/spec-tasks-T-02-01/worklogs/2025-10-29-work-summary.md | 4 ++--
 3 files changed, 9 insertions(+), 5 deletions(-)
```

**Checklist**:

- [ ] Only artifact files staged
- [ ] No code files (e.g., `crates/`, `src/`) staged
- [ ] Verified with `git diff --cached`

---

## Step 4.5: Create Checkpoint Commit

**Commit Message Format**:

```text
checkpoint(worklog): T-XX-XX Day X complete - artifacts updated with code commit <hash>

[Optional body explaining updates made]
```

**Example**:

```shell
git commit -m "checkpoint(worklog): T-02-01 Day 5 complete - artifacts updated with code commit 52c2b7e

Updated artifacts for Day 5 work session (2025-10-29):
- Added commit 52c2b7e to metadata.yaml git_commits section
- Updated cumulative metrics (lines_added: 8,265, files_modified: 62)
- Fixed file count in worklogs (7 code files including mod.rs)
- Updated action log statistics to match git reality

All artifacts now 100% consistent with git operations.
Git notes added to code commit for task tracking."
```

**Verify commit**:

```shell
git log -1 --oneline
# Example: 664596b checkpoint(worklog): T-02-01 Day 5 complete - artifacts updated with code commit 52c2b7e
```

**Checklist**:

- [ ] Checkpoint commit created
- [ ] Message references code commit hash
- [ ] Body summarizes artifact updates
- [ ] Mentions git notes added

---

## Step 4.6: Push Everything to Remote ⭐

**⚠️ CRITICAL**: Must push BOTH commits AND git notes!

### Commands

```shell
# Step 1: Push commits
git push origin feat/task/T-XX-XX-task-name

# Step 2: Push git notes ⭐ DON'T FORGET THIS!
git push origin refs/notes/commits
```

### Why Two Pushes?

- **Git commits** push: Syncs your code commits and checkpoint commits
- **Git notes push**: Syncs task tracking metadata (not pushed by default!)

Without the second command, git notes remain local-only and won't be visible to:

- Other developers
- CI/CD systems
- Task tracking automation
- Remote viewers (GitHub, GitLab, etc.)

### Verify Push Succeeded

```shell
# Check status
git status
```

**Expected Output**:

```text
On branch feat/task/T-XX-XX-task-name
Your branch is up to date with 'origin/feat/task/T-XX-XX-task-name'.

nothing to commit, working tree clean
```

**Checklist**:

- [ ] Commits pushed: `git push origin <branch>` ✅
- [ ] Git notes pushed: `git push origin refs/notes/commits` ✅
- [ ] No errors during push
- [ ] `git status` shows "up to date"
- [ ] Remote branch verified on GitHub/GitLab

---

## Common Issues

### Issue: Forgot to Push Git Notes

**Symptoms**: Git notes visible locally but not on remote

**Fix**:

```shell
# Push notes separately
git push origin refs/notes/commits
```

### Issue: Git Notes Overwrite Conflict

**Symptoms**: `! [rejected] refs/notes/commits -> refs/notes/commits (fetch first)`

**Fix**:

```shell
# Fetch remote notes first
git fetch origin refs/notes/*:refs/notes/*

# Merge notes
git notes merge origin/notes/commits

# Push again
git push origin refs/notes/commits
```

### Issue: Pushed Code Commit Without Git Notes

**Symptoms**: Code commit exists but no notes attached

**Fix**:

```shell
# Add notes to existing commit (even if already pushed)
git notes add <commit-hash> -m "..."

# Push notes
git push origin refs/notes/commits
```

---

## Summary Checklist

**Complete Phase 4 Checklist**:

- [ ] **Step 4.1**: Code commit created (if applicable)
- [ ] **Step 4.2**: Git notes added to code commit ⭐
- [ ] **Step 4.3**: metadata.yaml updated with commit hash
- [ ] **Step 4.4**: Artifact files staged
- [ ] **Step 4.5**: Checkpoint commit created
- [ ] **Step 4.6a**: Commits pushed to remote ✅
- [ ] **Step 4.6b**: Git notes pushed to remote ⭐ ✅

**If all checked**: ✅ Phase 4 Complete → Proceed to [Phase 5: Final Verification](07-phase5-final.md)

---

**Navigation**:

- [← Back to Phase 3](05-phase3-update.md)
- [← Back to Main Index](./../WORK_SESSION_CHECKPOINT_WORKFLOW.md)
- [→ Next: Phase 5 - Final Verification](07-phase5-final.md)
