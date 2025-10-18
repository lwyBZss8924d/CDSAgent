# Tasks: CDS-Tools CLI - Command-Line Interface

**Work Stream**: Issue-03: CDS-Tools CLI
**Issue Reference**: [../../issues/04-0.1.0-mvp/03-cli-tools/](../../issues/04-0.1.0-mvp/03-cli-tools/)
**PRD Reference**: [PRD-03: CDS-Tools CLI](../../prd/0.1.0-MVP-PRDs-v0/03-cds-tools-cli.md)

## Task Overview

| Task ID | Title | Owner | Status | Week |
|---------|-------|-------|--------|------|
| T-03-01 | Core Commands - search, traverse, retrieve | Rust Dev 2 | ☐ Not Started | W4 |
| T-03-02 | Output Formatting - JSON, text, tree | Rust Dev 2 | ☐ Not Started | W4-5 |
| T-03-03 | Integration Tests - CLI E2E workflows | QA Lead | ☐ Not Started | W5 |
| T-03-04 | Documentation - CLI usage guide | Technical Writer | ☐ Not Started | W5 |

## Dependencies

- **Prerequisite**: T-02-03 (Service Layer) must be functional
- **Enables**: T-04-01 (Agent SDK Bootstrap) - agent uses CLI via bash tool

## Task Details

### T-03-01: Core Commands

**File**: `T-03-01-core-commands.md`
**Issue**: [Sub-Issue 03.01](../../issues/04-0.1.0-mvp/03-cli-tools/01-command-impl.md)
**PRD**: PRD-03 §2.1, §4.1

**Scope**:

- `cds search` - Entity name/keyword search
- `cds traverse` - Dependency graph traversal
- `cds retrieve` - Code content retrieval
- JSON-RPC client for Index Service

**Deliverables**:

- `cds-tools/src/commands/search.rs`
- `cds-tools/src/commands/traverse.rs`
- `cds-tools/src/commands/retrieve.rs`
- `cds-tools/src/client/rpc_client.rs`

---

### T-03-02: Output Formatting

**File**: `T-03-02-output-format.md`
**Issue**: [Sub-Issue 03.02](../../issues/04-0.1.0-mvp/03-cli-tools/02-output-format.md)
**PRD**: PRD-03 §2.2, §4.2

**Scope**:

- JSON format (--format json) for agent consumption
- Text format (--format text) for human readability
- Tree format (--format tree) for dependency visualization
- Parity with LocAgent output format

**Deliverables**:

- `cds-tools/src/formatters/json.rs`
- `cds-tools/src/formatters/text.rs`
- `cds-tools/src/formatters/tree.rs`

---

### T-03-03: Integration Tests

**File**: `T-03-03-integration-tests.md`
**Issue**: [Sub-Issue 03.03](../../issues/04-0.1.0-mvp/03-cli-tools/03-integration-tests.md)
**PRD**: PRD-03 §5

**Scope**:

- CLI invocation tests (bash scripts)
- Output validation tests
- Error handling tests
- Performance tests (latency <500ms)

**Deliverables**:

- `tests/cli_integration_test.sh`
- `tests/integration/cli_tests.rs`

---

### T-03-04: Documentation

**File**: `T-03-04-documentation.md`
**Issue**: [Sub-Issue 03.04](../../issues/04-0.1.0-mvp/03-cli-tools/04-docs.md)
**PRD**: PRD-03 §6

**Scope**:

- CLI usage guide with examples
- Command reference (man pages)
- Troubleshooting guide

**Deliverables**:

- `cds-tools/README.md`
- `docs/CLI.md`

---

## Phase Milestones

### Week 4: Commands Functional

- [ ] T-03-01: All 3 commands (search, traverse, retrieve) work
- [ ] T-03-02: JSON output format implemented

**Validation**: Agent can invoke CLI and parse JSON output

### Week 5: Production Ready

- [ ] T-03-02: Text and tree formats complete
- [ ] T-03-03: Integration tests pass
- [ ] T-03-04: Documentation published

**Validation**: User can follow CLI guide and complete tasks

---

## Quick Links

- [Issue-03 Overview](../../issues/04-0.1.0-mvp/03-cli-tools/00-overview.md)
- [PRD-03: CDS-Tools CLI](../../prd/0.1.0-MVP-PRDs-v0/03-cds-tools-cli.md)
- [Service Layer Tasks](../02-index-core/README.md#t-02-03-service-layer)
