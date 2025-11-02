# Template Usage - Complete Reference

This document provides detailed information about CDSAgent's template system, placeholder expansion, and customization.

## Table of Contents

- [Template System Architecture](#template-system-architecture)
- [Metadata Template Deep Dive](#metadata-template-deep-dive)
- [Worklog Template Variations](#worklog-template-variations)
- [Placeholder Expansion Rules](#placeholder-expansion-rules)
- [Template Customization](#template-customization)
- [Advanced Template Patterns](#advanced-template-patterns)
- [Template Validation](#template-validation)

---

## Template System Architecture

### Template Hierarchy

```text
.dev/templates/                    # Source templates
  ├── README.md                    # Template system documentation
  ├── metadata.template.yaml       # Task metadata structure
  └── worklogs/                    # Session artifact templates
      ├── work-summary.template.md
      ├── commit-log.template.md
      ├── notes.template.md
      ├── codereview.template.md
      └── raw-session.template.txt

↓ (create-task-worklog.sh, create-session-worklog.sh, create-raw-log.sh)

.artifacts/spec-tasks-T-XX-XX/     # Generated artifacts
  ├── metadata.yaml                # From metadata.template.yaml
  └── worklogs/
      ├── {date}-S{NN}-work-summary.md      # From work-summary.template.md
      ├── {date}-S{NN}-commit-log.md        # From commit-log.template.md
      ├── {date}-S{NN}-notes.md             # From notes.template.md
      ├── {date}-S{NN}-codereview.md        # From codereview.template.md (optional)
      └── raw/
          └── WORK-SESSIONS-{NN}-*-SUMMARY-{date}.txt  # From raw-session.template.txt
```

### Template Processing Flow

```text
┌────────────────────────────────────────────────────────────┐
│ Template Processing                                        │
├────────────────────────────────────────────────────────────┤
│                                                            │
│  1. Script Invocation                                      │
│     ↓                                                      │
│  2. Read Template File (.dev/templates/...)                │
│     ↓                                                      │
│  3. Extract Placeholders ({{TASK_ID}}, {{DATE}}, etc.)     │
│     ↓                                                      │
│  4. Replace with Actual Values                             │
│     ↓                                                      │
│  5. Write Expanded File (.artifacts/...)                   │
│     ↓                                                      │
│  6. Verify File Creation                                   │
│     ↓                                                      │
│  7. Return Exit Code (0=success, 1=fail, 2=warn)           │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

---

## Metadata Template Deep Dive

### Full metadata.template.yaml Structure

```yaml
# Task Metadata for {{TASK_ID}}
# Auto-generated on {{DATE}}

task:
  id: {{TASK_ID}}
  title: "{{TASK_TITLE}}"
  owner: "{{DEVELOPER}}"
  status: in_progress
  start_date: "{{DATE}}"
  last_updated: {{UTC_TIMESTAMP}}
  estimated_hours: {{ESTIMATED_HOURS}}
  actual_hours: 0

specs:
  prds:
    - {{PRD_PATH_1}}
  issues:
    - {{ISSUE_PATH_1}}
  tasks:
    - {{TASK_PATH_1}}

git:
  worktree: .worktrees/{{TASK_ID}}
  branch: feat/task/{{TASK_ID}}
  base_commit: {{BASE_COMMIT}}
  commits: []
  pr:
    number: null
    url: null
    status: null
    merged_at: null

deliverables: []

acceptance_criteria:
  - criterion: "{{CRITERION_1}}"
    status: not_started
    notes: ""

dependencies:
  requires: []
  blocks: []

notes: |
  {{NOTES}}

worklog:
  base_path: ".artifacts/spec-tasks-{{TASK_ID}}/worklogs/"
  entries: []

sessions: []

metrics:
  lines_added: 0
  lines_deleted: 0
  files_modified: 0
  tests_added: 0
  test_pass_rate: null
  test_coverage: null

comments:

related_artifacts:
```

### Metadata Sections Explained

**task**: Core task information

- `id`: Unique identifier (T-XX-XX format)
- `title`: Human-readable task name
- `owner`: Primary developer/AI agent
- `status`: Current state (in_progress, completed, blocked)
- `last_updated`: UTC timestamp (YYYY-MM-DDTHH:MM:SSZ)
- `actual_hours`: Cumulative hours (updated per session)

**specs**: Specification references

- `prds`: Links to Product Requirements Documents
- `issues`: Links to technical Issue specifications
- `tasks`: Links to Task specifications

**git**: Version control tracking

- `worktree`: Worktree directory path
- `branch`: Feature branch name
- `base_commit`: Initial commit hash
- `commits[]`: Array of commit objects (hash, message, date, notes)
- `pr`: Pull request metadata

**acceptance_criteria[]**: Success metrics

- `criterion`: Description of requirement
- `status`: not_started | in_progress | completed
- `notes`: Implementation details

**sessions[]**: Session history

- `id`: Session number (01, 02, 03...)
- `date`: Session date (YYYY-MM-DD)
- `threads`: Thread count and range
- `duration`: Start/end time, total hours
- `commits`: Session commit hashes
- `artifacts`: Links to session worklogs
- `metrics`: Session-specific stats

**metrics**: Cumulative task statistics

- `lines_added/deleted`: Code changes
- `files_modified`: File count
- `tests_added`: Test count
- `test_pass_rate`: Percentage (0.0-1.0)
- `test_coverage`: Coverage percentage (0.0-1.0)

---

## Worklog Template Variations

### Work Summary Template

**Purpose**: High-level session overview with deliverables.

**Structure**:

```markdown
# Work Summary - {{TASK_ID}} Session {{SESSION_ID}}

**Date**: {{DATE}}
**Session**: {{SESSION_ID}}
**Developer**: {{DEVELOPER}}
**Description**: {{DESCRIPTION}}
**Phase**: {{PHASE}}
**Threads**: {{THREADS_RANGE}}
**Duration**: {{START_TIME}}-{{END_TIME}} ({{DURATION}})

## Session Objectives

- [ ] Objective 1
- [ ] Objective 2
- [ ] Objective 3

## Work Completed

### Thread 01: [Thread Description]

- Work item 1
- Work item 2

### Thread 02: [Thread Description]

- Work item 1
- Work item 2

## Deliverables

- ✅ Deliverable 1 (path/to/file.rs)
- ✅ Deliverable 2 (path/to/test.rs)
- ⏳ Deliverable 3 (in progress)

## Metrics

- **Duration**: {{DURATION}}
- **Threads Completed**: {{THREADS_COUNT}}
- **Commits**: {{COMMITS_COUNT}}
- **Lines Added**: {{LINES_ADDED}}
- **Lines Deleted**: {{LINES_DELETED}}
- **Files Modified**: {{FILES_MODIFIED}}
- **Tests Added**: {{TESTS_ADDED}}
- **Test Pass Rate**: {{TEST_PASS_RATE}}%
- **Coverage**: {{COVERAGE}}%

## Next Steps

1. Next action 1
2. Next action 2
3. Next action 3
```

### Commit Log Template

**Purpose**: Git commit history with thread attribution.

**Structure**:

```markdown
# Commit Log - {{TASK_ID}} Session {{SESSION_ID}}

**Date**: {{DATE}}
**Session**: {{SESSION_ID}}
**Phase**: {{PHASE}}
**Threads**: {{THREADS_RANGE}}

## Session Info

- **Task**: {{TASK_ID}}
- **Developer**: {{DEVELOPER}}
- **Branch**: {{BRANCH_NAME}}
- **Duration**: {{DURATION}}

## Commits

### Thread 01: [Thread Description]

**Commit 1**:

- **Hash**: `abc123de`
- **Message**: `feat(scope): implementation`
- **Date**: {{COMMIT_DATE}}
- **Files**: 5 modified (+100/-20)
- **Git Note**: `spec-tasks/{{TASK_ID}}: Session {{SESSION_ID}} Thread 01 - description`

**Commit 2** (if multiple):

- **Hash**: `def456gh`
- **Message**: `test(scope): add tests`
- **Date**: {{COMMIT_DATE}}
- **Files**: 2 modified (+50/-0)
- **Git Note**: `spec-tasks/{{TASK_ID}}: Session {{SESSION_ID}} Thread 01 - tests`

### Thread 02: [Thread Description]

...

## Summary

- **Total Commits**: {{TOTAL_COMMITS}}
- **Files Changed**: {{FILES_CHANGED}}
- **Lines Added**: {{LINES_ADDED}}
- **Lines Deleted**: {{LINES_DELETED}}
```

### Technical Notes Template

**Purpose**: Detailed implementation notes and decisions.

**Structure**:

```markdown
# Technical Notes - {{TASK_ID}} Session {{SESSION_ID}}

**Date**: {{DATE}}
**Session**: {{SESSION_ID}}
**Phase**: {{PHASE}}

## Session Context

Brief overview of what was being accomplished in this session.

## Technical Decisions

### Decision 1: [Topic]

**Context**: Why this decision was needed

**Options Considered**:

1. Option A - pros/cons
2. Option B - pros/cons

**Decision**: Chose Option B because...

**Impact**: Changes to X, Y, Z

### Decision 2: [Topic]

...

## Implementation Details

### Component 1: [Name]

**File**: `path/to/file.rs`

**Implementation Approach**: Description of how it was implemented

**Key Code Patterns**:

\`\`\`rust
// Example code snippet
fn example() {
    // Implementation
}
\`\`\`

### Component 2: [Name]

...

## Issues Encountered

### Issue 1: [Description]

**Problem**: What went wrong

**Investigation**: How the issue was diagnosed

**Solution**: How it was resolved

**Prevention**: How to avoid in future

### Issue 2: [Description]

...

## References

- Link to documentation 1
- Link to documentation 2
- Related PRs/commits
```

### Code Review Template (Optional)

**Purpose**: Code quality and testing review.

**Structure**:

```markdown
# Code Review - {{TASK_ID}} Session {{SESSION_ID}}

**Date**: {{DATE}}
**Session**: {{SESSION_ID}}
**Reviewer**: {{REVIEWER}}
**Phase**: {{PHASE}}

## Review Scope

- **Files Reviewed**: {{FILES_COUNT}}
- **Lines Reviewed**: {{LINES_REVIEWED}}
- **Modules**: {{MODULES_LIST}}

## Code Quality

### Architecture

- ✅ Follows project patterns
- ✅ Proper separation of concerns
- ⚠️ Could improve: X

### Readability

- ✅ Clear naming conventions
- ✅ Adequate documentation
- ⚠️ Complex logic in: path/to/file.rs:42

### Maintainability

- ✅ Modular design
- ✅ No code duplication
- ⚠️ Potential refactor: X

## Test Coverage

- **Unit Tests**: {{UNIT_TESTS_COUNT}} ({{COVERAGE_UNIT}}% coverage)
- **Integration Tests**: {{INTEGRATION_TESTS_COUNT}}
- **Test Pass Rate**: {{TEST_PASS_RATE}}%

### Coverage Gaps

- ❌ Missing tests for: X
- ❌ Edge cases not covered: Y

## Performance Considerations

- ⚡ Benchmark: operation X takes {{TIME}}ms (target: {{TARGET}}ms)
- ✅ Memory usage acceptable
- ⚠️ Could optimize: Y

## Security Considerations

- ✅ No obvious vulnerabilities
- ✅ Input validation present
- ⚠️ Review: potential SQL injection in Z

## Recommendations

1. ✅ **Must Fix**: Critical issue X
2. ⚠️ **Should Fix**: Important improvement Y
3. 💡 **Nice to Have**: Enhancement Z

## Approval Status

- [ ] **Approved** - Ready to merge
- [ ] **Approved with Comments** - Minor fixes suggested
- [ ] **Changes Requested** - Must address issues before merge
```

---

## Placeholder Expansion Rules

### Standard Placeholders

| Placeholder | Type | Example | Source |
|-------------|------|---------|--------|
| `{{TASK_ID}}` | String | `T-02-02-sparse-index` | Script parameter |
| `{{SESSION_ID}}` | String | `05` | Script parameter |
| `{{DATE}}` | String | `2025-11-02` | System date (YYYY-MM-DD) |
| `{{UTC_TIMESTAMP}}` | String | `2025-11-02T13:30:00Z` | UTC now |
| `{{DEVELOPER}}` | String | `Claude Code Agent` | Script parameter |
| `{{DESCRIPTION}}` | String | `Phase 3 BM25 Integration` | Script parameter |
| `{{PHASE}}` | String | `Phase 2` | Inferred from description |
| `{{THREADS_RANGE}}` | String | `01-07` | Script parameters (START-END) |
| `{{THREADS_COUNT}}` | Number | `7` | Calculated (END - START + 1) |
| `{{START_TIME}}` | String | `01:39` | Script parameter or system time |
| `{{END_TIME}}` | String | `12:45` | Script parameter or system time |
| `{{DURATION}}` | String | `3.2h` | Calculated (END - START) |

### Computed Placeholders

| Placeholder | Computation | Example |
|-------------|-------------|---------|
| `{{LINES_ADDED}}` | `git diff --stat` | `2013` |
| `{{LINES_DELETED}}` | `git diff --stat` | `108` |
| `{{FILES_MODIFIED}}` | `git diff --name-only \| wc -l` | `17` |
| `{{COMMITS_COUNT}}` | `git log --oneline \| wc -l` | `2` |
| `{{TESTS_ADDED}}` | Manual count or script | `12` |
| `{{TEST_PASS_RATE}}` | `passing / total` | `1.0` (100%) |
| `{{COVERAGE}}` | From coverage tool | `0.972` (97.2%) |

### Conditional Placeholders

Some placeholders are optional/conditional:

```yaml
# In metadata.yaml
pr:
  number: {{PR_NUMBER}}  # null if not created yet
  status: {{PR_STATUS}}  # null if not created

acceptance_criteria:
  - criterion: "{{CRITERION}}"
    status: {{STATUS}}  # not_started | in_progress | completed
    notes: "{{NOTES}}"  # Empty string if no notes
```

---

## Template Customization

### Task-Specific Templates

Override default templates per task:

```text
.artifacts/spec-tasks-T-02-02-sparse-index/
  ├── metadata.yaml                     # From metadata.template.yaml
  ├── CLAUDE.md                          # Custom task guide (optional)
  └── worklogs/
      ├── {date}-S{NN}-work-summary.md  # From work-summary.template.md
      └── custom-benchmark.md            # Task-specific template
```

### Adding Custom Sections

**Example**: Add "Parity Analysis" section to notes template.

**Create**: `.artifacts/spec-tasks-T-02-02/notes-custom.template.md`

```markdown
# Technical Notes - {{TASK_ID}} Session {{SESSION_ID}}

... (standard sections) ...

## Parity Analysis

### LocAgent Comparison

- **Metric**: Graph node count
- **LocAgent**: 658 nodes
- **CDSAgent**: 662 nodes (+0.6%)
- **Status**: ✅ Within 2% threshold

### Performance Delta

- **LocAgent**: Index build 5.2s
- **CDSAgent**: Index build 2.3s (-55%)
- **Status**: ✅ 2-5x faster (target met)
```

### Template Variables Injection

Scripts can inject custom variables:

```shell
# In create-session-worklog.sh
export PARITY_TARGET="90%"
export PERFORMANCE_TARGET="<500ms p95"

# Use in template:
{{PARITY_TARGET}}  # Expands to "90%"
```

---

## Advanced Template Patterns

### Pattern 1: Combined Session Worklogs

**Scenario**: Sessions 01-02 are both planning (no code), combine worklogs.

**Solution**:

```markdown
# Work Summary - T-02-02 Sessions 01-02

**Date**: 2025-10-31
**Sessions**: 01 & 02
**Developer**: Claude Code Agent
**Description**: Phase 0 Planning & Analysis (Combined)

## Session 01 (07:17-08:30, 1.2h)

### Objectives
- Worktree initialization
- Documentation updates

### Work Completed
...

## Session 02 (10:22-10:55, 0.55h)

### Objectives
- Re-analysis & development roadmap

### Work Completed
...

## Combined Deliverables

- ✅ Planning complete (Sessions 01-02)
- ✅ Roadmap defined

## Next Steps

Session 03: Start Phase 1 implementation
```

### Pattern 2: Multi-Day Session

**Scenario**: Session paused overnight, resumed next day.

**Solution**:

**File**: `WORK-SESSIONS-05-THREADS-01-08-SUMMARY-2025-11-02-to-2025-11-03.txt`

```text
CDSAgent Task T-02-02 Session 05 Summary
========================================

SESSION METADATA
- Session ID: 05
- Dates: 2025-11-02 to 2025-11-03 (multi-day)
- Phase: Phase 3
- Threads: 01-08
- Duration: 22:00-23:30 (Day 1) + 08:00-10:30 (Day 2) = 4h total

... (rest of RAW log)
```

### Pattern 3: Incremental Metadata Updates

**Scenario**: Update metadata.yaml after each session.

**Solution**: Append to `sessions[]` array:

```shell
# After Session 04
cat >> .artifacts/spec-tasks-T-02-02/metadata.yaml <<EOF
  - id: "04"
    date: "2025-11-01"
    phase: "Phase 2"
    description: "Custom Tokenizer + BM25 Scaffold"
    threads:
      count: 7
      range: "01-07"
    duration:
      hours: 3.2
    status: completed
    raw_log: "./worklogs/raw/WORK-SESSIONS-04-THREADS-01-07-SUMMARY-2025-11-01.txt"
EOF
```

---

## Template Validation

### Validation Checklist

**Metadata Validation**:

```shell
# Check required fields present
grep -q "^task:" .artifacts/spec-tasks-T-XX-XX/metadata.yaml && echo "✅ task section"
grep -q "^specs:" .artifacts/spec-tasks-T-XX-XX/metadata.yaml && echo "✅ specs section"
grep -q "^git:" .artifacts/spec-tasks-T-XX-XX/metadata.yaml && echo "✅ git section"

# Check no placeholders remain
grep "{{" .artifacts/spec-tasks-T-XX-XX/metadata.yaml
# (should be empty)
```

**Worklog Validation**:

```shell
# Check session files exist
[ -f .artifacts/spec-tasks-T-XX-XX/worklogs/{date}-S{NN}-work-summary.md ] && echo "✅ work-summary"
[ -f .artifacts/spec-tasks-T-XX-XX/worklogs/{date}-S{NN}-commit-log.md ] && echo "✅ commit-log"
[ -f .artifacts/spec-tasks-T-XX-XX/worklogs/{date}-S{NN}-notes.md ] && echo "✅ notes"

# Check RAW log exists
[ -f .artifacts/spec-tasks-T-XX-XX/worklogs/raw/WORK-SESSIONS-{NN}-*-SUMMARY-{date}.txt ] && echo "✅ RAW log"
```

**Naming Validation**:

```shell
# Check session-specific naming pattern
ls .artifacts/spec-tasks-T-XX-XX/worklogs/ | grep -E "^[0-9]{4}-[0-9]{2}-[0-9]{2}-S[0-9]{2}-.*\.md$"

# Check RAW log naming pattern
ls .artifacts/spec-tasks-T-XX-XX/worklogs/raw/ | grep -E "^WORK-SESSIONS-[0-9]{2}-THREADS-[0-9]{2}-[0-9]{2}-SUMMARY-[0-9]{4}-[0-9]{2}-[0-9]{2}\.txt$"
```

---

## References

- **Primary Documentation**: `.dev/templates/README.md`
- **Template Files**: `.dev/templates/metadata.template.yaml`, `.dev/templates/worklogs/`
- **Session Lifecycle**: `.dev/workflows/WORKLOG-HANDBOOK.md`
- **Checkpoint Workflow**: `.dev/workflows/WORK_SESSION_CHECKPOINT_WORKFLOW.md`

**Important⚠️**: Wen Write [template-usage] job's templates, metadata.yaml, worklog templates, and session artifacts, MUST run (date -u '+%Y-%m-%dT%H:%M:%SZ') get UTC TIME NOW first!!!
