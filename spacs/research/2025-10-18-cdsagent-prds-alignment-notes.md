# spacs/research/2025-10-18-cdsagent-prds-alignment-notes

## Baseline Requirement Traceability Inputs

- CDSAgent must refactor LocAgent graph indexing flow to use Rust for core graph/index mechanics while exposing TypeScript integration via Claude Agent SDK (`@anthropic-ai/claude-agent-sdk`), with optional future swap to other SDKs (Issue: `spacs/issues/01-CDSAgent-MVP-definition.md`, Research summary `spacs/research/2025-10-18-cdsagent-requirement-analysis.md`).
- Provide CLI-accessible retrieval tools mirroring LocAgent tools: SearchEntity, TraverseGraph, RetrieveEntity, with commands accessible via `cds -s/-t/-r` and support for hybrid pipelines combining `rg`, `ast-grep`, etc. (Issue-01 requirement text, plus LocAgent tool definitions `tmp/LocAgent/arXiv-2503.09089v2/3_method.tex:235-256`).
- Claude Code subagent (`code-retrievaler`) must drive retrieval using bash toolcalls, with hooks (PreToolUse, PostToolUse, SubagentStop) to manage context & concurrency (Issue-01 requirement text, Research analysis `spacs/research/2025-10-18-cdsagent-requirement-analysis.md`).
- LocAgent graph representation: heterogeneous nodes (directory, file, class, function) and relations (contain, import, invoke, inherit); hierarchical entity index with multi-level outputs (fold/preview/full). Basis for parity checks (`tmp/LocAgent/arXiv-2503.09089v2/3_method.tex:205-256`, `tmp/LocAgent/arXiv-2503.09089v2/data/tab_api_lists.tex:9-19`, `tmp/LocAgent/arXiv-2503.09089v2/6_1_appendix_details.tex:22-32`).
- Performance rationale: LocAgent ablations show removing SearchEntity/TraverseGraph/RetrieveEntity harms accuracy, motivating inclusion (`tmp/LocAgent/arXiv-2503.09089v2/data/ablation.tex:16-29`, `tmp/LocAgent/arXiv-2503.09089v2/5_experiment.tex:88-95`).
- Index outputs must be structured/persisted for reuse (implied by LocAgent `tmp/LocAgent/dependency_graph/` and `tmp/LocAgent/repo_index/` modules).
- Deployment should allow codebase indexing via dedicated service/daemon plus CLI integration (Issue-01 requirement text, Research analysis).

## PRD Coverage Assessment (WS1 & WS2)

- **PRD-01 System Architecture (`spacs/prd/0.1.0-MVP-PRDs-v0/01-system-architecture.md`)**
  - Captures three-tier design (Index, Tools, Agent) consistent with requirement baseline and LocAgent workflow (§3, Fig.2). References hooks & bash orchestration per Issue-01 requirements.
  - Notes: Introduces immediate multi-language support (Python+TypeScript+Java) as part of architecture; LocAgent paper emphasizes Python-only scope (`tmp/LocAgent/arXiv-2503.09089v2/3_method.tex:211-220`). Flag as staged roadmap rather than v0 guarantee.

- **PRD-02 CDS-Index Service (`spacs/prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md`)**
  - Detailed mapping of graph construction and hierarchical search to LocAgent modules (`dependency_graph/build_graph.py`, `repo_index/`). Tool semantics align with SearchEntity description (`3_method.tex:235-256`).
  - Adds incremental update & multi-language milestones; need validation plan since LocAgent repo lacks TypeScript parser (supports Python + partial Java). Document includes acceptance criteria to compare with LocAgent outputs—good parity coverage.

- **PRD-03 CDS-Tools CLI (`spacs/prd/0.1.0-MVP-PRDs-v0/03-cds-tools-cli.md`)**
  - Command definitions mirror LocAgent tools (fold/preview/full, tree outputs referenced to `6_1_appendix_details.tex:13-32`). Hybrid pipeline guidance satisfies Issue-01 hybrid requirement (rg/ast-grep). Additional features (DOT export, `--no-bm25`, `combo`) are extensions; ensure downstream docs expect them.

- **PRD-04 CDS-Agent Integration (`spacs/prd/0.1.0-MVP-PRDs-v0/04-cds-agent-integration.md`)**
  - System prompt & CoT steps align with LocAgent agent loop (`3_method.tex:333-351`). Configures Claude hooks per Issue-01. Multi-SDK abstraction + subagent design matches requirement for future SDK swapping.
  - Verify hook permission defaults with Claude SDK docs (`tmp/claude-agent-sdk/guides-permissions.md`) during implementation.

- **PRD-05 API Specifications (`spacs/prd/0.1.0-MVP-PRDs-v0/05-api-specifications.md`)**
  - Defines JSON-RPC schema matching CLI outputs (fold/preview/full fields). Consistent with other PRDs assuming JSON default. Decision log on JSON-RPC vs gRPC notes upgrade path; dovetails with PRD-02 service section.

- **PRD-06 Rust Refactoring Plan (`spacs/prd/0.1.0-MVP-PRDs-v0/06-rust-refactoring-plan.md`)**
  - One-to-one mapping of Python modules to Rust crates with code snippets referencing LocAgent repo paths. Emphasizes fidelity (node/edge parity, tree-sitter queries). Highlights need for verification harness comparing counts.

- **PRD-07 Deployment & Operations (`spacs/prd/0.1.0-MVP-PRDs-v0/07-deployment-operations.md`)**
  - Addresses env vars `GRAPH_INDEX_DIR`, daemon model consistent with LocAgent scripts (`tmp/LocAgent/scripts/gen_graph_index.sh`). Adds containerization guidance; ensure index path immutability across nodes.

- **PRD-08 Testing & Quality (`spacs/prd/0.1.0-MVP-PRDs-v0/08-testing-quality.md`)**
  - Test strategy references LocAgent benchmarks (SWE-bench Lite). Includes regression expectations vs LocAgent Table 2. Suggests coverage targets for Rust/TS modules.

- **PRD-09 Implementation Roadmap (`spacs/prd/0.1.0-MVP-PRDs-v0/09-implementation-roadmap.md`)**
  - Phases align with requirement analysis roadmap. Dependencies between tasks consistent with PRD content (e.g., Phase 2 delivering JSON-RPC needed by PRD-05).

- **PRD-10 Extensibility & Future (`spacs/prd/0.1.0-MVP-PRDs-v0/10-extensibility-future.md`)**
  - Captures SDK abstraction, multi-language expansion, semantic search, aligning with Issue-01 requirement to keep SDK pluggable. Clear separation of future versions (v1.1+). Ensures backlog for features flagged as non-MVP.

## Cross-PRD Consistency Review (WS3)

- Transport protocol: PRD-05 selects JSON-RPC for v1.0 while PRD-02 lists gRPC modules (`cds_service/grpc_server.rs`). Recommend clarifying in PRD-02 that gRPC module is v1.1+ and JSON-RPC server is default to avoid conflicting expectations.
- CLI semantics: PRD-03 defines single `cds` binary; PRD-04’s `allowedTools` array (`cds_search`, `cds_traverse`, `cds_retrieve`) assumes SDK-level wrappers. Need explicit note (in PRD-04 or PRD-03) that MCP tool wrappers call the CLI or JSON-RPC endpoints to maintain consistency.
- Multi-language roadmap: PRD-02/06 claim TypeScript/Java support in later versions (v1.1/v1.2) and PRD-09 schedules Polyglot work in Phase 4; ensure PRD-01 summary highlights staged rollout to prevent misinterpretation that v1.0 ships multi-language.
- `cds combo`: Architecture (PRD-01), CLI (PRD-03), and roadmap (PRD-09) reference the command; Agent integration (PRD-04) does not yet describe how agents invoke combos. Consider adding usage guidance in PRD-04 or deferring combo usage to developer workflows only.
- Index daemon assumptions: PRD-07 defines default port 9876 and env var `GRAPH_INDEX_DIR`; verify PRD-05 API endpoints and PRD-04 hook configuration reference the same defaults to avoid drift.

## Gaps, Risks, and Follow-up Tasks (WS4)

- **GR-1: Multi-language scope messaging** – PRD-01 implied multi-language parsing at MVP while PRD-02/06 scheduled it later. **Status**: PRD-01 (Section 4.2) now codifies roadmap (v0.2.0, v0.3.0); PRD-06 goal statement updated accordingly.
- **GR-2: Service interface clarity** – PRD-02 scaffolds a gRPC server despite PRD-05 choosing JSON-RPC for v1.0. **Status**: PRD-02 crate structure + service strategy now mark JSON-RPC as v0.1.0 deliverable and gRPC as v0.2.0+.
- **GR-3: MCP tool naming vs CLI** – PRD-04’s `allowedTools` names (`cds_search`, etc.) did not exist natively. **Status**: PRD-04 now keeps `bash` only for v0.1.0 and documents MCP wrappers as v0.2.0 roadmap.
- **GR-4: Agent coverage of `cds combo`** – No guidance on agent-side usage of combo pipelines. **Status**: PRD-03 marks combo as developer-only in v0.1.0 and PRD-04 instructs agents to avoid it until v0.2.0.
- **GR-5: Hook configuration defaults** – Needed reference to deployment defaults. **Status**: PRD-04 PreToolUse hook now injects `GRAPH_INDEX_DIR` and `CDS_INDEX_URL` (default `http://localhost:9876`) aligned with PRD-07.

## Deliverable Packaging & Issue Updates (WS5)

- Alignment notes captured in this research file (`spacs/research/2025-10-18-cdsagent-prds-alignment-notes.md`).
- Pending: update `spacs/issues/03-CDSAgent-PRDs-alignment.md` checkboxes once remediation tasks (GR-1…GR-5) are scheduled or completed.
- Recommend linking this analysis from Issue-02 and PRD authorship doc in next commit.
