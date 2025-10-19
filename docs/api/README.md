# CDSAgent API Documentation

**Version:** 0.1.0
**Last Updated:** 2025-10-19
**Status:** Draft

---

## Overview

This directory contains the complete API documentation for CDSAgent's JSON-RPC 2.0 interface.

---

## Quick Links

| Document | Description |
|----------|-------------|
| [JSON-RPC Schema](./jsonrpc-schema.json) | Complete JSON Schema for all API methods |
| [Error Codes](./error-codes.md) | Comprehensive error code catalogue |
| [Versioning](./versioning.md) | API versioning strategy and compatibility policy |

---

## API Endpoint

**CDS-Index Service:**

```text
http://localhost:9876/rpc
```

**Protocol:** JSON-RPC 2.0 over HTTP

---

## Available Methods

### 1. search_entities

Hierarchical code search using name index and BM25 content search.

**Request:**

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "search_entities",
  "params": {
    "query": "sanitize input",
    "entity_types": ["function"],
    "limit": 10
  }
}
```

**Response:**

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "entities": [ ... ],
    "total_count": 3,
    "query_metadata": {
      "used_upper_index": true,
      "used_bm25": false,
      "execution_time_ms": 120
    }
  }
}
```

**See:** [jsonrpc-schema.json](./jsonrpc-schema.json#L322) for full specification

---

### 2. traverse_graph

BFS graph traversal from starting entities.

**Request:**

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "traverse_graph",
  "params": {
    "start_entities": ["entity_abc"],
    "depth": 2,
    "relations": ["invoke"]
  }
}
```

**Response:**

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "subgraph": {
      "nodes": [ ... ],
      "edges": [ ... ]
    },
    "metadata": {
      "total_nodes": 15,
      "total_edges": 14,
      "max_depth_reached": 2
    }
  }
}
```

**See:** [jsonrpc-schema.json](./jsonrpc-schema.json#L416) for full specification

---

### 3. retrieve_entity

Retrieve full entity details with optional context lines.

**Request:**

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "retrieve_entity",
  "params": {
    "entity_ids": ["entity_abc"],
    "include_context": 5,
    "include_metadata": true
  }
}
```

**Response:**

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "entities": [
      {
        "id": "entity_abc",
        "name": "sanitize_html",
        "entity_type": "function",
        "file_path": "src/utils.py",
        "line_range": [15, 32],
        "code": "def sanitize_html(...):\n    ...",
        "metadata": { ... }
      }
    ]
  }
}
```

**See:** [jsonrpc-schema.json](./jsonrpc-schema.json#L493) for full specification

---

### 4. rebuild_index

Rebuild graph and BM25 indices for a repository.

**Request:**

```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "method": "rebuild_index",
  "params": {
    "repo_path": "/path/to/repo",
    "languages": ["python"],
    "incremental": false
  }
}
```

**Response:**

```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "result": {
    "success": true,
    "stats": {
      "files_indexed": 150,
      "entities_found": { ... },
      "edges_created": { ... },
      "build_time_ms": 5000
    }
  }
}
```

**See:** [jsonrpc-schema.json](./jsonrpc-schema.json#L538) for full specification

---

## Error Handling

All errors follow JSON-RPC 2.0 error format:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32001,
    "message": "Index not found",
    "data": {
      "index_path": "/path/to/index",
      "suggestion": "Run 'cds init <repo>'"
    }
  }
}
```

**Common Error Codes:**

| Code | Error | Description |
|------|-------|-------------|
| -32001 | Index not found | Index directory missing |
| -32002 | Entity not found | Entity ID doesn't exist |
| -32003 | Parse error | Code parsing failed |
| -32004 | Query timeout | Search exceeded timeout |

**See:** [error-codes.md](./error-codes.md) for complete error catalogue

---

## Client Libraries

### TypeScript/Node.js

```typescript
import { CDSClient } from 'cds-agent';

const client = new CDSClient('http://localhost:9876/rpc');

// Search entities
const results = await client.call('search_entities', {
  query: 'sanitize',
  limit: 10
});

console.log(results.entities);
```

### Rust

```rust
use cds_tools::client::JsonRpcClient;

let client = JsonRpcClient::new("http://localhost:9876/rpc");

let response = client.call("search_entities", json!({
    "query": "sanitize",
    "limit": 10
})).await?;
```

### CLI

```bash
# Search for entities
cds search "sanitize" --limit 10

# Traverse graph
cds traverse "entity_abc" --depth 2

# Retrieve entity
cds retrieve "entity_abc" --context 5
```

---

## Testing

### Schema Validation Tests

Run contract tests to validate API responses:

```bash
cargo test --package cds-index --test service_contract_tests
```

**Test Fixtures:**

- [tests/fixtures/api/](../../tests/fixtures/api/) - Example requests/responses

### Manual Testing with curl

```bash
# Search entities
curl -X POST http://localhost:9876/rpc \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "search_entities",
    "params": {
      "query": "sanitize",
      "limit": 10
    }
  }'
```

### JSON Schema Validation

Validate responses using online validators:

- <https://www.jsonschemavalidator.net/>
- Upload [jsonrpc-schema.json](./jsonrpc-schema.json)
- Paste response JSON

---

## Versioning

**Current Version:** `0.1.0` (MVP)

**Version Header:**

All responses include:

```http
X-CDS-API-Version: 0.1.0
```

**Backward Compatibility:**

- During `v0.x.x`: API may change between minor versions
- From `v1.0.0`: Semantic versioning enforced
  - MAJOR: Breaking changes
  - MINOR: Backward-compatible additions
  - PATCH: Bug fixes

**See:** [versioning.md](./versioning.md) for full versioning strategy

---

## Related Documents

### PRDs (Requirements)

- [PRD-05: API Specifications](../../spacs/prd/0.1.0-MVP-PRDs-v0/05-api-specifications.md)
- [PRD-02: CDS-Index Service](../../spacs/prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md)
- [PRD-03: CDS-Tools CLI](../../spacs/prd/0.1.0-MVP-PRDs-v0/03-cds-tools-cli.md)

### Issues (Implementation)

- [Issue-05: API Contracts](../../spacs/issues/04-0.1.0-mvp/05-api-contracts.md)
- [Issue-02-03: Service Layer](../../spacs/issues/04-0.1.0-mvp/02-index-core/03-service-layer.md)

### Tasks (Development)

- [T-05-01: JSON-RPC Schema](../../spacs/tasks/0.1.0-mvp/05-api-contracts/T-05-01-jsonrpc-schema.md)
- [T-05-02: TypeScript Bindings](../../spacs/tasks/0.1.0-mvp/05-api-contracts/T-05-02-typescript-bindings.md)

---

## FAQ

### Q: Why JSON-RPC instead of REST?

**A:** JSON-RPC provides:

- Simpler client implementation (single endpoint)
- Batch request support
- Type-safe method routing
- Standard error handling

REST may be added in v0.2.0 for HTTP-friendly clients.

### Q: Is gRPC supported?

**A:** Not in v0.1.0. gRPC will be evaluated for v0.2.0 if:

- Latency exceeds 100ms p95
- Streaming large results is needed
- Type safety requires .proto definitions

### Q: Can I use the API from other languages?

**A:** Yes! JSON-RPC is language-agnostic. Use any HTTP client:

- **Python:** `requests` library
- **Java:** `okhttp` or `HttpClient`
- **Go:** `net/http`
- **Ruby:** `net/http` or `httparty`

### Q: How do I handle errors?

**A:** Check the `error` field in responses:

```python
import requests

response = requests.post('http://localhost:9876/rpc', json={
    "jsonrpc": "2.0",
    "id": 1,
    "method": "search_entities",
    "params": {"query": "test"}
}).json()

if "error" in response:
    code = response["error"]["code"]
    if code == -32001:
        print("Index not found, running rebuild...")
    else:
        raise Exception(response["error"]["message"])
else:
    print(response["result"]["entities"])
```

### Q: What about rate limiting?

**A:** v0.1.0 has no rate limiting. Future versions may add:

- Per-client request limits
- Token bucket algorithm
- HTTP 429 responses

---

## Contributing

### Reporting API Issues

If you find schema violations or documentation errors:

1. Check [error-codes.md](./error-codes.md) for expected behavior
2. Validate against [jsonrpc-schema.json](./jsonrpc-schema.json)
3. File issue with:
   - Request JSON
   - Response JSON
   - Expected vs actual behavior

### Proposing API Changes

For API modifications:

1. Discuss in GitHub Issues first
2. Follow [versioning.md](./versioning.md) compatibility rules
3. Update schema and tests
4. Add migration guide if breaking change

---

## License

CDSAgent is licensed under MIT. See [LICENSE](../../LICENSE) for details.

---

**Last Updated:** 2025-10-19 by Claude Code Agent
