# Quick Commands Reference

**Part of**: [Work Session Checkpoint Workflow](../WORK_SESSION_CHECKPOINT_WORKFLOW.md)

This document provides a comprehensive command reference for all checkpoint workflow phases.

---

## PHASE 1: REVIEW & DATA COLLECTION

```shell
# Read raw action log
cat .artifacts/spec-tasks-T-XX-XX/worklogs/raw/DEVCOOKING-WORK-ACTIONSLOGS-YYYY-MM-DD-*.txt

# Check git status
git status

# View recent commits
git log --oneline -5

# Show detailed stats for commit
git show --stat <commit-hash>
git show --stat HEAD  # Latest commit

# Check git notes
git notes show <commit-hash>
git notes list

# Verify remote sync
git status | grep "Your branch"

# Read metadata.yaml
cat .artifacts/spec-tasks-T-XX-XX/metadata.yaml | grep -A 10 "commits:"

# Read work-summary.md
cat .artifacts/spec-tasks-T-XX-XX/worklogs/YYYY-MM-DD-work-summary.md

# Read commit-log.md
cat .artifacts/spec-tasks-T-XX-XX/worklogs/YYYY-MM-DD-commit-log.md

# Read notes.md
cat .artifacts/spec-tasks-T-XX-XX/worklogs/YYYY-MM-DD-notes.md

# Check for template placeholders
grep -r "{[A-Z_]*}" .artifacts/spec-tasks-T-XX-XX/worklogs/
```

---

## PHASE 2: CONSISTENCY VERIFICATION

```shell
# Get git ground truth
git show --stat <hash> | tail -1

# Extract commit hash
git log --oneline -1 | awk '{print $1}'

# Count files changed
git show --stat <hash> | tail -1 | awk '{print $1}'

# Get lines added/deleted
git show --stat <hash> | tail -1 | sed 's/.*(\([^)]*\)).*/\1/'

# Count tests added
git diff HEAD~1 tests/ | grep "^+.*#\[test\]" | wc -l

# Compare metadata with git
diff <(git show --stat <hash> | tail -1) \
     <(cat .artifacts/spec-tasks-T-XX-XX/metadata.yaml | grep "files_changed")
```

---

## PHASE 3: UPDATE ARTIFACTS

```shell
# Edit metadata.yaml
vim .artifacts/spec-tasks-T-XX-XX/metadata.yaml

# Edit work-summary.md
vim .artifacts/spec-tasks-T-XX-XX/worklogs/YYYY-MM-DD-work-summary.md

# Edit commit-log.md
vim .artifacts/spec-tasks-T-XX-XX/worklogs/YYYY-MM-DD-commit-log.md

# Edit notes.md
vim .artifacts/spec-tasks-T-XX-XX/worklogs/YYYY-MM-DD-notes.md

# Edit action log
vim .artifacts/spec-tasks-T-XX-XX/worklogs/raw/DEVCOOKING-WORK-ACTIONSLOGS-YYYY-MM-DD-*.txt

# Replace PENDING with commit hash
sed -i 's/hash: "PENDING"/hash: "3083e00"/' .artifacts/spec-tasks-T-XX-XX/metadata.yaml
```

---

## PHASE 4: GIT OPERATIONS

```shell
# ============================================================
# STEP 4.1: Create Code Commit (if applicable)
# ============================================================

# Check for code changes
git status
git diff --stat

# Stage code files only
git add crates/cds-index/src/...
git add crates/cds-index/tests/...

# Create code commit
git commit -m "$(cat <<'EOF'
[type]([scope]): [subject]

[body with implementation details]

[parity results / test results]

Files changed: X code files, +XXX/-XXX lines
EOF
)"

# Get commit hash
git log -1 --oneline
# Example output: 52c2b7e fix(parity): align SWE fixtures with TYPE_CHECKING import semantics

# ============================================================
# STEP 4.2: Add Git Notes to Code Commit ⭐ REQUIRED
# ============================================================

# Standard git notes format
git notes add <commit-hash> -m "spec-tasks/T-XX-XX-task-name
Day: X
Date: YYYY-MM-DD
Sessions: X-XX to X-XX (HH:MM-HH:MM UTC)
Duration: Xh
Worklog: .artifacts/spec-tasks-T-XX-XX/worklogs/YYYY-MM-DD-*
Parity: [parity status or N/A]
Status: [one-line summary of work completed]
Files: X code files (+XXX/-XXX lines)"

# Real example
git notes add 52c2b7e -m "spec-tasks/T-02-01-graph-builder
Day: 5
Date: 2025-10-29
Sessions: 3-03 to 3-07 (13:30-18:30 UTC)
Duration: 5h
Worklog: .artifacts/spec-tasks-T-02-01-graph-builder/worklogs/2025-10-29-*
Parity: All fixtures ≤2% (imports/inherits 0%, invokes +1.29%)
Status: TYPE_CHECKING & scoped imports resolved
Files: 7 code files (+1,072/-140 lines)"

# Verify notes added
git notes show 52c2b7e
git notes list

# ============================================================
# STEP 4.3: Update metadata.yaml with Commit Hash
# ============================================================

# Edit metadata.yaml
vim .artifacts/spec-tasks-T-XX-XX/metadata.yaml

# Add commit entry (example):
# git:
#   commits:
#     - hash: "52c2b7e"
#       message: "fix(parity): align SWE fixtures with TYPE_CHECKING import semantics"
#       date: "2025-10-29"
#       files_changed: 7
#       notes: "Day 5 (Sessions 3-03 to 3-07): Scoped TYPE_CHECKING & SWE parity improvements. ..."

# ============================================================
# STEP 4.4: Stage Artifact Changes
# ============================================================

# Stage updated artifacts (NOT code files - already committed!)
git add .artifacts/spec-tasks-T-XX-XX/metadata.yaml
git add .artifacts/spec-tasks-T-XX-XX/worklogs/*.md
git add .artifacts/spec-tasks-T-XX-XX/worklogs/raw/*.txt

# Or stage all artifacts at once
git add .artifacts/spec-tasks-T-XX-XX/

# Verify staged files (should only be artifacts)
git diff --cached --stat
git diff --cached --name-only

# ============================================================
# STEP 4.5: Create Checkpoint Commit
# ============================================================

# Simple checkpoint commit
git commit -m "checkpoint(worklog): T-XX-XX Day X complete - artifacts updated with code commit <hash>"

# Detailed checkpoint commit
git commit -m "checkpoint(worklog): T-02-01 Day 5 complete - artifacts updated with code commit 52c2b7e

Updated artifacts for Day 5 work session (2025-10-29):
- Added commit 52c2b7e to metadata.yaml git_commits section
- Updated cumulative metrics (lines_added: 8,265, files_modified: 62)
- Fixed file count in worklogs (7 code files including mod.rs)
- Updated action log statistics to match git reality

All artifacts now 100% consistent with git operations.
Git notes added to code commit for task tracking."

# Verify checkpoint commit
git log -1 --oneline

# ============================================================
# STEP 4.6: Push Everything to Remote ⭐ CRITICAL
# ============================================================

# Push commits
git push origin feat/task/T-XX-XX-task-name

# Push git notes ⭐ DON'T FORGET THIS!
git push origin refs/notes/commits

# Verify push succeeded
git status
# Expected: "Your branch is up to date with 'origin/...'"
```

---

## PHASE 5: FINAL VERIFICATION

```shell
# Verify working tree clean
git status

# View last 2 commits
git log --oneline -2

# Show checkpoint commit
git show --stat HEAD

# Verify remote sync
git status | grep "Your branch"

# Re-check for placeholders
grep -r "{[A-Z_]*}" .artifacts/spec-tasks-T-XX-XX/worklogs/
# Should return: (no output)

# Re-check PENDING values
grep -r "PENDING" .artifacts/spec-tasks-T-XX-XX/
# Should return: (no output)

# Verify consistency
diff <(git show --stat HEAD~1 | tail -1) \
     <(cat .artifacts/spec-tasks-T-XX-XX/metadata.yaml | grep -A 1 "files_changed")
# Should return: (no output)
```

---

## UTILITY COMMANDS

```shell
# Count lines in worklog files
wc -l .artifacts/spec-tasks-T-XX-XX/worklogs/YYYY-MM-DD-*.md

# Find all action logs
find .artifacts/spec-tasks-T-XX-XX/worklogs/raw/ -name "DEVCOOKING-*.txt"

# Search for specific metric
grep -r "Import parity" .artifacts/spec-tasks-T-XX-XX/worklogs/

# Compare two commits
git diff --stat <hash1> <hash2>

# Show file contents at specific commit
git show <hash>:path/to/file

# List files changed in commit
git show --name-only <hash>
```

---

## TROUBLESHOOTING

```shell
# Find template placeholders
find .artifacts/spec-tasks-T-XX-XX/worklogs/ -name "*.md" -exec grep -Hn "{[A-Z_]*}" {} \;

# Find PENDING values
find .artifacts/spec-tasks-T-XX-XX/ -name "*.yaml" -exec grep -Hn "PENDING" {} \;

# Find empty files
find .artifacts/spec-tasks-T-XX-XX/worklogs/ -name "*.md" -size 0

# Compare metadata with git (detailed)
echo "Git:" && git show --stat HEAD~1 | tail -1
echo "Metadata:" && cat .artifacts/spec-tasks-T-XX-XX/metadata.yaml | grep -A 3 "hash:"

# Validate YAML syntax
yamllint .artifacts/spec-tasks-T-XX-XX/metadata.yaml

# Count occurrences of "test"
grep -o "test" .artifacts/spec-tasks-T-XX-XX/worklogs/raw/*.txt | wc -l
```

---

**Navigation**:

- [← Back to Example Walkthrough](10-example.md)
- [← Back to Main Index](../WORK_SESSION_CHECKPOINT_WORKFLOW.md)
