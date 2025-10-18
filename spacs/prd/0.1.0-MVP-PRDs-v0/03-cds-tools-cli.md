# PRD-03: CDS-Tools CLI - Unified Code Retrieval Interface

**Version:** 1.0 (Round 1 - Concise)
**Date:** 2025-10-18
**Status:** Draft
**Parent:** PRD-01 System Architecture

---

## 1. Component Overview

### 1.1 Purpose

CDS-Tools provides a unified command-line interface (CLI) that exposes LocAgent's code retrieval capabilities as composable Unix-style tools. The `cds` binary wraps the CDS-Index Service (PRD-02) and enables both human developers and LLM agents to perform sophisticated code searches through simple, pipeable commands.

### 1.2 Scope

- Core commands: `search`, `traverse`, `retrieve`, `combo`
- Multiple output formats: JSON (agent-friendly), text (human-readable)
- Unix pipeline integration (stdin/stdout, exit codes)
- Configuration management (index paths, defaults)
- Hybrid retrieval support (compose with `rg`, `ast-grep`, `jq`)

### 1.3 LocAgent References

- **Paper §3.2**: "LocAgent Tools" (SearchEntity, TraverseGraph, RetrieveEntity) - <https://arxiv.org/html/2503.09089v2#S3.SS2>
- **Table 1**: Tool definitions and parameters
- **Code**: `tmp/LocAgent/plugins/location_tools/locationtools.py`

---

## 2. Functional Requirements

### 2.1 Core Commands

#### FR-CMD-1: `cds search` - Entity Keyword Search

**Purpose**: Find code entities matching keywords using hierarchical indexing.

**Command Signature**:

```bash
cds search [OPTIONS] <QUERY>

OPTIONS:
  -t, --type <TYPE>        Filter by entity type (file|class|function|directory)
  -l, --limit <N>          Maximum results to return (default: 10)
  -f, --format <FORMAT>    Output format: json|text|fold|preview|full (default: json)
      --no-bm25            Disable BM25 fallback (name/ID only)
  -o, --output <FILE>      Write output to file instead of stdout
```

**Behavior**:

1. Query CDS-Index hierarchical search (name/ID + BM25)
2. Return results with fold/preview/full snippets
3. Rank by relevance (exact match > BM25 score)

**Output Format (JSON)**:

```json
{
  "query": "sanitize user input",
  "total_results": 3,
  "results": [
    {
      "entity_id": "abc123",
      "name": "sanitize_html",
      "type": "function",
      "file_path": "src/utils/sanitize.py",
      "line_range": [15, 32],
      "score": 1.0,
      "snippet": {
        "fold": "def sanitize_html(input: str) -> str:",
        "preview": "def sanitize_html(input: str) -> str:\n    \"\"\"Remove XSS vectors from HTML input.\"\"\"\n    return bleach.clean(input, ...)",
        "full": "def sanitize_html(input: str) -> str:\n    \"\"\"Remove XSS vectors...\""
      }
    }
  ]
}
```

**Output Format (Text)**:

```text
Found 3 results for "sanitize user input"

[1] sanitize_html (function) - src/utils/sanitize.py:15-32
    def sanitize_html(input: str) -> str:
    """Remove XSS vectors from HTML input."""
    return bleach.clean(input, ...)

[2] validate_user_input (function) - src/validators.py:45-60
    ...
```

**LocAgent Mapping**:

- Implements SearchEntity tool (LocAgent Table 1, §3.2)
- Preserves fold/preview/full snippet format (LocAgent Figure 6)

**Acceptance Criteria**:

- [ ] Returns results matching hierarchical search behavior
- [ ] Supports type filtering (e.g., `--type function`)
- [ ] JSON output is valid and parseable
- [ ] Text output is human-readable

#### FR-CMD-2: `cds traverse` - Graph BFS Navigation

**Purpose**: Explore code relationships via breadth-first search on the code graph.

**Command Signature**:

```bash
cds traverse [OPTIONS] <ENTITY_ID>...

OPTIONS:
  -d, --depth <N>          Maximum traversal depth (default: 1)
  -r, --relations <REL>    Filter by relation types: contain,import,invoke,inherit
                           (comma-separated, default: all)
  -t, --type <TYPE>        Filter by entity types: file,class,function,directory
  -D, --direction <DIR>    Traversal direction: forward|backward|bidirectional
                           (default: forward)
  -f, --format <FORMAT>    Output format: json|tree|dot (default: json)
  -o, --output <FILE>      Write output to file
```

**Behavior**:

1. Start from specified entity ID(s)
2. Perform BFS with depth limit, filtering by relation/entity types
3. Return subgraph in tree-structured format

**Output Format (JSON)**:

```json
{
  "start_entities": ["entity_abc"],
  "depth": 2,
  "filters": {
    "relations": ["invoke"],
    "types": ["function"]
  },
  "subgraph": {
    "nodes": [
      {"id": "entity_abc", "name": "process_request", "type": "function", "file": "server.py"},
      {"id": "entity_def", "name": "validate_input", "type": "function", "file": "validators.py"}
    ],
    "edges": [
      {"source": "entity_abc", "target": "entity_def", "type": "invoke"}
    ]
  }
}
```

**Output Format (Tree - LocAgent expanded tree format)**:

```text
process_request (function) [entity_abc] - server.py:12
├─[invoke]→ validate_input (function) [entity_def] - validators.py:45
│  ├─[invoke]→ sanitize_html (function) [entity_ghi] - utils/sanitize.py:15
│  └─[invoke]→ check_csrf (function) [entity_jkl] - security.py:78
└─[invoke]→ save_to_db (function) [entity_mno] - database.py:123
```

**LocAgent Mapping**:

- Implements TraverseGraph tool (LocAgent Table 1, §3.2)
- Tree format from LocAgent Figure 7 (Appendix A.1.2)
- Type-aware BFS preserves meta-path semantics

**Acceptance Criteria**:

- [ ] BFS respects depth limit
- [ ] Filters by relation and entity types correctly
- [ ] Tree output matches LocAgent format
- [ ] Can traverse forward, backward, bidirectional

#### FR-CMD-3: `cds retrieve` - Full Entity Retrieval

**Purpose**: Fetch complete code and metadata for specific entities.

**Command Signature**:

```bash
cds retrieve [OPTIONS] <ENTITY_ID>...

OPTIONS:
  -f, --format <FORMAT>    Output format: json|text|code (default: json)
  -c, --context <N>        Include N lines before/after entity (default: 0)
  -o, --output <FILE>      Write output to file
```

**Output Format (JSON)**:

```json
{
  "entities": [
    {
      "id": "entity_abc",
      "name": "sanitize_html",
      "type": "function",
      "file_path": "src/utils/sanitize.py",
      "line_range": [15, 32],
      "code": "def sanitize_html(input: str) -> str:\n    \"\"\"Remove XSS...\"\"\"\n    return bleach.clean(input, tags=['b', 'i'])",
      "metadata": {
        "parameters": ["input: str"],
        "returns": "str",
        "docstring": "Remove XSS vectors from HTML input."
      }
    }
  ]
}
```

**LocAgent Mapping**:

- Implements RetrieveEntity tool (LocAgent Table 1, §3.2)

**Acceptance Criteria**:

- [ ] Returns full code for entity ID
- [ ] Includes file path, line range, metadata
- [ ] Can retrieve multiple entities in one call

#### FR-CMD-4: `cds combo` - Hybrid Retrieval Pipelines

**Purpose**: Execute pre-defined or custom multi-step retrieval workflows (developer-operated in v0.1.0; agent integration targeted for v0.2.0).

**Command Signature**:

```bash
cds combo [OPTIONS] <PLAN_FILE>

OPTIONS:
  -v, --variables <JSON>   Pass variables to plan (JSON object)
  -o, --output <FILE>      Write final output to file
  -d, --dry-run            Print plan without executing
```

**Plan File Format (YAML)**:

```yaml
# Example: Find XSS vulnerabilities and trace call chains
name: "XSS Vulnerability Analysis"
steps:
  - name: "Search for sanitize functions"
    command: search
    args:
      query: "sanitize XSS"
      type: function
      limit: 5
    output_var: sanitize_funcs

  - name: "Find callers of sanitize functions"
    command: traverse
    args:
      entity_ids: "${sanitize_funcs.ids}"
      direction: backward
      relations: [invoke]
      depth: 2
    output_var: call_graph

  - name: "Filter for user-facing endpoints"
    shell: |
      echo "${call_graph}" | jq '.nodes[] | select(.file | test("views|routes"))'
    output_var: entry_points

final_output: "${entry_points}"
```

**LocAgent Mapping**:

- Enables complex retrieval strategies beyond single tool calls
- Mimics LocAgent's chain-of-thought multi-step planning

**Acceptance Criteria**:

- [ ] Can parse YAML plan files
- [ ] Executes steps sequentially with variable substitution
- [ ] Supports mixing cds commands and shell commands
- [ ] Returns aggregated results

### 2.2 Configuration and Environment

#### FR-CFG-1: Configuration Management

**Purpose**: Allow users to configure index paths, defaults, and behavior.

**Configuration File** (`~/.config/cds/config.toml`):

```toml
[index]
# Path to graph index directory (overrides GRAPH_INDEX_DIR env var)
graph_dir = "/path/to/graph_index_v2.3"

# Path to BM25 index directory
bm25_dir = "/path/to/BM25_index"

[search]
# Default result limit
default_limit = 10

# Use BM25 fallback when name/ID results < threshold
bm25_threshold = 5

[output]
# Default output format
default_format = "json"

# Pretty-print JSON
json_pretty = true
```

**Command**:

```bash
cds config [get|set|list] <KEY> [VALUE]

# Examples:
cds config get index.graph_dir
cds config set search.default_limit 20
cds config list
```

**Environment Variables** (LocAgent compatibility):

- `GRAPH_INDEX_DIR`: Override graph index path
- `BM25_INDEX_DIR`: Override BM25 index path
- `CDS_CONFIG`: Path to custom config file

**Acceptance Criteria**:

- [ ] Reads config from file and env vars
- [ ] Env vars override config file
- [ ] `cds config` command manages settings

#### FR-CFG-2: Index Initialization

**Purpose**: Initialize or rebuild index for a repository.

**Command**:

```bash
cds init [OPTIONS] <REPO_PATH>

OPTIONS:
  -l, --languages <LANGS>  Languages to index (default: python)
  -o, --output <DIR>       Index output directory (default: .cds-index/)
  -j, --jobs <N>           Parallel indexing jobs (default: num_cpus)
  --incremental            Incremental update (skip unchanged files)
```

**Behavior**:

1. Call CDS-Index Service to build graph and sparse indices
2. Save to specified output directory
3. Display progress (files indexed, entities found)

**Acceptance Criteria**:

- [ ] Can index repository from scratch
- [ ] Supports incremental updates
- [ ] Displays progress and statistics

### 2.3 Unix Pipeline Integration

#### FR-PIPE-1: Stdin/Stdout Composability

**Requirement**: Commands accept input from stdin and output to stdout for piping.

**Examples**:

```bash
# Pipe search results to traverse
cds search "sanitize" -f json | jq '.results[].entity_id' | xargs cds retrieve

# Pipe with ripgrep
cds search "user input" | jq -r '.results[].file_path' | xargs rg "TODO"

# Pipe with ast-grep
cds search "validate" -t function | jq -r '.results[].file_path' | xargs ast-grep --pattern 'if $COND: raise $ERR'
```

**Acceptance Criteria**:

- [ ] JSON output is valid for piping to `jq`
- [ ] Can read entity IDs from stdin
- [ ] Works with standard Unix tools (`grep`, `awk`, `xargs`)

#### FR-PIPE-2: Exit Codes and Error Handling

**Requirement**: Return standard exit codes for shell scripting.

**Exit Codes**:

- `0`: Success
- `1`: No results found (search/traverse)
- `2`: Invalid arguments
- `3`: Index not found or corrupted
- `4`: CDS-Index Service error
- `5`: IO error (file write, etc.)

**Error Output** (stderr):

```text
Error: Index directory not found: /path/to/index
Hint: Run 'cds init <repo>' to create an index, or set GRAPH_INDEX_DIR
```

**Acceptance Criteria**:

- [ ] Exit codes follow standard conventions
- [ ] Errors printed to stderr (not stdout)
- [ ] Error messages are actionable

---

## 3. Non-Functional Requirements

### 3.1 Performance

| Metric | Target | Rationale |
|--------|--------|-----------|
| CLI startup time | <100ms | Avoid Python-like slow startup |
| Search command latency | <500ms | Interactive use |
| Traverse command (2-hop) | <1s | Multi-hop exploration |
| JSON parse overhead | <50ms | Minimal serialization cost |

### 3.2 Usability

**Developer Experience Goals**:

- Intuitive command naming (verb-based: search, traverse, retrieve)
- Consistent flag naming across commands
- Rich help text (`cds --help`, `cds search --help`)
- Examples in help output

**Help Output Example**:

```bash
$ cds search --help
Search for code entities using keywords

USAGE:
    cds search [OPTIONS] <QUERY>

ARGS:
    <QUERY>    Search query (keywords, supports phrases)

OPTIONS:
    -t, --type <TYPE>        Filter by entity type [possible values: file, class, function]
    -l, --limit <N>          Maximum results [default: 10]
    -f, --format <FORMAT>    Output format [default: json] [possible values: json, text]
        --no-bm25            Disable BM25 fallback

EXAMPLES:
    # Find all functions matching "sanitize"
    cds search "sanitize" --type function

    # Search with phrase and limit results
    cds search "user authentication" --limit 5

    # Output as human-readable text
    cds search "parse config" --format text
```

### 3.3 Reliability

- Graceful degradation if index partially corrupted
- Retry logic for CDS-Index Service calls (if using RPC)
- Validate input before expensive operations

---

## 4. Architecture

### 4.1 CLI Structure (Rust)

```tree
cds-cli/
  ├── src/
  │   ├── main.rs              # Entry point, clap CLI setup
  │   ├── commands/
  │   │   ├── mod.rs
  │   │   ├── search.rs        # cds search implementation
  │   │   ├── traverse.rs      # cds traverse
  │   │   ├── retrieve.rs      # cds retrieve
  │   │   ├── combo.rs         # cds combo
  │   │   ├── config.rs        # cds config
  │   │   └── init.rs          # cds init
  │   ├── output/
  │   │   ├── formatter.rs     # JSON/text formatting
  │   │   └── tree.rs          # Tree-structured output
  │   ├── config.rs            # Config file parsing
  │   ├── client.rs            # CDS-Index Service client (RPC or direct)
  │   └── error.rs             # Error types and exit codes
  ├── Cargo.toml
  └── README.md
```

### 4.2 Key Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing with derive macros |
| `serde_json` | JSON output formatting |
| `serde_yaml` | Combo plan file parsing |
| `tokio` | Async runtime (if using RPC client) |
| `tonic` | gRPC client (if CDS-Index uses gRPC) |
| `colored` | Colored text output |
| `indicatif` | Progress bars (for `cds init`) |

---

## 5. LocAgent Refactoring Plan

### 5.1 Tool Mapping

| LocAgent Tool | CDS Command | LocAgent Code Reference |
|--------------|-------------|------------------------|
| SearchEntity | `cds search` | `plugins/location_tools/locationtools.py::search_entity()` |
| TraverseGraph | `cds traverse` | `plugins/location_tools/locationtools.py::traverse_graph()` |
| RetrieveEntity | `cds retrieve` | `plugins/location_tools/locationtools.py::retrieve_entity()` |

### 5.2 Output Format Preservation

**Critical**: Must match LocAgent's output structure for agent compatibility.

**LocAgent SearchEntity Output** (from LocAgent code):

```python
# Fold: one-line summary
fold = f"{entity.type} {entity.name} - {entity.file}:{entity.line}"

# Preview: signature + first few lines
preview = entity.signature + "\n" + entity.body[:5]

# Full: complete code
full = entity.code
```

**CDSAgent `cds search` Output**:

```rust
// Must produce identical structure
struct SearchResult {
    fold: String,    // "function sanitize_html - sanitize.py:15"
    preview: String, // "def sanitize_html(...):\n    \"\"\"Remove XSS...\""
    full: String,    // Complete function code
}
```

### 5.3 Tree Format (TraverseGraph)

**LocAgent Tree Format** (from LocAgent Figure 7):

```text
RootEntity (type) [id]
├─[relation]→ ChildEntity1 (type) [id]
│  └─[relation]→ GrandchildEntity (type) [id]
└─[relation]→ ChildEntity2 (type) [id]
```

**CDSAgent Implementation**:

```rust
// Tree formatting logic (output/tree.rs)
fn format_tree(graph: &SubGraph, root: &NodeID) -> String {
    // Recursive tree builder with indentation
    // Preserve exact LocAgent format: ├─, └─, │, arrows, [relation]
}
```

---

## 6. Hybrid Retrieval Examples

### 6.1 Example 1: XSS Vulnerability Search

```bash
# Step 1: Find sanitization functions
cds search "sanitize XSS" --type function --format json > sanitize_funcs.json

# Step 2: Extract entity IDs
cat sanitize_funcs.json | jq -r '.results[].entity_id' > entity_ids.txt

# Step 3: Traverse to find callers (backward BFS)
cat entity_ids.txt | xargs -I {} cds traverse {} --direction backward --depth 2 --relations invoke > call_graph.json

# Step 4: Filter for route handlers (user-facing)
cat call_graph.json | jq '.nodes[] | select(.file | test("routes|views"))' > entry_points.json

# Step 5: Retrieve full code for analysis
cat entry_points.json | jq -r '.id' | xargs cds retrieve --format code
```

### 6.2 Example 2: Dead Code Detection

```bash
# Find all functions
cds search "*" --type function --format json > all_funcs.json

# Find functions never called (no incoming invoke edges)
cat all_funcs.json | jq -r '.results[].entity_id' | while read id; do
  callers=$(cds traverse $id --direction backward --relations invoke --depth 1 | jq '.nodes | length')
  if [ "$callers" -eq 1 ]; then  # Only the function itself
    cds retrieve $id --format text
  fi
done
```

---

## 7. Testing and Validation

### 7.1 Unit Tests

- [ ] CLI argument parsing (valid/invalid inputs)
- [ ] Output formatting (JSON schema validation)
- [ ] Tree formatting (matches LocAgent format)
- [ ] Error handling (exit codes, stderr messages)

### 7.2 Integration Tests

- [ ] End-to-end: `cds search` on test repository
- [ ] Piping: `cds search | jq | cds retrieve`
- [ ] Config: Read from file, env vars, precedence

### 7.3 Compatibility Tests

- [ ] Output matches LocAgent SearchEntity JSON structure
- [ ] Tree format identical to LocAgent TraverseGraph

---

## 8. Acceptance Criteria Summary

### Must-Have (v1.0)

- [ ] `cds search`, `cds traverse`, `cds retrieve` commands functional
- [ ] JSON and text output formats
- [ ] Unix pipeline support (stdin/stdout)
- [ ] Config file and env var support
- [ ] Help text and examples
- [ ] Exit codes and error messages

### Should-Have (v1.1)

- [ ] `cds combo` for hybrid workflows
- [ ] `cds init` for index initialization
- [ ] Progress indicators
- [ ] Colored output

### Nice-to-Have (Future)

- [ ] Shell completions (bash, zsh, fish)
- [ ] Interactive mode (REPL)
- [ ] Query history

---

## 9. Open Questions

1. **Direct vs RPC**: Should `cds` link directly to CDS-Index crates or call via RPC? (Performance vs modularity)
2. **Output Schema**: Document JSON schema for API stability? (Use JSON Schema or Rust types)
3. **Combo DSL**: YAML sufficient or need custom DSL for complex plans?

---

## 10. Next Steps

1. Implement basic `cds search` with JSON output
2. Add `cds traverse` with tree formatting
3. Integration test with Claude agent (PRD-04)
4. Performance benchmarking against LocAgent

---

**Status**: Ready for implementation sprint. Requires CDS-Index Service (PRD-02) foundation.
