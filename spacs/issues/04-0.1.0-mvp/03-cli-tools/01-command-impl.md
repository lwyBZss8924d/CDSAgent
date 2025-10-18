# Sub-Issue 03.01: Command Implementation - Search, Traverse, Retrieve

**Priority**: P1 (Critical Path - CLI Core)
**Status**: ☐ Not Started
**Owner**: Rust Dev 2
**Parent**: [03-cli-tools/00-overview.md](00-overview.md)
**PRD Reference**: [PRD-03 §2.1-2.2](../../../prd/0.1.0-MVP-PRDs-v0/03-cds-tools-cli.md)
**Timing**: Phase 2, Week 3-4

---

## Objective

Implement core CLI commands (`search`, `traverse`, `retrieve`, `init`, `config`) that call CDS-Index Service JSON-RPC APIs and provide Unix-style composable interfaces.

## Scope

**In Scope**:

- `cds search`: Hierarchical entity search with type filtering
- `cds traverse`: Graph BFS navigation with relation/type filters
- `cds retrieve`: Full entity code retrieval
- `cds init`: Index building (calls CDS-Index Service)
- `cds config`: Configuration management (get/set/list)
- JSON-RPC client for CDS-Index Service
- Exit codes and error handling

**Out of Scope (v0.2.0)**:

- `cds combo`: YAML plan execution (developer-only in v0.1.0, full integration v0.2.0)
- Shell completions
- Interactive REPL mode

---

## Dependencies

- **Requires**: [02-index-core/03-service-layer.md](../02-index-core/03-service-layer.md) (JSON-RPC service running)
- **Blocks**: [03-integration-tests.md](03-integration-tests.md)
- **Coordinates With**: [05-api-contracts.md](../05-api-contracts.md)

---

## Implementation Tasks

### Week 3, Day 1-2: CLI Framework & RPC Client

Task 1: Clap CLI Setup

```rust
// cds-cli/src/main.rs
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cds")]
#[command(about = "Code Discovery System - Unified code retrieval interface")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Config file path (overrides default: ~/.config/cds/config.toml)
    #[arg(long, env = "CDS_CONFIG")]
    config: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for code entities using keywords
    Search(SearchArgs),
    /// Traverse code graph via BFS
    Traverse(TraverseArgs),
    /// Retrieve full code and metadata for entities
    Retrieve(RetrieveArgs),
    /// Initialize or rebuild index for a repository
    Init(InitArgs),
    /// Manage configuration settings
    Config(ConfigArgs),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Load configuration
    let config = load_config(&cli.config)?;

    // Route to subcommand
    match cli.command {
        Commands::Search(args) => commands::search::run(args, &config).await?,
        Commands::Traverse(args) => commands::traverse::run(args, &config).await?,
        Commands::Retrieve(args) => commands::retrieve::run(args, &config).await?,
        Commands::Init(args) => commands::init::run(args, &config).await?,
        Commands::Config(args) => commands::config::run(args, &config).await?,
    }

    Ok(())
}
```

**Acceptance**:

- [ ] CLI parses all commands and arguments
- [ ] `--help` works for root and subcommands
- [ ] Version flag works
- [ ] Config file and env vars load correctly

---

Task 2: JSON-RPC Client

```rust
// cds-cli/src/client.rs
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use reqwest::Client;

pub struct CdsIndexClient {
    base_url: String,
    client: Client,
}

#[derive(Serialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Value,
    id: u64,
}

#[derive(Deserialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    result: Option<Value>,
    error: Option<JsonRpcError>,
    id: u64,
}

#[derive(Deserialize)]
struct JsonRpcError {
    code: i32,
    message: String,
}

impl CdsIndexClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: Client::new(),
        }
    }

    pub async fn call(&self, method: &str, params: Value) -> Result<Value, ClientError> {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
            id: 1,
        };

        let response = self.client
            .post(&self.base_url)
            .json(&req)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ClientError::HttpError(response.status()));
        }

        let rpc_response: JsonRpcResponse = response.json().await?;

        if let Some(error) = rpc_response.error {
            return Err(ClientError::RpcError(error.code, error.message));
        }

        rpc_response.result.ok_or_else(|| ClientError::MissingResult)
    }

    pub async fn search_entities(
        &self,
        query: &str,
        entity_type: Option<String>,
        limit: usize,
    ) -> Result<SearchResult, ClientError> {
        let params = json!({
            "query": query,
            "entity_type": entity_type,
            "limit": limit
        });

        let result = self.call("search_entities", params).await?;
        Ok(serde_json::from_value(result)?)
    }

    pub async fn traverse_graph(
        &self,
        start_id: &str,
        relation_types: Option<Vec<String>>,
        max_depth: usize,
        limit: usize,
    ) -> Result<TraverseResult, ClientError> {
        let params = json!({
            "start_id": start_id,
            "relation_types": relation_types,
            "max_depth": max_depth,
            "limit": limit
        });

        let result = self.call("traverse_graph", params).await?;
        Ok(serde_json::from_value(result)?)
    }

    pub async fn retrieve_entity(
        &self,
        entity_id: &str,
        snippet_mode: &str,
    ) -> Result<EntityResult, ClientError> {
        let params = json!({
            "entity_id": entity_id,
            "snippet_mode": snippet_mode
        });

        let result = self.call("retrieve_entity", params).await?;
        Ok(serde_json::from_value(result)?)
    }
}

#[derive(Debug)]
pub enum ClientError {
    HttpError(reqwest::StatusCode),
    RpcError(i32, String),
    MissingResult,
    SerdeError(serde_json::Error),
    RequestError(reqwest::Error),
}

impl From<reqwest::Error> for ClientError {
    fn from(err: reqwest::Error) -> Self {
        ClientError::RequestError(err)
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(err: serde_json::Error) -> Self {
        ClientError::SerdeError(err)
    }
}
```

**Acceptance**:

- [ ] Client can call JSON-RPC methods
- [ ] Error handling for HTTP and RPC errors
- [ ] Type-safe API (deserialize to structs)
- [ ] Configurable base URL

---

### Week 3, Day 3-4: Core Commands

**Task 3: `cds search` Command**

```rust
// cds-cli/src/commands/search.rs
use clap::Args;
use crate::client::CdsIndexClient;
use crate::output::formatter;
use crate::config::Config;

#[derive(Args)]
pub struct SearchArgs {
    /// Search query (keywords, supports phrases)
    query: String,

    /// Filter by entity type
    #[arg(short = 't', long)]
    entity_type: Option<String>,

    /// Maximum results to return
    #[arg(short = 'l', long, default_value = "10")]
    limit: usize,

    /// Output format: json|text|fold|preview|full
    #[arg(short = 'f', long, default_value = "json")]
    format: String,

    /// Disable BM25 fallback (name/ID only)
    #[arg(long)]
    no_bm25: bool,

    /// Write output to file instead of stdout
    #[arg(short = 'o', long)]
    output: Option<String>,
}

pub async fn run(args: SearchArgs, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    // Create client
    let client = CdsIndexClient::new(config.service_url.clone());

    // Call search API
    let result = client
        .search_entities(&args.query, args.entity_type, args.limit)
        .await?;

    // Format output
    let output_str = match args.format.as_str() {
        "json" => formatter::format_search_json(&result, config.json_pretty)?,
        "text" => formatter::format_search_text(&result),
        "fold" => formatter::format_search_fold(&result),
        "preview" => formatter::format_search_preview(&result),
        "full" => formatter::format_search_full(&result),
        _ => return Err(format!("Invalid format: {}", args.format).into()),
    };

    // Write output
    if let Some(file_path) = args.output {
        std::fs::write(file_path, output_str)?;
    } else {
        println!("{}", output_str);
    }

    // Exit code: 0 if results found, 1 if empty
    if result.results.is_empty() {
        std::process::exit(1);
    }

    Ok(())
}
```

**Acceptance** (from PRD-03 FR-CMD-1):

- [ ] Accepts query, type, limit, format arguments
- [ ] Calls `search_entities` JSON-RPC method
- [ ] Returns results with fold/preview/full snippets
- [ ] Exit code 0 on success, 1 on no results

---

**Task 4: `cds traverse` Command**

```rust
// cds-cli/src/commands/traverse.rs
use clap::Args;
use crate::client::CdsIndexClient;
use crate::output::tree;
use crate::config::Config;

#[derive(Args)]
pub struct TraverseArgs {
    /// Starting entity ID(s)
    entity_ids: Vec<String>,

    /// Maximum traversal depth
    #[arg(short = 'd', long, default_value = "1")]
    depth: usize,

    /// Filter by relation types (comma-separated)
    #[arg(short = 'r', long)]
    relations: Option<String>,

    /// Filter by entity types (comma-separated)
    #[arg(short = 't', long)]
    entity_type: Option<String>,

    /// Traversal direction: forward|backward|bidirectional
    #[arg(short = 'D', long, default_value = "forward")]
    direction: String,

    /// Output format: json|tree|dot
    #[arg(short = 'f', long, default_value = "json")]
    format: String,

    /// Write output to file
    #[arg(short = 'o', long)]
    output: Option<String>,
}

pub async fn run(args: TraverseArgs, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let client = CdsIndexClient::new(config.service_url.clone());

    // Parse relation types
    let relation_types = args.relations.map(|s| {
        s.split(',').map(|r| r.trim().to_string()).collect()
    });

    // Traverse each entity
    let mut all_results = Vec::new();
    for entity_id in &args.entity_ids {
        let result = client
            .traverse_graph(entity_id, relation_types.clone(), args.depth, 100)
            .await?;
        all_results.push(result);
    }

    // Format output
    let output_str = match args.format.as_str() {
        "json" => formatter::format_traverse_json(&all_results, config.json_pretty)?,
        "tree" => tree::format_traverse_tree(&all_results),
        "dot" => tree::format_traverse_dot(&all_results),
        _ => return Err(format!("Invalid format: {}", args.format).into()),
    };

    // Write output
    if let Some(file_path) = args.output {
        std::fs::write(file_path, output_str)?;
    } else {
        println!("{}", output_str);
    }

    Ok(())
}
```

**Acceptance** (from PRD-03 FR-CMD-2):

- [ ] BFS respects depth limit
- [ ] Filters by relation and entity types
- [ ] Tree output matches LocAgent format (see [02-output-format.md](02-output-format.md))
- [ ] Supports forward, backward, bidirectional traversal

---

**Task 5: `cds retrieve` Command**

```rust
// cds-cli/src/commands/retrieve.rs
use clap::Args;
use crate::client::CdsIndexClient;
use crate::output::formatter;
use crate::config::Config;

#[derive(Args)]
pub struct RetrieveArgs {
    /// Entity ID(s) to retrieve
    entity_ids: Vec<String>,

    /// Output format: json|text|code
    #[arg(short = 'f', long, default_value = "json")]
    format: String,

    /// Include N lines before/after entity
    #[arg(short = 'c', long, default_value = "0")]
    context: usize,

    /// Write output to file
    #[arg(short = 'o', long)]
    output: Option<String>,
}

pub async fn run(args: RetrieveArgs, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let client = CdsIndexClient::new(config.service_url.clone());

    // Retrieve each entity
    let mut results = Vec::new();
    for entity_id in &args.entity_ids {
        let result = client.retrieve_entity(entity_id, "full").await?;
        results.push(result);
    }

    // Format output
    let output_str = match args.format.as_str() {
        "json" => formatter::format_retrieve_json(&results, config.json_pretty)?,
        "text" => formatter::format_retrieve_text(&results),
        "code" => formatter::format_retrieve_code(&results),
        _ => return Err(format!("Invalid format: {}", args.format).into()),
    };

    // Write output
    if let Some(file_path) = args.output {
        std::fs::write(file_path, output_str)?;
    } else {
        println!("{}", output_str);
    }

    Ok(())
}
```

**Acceptance** (from PRD-03 FR-CMD-3):

- [ ] Returns full code for entity IDs
- [ ] Includes file path, line range, metadata
- [ ] Can retrieve multiple entities in one call
- [ ] Context lines supported (deferred to v0.2.0)

---

### Week 4: Configuration & Init

**Task 6: `cds config` Command**

```rust
// cds-cli/src/commands/config.rs
use clap::Args;
use crate::config::{Config, set_config_value, get_config_value, list_config};

#[derive(Args)]
pub struct ConfigArgs {
    /// Action: get|set|list
    action: String,

    /// Config key (e.g., "index.graph_dir")
    key: Option<String>,

    /// Config value (for set action)
    value: Option<String>,
}

pub async fn run(args: ConfigArgs, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    match args.action.as_str() {
        "get" => {
            let key = args.key.ok_or("Key required for 'get' action")?;
            let value = get_config_value(&key)?;
            println!("{}", value);
        }
        "set" => {
            let key = args.key.ok_or("Key required for 'set' action")?;
            let value = args.value.ok_or("Value required for 'set' action")?;
            set_config_value(&key, &value)?;
            println!("Set {} = {}", key, value);
        }
        "list" => {
            let all_config = list_config()?;
            for (key, value) in all_config {
                println!("{} = {}", key, value);
            }
        }
        _ => return Err(format!("Invalid action: {}", args.action).into()),
    }

    Ok(())
}
```

**Acceptance** (from PRD-03 FR-CFG-1):

- [ ] Can get/set/list config values
- [ ] Reads from `~/.config/cds/config.toml`
- [ ] Env vars override config file
- [ ] LocAgent-compatible env vars (GRAPH_INDEX_DIR, BM25_INDEX_DIR)

---

**Task 7: `cds init` Command**

```rust
// cds-cli/src/commands/init.rs
use clap::Args;
use indicatif::{ProgressBar, ProgressStyle};
use crate::config::Config;

#[derive(Args)]
pub struct InitArgs {
    /// Repository path to index
    repo_path: String,

    /// Languages to index
    #[arg(short = 'l', long, default_value = "python")]
    languages: String,

    /// Index output directory
    #[arg(short = 'o', long)]
    output: Option<String>,

    /// Parallel indexing jobs
    #[arg(short = 'j', long)]
    jobs: Option<usize>,

    /// Incremental update (skip unchanged files)
    #[arg(long)]
    incremental: bool,
}

pub async fn run(args: InitArgs, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing index for: {}", args.repo_path);

    // Create progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {msg}")
        .unwrap());
    pb.set_message("Building graph...");

    // Call CDS-Index Service to build index
    // (Simplified: in reality, this would call a build_index RPC method)
    let client = CdsIndexClient::new(config.service_url.clone());
    // let result = client.build_index(&args.repo_path, ...).await?;

    pb.finish_with_message("Index built successfully!");

    println!("Indexed {} files, {} entities", 150, 1234);

    Ok(())
}
```

**Acceptance** (from PRD-03 FR-CFG-2):

- [ ] Calls CDS-Index Service to build graph
- [ ] Displays progress (files indexed, entities found)
- [ ] Supports incremental updates (v0.2.0)
- [ ] Parallel indexing (uses config or --jobs flag)

---

## Acceptance Criteria (from PRD-03 §8)

### Must-Pass

- [ ] All commands parse arguments correctly
- [ ] Commands call CDS-Index Service JSON-RPC APIs
- [ ] Error handling with actionable messages
- [ ] Exit codes: 0 (success), 1 (no results), 2 (invalid args), 3 (index error), 4 (service error), 5 (IO error)
- [ ] Unit test coverage >85% for `cds-cli` crate
- [ ] Performance: CLI startup <100ms

### Configuration

- [ ] Config file: `~/.config/cds/config.toml`
- [ ] Env vars: `CDS_CONFIG`, `GRAPH_INDEX_DIR`, `BM25_INDEX_DIR`
- [ ] Precedence: Env vars > Config file > Defaults

---

## Testing Strategy

### Unit Tests

```rust
// cds-cli/tests/command_parse_test.rs
#[test]
fn test_search_args() {
    let cli = Cli::parse_from(["cds", "search", "query", "--type", "function", "--limit", "5"]);
    match cli.command {
        Commands::Search(args) => {
            assert_eq!(args.query, "query");
            assert_eq!(args.entity_type, Some("function".to_string()));
            assert_eq!(args.limit, 5);
        }
        _ => panic!("Wrong command"),
    }
}

#[test]
fn test_traverse_args() {
    let cli = Cli::parse_from([
        "cds", "traverse", "entity_abc", "--depth", "2", "--relations", "invoke,contain"
    ]);
    match cli.command {
        Commands::Traverse(args) => {
            assert_eq!(args.entity_ids, vec!["entity_abc"]);
            assert_eq!(args.depth, 2);
            assert_eq!(args.relations, Some("invoke,contain".to_string()));
        }
        _ => panic!("Wrong command"),
    }
}
```

### Integration Tests

- See [03-integration-tests.md](03-integration-tests.md)

---

## Open Questions & Risks

### Q1: Stdin Support

**Question**: Should commands read entity IDs from stdin for piping?
**Decision**: Yes, implement stdin reader (Week 4)
**Example**: `echo "entity_abc" | cds retrieve`

### Q2: Service Discovery

**Risk**: How does CLI find running CDS-Index Service?
**Mitigation**: Default to `http://127.0.0.1:9001`, configurable via `CDS_SERVICE_URL` env var
**Escalation**: If service not found, print actionable error with instructions

### Q3: Error Granularity

**Risk**: Generic error messages unhelpful for debugging
**Mitigation**: Include JSON-RPC error codes in CLI output, suggest fixes
**Example**: "Error -32001: Index not ready. Hint: Run `cds init <repo>`"

---

## Related Issues

- **Parent**: [00-overview.md](00-overview.md)
- **Depends On**: [../02-index-core/03-service-layer.md](../02-index-core/03-service-layer.md)
- **Blocks**: [03-integration-tests.md](03-integration-tests.md)
- **Coordinates With**: [05-api-contracts.md](../05-api-contracts.md)
- **Tests**: [../08-testing/01-unit.md](../08-testing/01-unit.md)

---

## Next Steps

1. [ ] Set up cds-cli crate with clap and dependencies (Week 3, Day 1)
2. [ ] Implement JSON-RPC client (Week 3, Day 2)
3. [ ] Implement `cds search` command (Week 3, Day 3)
4. [ ] Implement `cds traverse` and `cds retrieve` (Week 3, Day 4)
5. [ ] Implement `cds config` and `cds init` (Week 4)
6. [ ] Write unit tests for all commands
7. [ ] Integration test with CDS-Index Service

---

**Status Updates**:

- *2025-10-19*: Sub-issue created, awaiting CDS-Index Service completion
