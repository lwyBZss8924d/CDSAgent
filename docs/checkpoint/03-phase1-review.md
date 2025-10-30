# Phase 1: Review & Data Collection

**Part of**: [Work Session Checkpoint Workflow](../WORK_SESSION_CHECKPOINT_WORKFLOW.md)

---

**Objective**: Gather all information needed for consistency verification

**Time Estimate**: 5-10 minutes

**Mode**: Read-only (no file modifications)

---

## Step 1.1: Read Raw Action Logs

**Location**: `.artifacts/spec-tasks-T-XX-XX/worklogs/raw/DEVCOOKING-WORK-ACTIONSLOGS-YYYY-MM-DD-*.txt`

**What to Extract**:

1. **Timestamps**: When session started/ended
2. **Progress Updates**: What was accomplished
3. **Statistics**: Files changed, lines added/deleted, tests added
4. **What's Next**: Planned next steps

**Example**:

```shell
cd ~/dev-space/CDSAgent-T-XX-XX-task-name
cat .artifacts/spec-tasks-T-XX-XX/worklogs/raw/DEVCOOKING-WORK-ACTIONSLOGS-2025-10-27-01.txt
```

**Extract Key Info**:

```text
• Progress Update
  - Extended graph::builder with full AST-driven export tracking
  - Added three focused unit tests (import_edges_follow_package_reexports, ...)
  - Parity harness: imports now match (218 / 218)

  Statistics
  - Files changed: 8 (builder.rs, graph_builder_tests.rs, metadata.yaml, 3 worklogs, 2 action logs)
  - Lines added: +1,361 (code: +730, worklogs: +554, action logs: +135, metadata: +36, deletions: -64)
  - Tests: 3 new unit tests (total 6/6 passing)
```

**Checklist**:

- [ ] Read action log for current day
- [ ] Note timestamp range (start/end)
- [ ] Extract progress summary
- [ ] Copy statistics section
- [ ] Note "What's Next" items

---

## Step 1.2: Check Git Operations

**Commands to Run**:

```shell
# 1. Check working tree status
git status

# 2. View recent commits
git log --oneline -5

# 3. Show detailed stats for latest commit
git show --stat <commit-hash>

# 4. Check git notes (if used)
git notes show <commit-hash>

# 5. Verify remote sync status
git status | grep "Your branch"
```

**What to Record**:

**From `git status`**:

- Working tree clean? (should be, if last commit was made)
- Untracked files? (should be none after commit)
- Branch status relative to origin

**From `git log`**:

- Latest commit hash (short: 7 chars, full: 40 chars)
- Commit message
- Commit date

**From `git show --stat`**:

```text
commit 3083e00
feat(graph): T-02-01 - implement export tracking system and resolve import parity

 .artifacts/spec-tasks-T-02-01-graph-builder/metadata.yaml              |  33 ++--
 .../worklogs/2025-10-27-commit-log.md                                  |  89 ++++++++++
 .../worklogs/2025-10-27-notes.md                                       | 137 +++++++++++++++
 .../worklogs/2025-10-27-work-summary.md                                | 255 ++++++++++++++++++++++++++++
 .../raw/DEVCOOKING-WORK-ACTIONSLOGS-2025-10-25-01.txt                  |  87 ++++++++++
 .../raw/DEVCOOKING-WORK-ACTIONSLOGS-2025-10-27-01.txt                  |  48 ++++++
 crates/cds-index/src/graph/builder.rs                                  | 548 +++++++++++++++++++++++++++++++++++++++++++++---------
 crates/cds-index/tests/graph_builder_tests.rs                          | 182 +++++++++++++++++++
 8 files changed, 1361 insertions(+), 64 deletions(-)
```

**Record Git Ground Truth**:

- Commit hash: `3083e00`
- Files changed: `8`
- Lines added: `+1,361`
- Lines deleted: `-64`
- Net change: `+1,297`

**Checklist**:

- [ ] Recorded commit hash (short & full)
- [ ] Recorded commit message
- [ ] Recorded files changed count
- [ ] Recorded lines added/deleted
- [ ] Noted all modified files
- [ ] Verified working tree clean

---

## Step 1.3: Read All Artifact Files

**Files to Read** (5 total):

### 1. metadata.yaml

```shell
cat .artifacts/spec-tasks-T-XX-XX/metadata.yaml
```

**What to Check**:

- Latest commit hash in `git.commits[]` list
- `files_changed` count
- `notes` field completeness
- `metrics.actual_hours` updated
- `metrics.lines_added/lines_deleted` accurate
- `metrics.tests_added` count
- `acceptance_criteria[].status` current

### 2. YYYY-MM-DD-work-summary.md

```shell
cat .artifacts/spec-tasks-T-XX-XX/worklogs/2025-10-27-work-summary.md
```

**What to Check**:

- "Today's Objectives" marked complete
- "Work Completed" section filled
- "Code Changes" section lists files
- "Key Decisions" documented
- "Next Steps" listed
- Statistics match git

### 3. YYYY-MM-DD-commit-log.md

```shell
cat .artifacts/spec-tasks-T-XX-XX/worklogs/2025-10-27-commit-log.md
```

**What to Check**:

- Commit hash present (not placeholder)
- Commit message full text
- Files changed count matches git
- Diff summary included
- Context notes explain "why"

### 4. YYYY-MM-DD-notes.md

```shell
cat .artifacts/spec-tasks-T-XX-XX/worklogs/2025-10-27-notes.md
```

**What to Check**:

- Architecture decisions documented
- Implementation details present
- Research questions answered
- Test cases described
- Performance notes (if applicable)
- Not just template placeholders

### 5. DEVCOOKING-WORK-ACTIONSLOGS-YYYY-MM-DD-*.txt

```shell
cat .artifacts/spec-tasks-T-XX-XX/worklogs/raw/DEVCOOKING-WORK-ACTIONSLOGS-2025-10-27-01.txt
```

**What to Check**:

- Progress Update section filled
- Statistics section present
- Test count accurate
- What's Next items listed

**Checklist**:

- [ ] Read metadata.yaml commit list
- [ ] Read work-summary.md completeness
- [ ] Read commit-log.md commit details
- [ ] Read notes.md technical content
- [ ] Read action log statistics
- [ ] Noted any placeholders or "PENDING" values

---

**Navigation**:

- [← Back to Phases Overview](02-phases-overview.md)
- [← Back to Main Index](../WORK_SESSION_CHECKPOINT_WORKFLOW.md)
- [→ Next: Phase 2 - Consistency Verification](04-phase2-verification.md)
