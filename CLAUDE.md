# CLAUDE.md

This file provides High-level skills and guidance to "codex" AI Engineer when working with code in this "CDSAgent" repository. **CDSAgent Rust Project Refactor from LocAgent Reference(arxiv-Paper & Python Repo)**: (tmp/LocAgent/), (tmp/LocAgent/arXiv-2503.09089v2/) .

**PMP**: @spacs/tasks/0.1.0-mvp/TODO.yaml
**In-progress**: @.artifacts/spec-tasks-T-02-02-sparse-index/metadata.yaml

## CDSAgent AI Engineer Teams Skills Package

CDSAgent AI Engineer Teams Skills: (.claude/skills/csda_aiengineer_team/README.md)
Project Skills package Store them in project path and worktrees .claude/skills/ within DSCAgent project. Master Agent ("claude" and "codex") can use and manage editing these skills to perform tasks.

```tree
(.claude/skills)
.
└── csda_aiengineer_team
    ├── README.md
    ├── git-workflow-validation
    │   ├── EXAMPLES.md
    │   ├── REFERENCE.md
    │   ├── SKILL.md
    │   └── scripts
    │       ├── checkpoint-helper.sh -> ../../../../../.dev/scripts/validation/checkpoint-helper.sh
    │       └── git-notes-check.sh -> ../../../../../.dev/scripts/validation/git-notes-check.sh
    ├── session-management
    │   ├── EXAMPLES.md
    │   ├── REFERENCE.md
    │   ├── SKILL.md
    │   └── scripts
    │       ├── create-raw-log.sh -> ../../../../../.dev/scripts/session/create-raw-log.sh
    │       └── create-session-worklog.sh -> ../../../../../.dev/scripts/session/create-session-worklog.sh
    ├── task-initialization
    │   ├── EXAMPLES.md
    │   ├── REFERENCE.md
    │   ├── SKILL.md
    │   └── scripts
    │       ├── create-task-worklog.sh -> ../../../../../.dev/scripts/task/create-task-worklog.sh
    │       ├── sync-worktrees.sh -> ../../../../../.dev/scripts/task/sync-worktrees.sh
    │       └── worktree-symlink.sh -> ../../../../../.dev/scripts/task/worktree-symlink.sh
    ├── template-usage
    │   ├── EXAMPLES.md
    │   ├── REFERENCE.md
    │   └── SKILL.md
    └── worktree-management
        ├── EXAMPLES.md
        ├── REFERENCE.md
        └── SKILL.md

10 directories, 23 files
```

## TIPS!: Your macOS environment available other command-line tools $ for you optional

In your macOS environment, additional command-line tools have been installed to facilitate coding
When coding, in addition to your default shell command-line tools CMDs, you can call other CLI tools from the shell by following this rubric:

**Environment Available $**:

- Find files by file name: `fd` <https://github.com/sharkdp/fd>
- Find files with path name: `fd -p <file-path>`
- List files in a directory: `fd . <directory>`
- Find files with extension and pattern: `fd -e <extension> <pattern>`
- Find Text: `rg` (**ripgrep**) <https://github.com/BurntSushi/ripgrep>
- Structured code search: `ast-grep`
  - Default to Rust:
    - Rust → `ast-grep --lang rust -p '<pattern>'`
  - Common languages:
    - Bash → `ast-grep --lang bash -p '<pattern>'`
    - Python → `ast-grep --lang python -p '<pattern>'`
    - TypeScript → `ast-grep --lang ts -p '<pattern>'`
    - TSX (React) → `ast-grep --lang tsx -p '<pattern>'`
    - JavaScript → `ast-grep --lang js -p '<pattern>'`
    - JSON → `ast-grep --lang json -p '<pattern>'`
  - For other languages, set `--lang` appropriately.
- Select among matches: pipe to `fzf`
- JSON: `jq`
- YAML/XML: `yq`
- Dev command line tools: **Cargo**, **Rustfmt**, **Clippy**, **Ruff**, **Prettier**, **ESLint**, etc.

and any Dev command line tools, IDE API tools, Language Server Protocol (LSP) tools, etc.

Tips: `ast-grep` is available! avoid plain‑text searches (`rg`/`grep`) when you need syntax‑aware matching. Use `rg` only when a plain‑text search is explicitly requested.

## CDSAgent Dev CONSTITUTION & Dev Toolkit

### 📚 Development Process Entry Point

- .dev/README.md <📌> ["Development Process Documentation"](.dev/README.md) </📌> - **Start here**: Entry point for all workflows, tools, and templates

### 🏛️ CONSTITUTION Documents (Foundational)

- .dev/workflows/WORKTREE_WORKFLOW.md <📌> ["CDSAgent Spec-Tasks DEV-COOKING Workflow SOP"](.dev/workflows/WORKTREE_WORKFLOW.md) </📌> - Main SOP: Complete task development workflow
- .dev/tools/RFC-DEV-TOOLS.md <📌> ["Dev Toolkit Architecture & Reference"](.dev/tools/RFC-DEV-TOOLS.md) </📌> - Dev Toolkit: Script reference, usage patterns, exit codes
- .dev/workflows/WORK_SESSION_CHECKPOINT_WORKFLOW.md <📌> ["Work Session Checkpoint Workflow"](.dev/workflows/WORK_SESSION_CHECKPOINT_WORKFLOW.md) </📌> - Checkpoint Process: End-of-session review
- .dev/workflows/WORKLOG-HANDBOOK.md <📌> ["Worklog & Checkpoint Management Handbook"](.dev/workflows/WORKLOG-HANDBOOK.md) </📌> - Session Lifecycle: Worklog management, RAW logs

### 🛠️ Dev Toolkit & Scripts

- .dev/scripts/session/ <📌> ["Session Scripts"](.dev/scripts/session/) </📌> - create-session-worklog.sh, create-raw-log.sh
- .dev/scripts/task/ <📌> ["Task Scripts"](.dev/scripts/task/) </📌> - create-task-worklog.sh, worktree management
- .dev/scripts/validation/ <📌> ["Validation Scripts"](.dev/scripts/validation/) </📌> - checkpoint-helper.sh, git-notes-check.sh

### 📝 Templates & Artifacts

- .dev/templates/README.md <📌> ["Template System Documentation"](.dev/templates/README.md) </📌> - Complete template documentation
- .dev/templates/metadata.template.yaml <📌> ["Task Metadata Template"](.dev/templates/metadata.template.yaml) </📌> - Task metadata structure
- .dev/templates/worklogs/ <📌> ["Worklog Templates"](.dev/templates/worklogs/) </📌> - Session artifacts (work-summary, commit-log, notes, codereview, RAW log)

### 🔄 Workflow Guides

- .dev/workflows/NEXT_TASK_CHECKLIST.md <📌> ["Next Task Selection & Environment Setup"](.dev/workflows/NEXT_TASK_CHECKLIST.md) </📌> - Choose which task to start
- .dev/workflows/SESSION_INITIALIZATION_WORKFLOW.md <📌> ["Session Initialization Guide"](.dev/workflows/SESSION_INITIALIZATION_WORKFLOW.md) </📌> - Start new work sessions
- .dev/workflows/checkpoint/ <📌> ["Checkpoint Guides"](.dev/workflows/checkpoint/) </📌> - 11 detailed checkpoint chapters

### 📋 Task Management

- spacs/tasks/0.1.0-mvp/README.md <📌> ["CDSAgent 0.1.0-MVP Task Tracking"](spacs/tasks/0.1.0-mvp/README.md) </📌> - Task organization, workflow, and dependency flow
- spacs/tasks/0.1.0-mvp/TODO.yaml <📌> ["CDSAgent 0.1.0-MVP TODO List (PMP)"](spacs/tasks/0.1.0-mvp/TODO.yaml) </📌> - **PMP Central Registry**: All tasks, milestones, dependencies, status tracking
- spacs/issues/04-0.1.0-mvp/README.md <📌> ["Issue Organization"](spacs/issues/04-0.1.0-mvp/README.md) </📌> - Technical specifications organized by component
- spacs/prd/0.1.0-MVP-PRDs-v0/ <📌> ["Product Requirements"](spacs/prd/0.1.0-MVP-PRDs-v0/) </📌> - Product requirements documents (10 PRDs)

---

## PMP & Task Hierarchy

### Specification Hierarchy (PRD → Issue → Task)

CDSAgent follows a hierarchical design flow from requirements to implementation:

```text
PRDs (spacs/prd/)           → What to build (Product Requirements)
    ↓
Issues (spacs/issues/)      → How to build (Technical Specifications)
    ↓
Tasks (spacs/tasks/)        → Concrete work (Implementation Tasks)
    ↓
metadata.yaml (.artifacts/) → In-progress tracking (Session Details)
```

**Navigation Pattern**:

1. **Start with PMP**: `spacs/tasks/0.1.0-mvp/TODO.yaml` - Central registry with all tasks, milestones, dependencies
2. **Find Task**: Navigate to `spacs/tasks/0.1.0-mvp/{component}/T-XX-XX-task-name.md`
3. **Check Dependencies**: Task file references Issue and PRD sections
4. **Track Progress**: `.artifacts/spec-tasks-T-XX-XX/metadata.yaml` - Current task metadata, sessions, worklogs

**Key Files**:

- **PMP Metadata**: `spacs/tasks/0.1.0-mvp/TODO.yaml` - Project Management Plan (milestones, task status, dependencies)
- **In-Progress Metadata**: `.artifacts/spec-tasks-T-XX-XX/metadata.yaml` - Active task details, session tracking, worklogs
- **Task Specification**: `spacs/tasks/0.1.0-mvp/{component}/T-XX-XX-task-name.md` - Links to Issue and PRD references

**Task Dependencies**:

Each task file includes:

- **Issue Reference**: Links to `spacs/issues/04-0.1.0-mvp/{component}/`
- **PRD References**: Links to specific PRD sections
- **Dependencies**: Prerequisite tasks (blocks/blocks-by)
- **Acceptance Criteria**: Definition of done

> **See**: [spacs/tasks/0.1.0-mvp/README.md](spacs/tasks/0.1.0-mvp/README.md) for complete task workflow and dependency flow.

---

## AI Engineer's Skills & Development Tools Quick Reference

### Essential Skills for CDSAgent Development

**Session-Based Development**:

- Multiple sessions per day are normal (Sessions 01-03 on same day)
- Sessions numbered sequentially across all days: 01, 02, 03, 04, 05...
- Thread numbers reset to 01 for each new session
- File naming: `{date}-S{NN}-work-summary.md` (session-specific)

**Worktree Management**:

- Each task has dedicated worktree: `~/dev-space/CDSAgent-T-XX-XX-task-name`
- Always run scripts FROM worktree using relative paths: `./.dev/scripts/...`
- Symlinks managed via: `./.dev/scripts/task/worktree-symlink.sh create`

**Session Lifecycle**:

1. **Start Session**: `./.dev/scripts/session/create-session-worklog.sh T-XX-XX NN "Description" "Developer Name"`
2. **Development Work**: Work through threads 01-NN
3. **End Session**: `./.dev/scripts/session/create-raw-log.sh T-XX-XX NN START END "Description"`
4. **Checkpoint**: `./.dev/scripts/validation/checkpoint-helper.sh T-XX-XX` → Follow checkpoint workflow

**Before Every Git Push**:

- Run: `./.dev/scripts/validation/git-notes-check.sh`
- Add missing notes: `git notes add <hash> -m "..."` → `git push origin refs/notes/commits`

**Key Scripts** (8 total):

- **Session**: `create-session-worklog.sh`, `create-raw-log.sh`
- **Task**: `create-task-worklog.sh`, `create-daily-worklog.sh` (legacy), `sync-worktrees.sh`, `worktree-symlink.sh`
- **Validation**: `checkpoint-helper.sh`, `git-notes-check.sh`

**Exit Codes**:

- `0` = Success, proceed
- `1` = Failure, fix and retry
- `2` = Warnings, review and decide

> **See**: [RFC-DEV-TOOLS.md](.dev/tools/RFC-DEV-TOOLS.md) for complete script documentation, signatures, and troubleshooting.

---

## Status Snapshot — 2025-10-31

- **Current milestone**: M2 – Core Indexing Prototype (Week 2-3) is active in this worktree. T-02-02 "Sparse Index" kicked off on 2025-10-31 after clearing T-02-01.
- **Completed 2025-10-30**: T-02-01 "Graph Builder" merged with ≤2% parity variance across 6 SWE-bench Lite fixtures, 23 Rust unit tests (>80% coverage), and parity baselines under `tests/fixtures/parity/golden_outputs/`.
- **In progress**: T-02-02 is implementing the hierarchical sparse index (name/ID map + BM25) in `crates/cds-index/src/index/`, with benchmarks in `crates/cds-index/benches/search_bench.rs` and parity/latency tests slated for `crates/cds-index/tests/`.
- **Next unlocks**: T-02-03 (service layer) and T-03-01 (CLI core commands) stay blocked until T-02-02 reaches acceptance (search overlap@10 ≥90%, p95 latency <500 ms).
- **Task tracker hygiene**: Mirror status updates in `spacs/tasks/0.1.0-mvp/TODO.yaml` and associated metadata files inside `.artifacts/spec-tasks-*` after each work session checkpoint.

## Repository Overview

CDSAgent is a graph-based code retrieval system built with Rust (core indexing) and TypeScript (LLM orchestration). The repository contains:

- **Rust**: `crates/cds-index/` — Core indexing service (graph builder completed 2025-10-30; sparse index in development)
- **Rust**: `crates/cds-tools/` — CLI tools for search, traverse, and retrieve operations
- **TypeScript/Bun**: `cds-agent/` — Claude Agent SDK integration with hooks and prompts
- **Documentation**: `spacs/` — PRDs, issues, tasks, and planning documents
- **Reference**: `tmp/LocAgent/` — Original Python implementation for parity validation

## CDSAgent Core Tech Methods References from Paper & Repo !!! IMPORTANT

- **CDSAgent follows LocAgent Paper**: `tmp/LocAgent/arXiv-2503.09089v2`
- **LocAgent paper source**: <https://arxiv.org/html/2503.09089v2>
- **CDSAgent refactors LocAgent Repo**: `tmp/LocAgent/`
- **LocAgent repo source**: <https://github.com/gersteinlab/LocAgent>

## Project Structure

```tree
CDSAgent/
├── crates/
│   ├── cds-index/          # Rust: Index Service
│   │   ├── src/
│   │   │   ├── graph/      # AST parsing & graph building (T-02-01 completed)
│   │   │   │   ├── builder/   # Modular builders: aliases, imports, language, python/
│   │   │   │   ├── parser.rs  # Tree-sitter helpers
│   │   │   │   └── traversal.rs
│   │   │   ├── index/      # Name index + BM25 search (T-02-02 in progress)
│   │   │   │   ├── bm25.rs     # Ranking backend scaffold
│   │   │   │   ├── name_index.rs
│   │   │   │   └── mod.rs
│   │   │   ├── service/    # JSON-RPC server
│   │   │   └── bin/        # cds-index-service binary
│   │   └── tests/          # Integration tests
│   └── cds-tools/          # Rust: CLI Tools
│       ├── src/
│       │   ├── commands/   # search, traverse, retrieve
│       │   ├── client/     # JSON-RPC client
│       │   └── formatters/ # JSON, text, tree output
│       └── tests/          # CLI integration tests
├── cds-agent/              # TypeScript/Bun: Agent
│   ├── src/
│   │   ├── agent-config.ts
│   │   ├── system-prompt.ts
│   │   └── hooks/          # PreToolUse, PostToolUse, SubagentStop
│   └── .claude/agents/     # Subagent configurations
├── spacs/                  # Specifications
│   ├── prd/                # Product requirements
│   ├── issues/             # Technical specs
│   ├── tasks/              # Implementation tasks
│   └── plan/               # Backlog and roadmap
├── tmp/                    # Reference implementations
│   └── LocAgent/           # Original Python code (parity reference)
├── justfile                # Build automation
├── Cargo.toml              # Rust workspace
└── README.md               # Development guide
```

## Commands and workflows

### Refactor Reference: LocAgent (Python) — tmp/LocAgent

Environment

- Python 3.12 (conda recommended)
- Install deps:

```shell
# from repo root
conda create -n locagent python=3.12 -y
conda activate locagent
pip install -r tmp/LocAgent/requirements.txt
```

Key environment variables (export before running):

```shell
# Add project root (or tmp/LocAgent) to PYTHONPATH for imports
export PYTHONPATH="$PYTHONPATH:$(pwd)/tmp/LocAgent"
# Prebuilt index locations (optional but recommended)
export GRAPH_INDEX_DIR="{INDEX_DIR}/{DATASET_NAME}/graph_index_v2.3"
export BM25_INDEX_DIR="{INDEX_DIR}/{DATASET_NAME}/BM25_index"
```

Build indexes (optional but recommended)

- Graph index (batch over datasets):

```shell
python tmp/LocAgent/dependency_graph/batch_build_graph.py \
  --dataset 'czlll/SWE-bench_Lite' \
  --split 'test' \
  --num_processes 50 \
  --download_repo
```

- BM25 sparse index:

```shell
python tmp/LocAgent/build_bm25_index.py \
  --dataset 'czlll/SWE-bench_Lite' \
  --split 'test' \
  --num_processes 100 \
  --download_repo
```

Run code localization

```shell
# Set your model endpoint first (example)
# export OPENAI_API_KEY="..."
# export OPENAI_API_BASE="https://api.openai.com/v1"

python tmp/LocAgent/auto_search_main.py \
  --dataset 'czlll/SWE-bench_Lite' \
  --split 'test' \
  --model 'azure/gpt-4o' \
  --localize \
  --merge \
  --output_folder results/location \
  --eval_n_limit 300 \
  --num_processes 50 \
  --use_function_calling \
  --simple_desc
```

Scripts (shortcuts)

```shell
# generate graph index for SWE-bench and Loc-Bench
bash tmp/LocAgent/scripts/gen_graph_index.sh
# generate BM25 index
bash tmp/LocAgent/scripts/gen_bm25_index.sh
# example run with env var template
bash tmp/LocAgent/scripts/run.sh
```

Format (no dedicated linter configured)

```shell
black tmp/LocAgent
```

Tests

- No test suite currently included for LocAgent.

### Email Agent demo (TypeScript + Bun) — tmp/claude-agent-sdk/claude-code-sdk-demos/email-agent

Important rules (from this subproject’s CLAUDE.md)

- Always use Bun for this project
- Subagents live under agent/.claude/agents

Setup and run (local-only demo)

```shell
# from repo root
cd tmp/claude-agent-sdk/claude-code-sdk-demos/email-agent
bun install
cp .env.example .env  # edit credentials for local IMAP testing
bun run dev           # starts server and client
# open http://localhost:3000
```

Build client bundle

```shell
bun run build
```

Tests (Jest)

```shell
# all tests
bun run test
# watch mode
bun run test:watch
# coverage
bun run test:coverage
# run a single test file or by name
bun run jest -- path/to/file.test.ts -t "test name pattern"
```

Linting

- No ESLint/Prettier config is wired in this demo; rely on TypeScript + Jest checks.

## High-level architecture and structure

### LocAgent (Python)

- dependency_graph/ — builds and traverses a directed heterogeneous code graph
  - build_graph.py (graph construction, graph version v2.3)
  - traverse_graph.py (RepoEntitySearcher, RepoDependencySearcher APIs)
  - batch_build_graph.py (dataset-scale index build)
- repo_index/ — semantic code search and indexing
  - index/ (FAISS-backed retrieval, settings/types)
  - codeblocks/ (code parsing to blocks, language-specific parsers)
  - utils/ (repo utilities, tokenization)
- plugins/location_tools/ — retrieval and repo operations
  - retriever/ (BM25 and fuzzy retrievers)
  - repo_ops/ (repo-level ops, issue handling)
  - utils/ (formatting, dependency helpers)
- util/ — runtime, prompts, and action framework
  - runtime/ (IPython execution, function-calling glue)
  - prompts/ (Jinja2 templates and pipelines)
  - actions/ (action spec and parser)
  - benchmark/ (git repo prep, patch parsing, oracle generation)
- evaluation/ — metrics and notebook for evaluation
- Entry points — auto_search_main.py (main agent), scripts/*.sh (common flows)

Data flow (big picture)

- Retrieve candidates via BM25/semantic index (repo_index, plugins/retriever)
- Traverse code graph for multi-hop reasoning (dependency_graph)
- Orchestrate agent actions and tool calls (util/runtime, actions, prompts)
- Produce localization outputs and optional evaluation artifacts (evaluation)

Key environment assumptions

- GRAPH_INDEX_DIR and BM25_INDEX_DIR point to prebuilt indices (optional; built on-demand otherwise)
- Model access via litellm-compatible endpoints (e.g., OpenAI/Azure/Claude/Qwen)

### Email Agent demo (TypeScript + Bun)

- ccsdk/ — Claude Agent SDK integration (client/session/tools)
- server/ — REST/WebSocket endpoints and server bootstrap
- client/ — React UI (bundled via Bun), real-time streaming
- database/ — SQLite-backed local cache and search
- agent/.claude/agents — subagents and prompts for Claude Code agent flows

Notes

- This is a local demo with plain-text env vars and no auth; do not deploy to production.

## Cross-references and source materials

- tmp/LocAgent/README.md and CLAUDE.md — commands, datasets, env vars, and model support
- tmp/LocAgent/AGENTS.md — architecture summary and common commands
- tmp/claude-agent-sdk/claude-code-sdk-demos/email-agent/README.md and CLAUDE.md — demo setup, Bun usage, test scripts
- tmp/claude-code-cli-docs — CLI documentation (no build/run)
