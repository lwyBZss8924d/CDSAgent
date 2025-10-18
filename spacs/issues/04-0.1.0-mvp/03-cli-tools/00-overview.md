# Issue-03: CDS-Tools CLI - Unified Code Retrieval Interface

**Priority**: P1 (Critical Path - CLI Layer)
**Status**: ☐ Not Started
**Owner**: Rust Dev 2 + Rust Dev 3
**PRD Reference**: [PRD-03: CDS-Tools CLI](../../../prd/0.1.0-MVP-PRDs-v0/03-cds-tools-cli.md)

---

## Overview

CDS-Tools provides a unified command-line interface (CLI) that exposes LocAgent's code retrieval capabilities as composable Unix-style tools. The `cds` binary wraps the CDS-Index Service and enables both human developers and LLM agents to perform sophisticated code searches through simple, pipeable commands.

## Objective

Deliver a high-performance Rust-based CLI that:

- Implements 4 core commands: `search`, `traverse`, `retrieve`, `combo`
- Provides multiple output formats (JSON, text, tree) with LocAgent parity
- Supports Unix pipeline integration (stdin/stdout, exit codes)
- Maintains intuitive developer experience (help text, examples)
- Achieves <100ms startup time (vs Python's ~300ms)

## Dependencies

- **Requires**: CDS-Index Service ([02-index-core/](../02-index-core/)) must be functional
- **Blocks**: [04-agent-integration/](../04-agent-integration/) - Agent uses CLI via bash tool
- **Timing**: Phase 2 (Weeks 3-5)

---

## Sub-Issues Breakdown

### 1. [Command Implementation](01-command-impl.md) - **P1, Week 3-4**

**Owner**: Rust Dev 2
**Scope**: Core command logic for search, traverse, retrieve, combo

- `cds search`: Entity keyword search (hierarchical indexing)
- `cds traverse`: Graph BFS navigation with type/relation filters
- `cds retrieve`: Full entity code and metadata fetching
- `cds combo`: Hybrid retrieval pipelines (YAML plans)
- `cds init`: Index initialization (calls CDS-Index Service)
- `cds config`: Configuration management

**Acceptance**:

- [ ] All commands accept correct arguments (clap validation)
- [ ] Commands call CDS-Index Service JSON-RPC APIs
- [ ] Error handling with actionable messages
- [ ] Unit tests >85% coverage

---

### 2. [Output Format](02-output-format.md) - **P1, Week 4**

**Owner**: Rust Dev 2 (parallel with commands)
**Scope**: JSON/text/tree formatting with LocAgent parity

- JSON output schema (agent-compatible)
- Text output (human-readable)
- Tree output (traverse command, LocAgent format)
- Snippet formatting (fold/preview/full)

**Acceptance**:

- [ ] JSON output matches LocAgent SearchEntity structure
- [ ] Tree format character-for-character identical to LocAgent
- [ ] Text output is readable and well-formatted
- [ ] Output validation tests pass (schema compliance)

---

### 3. [Integration Tests](03-integration-tests.md) - **P1, Week 5**

**Owner**: Rust Dev 3
**Scope**: End-to-end CLI testing, pipeline integration

- Command integration tests (search → traverse → retrieve)
- Unix pipeline tests (cds | jq | xargs)
- Config and env var precedence
- Exit code validation
- Agent workflow tests (bash tool invocation)

**Acceptance**:

- [ ] All integration test scenarios pass
- [ ] Commands work with standard Unix tools (jq, xargs, rg)
- [ ] Agent can invoke CLI and parse results
- [ ] Error scenarios handled gracefully

---

### 4. [Documentation](04-docs.md) - **P1, Week 5**

**Owner**: Rust Dev 3
**Scope**: Help text, examples, user guide

- Command help text (clap annotations)
- Usage examples per command
- Configuration guide
- Hybrid retrieval cookbook
- Troubleshooting guide

**Acceptance**:

- [ ] `cds --help` and subcommand help complete
- [ ] Examples included in help output
- [ ] User guide documents common workflows
- [ ] Hybrid retrieval examples work end-to-end

---

## Rust Crate Structure

```tree
cds-cli/
├── src/
│   ├── main.rs              # Entry point, clap CLI setup
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── search.rs        # cds search implementation
│   │   ├── traverse.rs      # cds traverse
│   │   ├── retrieve.rs      # cds retrieve
│   │   ├── combo.rs         # cds combo (YAML plan execution)
│   │   ├── config.rs        # cds config (get/set/list)
│   │   └── init.rs          # cds init (index building)
│   ├── output/
│   │   ├── formatter.rs     # JSON/text formatting
│   │   └── tree.rs          # Tree-structured output (traverse)
│   ├── config.rs            # Config file parsing (TOML)
│   ├── client.rs            # CDS-Index Service JSON-RPC client
│   └── error.rs             # Error types and exit codes
├── Cargo.toml
└── README.md
```

**Key Dependencies**:

| Crate | Purpose |
|-------|---------|
| `clap` (v4) | CLI argument parsing with derive macros |
| `serde_json` | JSON output formatting and parsing |
| `serde_yaml` | Combo plan file parsing |
| `reqwest` | HTTP client for JSON-RPC service |
| `tokio` | Async runtime |
| `colored` | Colored text output |
| `indicatif` | Progress bars (`cds init`) |

---

## Acceptance Criteria Summary (from PRD-03 §8)

### Must-Have (v0.1.0 MVP)

- [ ] `cds search`, `cds traverse`, `cds retrieve` commands functional
- [ ] JSON and text output formats
- [ ] Unix pipeline support (stdin/stdout, proper exit codes)
- [ ] Config file and env var support (LocAgent compatibility)
- [ ] Help text and usage examples
- [ ] Performance: <100ms CLI startup, <500ms search command

### Should-Have (v0.2.0)

- [ ] `cds combo` for hybrid workflows (YAML plan execution)
- [ ] `cds init` for index initialization
- [ ] Progress indicators and colored output
- [ ] Shell completions (bash, zsh, fish)

---

## Performance Targets (from PRD-03 §3.1)

| Metric | Target | Validation Method |
|--------|--------|-------------------|
| CLI startup time | <100ms | `hyperfine "cds --version"` |
| Search command | <500ms | `hyperfine "cds search query"` |
| Traverse command (2-hop) | <1s | Integration test timing |
| JSON parse overhead | <50ms | Unit test benchmark |

---

## Dependencies & Coordination

### Internal Dependencies

- Command Impl (01) must complete before Integration Tests (03)
- Output Format (02) runs in parallel with Command Impl
- Documentation (04) references all completed commands

### External Coordination

- **PRD-02 (Index Service)**: CLI calls JSON-RPC APIs, must match schemas
- **PRD-05 (API Contracts)**: CLI client uses contract-defined endpoints
- **PRD-04 (Agent Integration)**: Agent invokes CLI via bash tool
- **06-refactor-parity.md**: Output format must match LocAgent exactly

---

## Implementation Phases

### Phase 2, Week 3: Core Commands

- [ ] Sub-issue 01: Search, Traverse, Retrieve implementation
- [ ] Sub-issue 02: JSON/text output formatting
- [ ] Milestone: `cds search` and `cds traverse` functional

### Phase 2, Week 4-5: Integration & Polish

- [ ] Sub-issue 02: Tree output (LocAgent parity)
- [ ] Sub-issue 03: Integration tests and pipeline validation
- [ ] Sub-issue 04: Help text and documentation
- [ ] Milestone: CLI passes all integration tests, agent can invoke

---

## Testing Strategy

### Unit Tests (>85% coverage per module)

- See [../08-testing/01-unit.md](../08-testing/01-unit.md)
- Command argument parsing
- Output formatting (JSON schema validation)
- Error handling and exit codes

### Integration Tests

- See [../08-testing/02-integration.md](../08-testing/02-integration.md)
- End-to-end: Index repo → search → traverse → retrieve
- Pipeline: `cds search | jq | cds retrieve`
- Agent workflow: Bash tool invocation from TypeScript

### Output Parity Tests

- Compare `cds search` JSON with LocAgent SearchEntity output
- Compare `cds traverse` tree format with LocAgent character-by-character
- Validate on 10 sample queries

---

## Open Questions & Risks

### 1. Direct Linking vs JSON-RPC Client

**Question**: Should `cds` binary link directly to CDS-Index crates or call via JSON-RPC?
**Status**: Use JSON-RPC client (Week 3 decision)
**Rationale**: Modularity, allows remote index service in v0.2.0, matches LocAgent architecture
**Trade-off**: +50ms RPC overhead vs direct linking, acceptable for v0.1.0

### 2. Combo Command Scope

**Question**: Implement full YAML plan executor in v0.1.0 or defer to v0.2.0?
**Decision**: Mark developer-only in v0.1.0 (per PRD-03 FR-CMD-4), full agent integration in v0.2.0
**Rationale**: Reduces v0.1.0 scope, agent uses bash tool for simpler workflows first

### 3. Output Schema Stability

**Risk**: JSON schema changes break agent integration
**Mitigation**: Define JSON schema in [05-api-contracts.md](../05-api-contracts.md), validate with tests
**Escalation**: If schema must change, version APIs (e.g., `--format json-v2`)

---

## Related Issues

- **Sub-Issues**: [01-command-impl.md](01-command-impl.md), [02-output-format.md](02-output-format.md), [03-integration-tests.md](03-integration-tests.md), [04-docs.md](04-docs.md)
- **Depends On**: [02-index-core/](../02-index-core/) - CDS-Index Service must be running
- **Blocks**: [04-agent-integration/](../04-agent-integration/) - Agent uses CLI via bash
- **Coordinates With**: [05-api-contracts.md](../05-api-contracts.md) - JSON-RPC schemas
- **Validates**: [06-refactor-parity.md](../06-refactor-parity.md) - Output format parity
- **Tests**: [08-testing/02-integration.md](../08-testing/02-integration.md)

---

## Next Steps

1. [ ] Review PRD-03 and PRD-05 for final schema alignment
2. [ ] Set up cds-cli crate with clap structure (Week 3, Day 1)
3. [ ] Implement JSON-RPC client for CDS-Index Service (Week 3, Day 1-2)
4. [ ] Begin Sub-issue 01: Command implementation (Week 3, Day 3)
5. [ ] Begin Sub-issue 02: Output formatting (Week 3, parallel)
6. [ ] Establish weekly sync for CLI/Service integration

---

**Status Updates**:

- *2025-10-19*: Issue created, sub-issues defined, awaiting CDS-Index Service completion
