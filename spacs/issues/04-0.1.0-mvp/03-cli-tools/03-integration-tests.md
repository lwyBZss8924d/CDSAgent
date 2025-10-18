# Sub-Issue 03.03: Integration Tests - End-to-End CLI Validation

**Priority**: P1 (Critical Path - Validation)
**Status**: ☐ Not Started
**Owner**: Rust Dev 3
**Parent**: [03-cli-tools/00-overview.md](00-overview.md)
**PRD Reference**: [PRD-03 §7](../../../prd/0.1.0-MVP-PRDs-v0/03-cds-tools-cli.md), [PRD-08 §2.2](../../../prd/0.1.0-MVP-PRDs-v0/08-testing-quality.md)
**Timing**: Phase 2, Week 5

---

## Objective

Implement end-to-end integration tests that validate CLI commands work correctly with CDS-Index Service, support Unix pipeline integration, and can be invoked by the TypeScript agent.

## Scope

**In Scope**:

- Command integration tests (search → traverse → retrieve workflows)
- Unix pipeline tests (cds | jq | xargs | rg)
- Config and env var precedence tests
- Exit code validation
- Agent workflow tests (bash tool invocation)
- Error scenario handling

**Out of Scope (v0.2.0)**:

- Performance regression tests
- Stress testing (concurrent CLI invocations)
- Cross-platform compatibility tests

---

## Dependencies

- **Requires**: [01-command-impl.md](01-command-impl.md), [02-output-format.md](02-output-format.md), [../02-index-core/03-service-layer.md](../02-index-core/03-service-layer.md)
- **Feeds Into**: [../08-testing/02-integration.md](../08-testing/02-integration.md)

---

## Implementation Tasks

### Week 5, Day 1-2: Command Integration Tests

Task 1: Search Command Tests

```rust
// cds-cli/tests/integration/search_test.rs
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_search_basic() {
    let mut cmd = Command::cargo_bin("cds").unwrap();
    cmd.arg("search")
        .arg("sanitize")
        .arg("--format")
        .arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"query\":\"sanitize\""))
        .stdout(predicate::str::contains("\"results\""));
}

#[test]
fn test_search_with_type_filter() {
    let mut cmd = Command::cargo_bin("cds").unwrap();
    cmd.arg("search")
        .arg("authenticate")
        .arg("--type")
        .arg("function")
        .arg("--limit")
        .arg("5");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"type\":\"function\""));
}

#[test]
fn test_search_no_results_exit_code() {
    let mut cmd = Command::cargo_bin("cds").unwrap();
    cmd.arg("search")
        .arg("nonexistent_function_xyz123");

    cmd.assert()
        .code(1) // Exit code 1 for no results
        .stdout(predicate::str::contains("\"total_results\":0"));
}

#[test]
fn test_search_text_format() {
    let mut cmd = Command::cargo_bin("cds").unwrap();
    cmd.arg("search")
        .arg("sanitize")
        .arg("--format")
        .arg("text");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Found"))
        .stdout(predicate::str::contains("results for"));
}
```

**Acceptance**:

- [ ] Search command returns valid JSON
- [ ] Type filtering works
- [ ] Exit code 1 on no results
- [ ] Text format is human-readable

---

Task 2: Traverse Command Tests

```rust
// cds-cli/tests/integration/traverse_test.rs
#[test]
fn test_traverse_basic() {
    let mut cmd = Command::cargo_bin("cds").unwrap();
    cmd.arg("traverse")
        .arg("entity_abc123")
        .arg("--depth")
        .arg("2")
        .arg("--format")
        .arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"start_entities\""))
        .stdout(predicate::str::contains("\"subgraph\""));
}

#[test]
fn test_traverse_tree_format() {
    let mut cmd = Command::cargo_bin("cds").unwrap();
    cmd.arg("traverse")
        .arg("entity_root")
        .arg("--format")
        .arg("tree");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("├─"))
        .stdout(predicate::str::contains("→"));
}

#[test]
fn test_traverse_relation_filter() {
    let mut cmd = Command::cargo_bin("cds").unwrap();
    cmd.arg("traverse")
        .arg("entity_abc")
        .arg("--relations")
        .arg("invoke,contain");

    cmd.assert()
        .success();
}

#[test]
fn test_traverse_entity_not_found() {
    let mut cmd = Command::cargo_bin("cds").unwrap();
    cmd.arg("traverse")
        .arg("nonexistent_entity");

    cmd.assert()
        .code(2) // Invalid args / entity not found
        .stderr(predicate::str::contains("not found"));
}
```

**Acceptance**:

- [ ] Traverse returns subgraph JSON
- [ ] Tree format uses LocAgent characters (├─, └─)
- [ ] Relation filtering works
- [ ] Entity not found returns error

---

Task 3: Retrieve Command Tests

```rust
// cds-cli/tests/integration/retrieve_test.rs
#[test]
fn test_retrieve_single_entity() {
    let mut cmd = Command::cargo_bin("cds").unwrap();
    cmd.arg("retrieve")
        .arg("entity_abc123")
        .arg("--format")
        .arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"entity_id\":\"entity_abc123\""))
        .stdout(predicate::str::contains("\"code\""));
}

#[test]
fn test_retrieve_multiple_entities() {
    let mut cmd = Command::cargo_bin("cds").unwrap();
    cmd.arg("retrieve")
        .arg("entity_1")
        .arg("entity_2")
        .arg("entity_3");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"entities\""));
}

#[test]
fn test_retrieve_code_format() {
    let mut cmd = Command::cargo_bin("cds").unwrap();
    cmd.arg("retrieve")
        .arg("entity_abc")
        .arg("--format")
        .arg("code");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("def ")); // Python code
}
```

**Acceptance**:

- [ ] Retrieve returns full entity code
- [ ] Multiple entities supported
- [ ] Code format shows raw code

---

### Week 5, Day 3: Pipeline Integration Tests

Task 4: Unix Pipeline Tests

```rust
// cds-cli/tests/integration/pipeline_test.rs
use std::process::{Command, Stdio};

#[test]
fn test_pipeline_search_to_jq() {
    // cds search | jq '.results[].entity_id'
    let search = Command::new("cds")
        .arg("search")
        .arg("sanitize")
        .arg("--format")
        .arg("json")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let jq = Command::new("jq")
        .arg(".results[].entity_id")
        .stdin(search.stdout.unwrap())
        .output()
        .unwrap();

    assert!(jq.status.success());
    assert!(String::from_utf8_lossy(&jq.stdout).contains("entity_"));
}

#[test]
fn test_pipeline_search_to_retrieve() {
    // cds search | jq -r '.results[].entity_id' | xargs cds retrieve
    let search = Command::new("cds")
        .arg("search")
        .arg("authenticate")
        .arg("--limit")
        .arg("1")
        .arg("--format")
        .arg("json")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let jq = Command::new("jq")
        .arg("-r")
        .arg(".results[0].entity_id")
        .stdin(search.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let retrieve = Command::new("cds")
        .arg("retrieve")
        .stdin(jq.stdout.unwrap())
        .output()
        .unwrap();

    assert!(retrieve.status.success());
}

#[test]
fn test_pipeline_with_rg() {
    // cds search | jq -r '.results[].file_path' | xargs rg "TODO"
    let search = Command::new("cds")
        .arg("search")
        .arg("utils")
        .arg("--format")
        .arg("json")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let jq = Command::new("jq")
        .arg("-r")
        .arg(".results[].file_path")
        .stdin(search.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let rg = Command::new("rg")
        .arg("TODO")
        .stdin(jq.stdout.unwrap())
        .output()
        .unwrap();

    // Success or no results (exit code 1) both OK
    assert!(rg.status.code().unwrap() <= 1);
}
```

**Acceptance**:

- [ ] JSON output is valid for piping to jq
- [ ] Can read entity IDs from stdin
- [ ] Works with standard Unix tools (jq, xargs, rg)
- [ ] Pipeline errors propagate correctly

---

### Week 5, Day 4: Configuration & Error Tests

Task 5: Config and Env Var Tests

```rust
// cds-cli/tests/integration/config_test.rs
use std::env;

#[test]
fn test_env_var_precedence() {
    env::set_var("GRAPH_INDEX_DIR", "/tmp/custom_index");

    let mut cmd = Command::cargo_bin("cds").unwrap();
    cmd.arg("config")
        .arg("get")
        .arg("index.graph_dir");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("/tmp/custom_index"));

    env::remove_var("GRAPH_INDEX_DIR");
}

#[test]
fn test_config_set_and_get() {
    let mut set_cmd = Command::cargo_bin("cds").unwrap();
    set_cmd.arg("config")
        .arg("set")
        .arg("search.default_limit")
        .arg("20");
    set_cmd.assert().success();

    let mut get_cmd = Command::cargo_bin("cds").unwrap();
    get_cmd.arg("config")
        .arg("get")
        .arg("search.default_limit");
    get_cmd.assert()
        .success()
        .stdout(predicate::str::contains("20"));
}

#[test]
fn test_config_list() {
    let mut cmd = Command::cargo_bin("cds").unwrap();
    cmd.arg("config")
        .arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("index.graph_dir"))
        .stdout(predicate::str::contains("search.default_limit"));
}
```

**Acceptance**:

- [ ] Env vars override config file
- [ ] Config get/set/list work
- [ ] LocAgent-compatible env vars supported

---

Task 6: Error Scenario Tests

```rust
// cds-cli/tests/integration/error_test.rs
#[test]
fn test_invalid_command() {
    let mut cmd = Command::cargo_bin("cds").unwrap();
    cmd.arg("invalid_command");

    cmd.assert()
        .code(2) // Invalid args
        .stderr(predicate::str::contains("error"));
}

#[test]
fn test_invalid_format() {
    let mut cmd = Command::cargo_bin("cds").unwrap();
    cmd.arg("search")
        .arg("query")
        .arg("--format")
        .arg("invalid_format");

    cmd.assert()
        .code(2)
        .stderr(predicate::str::contains("Invalid format"));
}

#[test]
fn test_service_not_running() {
    // Simulate service down by using wrong port
    env::set_var("CDS_SERVICE_URL", "http://localhost:9999");

    let mut cmd = Command::cargo_bin("cds").unwrap();
    cmd.arg("search")
        .arg("query");

    cmd.assert()
        .code(4) // Service error
        .stderr(predicate::str::contains("service"))
        .stderr(predicate::str::contains("error"));

    env::remove_var("CDS_SERVICE_URL");
}

#[test]
fn test_index_not_found() {
    env::set_var("GRAPH_INDEX_DIR", "/nonexistent/path");

    let mut cmd = Command::cargo_bin("cds").unwrap();
    cmd.arg("search")
        .arg("query");

    cmd.assert()
        .code(3) // Index error
        .stderr(predicate::str::contains("Index"))
        .stderr(predicate::str::contains("not found"));

    env::remove_var("GRAPH_INDEX_DIR");
}
```

**Acceptance**:

- [ ] Exit codes follow standard conventions (0, 1, 2, 3, 4, 5)
- [ ] Errors printed to stderr
- [ ] Error messages are actionable
- [ ] Hints provided for common errors

---

### Week 5, Day 5: Agent Workflow Tests

Task 7: Bash Tool Invocation Tests

```typescript
// tests/agent_integration/bash_tool_test.ts (to be run from agent layer)
import { describe, it, expect } from 'bun:test';
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

describe('CLI Bash Tool Integration', () => {
  it('should execute cds search from bash tool', async () => {
    const { stdout, stderr } = await execAsync('cds search "sanitize" --format json');

    expect(stderr).toBe('');
    const result = JSON.parse(stdout);
    expect(result.query).toBe('sanitize');
    expect(result.results).toBeArray();
  });

  it('should handle pipeline from agent', async () => {
    const cmd = 'cds search "auth" --format json | jq -r ".results[0].entity_id"';
    const { stdout } = await execAsync(cmd);

    expect(stdout).toMatch(/entity_/);
  });

  it('should handle errors gracefully', async () => {
    try {
      await execAsync('cds search "nonexistent_query_xyz123"');
    } catch (error: any) {
      expect(error.code).toBe(1); // No results exit code
    }
  });

  it('should parse JSON output in agent code', async () => {
    const { stdout } = await execAsync('cds search "utils" --limit 3 --format json');
    const result = JSON.parse(stdout);

    expect(result.total_results).toBeLessThanOrEqual(3);
    result.results.forEach((entity: any) => {
      expect(entity).toHaveProperty('entity_id');
      expect(entity).toHaveProperty('name');
      expect(entity).toHaveProperty('snippet');
    });
  });
});
```

**Acceptance**:

- [ ] CLI can be invoked from TypeScript agent via bash tool
- [ ] Agent can parse JSON output
- [ ] Exit codes detectable from agent
- [ ] Pipeline workflows work from agent

---

## Acceptance Criteria (from PRD-03 §7, PRD-08 §2.2)

### Must-Pass

- [ ] All command integration tests pass
- [ ] Unix pipeline tests pass (cds | jq | xargs)
- [ ] Config and env var tests pass
- [ ] Error scenario tests pass
- [ ] Agent workflow tests pass
- [ ] Exit codes validated: 0 (success), 1 (no results), 2 (invalid args), 3 (index error), 4 (service error), 5 (IO error)
- [ ] Integration test coverage >80% of CLI scenarios

### CI Integration

- [ ] Tests run on every PR
- [ ] Tests run against running CDS-Index Service
- [ ] Test fixtures maintained

---

## Testing Strategy

### Test Environment Setup

```bash
# tests/integration/setup.sh

#!/bin/bash
set -e

echo "Setting up integration test environment..."

# 1. Start CDS-Index Service (background)
cds-indexd --port 9001 &
SERVICE_PID=$!

# 2. Wait for service to be ready
while ! curl -s http://localhost:9001/health > /dev/null; do
    sleep 1
done

echo "CDS-Index Service running (PID: $SERVICE_PID)"

# 3. Initialize test index
cds init tests/fixtures/sample_repo --output /tmp/test_index

# 4. Set env vars
export GRAPH_INDEX_DIR=/tmp/test_index
export CDS_SERVICE_URL=http://localhost:9001

echo "Test environment ready!"
```

### Test Teardown

```bash
# tests/integration/teardown.sh

#!/bin/bash

echo "Cleaning up test environment..."

# Stop service
pkill -f cds-indexd || true

# Clean up test index
rm -rf /tmp/test_index

echo "Cleanup complete!"
```

---

## Open Questions & Risks

### Q1: Flaky Tests

**Risk**: Integration tests may be flaky if service not fully started
**Mitigation**: Add health check polling with timeout
**Escalation**: If flakiness persists, investigate timing issues

### Q2: Test Fixtures

**Question**: Use real repos or synthetic fixtures?
**Decision**: Use small synthetic repo (Week 5, Day 1) + LocAgent repo for parity
**Rationale**: Synthetic repo ensures fast tests, LocAgent repo validates real-world behavior

### Q3: Cross-Platform Testing

**Risk**: Tests may fail on Windows due to path separators
**Mitigation**: Use `std::path::PathBuf` in tests, normalize paths
**Escalation**: If Windows compatibility needed, add CI matrix

---

## Related Issues

- **Parent**: [00-overview.md](00-overview.md)
- **Depends On**: [01-command-impl.md](01-command-impl.md), [02-output-format.md](02-output-format.md)
- **Feeds Into**: [../08-testing/02-integration.md](../08-testing/02-integration.md)
- **Coordinates With**: [../04-agent-integration/](../04-agent-integration/)

---

## Next Steps

1. [ ] Set up integration test environment (Week 5, Day 1)
2. [ ] Write command integration tests (Week 5, Day 1-2)
3. [ ] Write pipeline integration tests (Week 5, Day 3)
4. [ ] Write config and error tests (Week 5, Day 4)
5. [ ] Write agent workflow tests (Week 5, Day 5)
6. [ ] Integrate tests into CI pipeline

---

**Status Updates**:

- *2025-10-19*: Sub-issue created, awaiting command implementation completion
