# Issue-05: API Contracts & Data Schemas

**Priority**: P0 (Critical Path - Interface Definitions)
**Status**: ☐ Not Started
**Owner**: Rust Lead + TypeScript Developer
**PRD Reference**: [PRD-05: API Specifications](../../prd/0.1.0-MVP-PRDs-v0/05-api-specifications.md)

---

## Overview

Define and validate all APIs, interfaces, and data schemas for CDSAgent components, ensuring type-safe, consistent communication between Rust (CDS-Index/Tools) and TypeScript (CDS-Agent) layers.

## Objective

Establish contract-first development where:

- JSON-RPC schemas are fully specified before implementation
- TypeScript types are auto-generated from Rust definitions
- Breaking changes are detected early via schema validation
- All stakeholders agree on API surface before coding begins

## Dependencies

- **Requires**: Architecture decisions (JSON-RPC vs gRPC)
- **Blocks**: Implementation of 02-index-core/03-service-layer, 04-agent-integration
- **Shared With**: All teams (provides shared types)

---

## API Surface Definition

### 1. JSON-RPC Protocol Specification (PRD-05 §2.2)

**Endpoint**: `http://localhost:9876/rpc` (CDS-Index Service)

**Methods**:

1. `search_entities` - Hierarchical code search
2. `traverse_graph` - BFS graph traversal
3. `retrieve_entity` - Full entity content retrieval
4. `rebuild_index` - Trigger index rebuild (admin only)

**v0.1.0 Decision** (from PRD-02 §4.1 "Service Interface Strategy"):

- Ship JSON-RPC HTTP server as MVP deliverable
- gRPC module scaffolded but marked experimental (v0.2.0+)

---

### 2. Core Data Schemas

#### 2.1 Entity Schema (PRD-05 §3.1)

**TypeScript Definition**:

```typescript
interface Entity {
  id: string;                          // Unique entity ID (hash or qualified name)
  name: string;                        // Entity name
  entity_type: EntityType;             // "directory" | "file" | "class" | "function"
  file_path: string;
  line_range: [number, number];
  score: number;                       // Relevance score (1.0 = exact match)
  snippet: {
    fold: string;                      // One-line summary
    preview: string;                   // ~5 lines
    full: string;                      // Complete code
  };
}

type EntityType = "directory" | "file" | "class" | "function";
```

**Rust Definition** (PRD-05 §6.1):

```rust
// cds_core/src/types.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    Directory,
    File,
    Class,
    Function,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub entity_type: EntityType,
    pub file_path: String,
    pub line_range: Option<(usize, usize)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSnippet {
    pub fold: String,
    pub preview: String,
    pub full: String,
}
```

**Type Generation**: Use `typeshare` crate to auto-generate TypeScript from Rust (PRD-05 §7.1).

#### 2.2 Graph Edge Schema (PRD-05 §3.2)

**TypeScript**:

```typescript
interface GraphEdge {
  source: string;                      // Source entity ID
  target: string;                      // Target entity ID
  relation: RelationType;              // "contain" | "import" | "invoke" | "inherit"
}

type RelationType = "contain" | "import" | "invoke" | "inherit";
```

**Rust**:

```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RelationType {
    Contain,
    Import,
    Invoke,
    Inherit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    pub relation: RelationType,
}
```

---

### 3. RPC Method Schemas

#### 3.1 SearchEntities (PRD-05 §3.1)

**Request**:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "search_entities",
  "params": {
    "query": "sanitize input",
    "entity_types": ["function"],
    "limit": 10,
    "use_bm25": true,
    "snippet_mode": "preview"
  }
}
```

**Response**:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "entities": [/* Entity[] */],
    "total_count": 3,
    "query_metadata": {
      "used_upper_index": true,
      "used_bm25": false,
      "execution_time_ms": 120
    }
  }
}
```

#### 3.2 TraverseGraph (PRD-05 §3.2)

**Request**:

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "traverse_graph",
  "params": {
    "start_entities": ["entity_abc"],
    "depth": 2,
    "relations": ["invoke"],
    "entity_types": ["function"],
    "direction": "forward",
    "format": "tree"
  }
}
```

**Response**:

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "subgraph": {
      "nodes": [/* GraphNode[] */],
      "edges": [/* GraphEdge[] */]
    },
    "metadata": {
      "total_nodes": 15,
      "total_edges": 14,
      "max_depth_reached": 2,
      "execution_time_ms": 450
    }
  }
}
```

---

### 4. Error Handling

#### JSON-RPC Error Codes (PRD-05 §5.2)

| Code | Error | Description | Handling |
|------|-------|-------------|----------|
| -32700 | Parse error | Invalid JSON | Return 400 Bad Request |
| -32600 | Invalid Request | Missing required fields | Return 400 |
| -32601 | Method not found | Unknown RPC method | Return 404 |
| -32602 | Invalid params | Bad parameter types | Return 400 |
| -32603 | Internal error | Service error | Return 500 |
| -32001 | Index not found | GRAPH_INDEX_DIR missing | Return 503 |
| -32002 | Entity not found | Entity ID doesn't exist | Return 404 |
| -32003 | Parse error | Code parsing failed | Return 500 |
| -32004 | Query timeout | Search took too long | Return 503 |

**Error Response Format**:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32001,
    "message": "Index not found",
    "data": {
      "index_path": "/path/to/missing/index",
      "suggestion": "Run 'cds init <repo>' to create an index"
    }
  }
}
```

---

### 5. CLI Output Schemas (PRD-05 §4)

#### `cds search` JSON Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["query", "results"],
  "properties": {
    "query": { "type": "string" },
    "total_results": { "type": "integer" },
    "results": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["entity_id", "name", "type", "file_path"],
        "properties": {
          "entity_id": { "type": "string" },
          "name": { "type": "string" },
          "type": { "enum": ["directory", "file", "class", "function"] },
          "file_path": { "type": "string" },
          "line_range": {
            "type": "array",
            "items": { "type": "integer" },
            "minItems": 2,
            "maxItems": 2
          },
          "score": { "type": "number", "minimum": 0, "maximum": 1 },
          "snippet": {
            "type": "object",
            "properties": {
              "fold": { "type": "string" },
              "preview": { "type": "string" },
              "full": { "type": "string" }
            }
          }
        }
      }
    }
  }
}
```

---

## Deliverables

### Phase 1 (Week 1-2): Schema Definition

- [ ] Define all Rust types in `cds_core/src/types.rs`
- [ ] Write JSON Schema files for all RPC methods
- [ ] Generate TypeScript types using `typeshare`
- [ ] Document all error codes in shared doc

### Phase 2 (Week 3-4): Validation Setup

- [ ] Add `zod` schemas in TypeScript client
- [ ] Add `serde_json` validation in Rust server
- [ ] Create contract test suite (see Acceptance Criteria)
- [ ] Set up CI to validate schemas on every PR

### Phase 3 (Week 5+): Evolution Management

- [ ] Document versioning strategy (URL-based: `/v1/rpc`, `/v2/rpc`)
- [ ] Add `X-CDS-API-Version` response header
- [ ] Create migration guide for breaking changes

---

## Acceptance Criteria (from PRD-05 §10)

### Must-Pass

- [ ] All API methods defined with TypeScript signatures (§3)
- [ ] JSON schemas validated using online validators (e.g., jsonschemavalidator.net)
- [ ] Error codes documented and consistent (§5.2)
- [ ] Rust types serialize to match TypeScript expectations (validated via tests)
- [ ] CLI output parseable by `jq` without errors (run on 10 sample outputs)

### Contract Tests

```typescript
// cds-agent/src/__tests__/api-contract.test.ts

import { z } from 'zod';
import { EntitySchema, SearchResultSchema } from '../schemas';

test('search API returns valid schema', async () => {
  const response = await fetch('http://localhost:9876/rpc', {
    method: 'POST',
    body: JSON.stringify({
      jsonrpc: '2.0',
      id: 1,
      method: 'search_entities',
      params: { query: 'test', limit: 10 }
    })
  });

  const data = await response.json();
  const parsed = SearchResultSchema.parse(data.result);
  expect(parsed.entities.length).toBeGreaterThan(0);
});

test('entity schema matches Rust serialization', () => {
  const rustEntity = {
    id: "abc123",
    name: "foo",
    entity_type: "function",
    file_path: "src/foo.py",
    line_range: [10, 20]
  };

  expect(() => EntitySchema.parse(rustEntity)).not.toThrow();
});
```

---

## Open Questions

1. **gRPC Promotion Timing** (from PRD-05 §9): Confirmed JSON-RPC for v0.1.0; reassess gRPC for v0.2.0 if latency >100ms.
2. **Type Generation Tool**: Confirmed `typeshare` crate for auto-gen (PRD-05 §7.1).
3. **Streaming Support**: Not needed in v0.1.0; defer to v0.2.0 if large result sets cause issues.

---

## Related Issues

- [02-index-core/03-service-layer.md](02-index-core/03-service-layer.md) - JSON-RPC server implementation
- [04-agent-integration/01-sdk-bootstrap.md](04-agent-integration/01-sdk-bootstrap.md) - TypeScript client usage
- [08-testing/02-integration.md](08-testing/02-integration.md) - API contract tests

---

**Status Updates**:

- *2025-10-19*: API contracts drafted, awaiting stakeholder review before implementation freeze
