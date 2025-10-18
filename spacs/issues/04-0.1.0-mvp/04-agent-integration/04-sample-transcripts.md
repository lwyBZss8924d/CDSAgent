# Sub-Issue 04.04: Sample Transcripts - E2E Test Scenarios

**Priority**: P1
**Owner**: TypeScript Dev 1 + QA Lead
**Timing**: Phase 3, Week 7
**PRD Reference**: [PRD-04 §7](../../../prd/0.1.0-MVP-PRDs-v0/04-cds-agent-integration.md), [PRD-08 §5](../../../prd/0.1.0-MVP-PRDs-v0/08-testing-quality.md)

## Objective

Validate agent workflows with 5-10 sample code localization tasks from SWE-bench Lite.

## Test Scenarios

### Scenario 1: XSS Vulnerability Search

```json
{
  "task": "Find all functions that handle user input sanitization",
  "expected_cot": [
    "Extract keywords: sanitize, XSS, user input",
    "Search: cds search 'sanitize' --type function",
    "Traverse: Find callers of sanitize functions",
    "Retrieve: Verify sanitization logic"
  ],
  "expected_locations": [
    {"file": "utils/sanitize.py", "function": "sanitize_html"}
  ]
}
```

### Scenario 2-5: Additional test cases covering

- Function call traversal
- Class inheritance exploration
- Import dependency analysis
- Error handling pattern search

## Validation Scripts

```typescript
// tests/integration/agent-workflows.test.ts
describe('Agent Code Localization', () => {
  it('should find XSS sanitization functions', async () => {
    const result = await runAgent(scenarios.xss_search);
    expect(result.locations).toContainFile('utils/sanitize.py');
    expect(result.reasoning).toInclude('sanitize_html');
  });
});
```

## Acceptance Criteria

- [ ] Agent completes 5 sample tasks successfully
- [ ] File Acc@5 ≥75% on test set
- [ ] Reasoning traces show CoT steps
- [ ] Tool call efficiency comparable to LocAgent

**Related**: [00-overview.md](00-overview.md), [../08-testing/04-benchmark.md](../08-testing/04-benchmark.md)
