# Phase 4: Git Operations

**Part of**: [Work Session Checkpoint Workflow](../WORK_SESSION_CHECKPOINT_WORKFLOW.md)

---

**Objective**: Commit artifact updates and push to remote

**Time Estimate**: 2-5 minutes

**Mode**: Git operations only (no code changes)

---

## Step 4.1: Stage Artifact Changes Only

**⚠️ CRITICAL**: Only stage artifact files, not code files

```shell
# Navigate to worktree
cd ~/dev-space/CDSAgent-T-XX-XX-task-name

# Check what will be staged
git status

# Stage artifact files only
git add .artifacts/spec-tasks-T-XX-XX/metadata.yaml
git add .artifacts/spec-tasks-T-XX-XX/worklogs/*.md
git add .artifacts/spec-tasks-T-XX-XX/worklogs/raw/*.txt

# Verify staged files
git diff --cached --stat
```

**Expected Output**:

```text
 .artifacts/spec-tasks-T-XX-XX/metadata.yaml                     |  5 +++--
 .artifacts/spec-tasks-T-XX-XX/worklogs/2025-10-27-commit-log.md |  2 +-
 .artifacts/spec-tasks-T-XX-XX/worklogs/raw/DEVCOOKING-*.txt     | 15 +++++++++++++++
 3 files changed, 19 insertions(+), 3 deletions(-)
```

**Checklist**:

- [ ] Only artifact files staged
- [ ] No code files (e.g., `src/`, `crates/`) staged
- [ ] No test files staged (unless adding worklog tests)
- [ ] Verified with `git diff --cached`

---

## Step 4.2: Commit with Descriptive Message

**Commit Message Format**:

```text
fix(worklog): correct Day X [task-id] [component] consistency

[Optional body explaining what was fixed]
```

**Examples**:

**Simple Fix**:

```shell
git commit -m "fix(worklog): correct Day 3 T-02-01 metadata and action log consistency"
```

**Detailed Fix**:

```shell
git commit -m "$(cat <<'EOF'
fix(worklog): correct Day 3 T-02-01 metadata and action log consistency

Fixes identified during work session checkpoint review:
- Updated metadata.yaml: hash PENDING → 3083e00, files_changed 3 → 8
- Updated action log: test count "two" → "three", added Statistics section
- Verified all worklogs filled out (commit-log.md, notes.md)

Consistency score improved from 64% to 100%.
EOF
)"
```

**Commit Message Rules**:

- Start with `fix(worklog):` type
- Include day number
- Include task ID
- Keep subject line <72 chars
- Optional: Add body with bullet list of fixes

---

## Step 4.3: Add Git Notes (Optional)

**Purpose**: Attach metadata to commit without cluttering commit history

**When to Use**:

- Significant session milestones
- Major breakthroughs (e.g., parity resolved)
- Want to record detailed checkpoint status

**Example**:

```shell
git notes add -m "$(cat <<'EOF'
T-02-01 Graph Builder - Day 3 Work Session Checkpoint

## Checkpoint Status: COMPLETED ✅

## Consistency Verification
- Consistency Score: 100% (14/14 metrics match)
- All artifacts aligned with git commit 3083e00

## Session Achievements
- Import parity RESOLVED: 218/218 (0% variance)
- Invoke variance improved to +1.9%
- ModuleExports system implemented (+548 lines)
- 3 new unit tests added (6/6 passing)

## Artifacts Updated
- metadata.yaml: hash, files_changed, notes
- action log: test count, Statistics section
- All worklogs verified complete

## Next Session
- Day 4: Mirror LocAgent's find_all_possible_callee
- Target: Eliminate remaining +1.9% invoke variance
EOF
)"
```

**View Git Notes**:

```shell
git notes show
```

**Checklist**:

- [ ] Git notes added (if desired)
- [ ] Notes include checkpoint status
- [ ] Notes summarize fixes applied
- [ ] Notes include next session plan

---

## Step 4.4: Push to Remote

**Commands**:

```shell
# Push checkpoint commit
git push origin feat/task/T-XX-XX-task-name

# Verify push succeeded
git status
```

**Expected Output**:

```text
Everything up-to-date
Your branch is up to date with 'origin/feat/task/T-XX-XX-task-name'.
```

**Checklist**:

- [ ] Push completed without errors
- [ ] Remote branch up to date
- [ ] Verified with `git status`

---

**Navigation**:

- [← Back to Phase 3](05-phase3-update.md)
- [← Back to Main Index](../WORK_SESSION_CHECKPOINT_WORKFLOW.md)
- [→ Next: Phase 5 - Final Verification](07-phase5-final.md)
