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
# Stage artifact changes only
git add .artifacts/spec-tasks-T-XX-XX/metadata.yaml
git add .artifacts/spec-tasks-T-XX-XX/worklogs/*.md
git add .artifacts/spec-tasks-T-XX-XX/worklogs/raw/*.txt

# Or stage all artifacts at once
git add .artifacts/spec-tasks-T-XX-XX/

# Verify staged files
git diff --cached --stat
git diff --cached --name-only

# Commit checkpoint
git commit -m "fix(worklog): correct Day X T-XX-XX artifact consistency"

# Detailed commit
git commit -m "$(cat <<'EOF'
fix(worklog): correct Day X T-XX-XX metadata and worklog consistency

Fixes identified during work session checkpoint review:
- Updated metadata.yaml: hash PENDING → 3083e00
- Updated metadata.yaml: files_changed 3 → 8
- Updated action log: test count "two" → "three"
- Added Statistics section to action log

Consistency score improved from 64% to 100%.
EOF
)"

# Add git notes (optional)
git notes add -m "Work session checkpoint - Day X completed with 100% artifact consistency"

# Push to remote
git push origin feat/task/T-XX-XX-task-name
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
