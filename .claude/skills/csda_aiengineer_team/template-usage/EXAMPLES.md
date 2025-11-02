# Template Usage - Real-World Examples

This document provides practical scenarios demonstrating template usage in CDSAgent development.

## Table of Contents

- [Scenario 1: Creating metadata.yaml from Template](#scenario-1-creating-metadatayaml-from-template)
- [Scenario 2: Generating Session Worklogs](#scenario-2-generating-session-worklogs)
- [Scenario 3: Updating metadata.yaml After Session](#scenario-3-updating-metadatayaml-after-session)
- [Scenario 4: Custom Template for Parity Tasks](#scenario-4-custom-template-for-parity-tasks)

---

## Scenario 1: Creating metadata.yaml from Template

**Context**: Initialize T-02-02-sparse-index task metadata from template.

**Step 1**: Run Initialization Script

```shell
# Navigate to worktree
cd ~/dev-space/CDSAgent-T-02-02-sparse-index

# Create metadata from template
./.dev/scripts/task/create-task-worklog.sh T-02-02 "Sparse Index - Name/ID + BM25 Search"

# Output:
# ✅ Created .artifacts/spec-tasks-T-02-02-sparse-index/metadata.yaml (from template)
```

**Step 2**: View Generated File

```shell
cat .artifacts/spec-tasks-T-02-02-sparse-index/metadata.yaml
```

**Generated metadata.yaml**:

```yaml
# Task Metadata for T-02-02-sparse-index
# Auto-generated on 2025-10-31

task:
  id: T-02-02-sparse-index
  title: "Sparse Index - Name/ID + BM25 Search"
  owner: "Claude Code Agent"
  status: in_progress
  start_date: "2025-10-31"
  last_updated: 2025-10-31T07:17:00Z
  estimated_hours: 32
  actual_hours: 0

specs:
  prds:
    - spacs/prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md
  issues:
    - spacs/issues/04-0.1.0-mvp/02-index-core/02-sparse-index.md
  tasks:
    - spacs/tasks/0.1.0-mvp/02-index-core/T-02-02-sparse-index.md

git:
  worktree: .worktrees/T-02-02-sparse-index
  branch: feat/task/T-02-02-sparse-index
  base_commit: 2a2ad34
  commits: []

deliverables: []

acceptance_criteria: []

dependencies:
  requires:
    - T-02-01-graph-builder
  blocks:
    - T-02-03-service-layer
    - T-03-01-cli-commands

sessions: []

metrics:
  lines_added: 0
  lines_deleted: 0
  files_modified: 0
  tests_added: 0
  test_pass_rate: null
  test_coverage: null
```

**Step 3**: Customize Metadata

```shell
# Edit metadata.yaml to add acceptance criteria
vim .artifacts/spec-tasks-T-02-02-sparse-index/metadata.yaml

# Add:
acceptance_criteria:
  - criterion: "Upper index (name/ID HashMap) with prefix matching"
    status: not_started
  - criterion: "Lower index (BM25 k1=1.5, b=0.75)"
    status: not_started
  - criterion: "Search latency <500ms p95"
    status: not_started
```

**Result**: Task metadata initialized from template, ready to customize.

---

## Scenario 2: Generating Session Worklogs

**Context**: Create Session 04 worklogs from templates.

**Step 1**: Run Session Initialization

```shell
# Create Session 04 worklogs
./.dev/scripts/session/create-session-worklog.sh T-02-02 04 "Phase 2 Custom Tokenizer + BM25 Scaffold" "Claude Code Agent"

# Output:
# ✅ Created .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-11-01-S04-work-summary.md
# ✅ Created .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-11-01-S04-notes.md
# ✅ Created .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-11-01-S04-commit-log.md
```

**Step 2**: View Generated work-summary.md

```markdown
# Work Summary - T-02-02 Session 04

**Date**: 2025-11-01
**Session**: 04
**Developer**: Claude Code Agent
**Description**: Phase 2 Custom Tokenizer + BM25 Scaffold
**Phase**: Phase 2
**Threads**: (to be filled)
**Duration**: (to be filled)

## Session Objectives

- [ ] Design custom tokenizer matching LocAgent behavior
- [ ] Implement BM25 index scaffold with Tantivy backend
- [ ] Add stop-word integration (NLTK parity)

## Work Completed

### Thread 01: [Phase 2 Planning]

(to be filled)

### Thread 02: [Tokenizer Parity Analysis]

(to be filled)

...

## Deliverables

- [ ] tokenizer.rs (custom tokenizer)
- [ ] bm25.rs (Tantivy backend)
- [ ] stop_words.rs (NLTK integration)

## Metrics

- **Duration**: (to be filled)
- **Threads Completed**: (to be filled)
- **Commits**: (to be filled)
- **Lines Added**: (to be filled)

## Next Steps

1. (to be filled)
```

**Step 3**: Fill Template During Session

After completing threads, update work-summary.md:

```markdown
## Work Completed

### Thread 01: Phase 2 Planning & Spec Alignment

- ✅ Reviewed Phase 2 requirements
- ✅ Analyzed tokenizer parity with LocAgent
- ✅ Defined implementation roadmap

### Thread 02: Tokenizer Parity Analysis

- ✅ Analyzed LocAgent tokenizer behavior
- ✅ Identified offset preservation requirement
- ✅ Designed custom tokenizer API

... (7 threads total)

## Deliverables

- ✅ tokenizer.rs (387 lines) - Custom tokenizer with offset preservation
- ✅ bm25.rs (+442 lines) - Tantivy backend scaffold
- ✅ stop_words.rs (180 lines) - NLTK stop-word integration
- ✅ 12 new tests (7 tokenizer, 2 BM25, 3 fixture)

## Metrics

- **Duration**: 3.2h (01:39-12:45 UTC)
- **Threads Completed**: 7 (01-07)
- **Commits**: 1 (414f7f2)
- **Lines Added**: 2013
- **Lines Deleted**: 108
- **Files Modified**: 17
- **Tests Added**: 12
- **Test Pass Rate**: 100% (78/78)

## Next Steps

1. Phase 3: BM25 integration with hierarchical search
2. Phase 4: Parity validation (overlap@10 ≥90%)
3. Phase 5: Performance benchmarking
```

**Result**: Session worklogs generated from templates, filled during/after session.

---

## Scenario 3: Updating metadata.yaml After Session

**Context**: Session 04 complete, update metadata.yaml with session details.

**Step 1**: Add Session Entry

```shell
# Edit metadata.yaml
vim .artifacts/spec-tasks-T-02-02-sparse-index/metadata.yaml

# Update sessions array:
sessions:
  - id: "04"
    date: "2025-11-01"
    day: 2
    phase: "Phase 2"
    description: "Custom Tokenizer + BM25 Scaffold"
    threads:
      count: 7
      range: "01-07"
    duration:
      start_time: "01:39"
      end_time: "12:45"
      hours: 3.2
    status: completed
    objectives:
      - "Phase 2 planning & spec alignment"
      - "Tokenizer parity analysis & design"
      - "Tokenizer module scaffolding"
      - "Stop-word fixtures & parity harness prep"
      - "Tantivy analyzer integration & offset plumbing"
      - "BM25 index scaffold & search API"
      - "BM25 persistence & benchmark planning"
    commits:
      - "414f7f2"
    raw_log: "./worklogs/raw/WORK-SESSIONS-04-THREADS-01-07-SUMMARY-2025-11-01.txt"
    artifacts:
      work_summary: "./worklogs/2025-11-01-S04-work-summary.md"
      commit_log: "./worklogs/2025-11-01-S04-commit-log.md"
      notes: "./worklogs/2025-11-01-S04-notes.md"
      codereview: "./worklogs/2025-11-01-S04-codereview.md"
    metrics:
      lines_added: 2013
      lines_deleted: 108
      files_modified: 17
      tests_added: 12
      test_pass_rate: 1.0
```

**Step 2**: Update Cumulative Metrics

```yaml
# Update task-level metrics (cumulative)
metrics:
  lines_added: 4180   # 2167 (Sessions 01-03) + 2013 (Session 04)
  lines_deleted: 890  # 782 (Sessions 01-03) + 108 (Session 04)
  files_modified: 33  # 16 (Sessions 01-03) + 17 (Session 04)
  tests_added: 17     # 5 (Sessions 01-03) + 12 (Session 04)
  test_pass_rate: 1.0
  test_coverage: 0.972

# Update actual_hours
task:
  actual_hours: 8.3  # 5.1h (Sessions 01-03) + 3.2h (Session 04)
  last_updated: 2025-11-01T12:45:00Z  # UTC timestamp of Session 04 end
```

**Step 3**: Update Acceptance Criteria

```yaml
acceptance_criteria:
  - criterion: "Upper index (name/ID HashMap) with prefix matching"
    status: completed  # ✅ Completed in Session 03
    notes: "Exact match 68 ns, prefix match 699 ns"
  - criterion: "Lower index (BM25 k1=1.5, b=0.75)"
    status: in_progress  # 🚧 Scaffold complete in Session 04, parity pending
    notes: "Tantivy backend scaffold complete, search API implemented"
  - criterion: "Search latency <500ms p95"
    status: completed  # ✅ Exceeded target in Session 03
    notes: "Prefix queries <1μs, far below 500ms target"
```

**Result**: metadata.yaml updated with Session 04 details, ready for checkpoint.

---

## Scenario 4: Custom Template for Parity Tasks

**Context**: T-06-01-parity-methodology needs custom sections for baseline extraction.

**Step 1**: Create Custom Notes Template

```shell
# Create task-specific notes template
mkdir -p .artifacts/spec-tasks-T-06-01-parity-methodology/templates
vim .artifacts/spec-tasks-T-06-01-parity-methodology/templates/notes-parity.template.md
```

**Custom Template** (notes-parity.template.md):

```markdown
# Technical Notes - {{TASK_ID}} Session {{SESSION_ID}}

**Date**: {{DATE}}
**Session**: {{SESSION_ID}}
**Phase**: {{PHASE}}

## Session Context

{{SESSION_CONTEXT}}

## Technical Decisions

{{TECHNICAL_DECISIONS}}

## Parity Baselines Extracted

### Graph Baselines

| Repository | Nodes | Edges | File |
|------------|-------|-------|------|
| {{REPO_1}} | {{NODES_1}} | {{EDGES_1}} | {{BASELINE_FILE_1}} |
| {{REPO_2}} | {{NODES_2}} | {{EDGES_2}} | {{BASELINE_FILE_2}} |

**Status**: {{BASELINE_STATUS}}

### Traverse Baselines

- **Scenarios**: {{SCENARIOS_COUNT}}
- **Coverage**: {{COVERAGE_PERCENTAGE}}%
- **Status**: {{TRAVERSE_STATUS}}

### Search Baselines

- **Queries**: {{QUERIES_COUNT}}
- **LocAgent Coverage**: {{LOCAGENT_COVERAGE}}
- **Status**: {{SEARCH_STATUS}}

## LocAgent Environment

- **Version**: {{LOCAGENT_VERSION}}
- **Python**: {{PYTHON_VERSION}}
- **Index Path**: {{INDEX_PATH}}

## References

- LocAgent repo: {{LOCAGENT_REPO_PATH}}
- Parity methodology: {{METHODOLOGY_DOC}}
```

**Step 2**: Generate Session 02 Notes with Custom Template

```shell
# Manual expansion of custom template
export TASK_ID="T-06-01-parity-methodology"
export SESSION_ID="02"
export DATE="2025-10-22"
export PHASE="Phase 2"

# Expand template
sed "s/{{TASK_ID}}/$TASK_ID/g" \
    .artifacts/spec-tasks-T-06-01/templates/notes-parity.template.md | \
sed "s/{{SESSION_ID}}/$SESSION_ID/g" | \
sed "s/{{DATE}}/$DATE/g" | \
sed "s/{{PHASE}}/$PHASE/g" \
> .artifacts/spec-tasks-T-06-01/worklogs/2025-10-22-S02-notes.md
```

**Step 3**: Fill Custom Sections

```markdown
## Parity Baselines Extracted

### Graph Baselines

| Repository | Nodes | Edges | File |
|------------|-------|-------|------|
| LocAgent | 658 | 1243 | graph_locagent.json |
| django__django-10914 | 6876 | 15234 | graph_django__django-10914.json |

**Status**: ✅ 6/6 baselines extracted

### Traverse Baselines

- **Scenarios**: 60 (10 scenarios × 6 repos)
- **Coverage**: 100%
- **Status**: ✅ Complete

### Search Baselines

- **Queries**: 50
- **LocAgent Coverage**: LocAgent only (llama-index limitation)
- **Status**: ⚠️ Partial (1/6 repos)
```

**Result**: Custom template for parity task with specialized sections.

---

## Summary

These examples demonstrate:

1. **Creating metadata.yaml**: Initialize from template, customize fields
2. **Generating Session Worklogs**: Templates create boilerplate, fill during session
3. **Updating metadata.yaml**: Add session entries and cumulative metrics
4. **Custom Templates**: Task-specific templates for specialized needs

**Key Takeaways**:

- ✅ **Templates provide structure** - consistent format across sessions
- ✅ **Fill incrementally** - update templates during/after work
- ✅ **Update UTC timestamps** - use `date -u '+%Y-%m-%dT%H:%M:%SZ'` for metadata
- ✅ **Customize when needed** - task-specific templates for special cases
- ✅ **Validate completeness** - ensure no `{{PLACEHOLDERS}}` remain

**Important⚠️**: Wen Write [template-usage] job's templates, metadata.yaml, worklog templates, and session artifacts, MUST run (date -u '+%Y-%m-%dT%H:%M:%SZ') get UTC TIME NOW first!!!
