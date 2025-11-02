# Task Initialization - Complete Reference

This document provides detailed information about CDSAgent's task initialization, worktree setup, and metadata management.

## Table of Contents

- [Task Lifecycle Overview](#task-lifecycle-overview)
- [Specification Hierarchy Deep Dive](#specification-hierarchy-deep-dive)
- [Worktree Management](#worktree-management)
- [Metadata Structure](#metadata-structure)
- [Script Signatures and Parameters](#script-signatures-and-parameters)
- [Template System](#template-system)
- [Dependency Management](#dependency-management)
- [Multi-Task Workflows](#multi-task-workflows)
- [Advanced Worktree Patterns](#advanced-worktree-patterns)
- [Troubleshooting](#troubleshooting)

---

## Task Lifecycle Overview

### Task States

Tasks progress through defined states tracked in `spacs/tasks/0.1.0-mvp/TODO.yaml`:

1. **not_started**: Task defined but not yet begun
2. **in_progress**: Worktree created, work underway
3. **completed**: All acceptance criteria met, PR merged
4. **blocked**: Waiting on dependencies

### State Transitions

```text
┌──────────────────────────────────────────────────────────────┐
│ Task Lifecycle                                               │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  [not_started]                                               │
│       ↓                                                      │
│  Check Dependencies ←── Verify prerequisites in TODO.yaml    │
│       ↓                                                      │
│  Create Worktree ←── git worktree add                        │
│       ↓                                                      │
│  Initialize Metadata ←── create-task-worklog.sh              │
│       ↓                                                      │
│  [in_progress]                                               │
│       ↓                                                      │
│  Work Sessions ←── Multiple sessions (01-NN)                 │
│       ↓                                                      │
│  Acceptance Criteria Met ←── All deliverables complete       │
│       ↓                                                      │
│  Create PR ←── git push + gh pr create                       │
│       ↓                                                      │
│  PR Merged ←── Review approved, CI passing                   │
│       ↓                                                      │
│  [completed]                                                 │
│       ↓                                                      │
│  Update TODO.yaml ←── Mark status: completed                 │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

### Parallel vs Sequential Tasks

**Sequential** (dependency-based):

```yaml
T-02-01-graph-builder:
  status: completed

T-02-02-sparse-index:
  status: in_progress
  dependencies:
    requires:
      - T-02-01-graph-builder  # ✅ Must complete before T-02-02
```

**Parallel** (independent):

```yaml
T-05-01-jsonrpc-schema:
  status: in_progress
  dependencies:
    requires: []  # No prerequisites

T-06-01-parity-methodology:
  status: in_progress
  dependencies:
    requires: []  # No prerequisites

# Both can run simultaneously
```

---

## Specification Hierarchy Deep Dive

### PRD → Issue → Task Flow

**Purpose**: Hierarchical design from requirements to implementation.

```text
PRDs (What to build)
  ↓
  Defines product requirements, user stories, acceptance criteria
  Example: spacs/prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md
  ↓
Issues (How to build)
  ↓
  Technical specifications, API contracts, implementation details
  Example: spacs/issues/04-0.1.0-mvp/02-index-core/02-sparse-index.md
  ↓
Tasks (Concrete work)
  ↓
  Actionable work items, time estimates, deliverables
  Example: spacs/tasks/0.1.0-mvp/02-index-core/T-02-02-sparse-index.md
  ↓
metadata.yaml (Execution tracking)
  ↓
  Real-time progress, sessions, commits, metrics
  Example: .artifacts/spec-tasks-T-02-02-sparse-index/metadata.yaml
```

### Navigation Pattern

**Step 1**: Start with PMP

```shell
# View all tasks
cat spacs/tasks/0.1.0-mvp/TODO.yaml

# Find T-02-02
grep -A 20 "T-02-02" spacs/tasks/0.1.0-mvp/TODO.yaml
```

**Step 2**: Read task file

```shell
# Task specification
cat spacs/tasks/0.1.0-mvp/02-index-core/T-02-02-sparse-index.md

# Contains:
# - Dependencies
# - Acceptance Criteria
# - Links to Issue and PRD
```

**Step 3**: Follow references

```shell
# Read Issue
cat spacs/issues/04-0.1.0-mvp/02-index-core/02-sparse-index.md

# Read PRD sections
cat spacs/prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md
```

**Step 4**: Initialize task

```shell
# Create worktree and metadata
# ... (initialization workflow)
```

---

## Worktree Management

### What is a Git Worktree?

A **worktree** is a separate working directory linked to the same repository, allowing multiple branches to be checked out simultaneously.

**Benefits**:

- ✅ **Isolation**: Each task has dedicated directory
- ✅ **Parallel work**: Multiple tasks can be developed simultaneously
- ✅ **Clean state**: No branch switching, no stashing
- ✅ **Artifact preservation**: Session artifacts stay in task directory

### Worktree Naming Convention

**Pattern**: `T-{COMPONENT}-{NUMBER}-{description}`

| Component | Examples | Description                    |
|-----------|----------|--------------------------------|
| 02        | T-02-01, T-02-02 | Index Core tasks       |
| 03        | T-03-01, T-03-02 | CLI Tools tasks        |
| 04        | T-04-01, T-04-02 | Agent Integration tasks|
| 05        | T-05-01, T-05-02 | API Contracts tasks    |
| 06        | T-06-01  | Parity Validation tasks        |
| 07        | T-07-01  | Deployment tasks               |
| 08        | T-08-01  | Testing tasks                  |

**Examples**:

- Directory: `.worktrees/T-02-02-sparse-index`
- Branch: `feat/task/T-02-02-sparse-index`
- Symlink: `~/dev-space/CDSAgent-T-02-02-sparse-index`

### Worktree Creation

**Create Worktree**:

```shell
# From main repository
cd ~/dev-space/CDSAgent

# Create worktree for T-02-02
git worktree add .worktrees/T-02-02-sparse-index feat/task/T-02-02-sparse-index

# Output:
# Preparing worktree (new branch 'feat/task/T-02-02-sparse-index')
# HEAD is now at <commit-hash> <last-commit-message>

# Navigate to worktree
cd .worktrees/T-02-02-sparse-index
```

**Worktree Structure**:

```text
.worktrees/T-02-02-sparse-index/
├── .git                       # Git directory (linked to main .git)
├── .dev/                      # Dev toolkit (accessible)
├── .artifacts/                # Task artifacts (task-specific)
│   └── spec-tasks-T-02-02-sparse-index/
├── spacs/                     # Specs (shared across worktrees)
├── crates/                    # Code (branch-specific)
├── Cargo.toml
└── ... (other files)
```

### Symlink Creation

**Purpose**: Convenient access without long paths.

**Create Symlink**:

```shell
# From worktree
./.dev/scripts/task/worktree-symlink.sh create

# Creates:
# ~/dev-space/CDSAgent-T-02-02-sparse-index -> .worktrees/T-02-02-sparse-index

# Access via symlink
cd ~/dev-space/CDSAgent-T-02-02-sparse-index
```

**Symlink Benefits**:

- ✅ Short path: `cd ~/dev-space/CDSAgent-T-02-02` instead of `cd ~/dev-space/CDSAgent/.worktrees/T-02-02-sparse-index`
- ✅ Autocomplete: Easier to tab-complete
- ✅ Session affinity: IDE/editor can remember symlink path

### Listing Worktrees

```shell
# List all worktrees
git worktree list

# Output:
# /Users/arthur/dev-space/CDSAgent         (main)
# /Users/arthur/dev-space/CDSAgent/.worktrees/T-02-01-graph-builder  (feat/task/T-02-01)
# /Users/arthur/dev-space/CDSAgent/.worktrees/T-02-02-sparse-index   (feat/task/T-02-02)

# List with commit hashes
git worktree list -v
```

### Removing Worktrees

```shell
# Remove worktree after task completion
git worktree remove .worktrees/T-02-01-graph-builder

# Force remove (if uncommitted changes)
git worktree remove --force .worktrees/T-02-01-graph-builder

# Remove symlink
rm ~/dev-space/CDSAgent-T-02-01-graph-builder
```

---

## Metadata Structure

### metadata.yaml Purpose

The `metadata.yaml` file tracks real-time task progress, sessions, commits, and metrics.

**Location**: `.artifacts/spec-tasks-{TASK_ID}/metadata.yaml`

### Full Metadata Schema

```yaml
# Task Metadata for T-02-02-sparse-index
# Auto-generated on 2025-10-31

task:
  id: T-02-02-sparse-index
  title: "Sparse Index - Name/ID + BM25 Search"
  owner: "Claude Code Agent + Rust Dev 2"
  status: in_progress
  start_date: "2025-10-31"
  last_updated: 2025-11-02T02:26:01Z
  estimated_hours: 32
  actual_hours: 8.3

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
  commits:
    - hash: "414f7f2"
      message: "feat(index): T-02-02 Phase 2 - tokenizer + BM25 scaffold"
      date: "2025-11-01"
      files_changed: 17
      notes: "Session 04 Thread 06"
    - hash: "7b624c4"
      message: "checkpoint(worklog): T-02-02 Day 1 Session 03 complete"
      date: "2025-10-31"
      files_changed: 5
      notes: "Session 03 Checkpoint"

  pr:
    number: null
    url: null
    status: null
    merged_at: null

deliverables:
  - crates/cds-index/src/index/name_index.rs
  - crates/cds-index/src/index/bm25.rs
  - crates/cds-index/src/index/tokenizer.rs
  - crates/cds-index/benches/search_bench.rs
  - crates/cds-index/tests/index_tests.rs
  - crates/cds-index/tests/search_parity_tests.rs

acceptance_criteria:
  - criterion: "Upper index (name/ID HashMap) with prefix matching"
    status: completed
    notes: "Exact match 68 ns, prefix match 699 ns"
  - criterion: "Lower index (BM25 k1=1.5, b=0.75)"
    status: in_progress
    notes: "Scaffold complete, parity validation pending"
  - criterion: "Search latency <500ms p95"
    status: completed
    notes: "Prefix queries <1μs, far below target"
  - criterion: "Index build <5s for 1K files"
    status: completed
    notes: "Build 1,024 entities in 2.287 ms"
  - criterion: "Search overlap@10 ≥90% on 50 queries"
    status: not_started
  - criterion: "Unit test coverage >95%"
    status: completed
    notes: "97.20% lines, 95.35% functions"

dependencies:
  requires:
    - T-02-01-graph-builder  # ✅ COMPLETED
  blocks:
    - T-02-03-service-layer
    - T-03-01-cli-commands

notes: |
  Implementation Phases Plan:
  - Phase 0: Planning & Analysis ✅
  - Phase 1: Upper Index ✅
  - Phase 2: Custom Tokenizer ✅
  - Phase 3: BM25 Lower Index (in progress)
  - Phase 4: Hierarchical Search Strategy
  - Phase 5: Parity & Benchmarking

worklog:
  base_path: ".artifacts/spec-tasks-T-02-02-sparse-index/worklogs/"
  entries:
    - date: "2025-10-31-S01-S02"
      session: "01&02"
      summary: "worklogs/2025-10-31-S01-S02-work-summary.md"
      commits: "worklogs/2025-10-31-S01-S02-commit-log.md"
      notes: "worklogs/2025-10-31-S01-S02-notes.md"
      raw:
        - "./worklogs/raw/WORK-SESSIONS-01-THREADS-01-03-SUMMARY-2025-10-31.txt"
        - "./worklogs/raw/WORK-SESSIONS-02-THREADS-01-03-SUMMARY-2025-10-31.txt"

sessions:
  - id: "01"
    date: "2025-10-31"
    day: 1
    phase: "Phase 0"
    description: "Planning & Analysis"
    threads:
      count: 3
      range: "01-03"
    duration:
      start_time: "07:17"
      end_time: "08:30"
      hours: 1.2
    status: completed
    objectives:
      - "Worktree initialization"
      - "Documentation updates"
      - "Implementation planning"
    commits:
      - "4f834f6"
    raw_log: "./worklogs/raw/WORK-SESSIONS-01-THREADS-01-03-SUMMARY-2025-10-31.txt"
    artifacts:
      work_summary: "./worklogs/2025-10-31-S01-S02-work-summary.md"
      commit_log: "./worklogs/2025-10-31-S01-S02-commit-log.md"
      notes: "./worklogs/2025-10-31-S01-S02-notes.md"
    metrics:
      lines_added: 986
      files_modified: 6

metrics:
  lines_added: 4180
  lines_deleted: 890
  files_modified: 33
  tests_added: 17
  test_coverage: 0.972

comments:

related_artifacts:
```

### Critical Metadata Fields

| Field | Purpose | Update Frequency |
|-------|---------|------------------|
| `task.status` | Current task state | Per state change |
| `task.last_updated` | UTC timestamp | Every metadata update |
| `task.actual_hours` | Cumulative hours | Per session |
| `git.commits[]` | Commit history | Per commit |
| `acceptance_criteria[]` | Progress tracking | Per criterion met |
| `sessions[]` | Session log | Per session |
| `metrics.*` | Cumulative stats | Per session |

---

## Script Signatures and Parameters

### create-task-worklog.sh

**Purpose**: Initialize task metadata from template.

**Signature**:

```shell
./.dev/scripts/task/create-task-worklog.sh TASK_ID "TITLE"
```

**Parameters**:

| Parameter | Type | Example | Description |
|-----------|------|---------|-------------|
| `TASK_ID` | String | `T-02-02` | Task identifier (T-XX-XX format) |
| `TITLE` | String | `"Sparse Index - Name/ID + BM25 Search"` | Task title from TODO.yaml |

**Validation**:

- Task ID must match `T-XX-XX` pattern
- Title required (non-empty)

**Exit Codes**:

- `0`: Success, metadata created
- `1`: Invalid parameters or task directory exists
- `2`: Template file not found

**Creates**:

```text
.artifacts/spec-tasks-{TASK_ID}/
  ├── metadata.yaml         # From template
  ├── worklogs/             # Empty directory
  │   └── raw/              # RAW logs directory
  └── CLAUDE.md (optional)  # Task-specific guide
```

**Template Source**: `.dev/templates/metadata.template.yaml`

### worktree-symlink.sh

**Purpose**: Manage worktree symlinks for convenient access.

**Signature**:

```shell
./.dev/scripts/task/worktree-symlink.sh <action>
```

**Actions**:

| Action | Description | Example |
|--------|-------------|---------|
| `create` | Create symlink from ~/dev-space/ to worktree | `worktree-symlink.sh create` |
| `remove` | Remove existing symlink | `worktree-symlink.sh remove` |
| `list` | List all worktree symlinks | `worktree-symlink.sh list` |

**Create Symlink**:

```shell
# From worktree directory
./.dev/scripts/task/worktree-symlink.sh create

# Creates:
# ~/dev-space/CDSAgent-{TASK_ID} -> .worktrees/{TASK_ID}
```

**Remove Symlink**:

```shell
# From worktree directory
./.dev/scripts/task/worktree-symlink.sh remove

# Removes:
# ~/dev-space/CDSAgent-{TASK_ID}
```

**List Symlinks**:

```shell
# From anywhere
./.dev/scripts/task/worktree-symlink.sh list

# Output:
# ~/dev-space/CDSAgent-T-02-01-graph-builder -> .worktrees/T-02-01-graph-builder
# ~/dev-space/CDSAgent-T-02-02-sparse-index -> .worktrees/T-02-02-sparse-index
```

**Exit Codes**:

- `0`: Success
- `1`: Failure (not in worktree, symlink exists, etc.)
- `2`: Warnings

### sync-worktrees.sh

**Purpose**: Sync .dev/ and .artifacts/ across worktrees.

**Signature**:

```shell
./.dev/scripts/task/sync-worktrees.sh [--dry-run]
```

**Options**:

| Option | Description |
|--------|-------------|
| `--dry-run` | Show what would be synced without making changes |

**Usage**:

```shell
# Sync all worktrees (from main repo)
./.dev/scripts/task/sync-worktrees.sh

# Preview changes
./.dev/scripts/task/sync-worktrees.sh --dry-run
```

**What Gets Synced**:

- ✅ `.dev/` directory (workflows, scripts, templates)
- ✅ `.artifacts/` metadata templates
- ❌ Task-specific artifacts (NOT synced)

**Exit Codes**:

- `0`: Success, all worktrees synced
- `1`: Failure (permission errors, etc.)

---

## Template System

### Template Files

Located in `.dev/templates/`:

| Template | Purpose | Used By |
|----------|---------|---------|
| `metadata.template.yaml` | Task metadata structure | `create-task-worklog.sh` |
| `worklogs/work-summary.template.md` | Session summary | `create-session-worklog.sh` |
| `worklogs/notes.template.md` | Technical notes | `create-session-worklog.sh` |
| `worklogs/commit-log.template.md` | Commit history | `create-session-worklog.sh` |
| `worklogs/raw-session.template.txt` | RAW log | `create-raw-log.sh` |

### Template Variables

| Variable | Example | Description |
|----------|---------|-------------|
| `{{TASK_ID}}` | `T-02-02` | Task identifier |
| `{{TASK_TITLE}}` | `Sparse Index - Name/ID + BM25 Search` | Task title |
| `{{DATE}}` | `2025-11-02` | Current date (YYYY-MM-DD) |
| `{{SESSION_NUM}}` | `05` | Session number (2 digits) |
| `{{DEVELOPER}}` | `Claude Code Agent` | Developer/AI agent name |

### Template Expansion Example

**Template** (metadata.template.yaml excerpt):

```yaml
task:
  id: {{TASK_ID}}
  title: "{{TASK_TITLE}}"
  owner: "{{DEVELOPER}}"
  status: in_progress
  start_date: "{{DATE}}"
  last_updated: {{UTC_TIMESTAMP}}
  estimated_hours: 32
  actual_hours: 0
```

**Expanded** (.artifacts/spec-tasks-T-02-02-sparse-index/metadata.yaml):

```yaml
task:
  id: T-02-02-sparse-index
  title: "Sparse Index - Name/ID + BM25 Search"
  owner: "Claude Code Agent"
  status: in_progress
  start_date: "2025-10-31"
  last_updated: 2025-10-31T07:17:00Z
  estimated_hours: 32
  actual_hours: 0
```

---

## Dependency Management

### Checking Dependencies

**Step 1**: View task in TODO.yaml

```shell
cat spacs/tasks/0.1.0-mvp/TODO.yaml | grep -A 20 "T-02-02"
```

**Output**:

```yaml
T-02-02-sparse-index:
  title: "Sparse Index - Name/ID + BM25 Search"
  milestone: M2
  status: not_started
  dependencies:
    requires:
      - T-02-01-graph-builder  # Prerequisite
    blocks:
      - T-02-03-service-layer  # Dependent tasks
      - T-03-01-cli-commands
```

**Step 2**: Verify prerequisites

```shell
# Check T-02-01 status
cat spacs/tasks/0.1.0-mvp/TODO.yaml | grep -A 5 "T-02-01"

# Output should show:
# status: completed  # ✅ Ready to start T-02-02
```

**Step 3**: Check blocked tasks

```shell
# View blocked_tasks list
cat spacs/tasks/0.1.0-mvp/TODO.yaml | grep -A 10 "blocked_tasks:"

# Ensure T-02-02 not in list
```

### Dependency Patterns

**Sequential Chain**:

```yaml
T-02-01 (Graph Builder)
  ↓ (blocks)
T-02-02 (Sparse Index)
  ↓ (blocks)
T-02-03 (Service Layer)
  ↓ (blocks)
T-03-01 (CLI Commands)
```

**Fan-Out** (one task unblocks many):

```yaml
T-05-01 (JSON-RPC Schema)
  ↓ (blocks)
  ├─→ T-02-03 (Service Layer)
  ├─→ T-05-02 (TypeScript Bindings)
  └─→ T-03-01 (CLI Commands)
```

**Fan-In** (many tasks required for one):

```yaml
T-03-01 (CLI Commands)
T-04-01 (Agent SDK)
  ↓ (both block)
T-04-03 (Agent Hooks)
```

---

## Multi-Task Workflows

### Parallel Task Development

**Scenario**: Working on multiple independent tasks simultaneously.

**Setup**:

```shell
# Create worktrees for T-05-01 and T-06-01 (no dependencies between them)
git worktree add .worktrees/T-05-01-jsonrpc-schema feat/task/T-05-01-jsonrpc-schema
git worktree add .worktrees/T-06-01-parity-methodology feat/task/T-06-01-parity-methodology

# Create symlinks
cd .worktrees/T-05-01-jsonrpc-schema
./.dev/scripts/task/worktree-symlink.sh create

cd .worktrees/T-06-01-parity-methodology
./.dev/scripts/task/worktree-symlink.sh create

# Work on both tasks in parallel
# Terminal 1: cd ~/dev-space/CDSAgent-T-05-01
# Terminal 2: cd ~/dev-space/CDSAgent-T-06-01
```

**Benefits**:

- ✅ No context switching (separate directories)
- ✅ Isolated branches (no conflicts)
- ✅ Parallel PR reviews

### Task Handoff

**Scenario**: AI agent completes Phase 1, hands off to human for Phase 2.

**Handoff Workflow**:

```shell
# AI Agent: Complete Phase 1
# ... (sessions, commits, checkpoint)

# Update metadata.yaml with handoff notes
# Update TODO.yaml status if needed

# Push to remote
git push origin feat/task/T-02-02-sparse-index
git push origin refs/notes/commits

# Human Developer: Pull and continue
cd ~/dev-space/CDSAgent-T-02-02-sparse-index
git pull origin feat/task/T-02-02-sparse-index
git fetch origin refs/notes/commits:refs/notes/commits

# Read handoff artifacts
cat .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-11-01-S04-work-summary.md
cat .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/raw/WORK-SESSIONS-04-*.txt

# Start next session
./.dev/scripts/session/create-session-worklog.sh T-02-02 05 "Phase 2 continuation" "Human Developer"
```

---

## Advanced Worktree Patterns

### Pattern 1: Long-Running Task Worktree

**Scenario**: Task takes 2+ weeks, keep worktree active.

**Best Practices**:

- ✅ Regular sync with main: `git pull origin main`
- ✅ Periodic push: Push commits/notes every few days
- ✅ Clean checkpoints: Checkpoint after each session
- ✅ Rebase strategy: Rebase on main before PR

### Pattern 2: Experimental Branch in Worktree

**Scenario**: Need to try alternative approach without disrupting main task.

**Setup**:

```shell
# From T-02-02 worktree
git branch feat/task/T-02-02-experiment

# Create experimental worktree
git worktree add .worktrees/T-02-02-sparse-index-experiment feat/task/T-02-02-experiment

# Work in experiment
cd .worktrees/T-02-02-sparse-index-experiment
# ... (experimental work)

# If successful, merge back
cd .worktrees/T-02-02-sparse-index
git merge feat/task/T-02-02-experiment

# If unsuccessful, abandon
git worktree remove .worktrees/T-02-02-sparse-index-experiment
git branch -D feat/task/T-02-02-experiment
```

### Pattern 3: Shared Worktree for Review

**Scenario**: Code reviewer wants to check out PR in separate worktree.

**Setup**:

```shell
# Reviewer: Fetch PR branch
git fetch origin feat/task/T-02-02-sparse-index

# Create review worktree
git worktree add .worktrees/T-02-02-sparse-index-review origin/feat/task/T-02-02-sparse-index

# Review code
cd .worktrees/T-02-02-sparse-index-review
cargo test
cargo clippy

# Remove after review
git worktree remove .worktrees/T-02-02-sparse-index-review
```

---

## Troubleshooting

### Issue: Worktree Already Exists

**Problem**: `git worktree add` fails with "worktree already exists".

**Solution**:

```shell
# List worktrees
git worktree list

# Remove old worktree
git worktree remove .worktrees/T-XX-XX

# Or force remove
git worktree remove --force .worktrees/T-XX-XX

# Create new worktree
git worktree add .worktrees/T-XX-XX feat/task/T-XX-XX
```

### Issue: Symlink Broken

**Problem**: Symlink points to non-existent directory.

**Solution**:

```shell
# Remove broken symlink
rm ~/dev-space/CDSAgent-T-XX-XX

# Navigate to worktree
cd .worktrees/T-XX-XX

# Recreate symlink
./.dev/scripts/task/worktree-symlink.sh create
```

### Issue: metadata.yaml Missing Fields

**Problem**: Metadata file incomplete after initialization.

**Solution**:

```shell
# Compare with template
diff .artifacts/spec-tasks-T-XX-XX/metadata.yaml .dev/templates/metadata.template.yaml

# Manually add missing fields from template
# Or regenerate metadata
rm .artifacts/spec-tasks-T-XX-XX/metadata.yaml
./.dev/scripts/task/create-task-worklog.sh T-XX-XX "Task Title"
```

### Issue: Wrong Dependencies in metadata.yaml

**Problem**: Dependencies don't match TODO.yaml.

**Solution**:

```shell
# Check TODO.yaml
cat spacs/tasks/0.1.0-mvp/TODO.yaml | grep -A 10 "T-XX-XX"

# Update metadata.yaml dependencies section
# Copy from TODO.yaml:
dependencies:
  requires:
    - T-YY-YY  # From TODO.yaml
  blocks:
    - T-ZZ-ZZ  # From TODO.yaml
```

### Issue: .dev/ Directory Not Accessible

**Problem**: Scripts not found in worktree.

**Solution**:

```shell
# Check if .dev/ exists
ls -la .dev/

# If missing, sync worktrees
cd ~/dev-space/CDSAgent  # Main repo
./.dev/scripts/task/sync-worktrees.sh

# Or manually create symlink (if worktree uses symlink pattern)
cd .worktrees/T-XX-XX
ln -s ../../.dev .dev
```

---

## References

- **Primary Workflow**: `.dev/workflows/WORKTREE_WORKFLOW.md`
- **Next Task Guide**: `.dev/workflows/NEXT_TASK_CHECKLIST.md`
- **Template Documentation**: `.dev/templates/README.md`
- **PMP**: `spacs/tasks/0.1.0-mvp/TODO.yaml`
- **Worktree SOP**: `.dev/workflows/WORKTREE_WORKFLOW.md`

**Important⚠️**: Wen Run [task-initialization] job changed any PMP and docs & metadata .yaml MUST run (date -u '+%Y-%m-%dT%H:%M:%SZ') get UTC TIME NOW first!!!
