# CDSAgent API Versioning Strategy

**Version:** 0.1.0
**Last Updated:** 2025-10-19
**Status:** Draft

---

## Overview

This document defines the versioning strategy for CDSAgent APIs, ensuring backward compatibility and smooth evolution of the JSON-RPC interface.

---

## 1. Versioning Scheme

### 1.1 Semantic Versioning

CDSAgent follows **Semantic Versioning 2.0.0** <https://semver.org/>:

```text
MAJOR.MINOR.PATCH

Example: 0.1.0 → 0.2.0 → 1.0.0
```

**Version Components:**

- **MAJOR**: Incompatible API changes (breaking changes)
- **MINOR**: Backward-compatible functionality additions
- **PATCH**: Backward-compatible bug fixes

### 1.2 Pre-1.0 Development Phase

During **v0.x.x** (pre-production):

- API is **unstable** and may change without notice
- Minor version bumps (0.1 → 0.2) **may** include breaking changes
- Production deployments should pin exact versions

**Current Version:** `0.1.0` (MVP release)

---

## 2. API Version Identification

### 2.1 Schema Version

JSON-RPC schema includes version in `$id`:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://github.com/lwyBZss8924d/CDSAgent/schemas/jsonrpc-v0.1.0",
  "version": "0.1.0"
}
```

### 2.2 HTTP Response Header

All JSON-RPC responses include API version header:

```http
HTTP/1.1 200 OK
Content-Type: application/json
X-CDS-API-Version: 0.1.0

{
  "jsonrpc": "2.0",
  "id": 1,
  "result": { ... }
}
```

### 2.3 URL-Based Versioning (Future)

For **v1.0+**, URL-based versioning will be introduced:

```text
http://localhost:9876/v1/rpc  # Current stable version
http://localhost:9876/v2/rpc  # Future breaking changes
```

**v0.1.0 Decision:** Use single `/rpc` endpoint without version prefix.

---

## 3. Backward Compatibility Rules

### 3.1 Backward-Compatible Changes (MINOR/PATCH)

These changes **do not** break existing clients:

✅ **Allowed:**

- Adding new optional parameters
- Adding new fields to response objects
- Adding new error codes
- Adding new JSON-RPC methods
- Performance improvements
- Bug fixes in existing behavior

**Example (0.1.0 → 0.2.0):**

```typescript
// v0.1.0
interface SearchParams {
  query: string;
  limit?: number;
}

// v0.2.0 (backward-compatible)
interface SearchParams {
  query: string;
  limit?: number;
  fuzzy?: boolean;          // NEW optional parameter
  highlight?: boolean;      // NEW optional parameter
}
```

### 3.2 Breaking Changes (MAJOR)

These changes **require** major version bump:

❌ **Not Allowed in MINOR/PATCH:**

- Removing or renaming existing parameters
- Changing parameter types
- Removing fields from responses
- Changing response structure
- Removing JSON-RPC methods
- Changing error code meanings

**Example (Requires 1.0.0 → 2.0.0):**

```typescript
// v1.0.0
interface Entity {
  id: string;
  name: string;
  entity_type: string;  // string enum
}

// v2.0.0 (breaking change)
interface Entity {
  id: string;
  name: string;
  entity_type: EntityType;  // Changed to enum type
  kind: string;             // NEW required field (breaking!)
}
```

---

## 4. Deprecation Policy

### 4.1 Deprecation Workflow

1. **Announcement** (N.0.0): Mark feature as deprecated in documentation
2. **Grace Period** (N+1.0.0): Feature still works, warnings in logs
3. **Removal** (N+2.0.0): Feature removed in next major version

**Minimum grace period:** 2 minor versions or 6 months (whichever is longer)

### 4.2 Deprecation Markers

**In Documentation:**

```markdown
## 3.1 SearchEntities

**DEPRECATED** as of v0.3.0. Use `search_entities_v2` instead.
This method will be removed in v1.0.0.
```

**In Response (Optional Warning):**

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": { ... },
  "warning": {
    "code": "DEPRECATED_METHOD",
    "message": "Method 'search_entities' deprecated as of v0.3.0",
    "use_instead": "search_entities_v2",
    "removal_version": "1.0.0"
  }
}
```

**In Code Comments:**

```rust
/// Search entities using upper and lower indices
///
/// # Deprecated
/// **Deprecated since v0.3.0**: Use `search_entities_v2` instead.
/// This method will be removed in v1.0.0.
#[deprecated(since = "0.3.0", note = "use `search_entities_v2` instead")]
pub fn search_entities(&self, params: SearchParams) -> Result<SearchResult> {
    // ...
}
```

---

## 5. Version Migration Guides

### 5.1 Breaking Change Migration (Example)

When releasing **v1.0.0** with breaking changes:

**Create Migration Guide:**

```markdown
# Migration Guide: v0.9.x → v1.0.0

## Breaking Changes

### 1. `entity_type` Field Type Change

**Before (v0.9.x):**
```json
{
  "entity_type": "function"  // lowercase string
}
```

**After (v1.0.0):**

```json
{
  "entity_type": "Function"  // PascalCase enum
}
```

**Migration:**

```typescript
// Update your code:
const entityType = response.entity_type.toLowerCase();
```

### 2. Removed `snippet.fold` Field

**Before (v0.9.x):**

```json
{
  "snippet": {
    "fold": "...",
    "preview": "...",
    "full": "..."
  }
}
```

**After (v1.0.0):**

```json
{
  "snippet": {
    "preview": "...",  // fold removed
    "full": "..."
  }
}
```

**Migration:**
Use `snippet.preview` instead of `snippet.fold`.

---

## 6. Version Support Policy

### 6.1 Support Windows

| Version Type | Support Duration | Updates |
|--------------|------------------|---------|
| **Current Stable** (e.g., v1.2.x) | Until next major | Bug fixes, security patches |
| **Previous Major** (e.g., v0.9.x) | 6 months after new major | Critical security fixes only |
| **Older Versions** | Unsupported | No updates |

**Example Timeline:**

```text
v0.1.0 ────────> v0.5.0 ────────> v1.0.0 ────────> v2.0.0
  │                │                │                │
  │                │                │ (6 months)     │
  │                │                v0.x unsupported │
  │                │                                 │
  │                │                                 v1.x unsupported
  │                │                                 (6 months after v2.0)
  │                │
  v0.1-0.4 unsupported after v0.5
```

### 6.2 Long-Term Support (LTS) - Future

Starting with **v1.0.0**, LTS versions will be released:

- **LTS releases**: Every 2 major versions (v1.0, v3.0, v5.0)
- **LTS support**: 18 months of bug fixes and security patches
- **Non-LTS support**: 6 months after next major release

---

## 7. Client Version Negotiation (Future)

### 7.1 Client Version Header (v1.0+)

Clients can specify their expected API version:

```http
POST /rpc HTTP/1.1
Host: localhost:9876
Content-Type: application/json
X-CDS-Client-API-Version: 1.2.0

{
  "jsonrpc": "2.0",
  "method": "search_entities",
  "params": { ... },
  "id": 1
}
```

**Server Behavior:**

- If `X-CDS-Client-API-Version` matches, proceed normally
- If version mismatch:
  - **Minor version behind**: Return compatibility warning
  - **Major version mismatch**: Return `-32600` error with upgrade message

### 7.2 Feature Detection (Alternative)

Instead of version checking, clients can detect feature availability:

```typescript
// Check if method exists
const response = await rpcClient.call('rpc.discover');
if (response.methods.includes('search_entities_v2')) {
  // Use new method
} else {
  // Fallback to old method
}
```

---

## 8. Schema Evolution Examples

### 8.1 Adding Optional Parameter (v0.1 → v0.2)

**v0.1.0:**

```json
{
  "method": "search_entities",
  "params": {
    "query": "sanitize",
    "limit": 10
  }
}
```

**v0.2.0 (Backward-Compatible):**

```json
{
  "method": "search_entities",
  "params": {
    "query": "sanitize",
    "limit": 10,
    "fuzzy": true  // NEW optional parameter (old clients ignore it)
  }
}
```

**Old clients (v0.1) continue to work** because:

- They don't send `fuzzy` parameter (defaults to `false`)
- Server still accepts requests without `fuzzy`

### 8.2 Adding Response Field (v0.1 → v0.2)

**v0.1.0 Response:**

```json
{
  "result": {
    "entities": [...],
    "total_count": 10
  }
}
```

**v0.2.0 Response (Backward-Compatible):**

```json
{
  "result": {
    "entities": [...],
    "total_count": 10,
    "execution_time_ms": 120  // NEW field (old clients ignore it)
  }
}
```

**Old clients (v0.1) continue to work** because:

- They only read `entities` and `total_count`
- Extra fields are ignored

### 8.3 Breaking Change Example (v0.9 → v1.0)

**v0.9.0 (Old):**

```typescript
interface Entity {
  line_range: [number, number];  // Tuple [start, end]
}
```

**v1.0.0 (Breaking):**

```typescript
interface Entity {
  line_range: {  // Changed to object (BREAKING!)
    start: number;
    end: number;
  };
}
```

**This requires:**

- Major version bump (v0.9 → v1.0)
- Migration guide
- Deprecation warnings in v0.9.x

---

## 9. Testing Version Compatibility

### 9.1 Compatibility Test Matrix

For each new minor version, test against clients using previous versions:

```text
┌─────────────┬────────┬────────┬────────┐
│ Client API  │ v0.1.0 │ v0.2.0 │ v0.3.0 │
├─────────────┼────────┼────────┼────────┤
│ v0.1.0      │   ✓    │   ✓    │   ✓    │
│ v0.2.0      │   ✗    │   ✓    │   ✓    │
│ v0.3.0      │   ✗    │   ✗    │   ✓    │
└─────────────┴────────┴────────┴────────┘

✓ = Backward compatible (newer server, older client)
✗ = Not supported (older server, newer client)
```

### 9.2 Automated Compatibility Tests

```rust
#[cfg(test)]
mod compatibility_tests {
    use super::*;

    #[test]
    fn test_v0_1_client_on_v0_2_server() {
        // Simulate v0.1 client request (no 'fuzzy' parameter)
        let request = json!({
            "jsonrpc": "2.0",
            "method": "search_entities",
            "params": {
                "query": "test",
                "limit": 10
            },
            "id": 1
        });

        let response = handle_request(request);
        assert!(response.is_ok());
        // v0.2 server should accept v0.1 request
    }
}
```

---

## 10. Version History

### v0.1.0 (2025-10-19) - Initial Release

**Added:**

- JSON-RPC 2.0 protocol
- 4 core methods: `search_entities`, `traverse_graph`, `retrieve_entity`, `rebuild_index`
- Error codes: -32001 to -32004
- HTTP endpoint: `http://localhost:9876/rpc`

**Notes:**

- Pre-1.0 release, API may change
- No URL-based versioning yet
- No deprecation policy enforcement

---

## 11. Future Considerations

### 11.1 GraphQL Migration (v2.0+)

If JSON-RPC proves limiting, consider GraphQL:

```graphql
type Query {
  searchEntities(query: String!, limit: Int): [Entity!]!
  traverseGraph(startEntities: [ID!]!, depth: Int): Subgraph!
  retrieveEntity(id: ID!): EntityDetails
}
```

### 11.2 gRPC Support (v1.1+)

Add gRPC endpoint alongside JSON-RPC:

```protobuf
service CDSIndex {
  rpc SearchEntities(SearchRequest) returns (SearchResponse);
  rpc TraverseGraph(TraverseRequest) returns (TraverseResponse);
  rpc RetrieveEntity(RetrieveRequest) returns (RetrieveResponse);
}
```

---

## 12. Related Documents

- [JSON-RPC Schema](./jsonrpc-schema.json) - Full API schema
- [Error Codes](./error-codes.md) - Error code catalogue
- [PRD-05: API Specifications](../../spacs/prd/0.1.0-MVP-PRDs-v0/05-api-specifications.md) - Requirements

---

**Status:** Ready for implementation. Requires review and approval before v0.1.0 release.
