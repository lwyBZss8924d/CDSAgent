# Sub-Issue 08.02: Integration Tests - End-to-End Workflow Validation

**Priority**: P1
**Owner**: QA Lead + TypeScript Dev 1
**Timing**: Phase 3, Week 7
**PRD Reference**: [PRD-08 §2.2, §4.2](../../../prd/0.1.0-MVP-PRDs-v0/08-testing-quality.md)

## Objective

Validate end-to-end agent workflows (Index Service → CLI → Agent) with integration tests covering multi-step code localization, error handling, and Docker deployment.

## Key Implementations

### Agent Workflow Tests (TypeScript)

```typescript
// tests/integration/agent-workflows.test.ts
import { describe, it, expect, beforeAll, afterAll } from 'bun:test';
import { AgentRunner } from '../test-utils/agent-runner';

describe('Agent Code Localization Workflows', () => {
  let agent: AgentRunner;

  beforeAll(async () => {
    // Start Index Service and CLI in Docker Compose
    await exec('docker-compose up -d index-service cli-tools');
    await waitForHealthy('http://localhost:3030/health');

    agent = new AgentRunner({
      indexServiceUrl: 'http://localhost:3030',
      cliPath: '/usr/local/bin/cds',
    });
  });

  afterAll(async () => {
    await exec('docker-compose down');
  });

  it('should locate XSS sanitization functions', async () => {
    const issue = {
      task: 'Find all functions that handle user input sanitization',
      expected_locations: [
        { file: 'utils/sanitize.py', function: 'sanitize_html' },
      ],
    };

    const result = await agent.runTask(issue.task);

    // Verify locations found
    expect(result.locations).toHaveLength(1);
    expect(result.locations[0].file).toContain('sanitize.py');
    expect(result.locations[0].reason).toContain('sanitize_html');

    // Verify reasoning chain
    expect(result.reasoning).toContain('Extract keywords');
    expect(result.reasoning).toContain('Search');
    expect(result.reasoning).toContain('Traverse');
  });

  it('should traverse function call dependencies', async () => {
    const issue = {
      task: 'Find all callers of the authenticate() function',
    };

    const result = await agent.runTask(issue.task);

    // Verify tool calls
    const toolCalls = result.toolCallHistory;
    expect(toolCalls).toContainEqual(
      expect.objectContaining({
        tool: 'bash',
        command: expect.stringContaining('cds search authenticate'),
      })
    );
    expect(toolCalls).toContainEqual(
      expect.objectContaining({
        tool: 'bash',
        command: expect.stringContaining('cds traverse'),
      })
    );

    // Verify results
    expect(result.locations.length).toBeGreaterThan(0);
  });

  it('should handle class inheritance exploration', async () => {
    const issue = {
      task: 'Find all subclasses of BaseException',
    };

    const result = await agent.runTask(issue.task);

    expect(result.locations.length).toBeGreaterThan(0);
    expect(result.locations[0].reason).toMatch(/inherit|subclass/i);
  });

  it('should recover from search errors', async () => {
    const issue = {
      task: 'Find nonexistent_function_xyz123',
    };

    const result = await agent.runTask(issue.task);

    // Should complete without crashing
    expect(result.status).toBe('completed');
    // Should explain no results found
    expect(result.reasoning).toMatch(/not found|no results/i);
  });

  it('should respect max iterations limit', async () => {
    const issue = {
      task: 'Find all functions',  // Overly broad query
    };

    const result = await agent.runTask(issue.task, { maxIterations: 5 });

    expect(result.toolCallHistory.length).toBeLessThanOrEqual(5);
  });
});
```

### Index Service Integration Tests (Rust)

```rust
// tests/integration/service_integration_test.rs
#[cfg(test)]
mod service_integration_tests {
    use super::*;
    use reqwest::Client;
    use serde_json::json;

    #[tokio::test]
    async fn test_search_endpoint() {
        // Assumes service running on localhost:3030
        let client = Client::new();

        let request = json!({
            "jsonrpc": "2.0",
            "method": "search_entities",
            "params": {
                "query": "Calculator",
                "entity_type": "class",
                "limit": 5
            },
            "id": 1
        });

        let response = client
            .post("http://localhost:3030/rpc")
            .json(&request)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 200);

        let body: serde_json::Value = response.json().await.unwrap();
        assert!(body["result"]["results"].as_array().is_some());
    }

    #[tokio::test]
    async fn test_traverse_endpoint() {
        let client = Client::new();

        let request = json!({
            "jsonrpc": "2.0",
            "method": "traverse_dependencies",
            "params": {
                "start_entity_id": "test.py::Calculator::add",
                "direction": "outgoing",
                "edge_types": ["invoke"],
                "max_depth": 2
            },
            "id": 2
        });

        let response = client
            .post("http://localhost:3030/rpc")
            .json(&request)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 200);

        let body: serde_json::Value = response.json().await.unwrap();
        assert!(body["result"]["nodes"].as_array().is_some());
    }

    #[tokio::test]
    async fn test_error_handling() {
        let client = Client::new();

        let request = json!({
            "jsonrpc": "2.0",
            "method": "invalid_method",
            "params": {},
            "id": 3
        });

        let response = client
            .post("http://localhost:3030/rpc")
            .json(&request)
            .send()
            .await
            .unwrap();

        let body: serde_json::Value = response.json().await.unwrap();
        assert!(body["error"].is_object());
        assert_eq!(body["error"]["code"], -32601);  // Method not found
    }
}
```

### CLI Integration Tests

```shell
#!/bin/bash
# tests/integration/cli_integration_test.sh

set -e

echo "Testing CLI integration..."

# Ensure Index Service is running
curl -f http://localhost:3030/health || exit 1

# Test: cds search
echo "Testing cds search..."
result=$(cds search "Calculator" --type class --format json)
echo "$result" | jq -e '.results | length > 0' || exit 1

# Test: cds traverse
echo "Testing cds traverse..."
result=$(cds traverse "test.py::Calculator::add" --direction outgoing --edge-type invoke)
echo "$result" | jq -e '.nodes | length >= 0' || exit 1

# Test: cds retrieve
echo "Testing cds retrieve..."
result=$(cds retrieve "test.py::Calculator::add" --context 5)
echo "$result" | jq -e '.code | length > 0' || exit 1

# Test: Error handling (invalid query)
echo "Testing error handling..."
if cds search "" --limit 10 2>/dev/null; then
  echo "ERROR: Empty query should fail"
  exit 1
fi

echo "All CLI integration tests passed!"
```

### Docker Deployment Tests

```typescript
// tests/integration/docker-deployment.test.ts
import { describe, it, expect } from 'bun:test';
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

describe('Docker Deployment Integration', () => {
  it('should start all services with docker-compose', async () => {
    // Start services
    await execAsync('docker-compose up -d');

    // Wait for health checks
    await new Promise(resolve => setTimeout(resolve, 10000));

    // Verify services running
    const { stdout } = await execAsync('docker-compose ps');
    expect(stdout).toContain('index-service');
    expect(stdout).toContain('cli-tools');
    expect(stdout).toContain('agent');

    // Verify health
    const { stdout: healthOutput } = await execAsync('./scripts/health-check.sh');
    expect(healthOutput).toContain('All health checks passed');
  });

  it('should persist volumes across restarts', async () => {
    // Build index
    await execAsync('docker-compose exec -T index-service cds-index-build --repo-path /data/test_repo');

    // Restart services
    await execAsync('docker-compose restart index-service');
    await new Promise(resolve => setTimeout(resolve, 5000));

    // Verify index still accessible
    const { stdout } = await execAsync('docker-compose exec -T cli-tools cds search "test" --format json');
    const result = JSON.parse(stdout);
    expect(result.results).toBeDefined();
  });

  it('should handle service failures gracefully', async () => {
    // Stop Index Service
    await execAsync('docker-compose stop index-service');

    // CLI should fail with clear error
    try {
      await execAsync('docker-compose exec -T cli-tools cds search "test"');
    } catch (error) {
      expect(error.stderr).toContain('connection refused');
    }

    // Restart and verify recovery
    await execAsync('docker-compose start index-service');
    await new Promise(resolve => setTimeout(resolve, 5000));

    const { stdout } = await execAsync('docker-compose exec -T cli-tools cds search "test"');
    expect(stdout).toBeTruthy();
  });
});
```

### Performance Under Load

```typescript
// tests/integration/load-test.ts
import { describe, it, expect } from 'bun:test';

describe('Performance Under Load', () => {
  it('should handle 10 concurrent search requests', async () => {
    const queries = Array(10).fill(null).map((_, i) => ({
      query: `function_${i}`,
      limit: 10,
    }));

    const start = Date.now();
    const results = await Promise.all(
      queries.map(q => fetch('http://localhost:3030/rpc', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          jsonrpc: '2.0',
          method: 'search_entities',
          params: q,
          id: Math.random(),
        }),
      }))
    );
    const duration = Date.now() - start;

    // All requests should succeed
    expect(results.every(r => r.ok)).toBe(true);

    // Should complete in <5 seconds
    expect(duration).toBeLessThan(5000);
  });
});
```

## Test Utilities

```typescript
// tests/test-utils/agent-runner.ts
export class AgentRunner {
  constructor(private config: { indexServiceUrl: string; cliPath: string }) {}

  async runTask(task: string, options = {}) {
    const agent = new Agent({
      systemPrompt: RETRIEVAL_PROMPT,
      tools: [bashTool],
      ...options,
    });

    const result = await agent.run(task);

    return {
      locations: this.parseLocations(result.output),
      reasoning: result.reasoning,
      toolCallHistory: result.toolCalls,
      status: result.status,
    };
  }

  private parseLocations(output: string) {
    // Parse agent output JSON
    const parsed = JSON.parse(output);
    return parsed.locations || [];
  }
}
```

## CI/CD Integration

```yaml
# .github/workflows/integration.yml
name: Integration Tests

on: [push, pull_request]

jobs:
  integration:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Start services
        run: docker-compose up -d

      - name: Wait for services
        run: |
          timeout 60 bash -c 'until curl -f http://localhost:3030/health; do sleep 2; done'

      - name: Run integration tests
        run: |
          bun test tests/integration/
          bash tests/integration/cli_integration_test.sh

      - name: Stop services
        run: docker-compose down
```

## Acceptance Criteria

- [ ] Agent completes 5 sample code localization tasks
- [ ] Integration tests run in Docker Compose environment
- [ ] Error scenarios tested (service down, invalid query)
- [ ] Performance under load (10 concurrent requests)
- [ ] All tests pass in CI/CD pipeline
- [ ] Test execution time <10 minutes

**Related**: [00-overview.md](00-overview.md), [01-unit-tests.md](01-unit-tests.md), [../04-agent-integration/04-sample-transcripts.md](../04-agent-integration/04-sample-transcripts.md)
