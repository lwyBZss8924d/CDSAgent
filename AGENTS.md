# AGENTS.md

This file provides guidance to Codex when working with code in this "CDSAgent" repository.

- @docs/RFC-DEV-TOOLS.md <📌> ["Development Tools Architecture & Quick Reference"](docs/RFC-DEV-TOOLS.md) </📌>
- @docs/WORKTREE_WORKFLOW.md <📌> ["CDSAgent" codebase Spec-Tasks DEV-COOKING Workflow SOP](docs/WORKTREE_WORKFLOW.md) </📌>
- @docs/NEXT_TASK_CHECKLIST.md <📌> ["Next Task Selection & Environment Setup Checklist"](docs/NEXT_TASK_CHECKLIST.md) </📌>
- @docs/WORK_SESSION_CHECKPOINT_WORKFLOW.md <📌> ["Work Session Checkpoint - Review & Update Workflow"](docs/WORK_SESSION_CHECKPOINT_WORKFLOW.md) </📌>
- @spacs/tasks/0.1.0-mvp/README.md <📌> ["CDSAgent 0.1.0-MVP Task Tracking"](spacs/tasks/0.1.0-mvp/README.md) </📌>
- @spacs/tasks/0.1.0-mvp/TODO.yaml <📌> ["CDSAgent 0.1.0-MVP TODO List"](spacs/tasks/0.1.0-mvp/TODO.yaml) </📌>

---

## Repository Overview

CDSAgent is a graph-based code retrieval system built with Rust (core indexing) and TypeScript (LLM orchestration). The repository contains:

- **Rust**: `crates/cds-index/` — Core indexing service with graph builder and sparse search
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
│   │   │   ├── graph/      # AST parsing & graph building
│   │   │   ├── index/      # Name index + BM25 search
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
