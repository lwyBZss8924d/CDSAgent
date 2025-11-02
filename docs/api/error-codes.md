# CDSAgent Error Code Catalogue

**Version:** 0.1.0
**Last Updated:** 2025-10-25
**Status:** Final (T-05-03 Complete)

---

## Overview

This document defines all error codes used across CDSAgent components, including JSON-RPC service errors, CLI exit codes, and HTTP status codes.

---

## 1. HTTP Status Codes

Used by CDS-Index Service JSON-RPC endpoint (`http://localhost:9876/rpc`)

| Code | Status | Description | When to Use |
|------|--------|-------------|-------------|
| **200** | OK | Successful RPC call | All successful JSON-RPC requests (even if result contains application errors) |
| **400** | Bad Request | Invalid JSON-RPC format | Malformed JSON, missing required JSON-RPC fields |
| **500** | Internal Server Error | CDS-Index Service crash | Unexpected service failures, panics |
| **503** | Service Unavailable | Index not loaded | Index directory missing or not initialized |

---

## 2. JSON-RPC Error Codes

### 2.1 Standard JSON-RPC 2.0 Errors

| Code | Error Name | Description | Example Scenario |
|------|-----------|-------------|------------------|
| **-32700** | Parse error | Invalid JSON received | `{"jsonrpc": "2.0", "method": "search", id: 1}` (unquoted property) |
| **-32600** | Invalid Request | Not a valid Request object | Missing `jsonrpc` field or wrong version |
| **-32601** | Method not found | Unknown RPC method | `method: "unknown_method"` |
| **-32602** | Invalid params | Bad parameter types/values | `query: 123` (number instead of string) |
| **-32603** | Internal error | Internal JSON-RPC error | Unhandled exception during method execution |

**Example Error Response:**

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32602,
    "message": "Invalid params",
    "data": {
      "field": "query",
      "expected": "string",
      "received": "number"
    }
  }
}
```

### 2.2 CDSAgent Custom Errors

| Code | Error Name | Description | Recovery Action |
|------|-----------|-------------|-----------------|
| **-32001** | Index not found | GRAPH_INDEX_DIR missing or corrupted | Run `cds init <repo>` or check `GRAPH_INDEX_DIR` env var |
| **-32002** | Entity not found | Entity ID doesn't exist in index | Verify entity ID or rebuild index |
| **-32003** | Parse error | Code parsing failed (tree-sitter error) | Check source file syntax or file encoding |
| **-32004** | Query timeout | Search operation exceeded timeout | Reduce search scope or increase timeout limit |

**Example Error Responses:**

#### Index Not Found (-32001)

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32001,
    "message": "Index not found",
    "data": {
      "index_path": "/path/to/repo/.cds-index",
      "suggestion": "Run 'cds init /path/to/repo' to create an index"
    }
  }
}
```

#### Entity Not Found (-32002)

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "error": {
    "code": -32002,
    "message": "Entity not found",
    "data": {
      "entity_id": "nonexistent_entity_id",
      "searched_in": "/path/to/repo/.cds-index"
    }
  }
}
```

#### Parse Error (-32003)

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "error": {
    "code": -32003,
    "message": "Parse error",
    "data": {
      "file_path": "src/malformed.py",
      "line": 42,
      "error_message": "Unexpected token 'def' at line 42",
      "suggestion": "Check syntax in src/malformed.py:42"
    }
  }
}
```

#### Query Timeout (-32004)

```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "error": {
    "code": -32004,
    "message": "Query timeout",
    "data": {
      "timeout_ms": 5000,
      "query": "complex search query",
      "suggestion": "Narrow search scope or increase timeout"
    }
  }
}
```

---

## 3. CLI Exit Codes

Used by `cds-tools` CLI commands (`cds search`, `cds traverse`, `cds retrieve`)

| Code | Meaning | Description | Example Command |
|------|---------|-------------|-----------------|
| **0** | Success | Operation completed successfully | `cds search "User"` (results found) |
| **1** | No results | Query executed but found no matches | `cds search "NonExistentEntity"` |
| **2** | Invalid arguments | Missing required parameter or invalid option | `cds search` (no query provided) |
| **3** | Index not found | GRAPH_INDEX_DIR not set or missing | `cds search "User"` (index not initialized) |
| **4** | Service error | CDS-Index Service unreachable | Service not running on `localhost:9876` |
| **5** | IO error | Can't read/write file | `cds search "User" -o /read-only/output.json` |

**Usage Example:**

```shell
#!/bin/bash

# Check if search found results
cds search "sanitize" --repo ./my-repo

EXIT_CODE=$?

case $EXIT_CODE in
  0)
    echo "✓ Results found"
    ;;
  1)
    echo "⚠ No results found"
    ;;
  2)
    echo "✗ Invalid arguments"
    exit 2
    ;;
  3)
    echo "✗ Index not found - run 'cds init ./my-repo'"
    exit 3
    ;;
  4)
    echo "✗ Service error - is cds-index-service running?"
    exit 4
    ;;
  5)
    echo "✗ IO error"
    exit 5
    ;;
esac
```

---

## 4. Error Handling Guidelines

### 4.1 For Service Implementers (Rust)

**When to Return Each Error:**

```rust
// -32001: Index not found
if !index_path.exists() {
    return Err(JsonRpcError {
        code: -32001,
        message: "Index not found".into(),
        data: Some(json!({
            "index_path": index_path.display().to_string(),
            "suggestion": format!("Run 'cds init {}'", repo_path.display())
        }))
    });
}

// -32002: Entity not found
if let None = graph.get_entity(&entity_id) {
    return Err(JsonRpcError {
        code: -32002,
        message: "Entity not found".into(),
        data: Some(json!({
            "entity_id": entity_id,
            "searched_in": index_path.display().to_string()
        }))
    });
}

// -32003: Parse error
if let Err(parse_err) = parser.parse_file(file_path) {
    return Err(JsonRpcError {
        code: -32003,
        message: "Parse error".into(),
        data: Some(json!({
            "file_path": file_path.display().to_string(),
            "error_message": parse_err.to_string()
        }))
    });
}

// -32004: Query timeout
if elapsed > timeout_duration {
    return Err(JsonRpcError {
        code: -32004,
        message: "Query timeout".into(),
        data: Some(json!({
            "timeout_ms": timeout_duration.as_millis(),
            "query": query_string
        }))
    });
}
```

### 4.2 For CLI Implementers (Rust)

**Map JSON-RPC errors to exit codes:**

```rust
fn handle_rpc_error(error: &JsonRpcError) -> i32 {
    match error.code {
        -32001 => {
            eprintln!("Error: Index not found");
            if let Some(data) = &error.data {
                if let Some(suggestion) = data.get("suggestion") {
                    eprintln!("Suggestion: {}", suggestion);
                }
            }
            3 // Exit code 3: Index not found
        }
        -32002 => {
            eprintln!("Error: Entity not found");
            1 // Exit code 1: No results
        }
        -32602 => {
            eprintln!("Error: Invalid arguments - {}", error.message);
            2 // Exit code 2: Invalid arguments
        }
        _ => {
            eprintln!("Service error: {} (code: {})", error.message, error.code);
            4 // Exit code 4: Service error
        }
    }
}
```

### 4.3 For Agent Implementers (TypeScript)

**Graceful error recovery:**

```typescript
import { JsonRpcError, ErrorCode } from './types';

async function searchWithRetry(query: string, maxRetries = 3): Promise<Entity[]> {
  for (let attempt = 0; attempt < maxRetries; attempt++) {
    try {
      const result = await rpcClient.call('search_entities', { query });
      return result.entities;
    } catch (error) {
      if (error instanceof JsonRpcError) {
        switch (error.code) {
          case ErrorCode.IndexNotFound:
            // Auto-rebuild index
            console.log('Index not found, rebuilding...');
            await rpcClient.call('rebuild_index', { repo_path: './repo' });
            continue; // Retry after rebuild

          case ErrorCode.EntityNotFound:
            return []; // Return empty results

          case ErrorCode.QueryTimeout:
            // Reduce query scope
            console.warn('Query timeout, narrowing search...');
            query = query.split(' ')[0]; // Use first keyword only
            continue;

          default:
            throw error; // Unhandled error
        }
      }
    }
  }
  throw new Error(`Failed after ${maxRetries} attempts`);
}
```

---

## 5. Error Testing Strategy

### 5.1 Unit Tests (Rust)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_not_found_error() {
        let result = search_entities("/nonexistent/index", "query");
        assert_eq!(result.unwrap_err().code, -32001);
    }

    #[test]
    fn test_entity_not_found_error() {
        let result = retrieve_entity("nonexistent_id");
        assert_eq!(result.unwrap_err().code, -32002);
    }
}
```

### 5.2 Integration Tests

Create test fixtures in `tests/fixtures/error-scenarios/`:

- `empty_index/` - Trigger -32001
- `malformed_python/` - Trigger -32003
- `slow_query.json` - Trigger -32004

---

## 6. Versioning and Evolution

### 6.1 Adding New Error Codes

When adding new error codes in future versions:

1. Reserve codes in range **-32005 to -32099** for CDSAgent custom errors
2. Document in this file with version annotation
3. Update JSON schema in `docs/api/jsonrpc-schema.json`
4. Add test cases
5. Update client libraries (TypeScript types)

**Example (v0.2.0):**

```markdown
| **-32005** | Concurrent modification | Index modified during query | Added in v0.2.0 |
```

### 6.2 Deprecation Policy

If an error code needs to be deprecated:

1. Mark as deprecated in this document
2. Keep returning it for 2 minor versions
3. Add `deprecated: true` field in error data:

```json
{
  "code": -32003,
  "message": "Parse error",
  "data": {
    "deprecated": true,
    "deprecated_in": "v0.3.0",
    "use_instead": "Use code -32005 for parse errors"
  }
}
```

---

## 7. Related Documents

- [JSON-RPC Schema](./jsonrpc-schema.json) - Full API schema
- [PRD-05: API Specifications](../../spacs/prd/0.1.0-MVP-PRDs-v0/05-api-specifications.md) - Requirements
- [Issue-05: API Contracts](../../spacs/issues/04-0.1.0-mvp/05-api-contracts.md) - Implementation details

---

## 8. FAQ

### Q: Why use negative error codes?

**A:** JSON-RPC 2.0 specification reserves negative codes for errors. Positive codes are for application-specific non-error status.

### Q: Can I add custom data to standard errors (-32600 to -32603)?

**A:** Yes, the `data` field is optional and can contain additional context.

### Q: What if I need to return multiple errors?

**A:** JSON-RPC 2.0 only supports one error per response. Use the `data.errors` array to list multiple issues:

```json
{
  "code": -32003,
  "message": "Parse error",
  "data": {
    "errors": [
      { "file": "a.py", "line": 10, "error": "..." },
      { "file": "b.py", "line": 25, "error": "..." }
    ]
  }
}
```

---

**Status:** Ready for implementation. Requires validation in service contract tests.
