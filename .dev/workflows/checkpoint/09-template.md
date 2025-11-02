# Consistency Check Template

**Part of**: [Work Session Checkpoint Workflow](./../WORK_SESSION_CHECKPOINT_WORKFLOW.md)

This document provides a reusable template for performing systematic consistency verification during work session checkpoints.

---

## How to Use This Template

1. Copy this entire template to a new document or text file
2. Replace all `[PLACEHOLDER]` values with actual data from your checkpoint review
3. Fill in each section sequentially as you work through the checkpoint phases
4. Save the completed report for future reference and audit trail

---

## Work Session Checkpoint - Consistency Check Report

**Date**: YYYY-MM-DD
**Task**: T-XX-XX-task-name
**Session**: Day X
**Reviewer**: [Your Name]

---

## Phase 1: Git Ground Truth

### Latest Commit

```shell
$ git log --oneline -1
[HASH] [COMMIT MESSAGE]
```

### Commit Statistics

```shell
$ git show --stat [HASH] | tail -1
[N] files changed, [M] insertions(+), [K] deletions(-)
```

**Extract**:

- **Commit Hash**: [HASH]
- **Files Changed**: [N]
- **Lines Added**: [M]
- **Lines Deleted**: [K]
- **Net Change**: [M-K]

### Modified Files List

```shell
$ git show --stat [HASH] --name-only
[FILE 1]
[FILE 2]
...
[FILE N]
```

---

## Phase 2: Artifact Review

### 1. Artifacts/spec-tasks-T-XX-XX/metadata.yaml

**Location**: `.artifacts/spec-tasks-T-XX-XX/metadata.yaml`

**Check**:

- [ ] Latest commit hash present (not "PENDING")
- [ ] `files_changed` matches git
- [ ] `notes` field complete with statistics
- [ ] `metrics.actual_hours` updated
- [ ] `metrics.lines_added` matches git
- [ ] `metrics.lines_deleted` matches git
- [ ] `metrics.tests_added` accurate

**Findings**:

- ✅ / ⚠️ / ❌ : [Description]

---

### 2. Artifacts/spec-tasks-T-XX-XX/worklogs/YYYY-MM-DD-work-summary.md

**Location**: `.artifacts/spec-tasks-T-XX-XX/worklogs/YYYY-MM-DD-work-summary.md`

**Check**:

- [ ] "Today's Objectives" filled out
- [ ] "Work Completed" section present
- [ ] "Code Changes" lists files
- [ ] "Key Decisions" documented
- [ ] "Next Steps" listed
- [ ] Statistics match git

**Findings**:

- ✅ / ⚠️ / ❌ : [Description]

---

### 3. Artifacts/spec-tasks-T-XX-XX/worklogs/YYYY-MM-DD-commit-log.md

**Location**: `.artifacts/spec-tasks-T-XX-XX/worklogs/YYYY-MM-DD-commit-log.md`

**Check**:

- [ ] Commit hash present (not placeholder)
- [ ] Commit message complete
- [ ] Files changed count matches git
- [ ] Diff summary included
- [ ] Context notes explain changes

**Findings**:

- ✅ / ⚠️ / ❌ : [Description]

---

### 4. Artifacts/spec-tasks-T-XX-XX/worklogs/YYYY-MM-DD-notes.md

**Location**: `.artifacts/spec-tasks-T-XX-XX/worklogs/YYYY-MM-DD-notes.md`

**Check**:

- [ ] Architecture decisions documented
- [ ] Implementation details present
- [ ] Research questions answered
- [ ] Test cases described
- [ ] Performance notes (if applicable)
- [ ] Not just template placeholders

**Findings**:

- ✅ / ⚠️ / ❌ : [Description]

---

### 5. Artifacts/spec-tasks-T-XX-XX/worklogs/raw/DEVCOOKING-WORK-ACTIONSLOGS-YYYY-MM-DD-*.txt

**Location**: `.artifacts/spec-tasks-T-XX-XX/worklogs/raw/DEVCOOKING-WORK-ACTIONSLOGS-YYYY-MM-DD-*.txt`

**Check**:

- [ ] Progress Update section filled
- [ ] Statistics section present
- [ ] Files changed matches git
- [ ] Lines added/deleted matches git
- [ ] Test count accurate
- [ ] "What's Next" items listed

**Findings**:

- ✅ / ⚠️ / ❌ : [Description]

---

## Phase 3: Consistency Matrix

| Metric | Git Actual | metadata.yaml | work-summary | commit-log | notes | action-log | Status |
|--------|-----------|---------------|--------------|------------|-------|------------|--------|
| Commit Hash | [HASH] | [VALUE] | N/A | [VALUE] | N/A | N/A | ✅/❌ |
| Files Changed | [N] | [VALUE] | [VALUE] | [VALUE] | N/A | [VALUE] | ✅/❌ |
| Lines Added | [M] | [VALUE] | [VALUE] | [VALUE] | N/A | [VALUE] | ✅/❌ |
| Lines Deleted | [K] | [VALUE] | [VALUE] | [VALUE] | N/A | [VALUE] | ✅/❌ |
| Tests Added | [COUNT] | [VALUE] | [VALUE] | [VALUE] | [VALUE] | [VALUE] | ✅/❌ |
| [Custom Metric 1] | [VALUE] | [VALUE] | [VALUE] | [VALUE] | [VALUE] | [VALUE] | ✅/❌ |
| [Custom Metric 2] | [VALUE] | [VALUE] | [VALUE] | [VALUE] | [VALUE] | [VALUE] | ✅/❌ |

**Legend**:

- ✅ = Value matches git actual or is documented correctly
- ⚠️ = Value differs slightly but not critical
- ❌ = Value significantly wrong or missing
- N/A = Metric not applicable to this file

---

## Phase 4: Discrepancies Identified

### Critical (Must Fix)

- [ ] **Issue 1**: [Description] - [File] - [Current] → [Should Be]
- [ ] **Issue 2**: [Description] - [File] - [Current] → [Should Be]

### Important (Should Fix)

- [ ] **Issue 1**: [Description] - [File] - [Current] → [Should Be]
- [ ] **Issue 2**: [Description] - [File] - [Current] → [Should Be]

### Minor (Nice to Fix)

- [ ] **Issue 1**: [Description] - [File] - [Current] → [Should Be]

---

## Phase 5: Consistency Score

**Calculation**:

```text
Total Metrics Checked: [N]
Correct (✅): [X]
Slightly Off (⚠️): [Y]
Wrong (❌): [Z]

Consistency Score = (X + Y×0.5) / N × 100%
                  = ([X] + [Y]×0.5) / [N] × 100%
                  = [SCORE]%
```

**Interpretation**:

- **100%**: Perfect ✅ - Ready for checkpoint
- **90-99%**: Good ⚠️ - Minor fixes needed
- **75-89%**: Needs work ❌ - Multiple fixes required
- **<75%**: Critical ⛔ - Major discrepancies

**Current Status**: [SCORE]% - [INTERPRETATION]

---

## Phase 6: Action Items

**To achieve 100% consistency**:

1. **Fix Critical Issues**:
   - [ ] [Action item 1]
   - [ ] [Action item 2]

2. **Fix Important Issues**:
   - [ ] [Action item 1]
   - [ ] [Action item 2]

3. **Fix Minor Issues** (optional):
   - [ ] [Action item 1]

**Estimated Time**: [X] minutes

---

## Phase 7: Post-Fix Verification

**After applying fixes**:

- [ ] Re-ran consistency check
- [ ] Consistency score = 100%
- [ ] No template placeholders remain
- [ ] All worklogs complete
- [ ] Committed artifact updates
- [ ] Pushed to remote

**Final Status**: ✅ / ⏳ / ❌

---

## Checkpoint Completion

```text
**Date**: YYYY-MM-DD HH:MM UTC
**Status**: ✅ COMPLETED / ⏳ IN PROGRESS / ❌ BLOCKED

**Final Consistency Score**: [SCORE]%

**Checkpoint Commit**: [HASH]

**Next Session Plan**:

- [Next task 1]
- [Next task 2]
- [Next task 3]

---

**Reviewer Signature**: [Your Name]
**Review Date**: YYYY-MM-DD
```

---

**Navigation**:

- [← Back to Common Issues](08-common-issues.md)
- [← Back to Main Index](./../WORK_SESSION_CHECKPOINT_WORKFLOW.md)
- [→ Next: Example Walkthrough](10-example.md)
