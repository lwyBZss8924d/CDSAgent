# PRD-05: API Specifications - Interfaces & Data Schemas

**Version:** 1.0 (Round 1 - Concise)
**Date:** 2025-10-18
**Status:** Draft
**Parent:** PRD-01 System Architecture

---

## 1. Document Overview

### 1.1 Purpose

This document defines all APIs, interfaces, and data schemas for CDSAgent components, ensuring consistent communication between Rust (CDS-Index/Tools) and TypeScript (CDS-Agent) layers.

### 1.2 Scope

- Rust ↔ TypeScript bridge protocol (gRPC vs JSON-RPC)
- CDS-Index Service API contracts
- CDS-Tools CLI output schemas
- Entity data models (nodes, edges, search results)
- Error codes and status types

---

## 2. Bridge Protocol Selection

### 2.1 Options Analysis

| Protocol | Pros | Cons | Decision |
|----------|------|------|----------|
| **gRPC** | Type-safe, streaming, fast | Complex setup, larger binary | **Recommended** for production |
| **JSON-RPC** | Simple, HTTP-based | Slower, no streaming | Acceptable for v1.0 |
| **FFI (Neon)** | In-process, fastest | Platform-specific builds | Future optimization |

**Decision (v1.0)**: Use **JSON-RPC over HTTP** for simplicity. Migrate to gRPC in v1.1 if performance bottlenecks occur.

### 2.2 JSON-RPC Service Specification

**Endpoint**: `http://localhost:9876/rpc` (CDS-Index Service)

**Request Format**:

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

**Response Format**:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "entities": [ /* ... */ ],
    "total_count": 3
  }
}
```

**Error Format**:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32001,
    "message": "Index not found",
    "data": {
      "index_path": "/path/to/missing/index"
    }
  }
}
```

---

## 3. CDS-Index Service APIs

### 3.1 SearchEntities

**Method**: `search_entities`

**Parameters**:

```typescript
interface SearchParams {
  query: string;                       // Search keywords
  entity_types?: EntityType[];         // Filter: ["file", "class", "function"]
  limit?: number;                      // Max results (default: 10)
  use_bm25?: boolean;                  // Enable BM25 fallback (default: true)
  snippet_mode?: "fold" | "preview" | "full"; // Detail level (default: "preview")
}

type EntityType = "directory" | "file" | "class" | "function";
```

**Response**:

```typescript
interface SearchResult {
  entities: Entity[];
  total_count: number;
  query_metadata: {
    used_upper_index: boolean;
    used_bm25: boolean;
    execution_time_ms: number;
  };
}

interface Entity {
  id: string;                          // Unique entity ID (hash or qualified name)
  name: string;                        // Entity name
  entity_type: EntityType;
  file_path: string;
  line_range: [number, number];
  score: number;                       // Relevance score (1.0 = exact match)
  snippet: {
    fold: string;                      // One-line summary
    preview: string;                   // ~5 lines
    full: string;                      // Complete code
  };
}
```

### 3.2 TraverseGraph

**Method**: `traverse_graph`

**Parameters**:

```typescript
interface TraverseParams {
  start_entities: string[];            // Entity IDs to start from
  depth: number;                       // Max traversal hops (default: 1)
  relations?: RelationType[];          // Filter: ["contain", "invoke", ...]
  entity_types?: EntityType[];         // Filter by node types
  direction?: "forward" | "backward" | "bidirectional"; // Default: forward
  format?: "graph" | "tree";           // Output structure (default: graph)
}

type RelationType = "contain" | "import" | "invoke" | "inherit";
```

**Response**:

```typescript
interface TraverseResult {
  subgraph: {
    nodes: GraphNode[];
    edges: GraphEdge[];
  };
  metadata: {
    total_nodes: number;
    total_edges: number;
    max_depth_reached: number;
    execution_time_ms: number;
  };
}

interface GraphNode {
  id: string;
  name: string;
  entity_type: EntityType;
  file_path: string;
  line_range?: [number, number];
  depth: number;                       // Distance from start node
}

interface GraphEdge {
  source: string;                      // Source entity ID
  target: string;                      // Target entity ID
  relation: RelationType;
}
```

### 3.3 RetrieveEntity

**Method**: `retrieve_entity`

**Parameters**:

```typescript
interface RetrieveParams {
  entity_ids: string[];
  include_context?: number;            // Lines before/after (default: 0)
  include_metadata?: boolean;          // Include AST metadata (default: false)
}
```

**Response**:

```typescript
interface RetrieveResult {
  entities: EntityDetails[];
}

interface EntityDetails {
  id: string;
  name: string;
  entity_type: EntityType;
  file_path: string;
  line_range: [number, number];
  code: string;                        // Full entity code
  context_before?: string;             // Lines before entity
  context_after?: string;              // Lines after entity
  metadata?: {
    parameters?: string[];             // For functions
    return_type?: string;
    docstring?: string;
    decorators?: string[];
    parent_class?: string;             // For methods
  };
}
```

### 3.4 RebuildIndex

**Method**: `rebuild_index`

**Parameters**:

```typescript
interface RebuildParams {
  repo_path: string;
  languages?: string[];                // ["python", "typescript"]
  incremental?: boolean;               // Update only changed files
  output_path?: string;                // Index directory (default: repo/.cds-index)
}
```

**Response**:

```typescript
interface RebuildResult {
  success: boolean;
  stats: {
    files_indexed: number;
    entities_found: {
      directories: number;
      files: number;
      classes: number;
      functions: number;
    };
    edges_created: {
      contain: number;
      import: number;
      invoke: number;
      inherit: number;
    };
    build_time_ms: number;
  };
  errors?: IndexError[];
}

interface IndexError {
  file_path: string;
  error: string;
  line?: number;
}
```

---

## 4. CDS-Tools CLI Output Schemas

### 4.1 `cds search` Output

**JSON Schema**:

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

### 4.2 `cds traverse` Output

**JSON Structure** (Graph Mode):

```json
{
  "start_entities": ["entity_abc"],
  "subgraph": {
    "nodes": [
      {
        "id": "entity_abc",
        "name": "process_request",
        "type": "function",
        "file": "server.py",
        "line_range": [12, 45],
        "depth": 0
      }
    ],
    "edges": [
      {
        "source": "entity_abc",
        "target": "entity_def",
        "relation": "invoke"
      }
    ]
  }
}
```

**Text Format** (Tree Mode):

```text
process_request (function) [entity_abc] - server.py:12
├─[invoke]→ validate_input (function) [entity_def] - validators.py:45
│  ├─[invoke]→ sanitize_html (function) - utils/sanitize.py:15
│  └─[invoke]→ check_csrf (function) - security.py:78
└─[invoke]→ save_to_db (function) - database.py:123
```

### 4.3 `cds retrieve` Output

**JSON Structure**:

```json
{
  "entities": [
    {
      "id": "entity_abc",
      "name": "sanitize_html",
      "type": "function",
      "file_path": "src/utils/sanitize.py",
      "line_range": [15, 32],
      "code": "def sanitize_html(input: str) -> str:\n    \"\"\"Remove XSS vectors...\"\"\"\n    return bleach.clean(input, tags=['b', 'i'])"
    }
  ]
}
```

---

## 5. Error Codes

### 5.1 HTTP Status Codes (for JSON-RPC)

| Code | Meaning | Usage |
|------|---------|-------|
| 200 | OK | Successful RPC call |
| 400 | Bad Request | Invalid JSON-RPC format |
| 500 | Internal Server Error | CDS-Index Service crash |
| 503 | Service Unavailable | Index not loaded |

### 5.2 JSON-RPC Error Codes

| Code | Error | Description |
|------|-------|-------------|
| -32700 | Parse error | Invalid JSON |
| -32600 | Invalid Request | Missing required fields |
| -32601 | Method not found | Unknown RPC method |
| -32602 | Invalid params | Bad parameter types |
| -32603 | Internal error | Service error |
| -32001 | Index not found | GRAPH_INDEX_DIR missing or corrupted |
| -32002 | Entity not found | Entity ID doesn't exist |
| -32003 | Parse error | Code parsing failed |
| -32004 | Query timeout | Search took too long |

### 5.3 CLI Exit Codes

| Code | Meaning | Example |
|------|---------|---------|
| 0 | Success | Results found and returned |
| 1 | No results | `cds search` found nothing |
| 2 | Invalid arguments | Missing required parameter |
| 3 | Index not found | GRAPH_INDEX_DIR not set |
| 4 | Service error | CDS-Index Service unreachable |
| 5 | IO error | Can't write output file |

---

## 6. Type Definitions (Rust)

### 6.1 Core Rust Types

```rust
// cds_core/src/types.rs

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EntityType {
    Directory,
    File,
    Class,
    Function,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RelationType {
    Contain,
    Import,
    Invoke,
    Inherit,
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
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    pub relation: RelationType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSnippet {
    pub fold: String,
    pub preview: String,
    pub full: String,
}
```

---

## 7. TypeScript Type Definitions

### 7.1 Generated from Rust

**Use `typeshare` crate** to auto-generate TypeScript types from Rust structs:

```rust
// In Rust
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
    // ...
}
```

**Generates TypeScript**:

```typescript
// Generated: cds-agent/src/generated/types.ts
export interface Entity {
  id: string;
  name: string;
  entity_type: EntityType;
  file_path: string;
  line_range?: [number, number];
}

export enum EntityType {
  Directory = "Directory",
  File = "File",
  Class = "Class",
  Function = "Function",
}
```

---

## 8. Validation and Compatibility

### 8.1 Schema Validation

- **Rust**: Use `serde_json` for validation during deserialization
- **TypeScript**: Use `zod` or `ajv` for runtime validation

```typescript
import { z } from 'zod';

const EntitySchema = z.object({
  id: z.string(),
  name: z.string(),
  entity_type: z.enum(['directory', 'file', 'class', 'function']),
  file_path: z.string(),
  line_range: z.tuple([z.number(), z.number()]).optional(),
});

// Validate API response
const entity = EntitySchema.parse(apiResponse);
```

### 8.2 API Versioning

**URL-based versioning**:

```text
http://localhost:9876/v1/rpc  // Current
http://localhost:9876/v2/rpc  // Future breaking changes
```

**Response header**:

```text
X-CDS-API-Version: 1.0.0
```

---

## 9. Open Questions

1. **gRPC Adoption**: When to migrate from JSON-RPC? (After v1.0 if latency >100ms)
2. **Type Generation**: Use `typeshare` or manual TypeScript definitions? (Auto-gen preferred)
3. **Streaming**: Do we need streaming search results? (Not in v1.0)

---

## 10. Acceptance Criteria

- [ ] All API methods defined with TypeScript signatures
- [ ] JSON schemas validated (use online validators)
- [ ] Error codes documented and consistent
- [ ] Rust types serialize to match TypeScript expectations
- [ ] CLI output parseable by `jq` without errors

---

**Status**: Ready for implementation. Requires alignment with PRD-02 (CDS-Index) and PRD-03 (CDS-Tools).
