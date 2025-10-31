#!/usr/bin/env lua
--[[
T-02-02 Sparse Index Worktree Initialization Plan
Version: 1.0
Date: 2025-10-31
Based on: WORKTREE_WORKFLOW.md Phase 1 + NEXT_TASK_CHECKLIST.md

PURPOSE:
  Systematic initialization of T-02-02-sparse-index worktree environment
  following the CDSAgent Spec-Tasks DEV-COOKING Workflow SOP.

ANALYSIS SOURCES:
  - .tmp-kanban/20251031/backlog/nextsteps-tasks-analysis.txt (comprehensive plan)
  - .tmp-kanban/20251031/backlog/nextsteps-tasks-analysis-result.txt (summary)

TASK METADATA:
  Task ID: T-02-02-sparse-index
  Title: Sparse Index - Name/ID + BM25 Search
  Milestone: M2 (Core Indexing Prototype)
  Week: 2
  Priority: P0 (Critical)
  Owner: Rust Dev 2
  Duration: 4 days (estimated)

DEPENDENCIES STATUS:
  ✅ T-02-01-graph-builder: COMPLETED & MERGED (PR #6, 2025-10-30)
  ✅ Main branch: Synced to 2a2ad34
  ✅ Worktree: Exists at 2a2ad34
  ✅ Graph API: Available (crates/cds-index/src/graph/)
  ✅ Parity baselines: 6 graphs, 50 queries ready

BLOCKS:
  - T-02-03-service-layer (requires T-02-02)
  - T-03-01-cli-commands (requires T-02-03)
  - T-02-04-serialization (requires T-02-01, T-02-02)
]]

--[[
╔════════════════════════════════════════════════════════════════════════════╗
║                   INITIALIZATION EXECUTION PLAN                             ║
║            Follow WORKTREE_WORKFLOW.md Phase 1: Steps 1-8                  ║
╚════════════════════════════════════════════════════════════════════════════╝
]]

local plan = {
  task_id = "T-02-02-sparse-index",
  task_title = "Sparse Index - Name/ID + BM25 Search",
  owner = "Rust Dev 2",
  worktree_path = "~/dev-space/CDSAgent-T-02-02-sparse-index",

  -- WORKTREE_WORKFLOW.md Phase 1: Worktree Environment Preparation
  steps = {

    -- ========================================================================
    -- STEP 1: Sync Main Branch (COMPLETED ✅)
    -- ========================================================================
    {
      id = "1.1",
      phase = "Sync Main",
      status = "COMPLETED",
      description = "Verify main branch at latest commit",
      commands = {
        "cd ~/dev-space/CDSAgent",
        "git checkout main",
        "git pull origin main",
        "git log -1 --oneline  # Should show: 2a2ad34"
      },
      verification = {
        "git rev-parse HEAD | grep 2a2ad34",
        "git log -1 --pretty='%s' | grep 'T-02-01'"
      },
      result = "✅ Main at 2a2ad34 - T-02-01 merged"
    },

    -- ========================================================================
    -- STEP 2: Verify Worktree (COMPLETED ✅)
    -- ========================================================================
    {
      id = "2.1",
      phase = "Verify Worktree",
      status = "COMPLETED",
      description = "Confirm worktree exists and is synced",
      commands = {
        "git worktree list | grep T-02-02",
        "cd ~/dev-space/CDSAgent-T-02-02-sparse-index",
        "git log -1 --oneline  # Should match main: 2a2ad34"
      },
      verification = {
        "test -d ~/dev-space/CDSAgent-T-02-02-sparse-index",
        "cd ~/dev-space/CDSAgent-T-02-02-sparse-index && git rev-parse HEAD | grep 2a2ad34"
      },
      result = "✅ Worktree exists at 2a2ad34, branch: feat/task/T-02-02-sparse-index"
    },

    -- ========================================================================
    -- STEP 3: Verify Symlink (COMPLETED ✅)
    -- ========================================================================
    {
      id = "3.1",
      phase = "Verify Symlink",
      status = "COMPLETED",
      description = "Confirm IDE-friendly symlink is active",
      commands = {
        "ls -la ~/dev-space/CDSAgent-T-02-02-sparse-index"
      },
      verification = {
        "test -L ~/dev-space/CDSAgent-T-02-02-sparse-index"
      },
      result = "✅ Symlink exists: ~/dev-space/CDSAgent-T-02-02-sparse-index -> .worktrees/T-02-02-sparse-index"
    },

    -- ========================================================================
    -- STEP 4: Initialize Artifacts FROM WORKTREE (TODO ⏳)
    -- ========================================================================
    {
      id = "4.1",
      phase = "Initialize Artifacts",
      status = "PENDING",
      description = "Create task artifacts (metadata.yaml, worklogs/)",
      priority = "CRITICAL",
      warning = [[
        ⚠️  CRITICAL: Must run FROM worktree, NOT from main!

        Reason: Worktrees have independent file systems (different inodes).
        Running from main creates artifacts in main's tree, which won't
        appear in worktree's snapshot.

        See: WORKTREE_WORKFLOW.md Phase 1 Step 4
      ]],
      commands = {
        "# Navigate to worktree FIRST",
        "cd ~/dev-space/CDSAgent-T-02-02-sparse-index",
        "",
        "# Run script with absolute path",
        "/Users/arthur/dev-space/CDSAgent/scripts/create-task-worklog.sh \\",
        "  T-02-02-sparse-index \\",
        "  \"Sparse Index - Name/ID + BM25 Search\" \\",
        "  \"Rust Dev 2\"",
        "",
        "# Verify artifacts created IN worktree",
        "ls -la .artifacts/spec-tasks-T-02-02-sparse-index/"
      },
      verification = {
        "test -f ~/dev-space/CDSAgent-T-02-02-sparse-index/.artifacts/spec-tasks-T-02-02-sparse-index/metadata.yaml",
        "test -f ~/dev-space/CDSAgent-T-02-02-sparse-index/.artifacts/spec-tasks-T-02-02-sparse-index/git-refs.txt",
        "test -d ~/dev-space/CDSAgent-T-02-02-sparse-index/.artifacts/spec-tasks-T-02-02-sparse-index/worklogs"
      },
      expected_files = {
        ".artifacts/spec-tasks-T-02-02-sparse-index/metadata.yaml",
        ".artifacts/spec-tasks-T-02-02-sparse-index/git-refs.txt",
        ".artifacts/spec-tasks-T-02-02-sparse-index/worklogs/"
      },
      troubleshooting = [[
        If artifacts don't appear in worktree:

        Option 1 (Recommended): Re-run from worktree
          cd ~/dev-space/CDSAgent-T-02-02-sparse-index
          /Users/arthur/dev-space/CDSAgent/scripts/create-task-worklog.sh \
            T-02-02-sparse-index "Sparse Index - Name/ID + BM25 Search" "Rust Dev 2"

        Option 2: Copy from main to worktree
          cp -r ~/dev-space/CDSAgent/.artifacts/spec-tasks-T-02-02-sparse-index \
            ~/dev-space/CDSAgent-T-02-02-sparse-index/.artifacts/

        Option 3: Delete and recreate worktree (nuclear option)
          cd ~/dev-space/CDSAgent
          git worktree remove .worktrees/T-02-02-sparse-index
          git worktree add .worktrees/T-02-02-sparse-index -b feat/task/T-02-02-sparse-index main
          cd ~/dev-space/CDSAgent-T-02-02-sparse-index
          /Users/arthur/dev-space/CDSAgent/scripts/create-task-worklog.sh ...
      ]]
    },

    -- ========================================================================
    -- STEP 5: Update Task-Specific CLAUDE.md (OPTIONAL ⚪)
    -- ========================================================================
    {
      id = "5.1",
      phase = "Update CLAUDE.md",
      status = "OPTIONAL",
      description = "Add T-02-02-specific development guidance for AI assistance",
      commands = {
        "cd ~/dev-space/CDSAgent-T-02-02-sparse-index",
        "vim CLAUDE.md  # or code CLAUDE.md"
      },
      content_to_add = [[
## Current Task: T-02-02 Sparse Index (2025-10-31 to ~2025-11-04)

### Objective
Implement hierarchical search index with upper (name/ID HashMap) and lower (BM25) retrieval layers.

### Deliverables
- `crates/cds-index/src/index/name_index.rs` - Upper index (exact/prefix matching)
- `crates/cds-index/src/index/bm25.rs` - Lower index (Tantivy-backed BM25)
- `crates/cds-index/src/index/tokenizer.rs` - camelCase/snake_case tokenizer
- `crates/cds-index/benches/search_bench.rs` - Performance benchmarks
- `crates/cds-index/tests/index_tests.rs` - Unit tests (>95% coverage)
- `crates/cds-index/tests/search_parity_tests.rs` - 50 query overlap validation

### Implementation Checklist
1. [ ] Upper Index - Name/ID HashMap (Days 1-2)
   - [ ] Exact match lookup (O(1))
   - [ ] Prefix match (binary search on sorted keys)
   - [ ] Entity type filtering (Directory/File/Class/Function)
   - [ ] Case-insensitive option
2. [ ] Custom Tokenizer (Day 3)
   - [ ] camelCase splitting: "AuthService" → ["Auth", "Service"]
   - [ ] snake_case splitting: "auth_service" → ["auth", "service"]
   - [ ] English stemming (match LocAgent)
3. [ ] BM25 Lower Index (Days 3-5)
   - [ ] Prototype Tantivy integration (k1=1.5, b=0.75)
   - [ ] Run 50-query parity test (search_queries.jsonl)
   - [ ] Decision: Use Tantivy if overlap ≥90%, else custom BM25
4. [ ] Hierarchical Search (Day 6)
   - [ ] Try upper index first (threshold: 5 results)
   - [ ] Fallback to BM25 if upper < threshold
   - [ ] Merge & deduplicate results
5. [ ] Parity & Benchmarking (Days 7-8)
   - [ ] Validate overlap@10 ≥90% on 50 queries
   - [ ] Measure search latency <500ms p95
   - [ ] Verify index build <5s for 1K files

### Key APIs to Reference
- Graph API: `crates/cds-index/src/graph/mod.rs` (from T-02-01)
  - `DependencyGraph::nodes()` - Iterate entities for indexing
  - `GraphNode` - Access display_name, kind, file_path
  - `NodeKind` - Filter by Directory/File/Class/Function
- LocAgent Reference: `tmp/LocAgent/plugins/location_tools/retriever/bm25_retriever.py`
  - BM25Retriever parameters and stemming setup
- Parity Baselines: `tests/fixtures/parity/golden_outputs/search_queries.jsonl`

### Testing Strategy
```bash
# Unit tests (run frequently)
cargo test --package cds-index --lib index

# Parity validation (run daily)
cargo test --package cds-index --test search_parity_tests

# Performance benchmarks (run before committing)
cargo bench --bench search_bench

# Full test suite (before PR)
cargo test --all && cargo clippy --all-targets
```

### Daily Worklog Commands
```bash
# Start of day
cd ~/dev-space/CDSAgent-T-02-02-sparse-index
/Users/arthur/dev-space/CDSAgent/scripts/create-daily-worklog.sh T-02-02-sparse-index
vim .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/$(date +%Y-%m-%d)-work-summary.md

# After each commit
vim .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/$(date +%Y-%m-%d)-commit-log.md

# End of day checkpoint
# Follow: docs/WORK_SESSION_CHECKPOINT_WORKFLOW.md
```

### Acceptance Criteria (from TODO.yaml)
- [ ] Upper index (name/ID HashMap) with prefix matching
- [ ] Lower index (BM25 k1=1.5, b=0.75)
- [ ] Search latency <500ms p95
- [ ] Index build <5s for 1K files
- [ ] Search overlap@10 ≥90% with LocAgent on 50 queries
- [ ] Unit test coverage >95%
- [ ] All benchmarks documented

### Technical Risks
1. **Tokenization Mismatch**: Tantivy default tokenizer may differ from LocAgent
   - Mitigation: Implement custom tokenizer matching LocAgent's camelCase/snake_case
2. **BM25 Parameter Tuning**: k1=1.5, b=0.75 may need adjustment for 90% overlap
   - Mitigation: Test multiple parameter combinations, document results
3. **Memory Usage**: In-memory BM25 may exceed 500MB for 10K files
   - Mitigation: Use Tantivy's on-disk storage (mmap)
      ]],
      verification = {
        "grep 'T-02-02' ~/dev-space/CDSAgent-T-02-02-sparse-index/CLAUDE.md"
      },
      result = "⚪ Optional - Improves AI-assisted development experience"
    },

    -- ========================================================================
    -- STEP 6: Create Daily Worklog (TODO ⏳)
    -- ========================================================================
    {
      id = "6.1",
      phase = "Create Daily Worklog",
      status = "PENDING",
      description = "Initialize today's (2025-10-31) worklog files",
      commands = {
        "cd ~/dev-space/CDSAgent-T-02-02-sparse-index",
        "",
        "/Users/arthur/dev-space/CDSAgent/scripts/create-daily-worklog.sh T-02-02-sparse-index",
        "",
        "# Verify worklogs created",
        "ls -la .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/$(date +%Y-%m-%d)-*.md"
      },
      verification = {
        "test -f ~/dev-space/CDSAgent-T-02-02-sparse-index/.artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-10-31-work-summary.md",
        "test -f ~/dev-space/CDSAgent-T-02-02-sparse-index/.artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-10-31-commit-log.md",
        "test -f ~/dev-space/CDSAgent-T-02-02-sparse-index/.artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-10-31-notes.md"
      },
      expected_files = {
        "2025-10-31-work-summary.md",
        "2025-10-31-commit-log.md",
        "2025-10-31-notes.md"
      }
    },

    -- ========================================================================
    -- STEP 7: Read Task Specifications (TODO ⏳)
    -- ========================================================================
    {
      id = "7.1",
      phase = "Read Specifications",
      status = "PENDING",
      description = "Review PRDs, Issues, and Task specifications",
      documents = {
        {
          type = "PRD",
          path = "spacs/prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md",
          sections = "§2.2 Hierarchical Search (FR-HI-3), §3.2 BM25 Configuration",
          command = "cat spacs/prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md | grep -A 50 'FR-HI-3'"
        },
        {
          type = "Issue",
          path = "spacs/issues/04-0.1.0-mvp/02-index-core/02-sparse-index.md",
          sections = "Full technical breakdown, implementation steps, acceptance criteria",
          command = "cat spacs/issues/04-0.1.0-mvp/02-index-core/02-sparse-index.md"
        },
        {
          type = "Task",
          path = "spacs/tasks/0.1.0-mvp/02-index-core/T-02-02-sparse-index.md",
          sections = "Deliverables, acceptance criteria, dependencies",
          command = "cat spacs/tasks/0.1.0-mvp/02-index-core/T-02-02-sparse-index.md"
        }
      },
      key_sections = {
        "Upper Index Design (DashMap-based name/ID lookup)",
        "Lower Index Design (BM25 k1=1.5, b=0.75)",
        "Hierarchical Search Strategy (upper threshold: 5 results)",
        "Tokenization Requirements (camelCase/snake_case splitting)",
        "Performance Targets (<500ms p95 search, <5s build for 1K files)",
        "Parity Requirements (≥90% overlap@10 on 50 queries)"
      }
    },

    -- ========================================================================
    -- STEP 8: Verify Environment Ready (TODO ⏳)
    -- ========================================================================
    {
      id = "8.1",
      phase = "Final Verification",
      status = "PENDING",
      description = "Confirm all prerequisites and resources available",
      checklist = {
        {
          item = "On correct branch",
          command = "cd ~/dev-space/CDSAgent-T-02-02-sparse-index && git branch --show-current",
          expected = "feat/task/T-02-02-sparse-index"
        },
        {
          item = "Artifacts initialized",
          command = "ls -la ~/dev-space/CDSAgent-T-02-02-sparse-index/.artifacts/spec-tasks-T-02-02-sparse-index/metadata.yaml",
          expected = "File exists"
        },
        {
          item = "Daily worklog created",
          command = "ls ~/dev-space/CDSAgent-T-02-02-sparse-index/.artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-10-31-*.md",
          expected = "3 files (work-summary, commit-log, notes)"
        },
        {
          item = "Graph API available",
          command = "ls ~/dev-space/CDSAgent-T-02-02-sparse-index/crates/cds-index/src/graph/mod.rs",
          expected = "File exists with DependencyGraph API"
        },
        {
          item = "Parity baselines available",
          command = "ls ~/dev-space/CDSAgent-T-02-02-sparse-index/tests/fixtures/parity/golden_outputs/search_queries.jsonl",
          expected = "File exists with 50 queries"
        },
        {
          item = "Clean git state",
          command = "cd ~/dev-space/CDSAgent-T-02-02-sparse-index && git status --short",
          expected = "Empty output (no uncommitted changes)"
        },
        {
          item = "Cargo dependencies ready",
          command = "cd ~/dev-space/CDSAgent-T-02-02-sparse-index && cargo check --package cds-index 2>&1 | grep -i 'Finished'",
          expected = "Finished check"
        }
      },
      final_command = "code ~/dev-space/CDSAgent-T-02-02-sparse-index  # Open in IDE"
    }
  },

  -- ========================================================================
  -- IMPLEMENTATION ROADMAP (from analysis)
  -- ========================================================================
  implementation_plan = {
    duration_estimate = "6-8 days (realistic)",
    complexity = "7/10 (BM25 + parity requirement)",

    phases = {
      {
        phase = "1",
        days = "1-2",
        title = "Upper Index - Name/ID Lookup",
        deliverables = {
          "crates/cds-index/src/index/name_index.rs",
          "Unit tests for exact/prefix/case-insensitive matching"
        },
        key_features = {
          "DashMap<String, Vec<NodeID>> for concurrent access",
          "Sorted keys Vec<String> for prefix search (binary search)",
          "Entity type filtering (Directory/File/Class/Function)",
          "Case-insensitive option"
        }
      },
      {
        phase = "2",
        days = "3",
        title = "Custom Tokenizer",
        deliverables = {
          "crates/cds-index/src/index/tokenizer.rs",
          "Tokenizer unit tests matching LocAgent behavior"
        },
        key_features = {
          "camelCase splitting: 'AuthService' → ['Auth', 'Service']",
          "snake_case splitting: 'auth_service' → ['auth', 'service']",
          "English stemming (Stemmer crate or rust-stemmers)",
          "Stop words removal"
        },
        locagent_reference = "tmp/LocAgent/plugins/location_tools/retriever/bm25_retriever.py:105-113"
      },
      {
        phase = "3",
        days = "3-5",
        title = "BM25 Lower Index",
        deliverables = {
          "crates/cds-index/src/index/bm25.rs",
          "Tantivy integration or custom BM25 implementation"
        },
        decision_point = [[
          Day 3: Prototype both Tantivy and Custom BM25
          Day 4: Run 50-query parity test (search_queries.jsonl)
          Decision: Use Tantivy if overlap@10 ≥90%, else Custom BM25
        ]],
        key_features = {
          "BM25 scoring (k1=1.5, b=0.75)",
          "Inverted index (term → [NodeID, positions])",
          "Document frequency (DF) and inverse document frequency (IDF)",
          "Top-K retrieval (K=10 default)"
        },
        tantivy_integration = {
          "Create Tantivy schema (text field for code content)",
          "Custom TextAnalyzer with our tokenizer",
          "IndexWriter for indexing",
          "Searcher for querying",
          "TopDocs collector for top-K results"
        }
      },
      {
        phase = "4",
        days = "6",
        title = "Hierarchical Search Strategy",
        deliverables = {
          "crates/cds-index/src/index/mod.rs - HierarchicalSearcher",
          "Integration tests for search flow"
        },
        search_flow = [[
          1. Query arrives: "sanitize user input"
          2. Try Upper Index: exact/prefix match on "sanitize"
             → Found ≥5 results? Return upper results (score 1.0)
          3. If upper < 5: Fallback to BM25
             → Search content with BM25 (score 0.0-1.0)
          4. Merge & deduplicate by NodeID
          5. Sort by score descending
          6. Return top-K (K=10 default)
        ]]
      },
      {
        phase = "5",
        days = "7-8",
        title = "Parity & Benchmarking",
        deliverables = {
          "crates/cds-index/benches/search_bench.rs",
          "crates/cds-index/tests/search_parity_tests.rs",
          "Performance metrics documentation"
        },
        parity_validation = [[
          Load 50 queries from search_queries.jsonl
          For each query:
            1. CDS results: HierarchicalSearcher::search()
            2. LocAgent baseline: search_queries.jsonl expected results
            3. Calculate overlap@10: |CDS ∩ LocAgent| / 10
          Assert: avg_overlap ≥ 90%
        ]],
        performance_targets = {
          "Upper index lookup: <10ms",
          "BM25 search: <500ms p95",
          "Memory: <500MB for 10K files",
          "Index build: <5s for 1K files"
        }
      }
    }
  },

  -- ========================================================================
  -- TECHNICAL RISKS & MITIGATION
  -- ========================================================================
  risks = {
    {
      risk = "Tokenization Mismatch",
      description = "Tantivy default tokenizer differs from LocAgent",
      impact = "Parity <90% overlap@10",
      mitigation = {
        "Implement custom tokenizer matching LocAgent (camelCase/snake_case)",
        "Use Tantivy's TextAnalyzer::from_iter() with custom Tokenizer",
        "Test tokenizer independently before BM25 integration"
      }
    },
    {
      risk = "BM25 Parameter Tuning",
      description = "k1=1.5, b=0.75 may not achieve 90% overlap",
      impact = "Need to tune parameters or switch to custom BM25",
      mitigation = {
        "Test parameter combinations: k1=[1.2, 1.5, 2.0], b=[0.5, 0.75, 1.0]",
        "Document parity results in spacs/issues/04-0.1.0-mvp/06-refactor-parity.md",
        "Implement custom BM25 if Tantivy can't meet parity requirement"
      }
    },
    {
      risk = "Memory Usage for Large Repos",
      description = "In-memory BM25 may exceed 500MB for 10K files",
      impact = "Service crashes or high memory pressure",
      mitigation = {
        "Use Tantivy's on-disk storage (mmap)",
        "Benchmark with largest fixture (Django: 6,876 nodes)",
        "Implement memory profiling (cargo-instruments or heaptrack)"
      }
    }
  },

  -- ========================================================================
  -- RESOURCES & REFERENCES
  -- ========================================================================
  resources = {
    locagent_reference = {
      path = "tmp/LocAgent/",
      key_files = {
        "plugins/location_tools/retriever/bm25_retriever.py - BM25 configuration",
        "dependency_graph/traverse_graph.py - RepoEntitySearcher (upper index logic)",
        "plugins/location_tools/repo_ops/repo_ops.py - search_entity() flow"
      }
    },
    parity_baselines = {
      path = "tests/fixtures/parity/golden_outputs/",
      files = {
        "search_queries.jsonl - 50 benchmark queries",
        "graph_*.json - 6 graph baselines for index building"
      }
    },
    graph_api = {
      path = "crates/cds-index/src/graph/",
      key_modules = {
        "mod.rs - DependencyGraph API documentation",
        "builder/mod.rs - GraphBuilder for constructing graphs",
        "traversal.rs - Graph traversal utilities"
      }
    },
    specifications = {
      prd = "spacs/prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md",
      issue = "spacs/issues/04-0.1.0-mvp/02-index-core/02-sparse-index.md",
      task = "spacs/tasks/0.1.0-mvp/02-index-core/T-02-02-sparse-index.md"
    }
  }
}

--[[
╔════════════════════════════════════════════════════════════════════════════╗
║                         EXECUTION INSTRUCTIONS                              ║
╚════════════════════════════════════════════════════════════════════════════╝

IMMEDIATE NEXT STEPS (in order):

1. ✅ COMPLETED: Main synced to 2a2ad34
2. ✅ COMPLETED: Worktree verified at 2a2ad34
3. ✅ COMPLETED: Symlink verified
4. ⏳ TODO: Initialize artifacts FROM worktree
5. ⚪ OPTIONAL: Update CLAUDE.md with task context
6. ⏳ TODO: Create daily worklog for 2025-10-31
7. ⏳ TODO: Read specifications (PRD, Issue, Task)
8. ⏳ TODO: Final verification checklist
9. ✅ READY TO CODE!

COMMAND SEQUENCE TO EXECUTE:

```bash
# Step 4: Initialize artifacts (CRITICAL - from worktree!)
cd ~/dev-space/CDSAgent-T-02-02-sparse-index
/Users/arthur/dev-space/CDSAgent/scripts/create-task-worklog.sh \
  T-02-02-sparse-index \
  "Sparse Index - Name/ID + BM25 Search" \
  "Rust Dev 2"

# Verify artifacts
ls -la .artifacts/spec-tasks-T-02-02-sparse-index/

# Step 6: Create daily worklog
/Users/arthur/dev-space/CDSAgent/scripts/create-daily-worklog.sh T-02-02-sparse-index

# Verify worklogs
ls -la .artifacts/spec-tasks-T-02-02-sparse-index/worklogs/2025-10-31-*.md

# Step 7: Read specifications
cat spacs/tasks/0.1.0-mvp/02-index-core/T-02-02-sparse-index.md
cat spacs/issues/04-0.1.0-mvp/02-index-core/02-sparse-index.md
cat spacs/prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md | grep -A 50 'FR-HI-3'

# Step 8: Final checks
git branch --show-current  # Should: feat/task/T-02-02-sparse-index
git status                  # Should: clean working tree
cargo check --package cds-index

# Step 9: Open in IDE and start coding!
code .
```

FIRST CODING SESSION PLAN (Day 1):

1. Fill out "Today's Objectives" in work-summary.md:
   - [ ] Understand Graph API (read crates/cds-index/src/graph/mod.rs)
   - [ ] Design NameIndex struct (exact + prefix matching)
   - [ ] Implement NameIndex::new() and insert()
   - [ ] Write first unit tests

2. Start with Upper Index (name_index.rs):
   - Create module: crates/cds-index/src/index/name_index.rs
   - Define NameIndex struct with DashMap and sorted keys
   - Implement basic insertion and exact lookup
   - Write 3-5 unit tests

3. Commit frequently:
   - "feat(index): T-02-02 - add NameIndex struct skeleton"
   - "feat(index): implement exact match lookup"
   - "test(index): add NameIndex unit tests"

4. End-of-day checkpoint:
   - Update work-summary.md with completed objectives
   - Update commit-log.md with git commit details
   - Update metadata.yaml with today's commits
   - Follow: docs/WORK_SESSION_CHECKPOINT_WORKFLOW.md

ESTIMATED COMPLETION:
  Start: 2025-10-31 (today)
  Target: 2025-11-04 to 2025-11-06 (4-6 days realistic)
  Milestone: M2 target 2025-11-09 (3-5 days buffer)

SUCCESS CRITERIA REMINDER:
  ✅ Upper index with exact/prefix matching
  ✅ Lower index BM25 (k1=1.5, b=0.75)
  ✅ Search latency <500ms p95
  ✅ Index build <5s for 1K files
  ✅ Search overlap@10 ≥90% on 50 queries
  ✅ Unit test coverage >95%

═══════════════════════════════════════════════════════════════════════════════
END OF INITIALIZATION PLAN
═══════════════════════════════════════════════════════════════════════════════
]]

return plan
