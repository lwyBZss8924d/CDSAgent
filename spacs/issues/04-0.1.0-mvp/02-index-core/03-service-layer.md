# Sub-Issue 02.03: Service Layer - JSON-RPC HTTP Server

**Priority**: P1 (Critical Path - Integration)
**Status**: ☐ Not Started
**Owner**: Rust Dev 3
**Parent**: [02-index-core/00-overview.md](00-overview.md)
**PRD Reference**: [PRD-02 §4.1](../../../prd/0.1.0-MVP-PRDs-v0/02-cds-index-service.md), [PRD-05 §2-3](../../../prd/0.1.0-MVP-PRDs-v0/05-api-specifications.md)
**Timing**: Phase 2, Week 4-5

---

## Objective

Implement a lightweight JSON-RPC 2.0 HTTP server (`cds-indexd`) that exposes graph indexing and search APIs to CDS-Tools CLI and TypeScript agent layer, maintaining in-memory index for low-latency queries.

## Scope

**In Scope**:

- JSON-RPC 2.0 HTTP server (axum framework)
- Core methods: `search_entities`, `traverse_graph`, `retrieve_entity`
- Health and metrics endpoints
- Coordinate with [05-api-contracts.md](../05-api-contracts.md) for schemas
- v0.1.0: Local-only deployment (single-user daemon)

**Out of Scope (v0.2.0)**:

- gRPC service (module scaffolded but marked experimental)
- Multi-user authentication/authorization
- Distributed index deployment
- WebSocket streaming responses

---

## Dependencies

- **Requires**: [01-graph-build.md](01-graph-build.md) (graph data structures), [02-sparse-index.md](02-sparse-index.md) (search APIs)
- **Blocks**: [03-cli-tools/01-command-impl.md](../../03-cli-tools/01-command-impl.md) (CLI needs service), [04-agent-integration/01-sdk-bootstrap.md](../../04-agent-integration/01-sdk-bootstrap.md) (agent needs service)
- **Coordinates With**: [05-api-contracts.md](../05-api-contracts.md)

---

## Implementation Tasks

### Week 4, Day 1-2: JSON-RPC Server Bootstrap

Task 1: Server Framework Setup

```rust
// cds_service/src/main.rs
use axum::{
    Router,
    routing::{get, post},
    extract::State,
    http::StatusCode,
};
use std::sync::Arc;
use tokio::net::TcpListener;

mod jsonrpc;
mod handlers;
mod state;

use state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = load_config()?;

    // Initialize graph and indices
    let graph = load_or_build_graph(&config.repo_path)?;
    let searcher = build_searcher(&graph)?;

    let state = Arc::new(AppState { graph, searcher, config });

    // Build router
    let app = Router::new()
        .route("/", post(handlers::jsonrpc_handler))
        .route("/health", get(handlers::health_check))
        .route("/metrics", get(handlers::metrics))
        .with_state(state);

    // Start server
    let addr = format!("{}:{}", config.host, config.port);
    println!("CDS-Index Service listening on {}", addr);

    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
```

```rust
// cds_service/src/state.rs
use crate::graph::CodeGraph;
use crate::search::HierarchicalSearcher;

pub struct AppState {
    pub graph: CodeGraph,
    pub searcher: HierarchicalSearcher<'static>,
    pub config: ServiceConfig,
}

pub struct ServiceConfig {
    pub repo_path: String,
    pub host: String,
    pub port: u16,
    pub max_results: usize,
}

fn load_config() -> Result<ServiceConfig, Box<dyn std::error::Error>> {
    Ok(ServiceConfig {
        repo_path: std::env::var("CDS_REPO_PATH").unwrap_or_else(|_| ".".to_string()),
        host: std::env::var("CDS_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        port: std::env::var("CDS_PORT")
            .unwrap_or_else(|_| "9001".to_string())
            .parse()?,
        max_results: 100,
    })
}
```

**Acceptance**:

- [ ] Server starts on configurable host:port (default: 127.0.0.1:9001)
- [ ] Loads graph from `CDS_REPO_PATH` or current directory
- [ ] Health endpoint returns 200 OK
- [ ] Server startup time <2s

---

Task 2: JSON-RPC Handler

```rust
// cds_service/src/jsonrpc.rs
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Value>,
    pub id: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    pub id: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

// Error codes (from PRD-05 §4)
pub const PARSE_ERROR: i32 = -32700;
pub const INVALID_REQUEST: i32 = -32600;
pub const METHOD_NOT_FOUND: i32 = -32601;
pub const INVALID_PARAMS: i32 = -32602;
pub const INTERNAL_ERROR: i32 = -32603;
pub const INDEX_NOT_READY: i32 = -32001;
pub const ENTITY_NOT_FOUND: i32 = -32002;
pub const TRAVERSAL_ERROR: i32 = -32003;
pub const SEARCH_ERROR: i32 = -32004;

impl JsonRpcResponse {
    pub fn success(result: Value, id: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: Some(result),
            error: None,
            id,
        }
    }

    pub fn error(code: i32, message: String, id: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JsonRpcError {
                code,
                message,
                data: None,
            }),
            id,
        }
    }
}
```

```rust
// cds_service/src/handlers.rs
use axum::{extract::State, Json};
use std::sync::Arc;
use serde_json::{json, Value};

use crate::jsonrpc::{JsonRpcRequest, JsonRpcResponse, METHOD_NOT_FOUND};
use crate::state::AppState;
use crate::methods;

pub async fn jsonrpc_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<JsonRpcRequest>,
) -> Json<JsonRpcResponse> {
    if req.jsonrpc != "2.0" {
        return Json(JsonRpcResponse::error(
            -32600,
            "Invalid Request: jsonrpc must be '2.0'".to_string(),
            req.id,
        ));
    }

    let result = match req.method.as_str() {
        "search_entities" => methods::search_entities(state, req.params).await,
        "traverse_graph" => methods::traverse_graph(state, req.params).await,
        "retrieve_entity" => methods::retrieve_entity(state, req.params).await,
        _ => Err(JsonRpcResponse::error(
            METHOD_NOT_FOUND,
            format!("Method '{}' not found", req.method),
            req.id.clone(),
        )),
    };

    Json(match result {
        Ok(value) => JsonRpcResponse::success(value, req.id),
        Err(err_response) => err_response,
    })
}

pub async fn health_check() -> Json<Value> {
    Json(json!({ "status": "ok", "service": "cds-indexd" }))
}

pub async fn metrics() -> Json<Value> {
    // TODO: Implement metrics collection
    Json(json!({
        "uptime_seconds": 0,
        "total_requests": 0,
        "graph_node_count": 0,
    }))
}
```

**Acceptance**:

- [ ] JSON-RPC 2.0 spec compliance (jsonrpc="2.0", id field handling)
- [ ] Returns standard error codes (see PRD-05 §4)
- [ ] Handles missing method with METHOD_NOT_FOUND error
- [ ] Logs requests and errors

---

### Week 4, Day 3-4: Core API Methods

Task 3: SearchEntities Method

```rust
// cds_service/src/methods.rs
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::state::AppState;
use crate::jsonrpc::{JsonRpcResponse, INVALID_PARAMS, SEARCH_ERROR};

#[derive(Deserialize)]
pub struct SearchEntitiesParams {
    pub query: String,
    #[serde(default)]
    pub entity_type: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize { 10 }

#[derive(Serialize)]
pub struct SearchResult {
    pub results: Vec<EntityResult>,
}

#[derive(Serialize)]
pub struct EntityResult {
    pub entity_id: String,
    pub name: String,
    pub entity_type: String,
    pub file_path: String,
    pub line_range: (usize, usize),
    pub score: f32,
    pub snippet: SnippetResult,
}

#[derive(Serialize)]
pub struct SnippetResult {
    pub fold: String,
    pub preview: String,
    pub full: String,
}

pub async fn search_entities(
    state: Arc<AppState>,
    params: Option<Value>,
) -> Result<Value, JsonRpcResponse> {
    let params: SearchEntitiesParams = serde_json::from_value(params.unwrap_or(json!({})))
        .map_err(|e| JsonRpcResponse::error(
            INVALID_PARAMS,
            format!("Invalid params: {}", e),
            None,
        ))?;

    let entity_type = params.entity_type.as_ref().and_then(|s| parse_entity_type(s));

    let matches = state.searcher.search(&params.query, SearchOptions {
        limit: params.limit,
        entity_type,
        ..Default::default()
    });

    let results: Vec<EntityResult> = matches
        .into_iter()
        .filter_map(|m| {
            state.graph.nodes.get(&m.entity_id).map(|node| {
                let snippet = format_snippet(&state.graph, &m.entity_id);
                EntityResult {
                    entity_id: m.entity_id.clone(),
                    name: node.name.clone(),
                    entity_type: format!("{:?}", node.entity_type),
                    file_path: node.file_path.clone().unwrap_or_default().display().to_string(),
                    line_range: node.line_range.unwrap_or((0, 0)),
                    score: m.score,
                    snippet,
                }
            })
        })
        .collect();

    Ok(json!(SearchResult { results }))
}
```

**Acceptance** (from PRD-05 §2.1):

- [ ] Accepts `query`, `entity_type`, `limit` parameters
- [ ] Returns results with entity metadata + snippets (fold/preview/full)
- [ ] Validates parameters and returns INVALID_PARAMS on error
- [ ] Respects max_results limit from config

---

Task 4: TraverseGraph Method

```rust
// cds_service/src/methods.rs (continued)
#[derive(Deserialize)]
pub struct TraverseGraphParams {
    pub start_id: String,
    #[serde(default)]
    pub relation_types: Option<Vec<String>>,
    #[serde(default = "default_depth")]
    pub max_depth: usize,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_depth() -> usize { 2 }

#[derive(Serialize)]
pub struct TraverseResult {
    pub tree: String,
    pub entities: Vec<EntityResult>,
}

pub async fn traverse_graph(
    state: Arc<AppState>,
    params: Option<Value>,
) -> Result<Value, JsonRpcResponse> {
    let params: TraverseGraphParams = serde_json::from_value(params.unwrap_or(json!({})))
        .map_err(|e| JsonRpcResponse::error(INVALID_PARAMS, format!("Invalid params: {}", e), None))?;

    // Validate start entity exists
    if !state.graph.nodes.contains_key(&params.start_id) {
        return Err(JsonRpcResponse::error(
            ENTITY_NOT_FOUND,
            format!("Entity '{}' not found", params.start_id),
            None,
        ));
    }

    let relation_types = params.relation_types.as_ref().map(|types| {
        types.iter().filter_map(|s| parse_relation_type(s)).collect()
    });

    let traversal_result = traverse_bfs(
        &state.graph,
        &params.start_id,
        relation_types,
        params.max_depth,
        params.limit,
    );

    let tree = format_tree(&traversal_result);
    let entities = format_entities(&state.graph, &traversal_result.visited);

    Ok(json!(TraverseResult { tree, entities }))
}
```

**Acceptance** (from PRD-05 §2.2):

- [ ] Accepts `start_id`, `relation_types`, `max_depth`, `limit` parameters
- [ ] Returns tree-formatted output + entity list
- [ ] Validates start entity exists (ENTITY_NOT_FOUND if missing)
- [ ] Respects depth and limit constraints

---

Task 5: RetrieveEntity Method

```rust
// cds_service/src/methods.rs (continued)
#[derive(Deserialize)]
pub struct RetrieveEntityParams {
    pub entity_id: String,
    #[serde(default = "default_snippet_mode")]
    pub snippet_mode: String,
}

fn default_snippet_mode() -> String { "full".to_string() }

pub async fn retrieve_entity(
    state: Arc<AppState>,
    params: Option<Value>,
) -> Result<Value, JsonRpcResponse> {
    let params: RetrieveEntityParams = serde_json::from_value(params.unwrap_or(json!({})))
        .map_err(|e| JsonRpcResponse::error(INVALID_PARAMS, format!("Invalid params: {}", e), None))?;

    let node = state.graph.nodes.get(&params.entity_id).ok_or_else(|| {
        JsonRpcResponse::error(
            ENTITY_NOT_FOUND,
            format!("Entity '{}' not found", params.entity_id),
            None,
        )
    })?;

    let snippet = format_snippet(&state.graph, &params.entity_id);

    Ok(json!({
        "entity_id": params.entity_id,
        "name": node.name,
        "entity_type": format!("{:?}", node.entity_type),
        "file_path": node.file_path.as_ref().map(|p| p.display().to_string()),
        "line_range": node.line_range,
        "snippet": snippet,
    }))
}
```

**Acceptance** (from PRD-05 §2.3):

- [ ] Accepts `entity_id`, `snippet_mode` parameters
- [ ] Returns full entity metadata + code snippet
- [ ] Returns ENTITY_NOT_FOUND if entity doesn't exist

---

### Week 5: Testing & Integration

Task 6: Contract Tests

```rust
// cds_service/tests/contract_tests.rs
use reqwest::Client;
use serde_json::json;

#[tokio::test]
async fn test_search_entities_contract() {
    let client = Client::new();
    let response = client
        .post("http://127.0.0.1:9001/")
        .json(&json!({
            "jsonrpc": "2.0",
            "method": "search_entities",
            "params": {
                "query": "authenticate",
                "limit": 5
            },
            "id": 1
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["jsonrpc"], "2.0");
    assert!(body["result"].is_object());
    assert!(body["result"]["results"].is_array());
}

#[tokio::test]
async fn test_method_not_found() {
    let client = Client::new();
    let response = client
        .post("http://127.0.0.1:9001/")
        .json(&json!({
            "jsonrpc": "2.0",
            "method": "nonexistent_method",
            "id": 1
        }))
        .send()
        .await
        .unwrap();

    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["error"]["code"], -32601);
}
```

**Acceptance**:

- [ ] All contract tests pass (see [05-api-contracts.md](../05-api-contracts.md))
- [ ] CLI can call service via HTTP (integration test)
- [ ] TypeScript agent can call service (integration test)

---

## Acceptance Criteria (from PRD-02 §8, PRD-05 §8)

### Must-Pass

- [ ] JSON-RPC service implements all 3 core methods
- [ ] Passes all API contract tests (see [05-api-contracts.md](../05-api-contracts.md))
- [ ] CLI can call service via HTTP and receive valid responses
- [ ] TypeScript agent can call service (bash tool in v0.1.0)
- [ ] Service startup time <2s
- [ ] Request latency <100ms p95 (excluding search/traversal time)
- [ ] Unit test coverage >85% for `cds_service` crate

### Configuration

- [ ] Configurable via environment variables (CDS_HOST, CDS_PORT, CDS_REPO_PATH)
- [ ] Default: 127.0.0.1:9001
- [ ] Logs to stdout/stderr

---

## Testing Strategy

### Unit Tests

- [ ] JSON-RPC request parsing and validation
- [ ] Method routing and error handling
- [ ] Parameter validation for each method

### Integration Tests

- [ ] Start service, call each method via HTTP
- [ ] Validate response schemas match PRD-05
- [ ] Test error scenarios (missing entity, invalid params)

### Performance Tests

- [ ] Measure request latency (target: <100ms p95)
- [ ] Test concurrent requests (100 parallel clients)

---

## Open Questions & Risks

### Q1: gRPC Scaffolding

**Question**: Should we scaffold gRPC module now or defer to v0.2.0?
**Decision**: Scaffold basic structure (see PRD-02 §4.1 "Service Interface Strategy"), mark experimental
**Rationale**: JSON-RPC sufficient for v0.1.0 parity with LocAgent's in-process model

### Q2: Index Reloading

**Risk**: If codebase changes, index becomes stale
**Mitigation**: v0.1.0 requires manual restart; v0.2.0 adds incremental updates
**Tracking**: [10-extensibility.md](../10-extensibility.md) v0.2.0 backlog

### Q3: Multi-User Concurrency

**Risk**: Multiple CLI invocations may conflict
**Mitigation**: v0.1.0 is single-user (local daemon); v0.2.0 adds proper locking
**Escalation**: If contention occurs, implement RwLock around state

---

## Related Issues

- **Parent**: [00-overview.md](00-overview.md)
- **Depends On**: [01-graph-build.md](01-graph-build.md), [02-sparse-index.md](02-sparse-index.md)
- **Blocks**: [03-cli-tools/01-command-impl.md](../../03-cli-tools/01-command-impl.md), [04-agent-integration/01-sdk-bootstrap.md](../../04-agent-integration/01-sdk-bootstrap.md)
- **Coordinates With**: [05-api-contracts.md](../05-api-contracts.md)
- **Tests**: [../08-testing/02-integration.md](../08-testing/02-integration.md)

---

## Next Steps

1. [ ] Review PRD-05 API specifications for final schema alignment
2. [ ] Implement axum server bootstrap (Week 4, Day 1)
3. [ ] Implement JSON-RPC handler and routing (Week 4, Day 2)
4. [ ] Implement all 3 core methods (Week 4, Day 3-4)
5. [ ] Write contract tests and validate with CLI (Week 5)
6. [ ] Deploy as local daemon and test agent integration

---

**Status Updates**:

- *2025-10-19*: Sub-issue created, awaiting graph and search completion
