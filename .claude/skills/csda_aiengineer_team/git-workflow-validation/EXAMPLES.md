# Git Workflow Validation - Real-World Examples

This document provides practical scenarios demonstrating git workflow validation in action.

## Table of Contents

- [Scenario 1: Before Every Git Push](#scenario-1-before-every-git-push)
- [Scenario 2: Adding Missing Git Notes After Validation Failure](#scenario-2-adding-missing-git-notes-after-validation-failure)
- [Scenario 3: Running Checkpoint Validation](#scenario-3-running-checkpoint-validation)
- [Scenario 4: Fixing Wrong Note Format](#scenario-4-fixing-wrong-note-format)
- [Scenario 5: Complete Session Checkpoint Workflow](#scenario-5-complete-session-checkpoint-workflow)
- [Scenario 6: CI/CD Integration](#scenario-6-cicd-integration)

---

## Scenario 1: Before Every Git Push

**Context**: You've completed work on T-02-02 Session 04 and want to push your commits.

**Workflow**:

```shell
# 1. Check current branch and status
git status
# On branch feat/task/T-02-02-sparse-index
# Your branch is ahead of 'origin/feat/task/T-02-02-sparse-index' by 2 commits.

# 2. CRITICAL: Validate git notes before push
./.dev/scripts/validation/git-notes-check.sh

# Output (SUCCESS):
# ✅ All 2 commits have git notes
# ✅ Ready to push
```

**Result**: All commits have notes, safe to push to remote.

```shell
# 3. Push commits and notes
git push origin feat/task/T-02-02-sparse-index
git push origin refs/notes/commits
```

---

## Scenario 2: Adding Missing Git Notes After Validation Failure

**Context**: Git notes validation fails, blocking your push.

**Workflow**:

```shell
# 1. Run validation
./.dev/scripts/validation/git-notes-check.sh

# Output (FAILURE):
# ❌ Missing git notes for 2 commits:
# 414f7f2 feat(index): T-02-02 Phase 2 - tokenizer + BM25 scaffold
# 9e1774e checkpoint(worklog): T-02-02 Session 04 complete

# 2. Add missing notes
git notes add 414f7f2 -m "spec-tasks/T-02-02: Session 04 Thread 06 - BM25 index scaffold"
git notes add 9e1774e -m "spec-tasks/T-02-02: Session 04 Checkpoint - Phase 2 delivered"

# 3. Verify notes were added
git log --show-notes -2

# Output:
# commit 9e1774e
# Author: Claude Code Agent
# Date:   Fri Nov 1 12:45:00 2025
#
#     checkpoint(worklog): T-02-02 Session 04 complete
#
# Notes:
#     spec-tasks/T-02-02: Session 04 Checkpoint - Phase 2 delivered
#
# commit 414f7f2
# Author: Claude Code Agent
# Date:   Fri Nov 1 11:15:00 2025
#
#     feat(index): T-02-02 Phase 2 - tokenizer + BM25 scaffold
#
# Notes:
#     spec-tasks/T-02-02: Session 04 Thread 06 - BM25 index scaffold

# 4. Re-run validation
./.dev/scripts/validation/git-notes-check.sh
# ✅ All 2 commits have git notes
# ✅ Ready to push

# 5. Push notes to remote
git push origin refs/notes/commits
```

**Result**: Missing notes added, validation passes, ready to push.

---

## Scenario 3: Running Checkpoint Validation

**Context**: End of Session 04, ready to create checkpoint commit.

**Workflow**:

```shell
# 1. Run checkpoint helper
./.dev/scripts/validation/checkpoint-helper.sh T-02-02

# Interactive prompts:
#
# === Phase 1: Review Session Work ===
# ✅ All threads completed (Threads 01-07)
# ✅ Code changes compile and pass tests (78/78 passing)
# ✅ Session objectives met (Phase 2 tokenizer + BM25 scaffold)
# ⚠️  Uncommitted changes detected:
#     - .artifacts/spec-tasks-T-02-02-sparse-index/metadata.yaml
#     - .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-11-01-S04-work-summary.md
#     - .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-11-01-S04-commit-log.md
#     - .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-11-01-S04-notes.md
#
# Continue? [y/N]: y
#
# === Phase 2: Verify Artifacts ===
# ✅ Work summary: .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-11-01-S04-work-summary.md
# ✅ Commit log: .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-11-01-S04-commit-log.md
# ✅ Technical notes: .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-11-01-S04-notes.md
# ✅ RAW log: .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/raw/WORK-SESSIONS-04-THREADS-01-07-SUMMARY-2025-11-01.txt
# ✅ Code review: .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-11-01-S04-codereview.md
#
# Continue? [y/N]: y
#
# === Phase 3: Update Metadata ===
# ✅ metadata.yaml: Session 04 entry added
# ✅ metadata.yaml: last_updated timestamp current
# ✅ metadata.yaml: actual_hours incremented (8.3h)
# ✅ metadata.yaml: metrics updated (lines_added, tests_added)
# ✅ metadata.yaml: git commits listed
#
# Continue? [y/N]: y
#
# === Phase 4: Git Operations ===
# Running git notes validation...
# ✅ All commits have git notes
#
# Create checkpoint commit? [y/N]: y
#
# Checkpoint commit message:
# checkpoint(worklog): T-02-02 Day 2 Session 04 complete - Phase 2 Tokenizer + BM25 delivered
#
# Confirm? [y/N]: y
#
# [feat/task/T-02-02-sparse-index 9e1774e] checkpoint(worklog): T-02-02 Day 2 Session 04 complete
#  5 files changed, 1232 insertions(+), 70 deletions(-)
#
# Add git note to checkpoint commit? [y/N]: y
# ✅ Git note added: spec-tasks/T-02-02: Session 04 Checkpoint - Phase 2 delivered
#
# === Phase 5: Final Validation ===
# ✅ All artifacts committed
# ✅ All commits have git notes
# ⚠️  Git notes not pushed to origin (run: git push origin refs/notes/commits)
# ✅ metadata.yaml updated
# ✅ Checkpoint commit created
#
# Ready to push? [y/N]: y
#
# Pushing to origin...
# ✅ Pushed feat/task/T-02-02-sparse-index
# ✅ Pushed refs/notes/commits
#
# === Checkpoint Complete ===
# Exit code: 0
```

**Result**: Checkpoint validated, committed, and pushed successfully.

---

## Scenario 4: Fixing Wrong Note Format

**Context**: Existing git note doesn't follow `spec-tasks/T-XX-XX: Session NN Thread MM` format.

**Workflow**:

```shell
# 1. Check current note
git notes show 414f7f2

# Output (WRONG FORMAT):
# T-02-02: BM25 scaffold

# 2. Edit note to correct format
git notes edit 414f7f2

# In editor, change to:
# spec-tasks/T-02-02: Session 04 Thread 06 - BM25 index scaffold

# Save and exit editor

# 3. Verify corrected note
git notes show 414f7f2

# Output (CORRECT FORMAT):
# spec-tasks/T-02-02: Session 04 Thread 06 - BM25 index scaffold

# 4. Push corrected note
git push origin refs/notes/commits --force
```

**Alternative Method** (remove and recreate):

```shell
# 1. Remove incorrect note
git notes remove 414f7f2

# 2. Add correct note
git notes add 414f7f2 -m "spec-tasks/T-02-02: Session 04 Thread 06 - BM25 index scaffold"

# 3. Push corrected note
git push origin refs/notes/commits --force
```

**Result**: Git note format corrected and synced to remote.

---

## Scenario 5: Complete Session Checkpoint Workflow

**Context**: End-to-end session checkpoint for T-02-01 Day 6 Session 03.

**Timeline**:

### Thread 01 (12:02-12:13, 11min) - Planning

```shell
# Work: Review Phase 1 spec, plan implementation
# No commits yet
```

### Thread 02 (12:13-14:48, 2.6h) - Implementation

```shell
# Work: Implement name_index.rs, exact_match, prefix_match, from_graph

# Commit work
git add crates/cds-index/src/index/name_index.rs
git add crates/cds-index/src/index/mod.rs
git commit -m "feat(index): T-02-01 Phase 1 - implement NameIndex upper tier"

# Add git note IMMEDIATELY after commit
git notes add HEAD -m "spec-tasks/T-02-01: Session 03 Thread 02 - NameIndex implementation"
```

### Thread 03 (14:49-15:05, 16min) - Validation

```shell
# Work: Add benchmarks, run tests, measure coverage

# Commit benchmarks
git add crates/cds-index/benches/search_bench.rs
git add crates/cds-index/tests/index_tests.rs
git commit -m "test(index): T-02-01 Phase 1 - add NameIndex benchmarks and tests"

# Add git note
git notes add HEAD -m "spec-tasks/T-02-01: Session 03 Thread 03 - benchmark validation"
```

### Thread 04 (15:14-15:17, 3min) - Coverage Hardening

```shell
# Work: Add edge case tests

# Commit coverage improvements
git add crates/cds-index/tests/index_tests.rs
git commit -m "test(index): T-02-01 Phase 1 - harden test coverage to 97%"

# Add git note
git notes add HEAD -m "spec-tasks/T-02-01: Session 03 Thread 04 - coverage hardening"
```

### Checkpoint (15:17-15:30, 13min)

```shell
# 1. Create session artifacts
./.dev/scripts/session/create-raw-log.sh T-02-01 03 01 04 "Phase 1 Upper Index delivered"

# Creates: .artifacts/spec-tasks-T-02-01-graph-builder/worklogs/raw/WORK-SESSIONS-03-THREADS-01-04-SUMMARY-2025-10-31.txt

# 2. Update metadata.yaml
# - Add Session 03 entry
# - Update actual_hours: 8.4h (Sessions 01-03)
# - Update metrics: lines_added, tests_added

# 3. Create session-specific worklogs
# - 2025-10-31-S03-work-summary.md
# - 2025-10-31-S03-commit-log.md
# - 2025-10-31-S03-notes.md

# 4. Stage checkpoint artifacts
git add .artifacts/spec-tasks-T-02-01-graph-builder/

# 5. Validate git notes
./.dev/scripts/validation/git-notes-check.sh
# ✅ All 3 commits have git notes

# 6. Create checkpoint commit
git commit -m "checkpoint(worklog): T-02-01 Day 1 Session 03 complete - Phase 1 Upper Index delivered"

# 7. Add git note to checkpoint
git notes add HEAD -m "spec-tasks/T-02-01: Session 03 Checkpoint - Phase 1 delivered"

# 8. Push everything
git push origin feat/task/T-02-01-graph-builder
git push origin refs/notes/commits
```

**Result**: Complete session with 4 commits (3 work + 1 checkpoint), all with git notes, artifacts committed, pushed to remote.

---

## Scenario 6: CI/CD Integration

**Context**: Setting up GitHub Actions to enforce git notes on all commits.

### GitHub Actions Workflow

Create `.github/workflows/validate-git-workflow.yml`:

```yaml
name: Validate Git Workflow

on:
  pull_request:
    branches: [main, feat/**]
  push:
    branches: [feat/**]

jobs:
  validate-notes:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v3
        with:
          fetch-depth: 0  # Fetch all history for git notes

      - name: Fetch git notes
        run: |
          git fetch origin refs/notes/commits:refs/notes/commits

      - name: Validate git notes
        run: |
          ./.dev/scripts/validation/git-notes-check.sh
          if [ $? -ne 0 ]; then
            echo "❌ Missing git notes! Please add notes to all commits."
            echo "Format: spec-tasks/T-XX-XX: Session NN Thread MM - description"
            echo ""
            echo "Example:"
            echo "  git notes add <hash> -m \"spec-tasks/T-02-02: Session 04 Thread 06 - BM25 scaffold\""
            echo "  git push origin refs/notes/commits"
            exit 1
          fi

      - name: Validate commit messages
        run: |
          # Check conventional commits format
          git log --format=%s origin/main..HEAD | while read msg; do
            if ! echo "$msg" | grep -qE '^(feat|fix|docs|chore|refactor|test|checkpoint|init)\(.*\):'; then
              echo "❌ Invalid commit message: $msg"
              echo "Expected format: <type>(<scope>): <description>"
              echo ""
              echo "Valid types: feat, fix, docs, chore, refactor, test, checkpoint, init"
              echo "Example: feat(index): T-02-02 Phase 2 - tokenizer implementation"
              exit 1
            fi
          done

      - name: Report success
        run: |
          echo "✅ Git workflow validation passed!"
          echo "✅ All commits have git notes"
          echo "✅ All commit messages follow conventional format"
```

### Pre-Push Git Hook

Create `.git/hooks/pre-push`:

```shell
#!/bin/bash
# CDSAgent pre-push hook - validates git notes before push

echo "🔍 Validating git workflow before push..."
echo ""

# Run git notes validation
./.dev/scripts/validation/git-notes-check.sh
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
  echo ""
  echo "✅ Git workflow validation passed"
  echo "✅ Proceeding with push..."
  exit 0
elif [ $EXIT_CODE -eq 1 ]; then
  echo ""
  echo "❌ Push blocked: Missing git notes!"
  echo ""
  echo "Fix: Add notes to commits before pushing"
  echo "  git notes add <hash> -m \"spec-tasks/T-XX-XX: Session NN Thread MM - description\""
  echo "  git push origin refs/notes/commits"
  echo ""
  exit 1
elif [ $EXIT_CODE -eq 2 ]; then
  echo ""
  echo "⚠️  Warnings detected. Review and decide:"
  echo ""
  read -p "Continue with push? [y/N]: " -n 1 -r
  echo
  if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "✅ Proceeding with push..."
    exit 0
  else
    echo "❌ Push cancelled by user"
    exit 1
  fi
fi
```

Make executable:

```shell
chmod +x .git/hooks/pre-push
```

### CI Failure Example

**Pull Request #7**: Developer forgets to add git notes.

```shell
❌ Validate Git Workflow / validate-notes
Missing git notes! Please add notes to all commits.

Commit without notes:
  a1b2c3d feat(index): add search optimization

Format: spec-tasks/T-XX-XX: Session NN Thread MM - description

Example:
  git notes add a1b2c3d -m "spec-tasks/T-03-01: Session 05 Thread 02 - search optimization"
  git push origin refs/notes/commits
```

**Developer Fix**:

```shell
# Add missing note
git notes add a1b2c3d -m "spec-tasks/T-03-01: Session 05 Thread 02 - search optimization"

# Push note
git push origin refs/notes/commits

# Re-run CI (automatic on push)
```

**CI Success**:

```text
✅ Validate Git Workflow / validate-notes
Git workflow validation passed!
✅ All commits have git notes
✅ All commit messages follow conventional format
```

**Result**: PR approved, ready to merge.

---

## Summary

These examples demonstrate:

1. **Before Push Validation**: Always run `git-notes-check.sh` before pushing
2. **Adding Missing Notes**: Quick recovery when validation fails
3. **Checkpoint Workflow**: End-to-end session completion with artifacts
4. **Note Format Correction**: Fixing existing notes to match standards
5. **Complete Session Example**: Real-world multi-thread session workflow
6. **CI/CD Integration**: Automated enforcement via GitHub Actions and git hooks

**Key Takeaways**:

- ✅ **Always validate before push** - prevents upstream issues
- ✅ **Add notes immediately after commit** - don't batch them later
- ✅ **Use consistent format** - `spec-tasks/T-XX-XX: Session NN Thread MM - description`
- ✅ **Push notes with commits** - include in same PR
- ✅ **Automate validation** - CI/CD catches mistakes early

**Important⚠️**: Wen Write [git-workflow-validation] job's commit message and git notes, MUST run (date -u '+%Y-%m-%dT%H:%M:%SZ') get UTC TIME NOW first!!!
