# Common Issues & Solutions

**Part of**: [Work Session Checkpoint Workflow](../WORK_SESSION_CHECKPOINT_WORKFLOW.md)

This document addresses the most frequent problems during checkpoint reviews and their solutions.

---

## Issue 1: Template Files Not Filled Out

**Root Cause**: Files created from template but content never updated

**Detection**:

```shell
grep -r "{[A-Z_]*}" .artifacts/spec-tasks-T-XX-XX/worklogs/
```

**Solution**: Fill templates completely using action log content as source. Time: 10-15 min/file

**Prevention**: Update worklogs throughout the day, verify EOD

---

## Issue 2: Metadata Hash = "PENDING"

**Root Cause**: Metadata updated before commit

**Detection**:

```shell
cat .artifacts/spec-tasks-T-XX-XX/metadata.yaml | grep "PENDING"
```

**Solution**: Replace with actual hash from `git log --oneline -1`. Time: 1-2 min

**Prevention**: Update metadata AFTER committing code

---

## Issue 3: Files Changed Count Wrong

**Root Cause**: Counted only code files, missed worklogs/artifacts

**Detection**: Compare `git show --stat` with metadata `files_changed`

**Solution**: Use git as authoritative source. Time: 2 min

**Prevention**: Always verify with `git show --stat`

---

## Issue 4: Test Count Discrepancy

**Root Cause**: Counted from memory instead of checking code

**Detection**: `git diff HEAD~1 tests/ | grep "#\[test\]" | wc -l`

**Solution**: Count tests in code, update documentation. Time: 2 min

**Prevention**: Verify test counts with grep, not memory

---

## Issue 5: Missing Statistics Section in Action Log

**Root Cause**: Forgot to add Statistics section

**Detection**: `grep "Statistics" .artifacts/.../raw/*.txt` returns empty

**Solution**: Add Statistics section with files changed, lines, tests. Time: 5 min

**Prevention**: Use action log template with Statistics section

---

## Issue 6: Lines Added/Deleted Slightly Off

**Root Cause**: Metadata updated incrementally, not from git

**Solution**: Update from `git show --stat`, not estimates. Time: 1 min

**Prevention**: Always verify with `git show --stat`

---

## Issue 7: Worklog Files Empty

**Root Cause**: Created files but never filled them

**Solution**: Use daily worklog script, fill from action log. Time: 20-30 min

**Prevention**: Fill worklogs throughout day, use checkpoint EOD

---

**Navigation**:

- [← Back to Phase 5](07-phase5-final.md)
- [← Back to Main Index](../WORK_SESSION_CHECKPOINT_WORKFLOW.md)
- [→ Next: Consistency Check Template](09-template.md)
