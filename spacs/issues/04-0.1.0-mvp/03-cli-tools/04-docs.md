# Sub-Issue 03.04: Documentation - Help Text, Examples & User Guide

**Priority**: P1 (Critical Path - User Experience)
**Status**: ☐ Not Started
**Owner**: Rust Dev 3
**Parent**: [03-cli-tools/00-overview.md](00-overview.md)
**PRD Reference**: [PRD-03 §3.2](../../../prd/0.1.0-MVP-PRDs-v0/03-cds-tools-cli.md)
**Timing**: Phase 2, Week 5 (parallel with integration tests)

---

## Objective

Create comprehensive help text, usage examples, and user documentation that makes CDS-Tools intuitive and easy to use for both human developers and LLM agents.

## Scope

**In Scope**:

- Command help text (clap annotations)
- Usage examples per command
- Configuration guide
- Hybrid retrieval cookbook (examples)
- Troubleshooting guide
- README for cds-cli crate

**Out of Scope (v0.2.0)**:

- Video tutorials
- Interactive tutorials
- Localization (non-English docs)

---

## Dependencies

- **Requires**: [01-command-impl.md](01-command-impl.md) (commands must be functional)
- **References**: [02-output-format.md](02-output-format.md) (format examples)

---

## Implementation Tasks

### Week 5, Day 1-2: Command Help Text

Task 1: Root Command Help

```rust
// cds-cli/src/main.rs
#[derive(Parser)]
#[command(name = "cds")]
#[command(about = "Code Discovery System - Unified code retrieval interface", long_about = None)]
#[command(version)]
#[command(after_help = "EXAMPLES:\n    \
    # Search for functions related to authentication\n    \
    cds search \"authenticate\" --type function\n\n    \
    # Traverse function call graph\n    \
    cds traverse entity_abc --depth 2 --relations invoke\n\n    \
    # Retrieve full code for an entity\n    \
    cds retrieve entity_xyz\n\n\
For more information on a specific command, run:\n    \
    cds <command> --help")]
struct Cli {
    // ...
}
```

**Acceptance**:

- [ ] `cds --help` shows overview and examples
- [ ] Version flag works
- [ ] Links to subcommand help

---

Task 2: Search Command Help

```rust
// cds-cli/src/commands/search.rs
#[derive(Args)]
#[command(about = "Search for code entities using keywords")]
#[command(after_help = "EXAMPLES:\n    \
    # Find all functions matching \"sanitize\"\n    \
    cds search \"sanitize\" --type function\n\n    \
    # Search with phrase and limit results\n    \
    cds search \"user authentication\" --limit 5\n\n    \
    # Output as human-readable text\n    \
    cds search \"parse config\" --format text\n\n    \
    # Save results to file\n    \
    cds search \"database\" --output results.json")]
pub struct SearchArgs {
    /// Search query (keywords, supports phrases)
    #[arg(help = "Keywords to search for (use quotes for phrases)")]
    query: String,

    /// Filter by entity type [possible values: file, class, function, directory]
    #[arg(short = 't', long, value_name = "TYPE")]
    entity_type: Option<String>,

    /// Maximum results to return [default: 10]
    #[arg(short = 'l', long, value_name = "N", default_value = "10")]
    limit: usize,

    /// Output format [possible values: json, text, fold, preview, full]
    #[arg(short = 'f', long, value_name = "FORMAT", default_value = "json")]
    format: String,

    /// Disable BM25 fallback (name/ID only)
    #[arg(long, help = "Use name/ID index only, skip content search")]
    no_bm25: bool,

    /// Write output to file instead of stdout
    #[arg(short = 'o', long, value_name = "FILE")]
    output: Option<String>,
}
```

**Acceptance**:

- [ ] `cds search --help` shows all options
- [ ] Examples included in help output
- [ ] Descriptions are clear and concise

---

Task 3: Traverse & Retrieve Help

```rust
// Similar patterns for traverse.rs and retrieve.rs
#[command(after_help = "EXAMPLES:\n    \
    # Traverse forward 2 hops from a function\n    \
    cds traverse entity_abc --depth 2\n\n    \
    # Find all callers (backward traversal)\n    \
    cds traverse entity_xyz --direction backward --relations invoke\n\n    \
    # Output as tree\n    \
    cds traverse entity_root --format tree")]
```

**Acceptance**:

- [ ] All commands have comprehensive help
- [ ] Examples cover common use cases
- [ ] Help text is consistent across commands

---

### Week 5, Day 3: User Guide & Cookbook

Task 4: User Guide

```markdown
<!-- cds-cli/docs/USER_GUIDE.md -->

# CDS-Tools User Guide

## Installation

```bash
cargo install cds-cli
```

Or build from source:

```bash
git clone https://github.com/your-org/CDSAgent.git
cd CDSAgent/cds-cli
cargo build --release
```

## Quick Start

### 1. Initialize Index

```bash
# Index your repository
cds init /path/to/repo
```

This creates `.cds-index/` in your repository with graph and search indices.

### 2. Search for Code

```bash
# Search for functions
cds search "authenticate" --type function

# Search with type filter and limit
cds search "database connection" --type class --limit 5
```

### 3. Explore Dependencies

```bash
# Find what a function calls
cds traverse entity_abc --depth 2 --relations invoke

# Find who calls a function (backward)
cds traverse entity_xyz --direction backward --relations invoke
```

### 4. Retrieve Full Code

```bash
# Get full code for an entity
cds retrieve entity_abc --format code
```

## Output Formats

### JSON (default)

Agent-friendly structured output. Pipe to `jq` for processing:

```bash
cds search "utils" --format json | jq '.results[].entity_id'
```

### Text

Human-readable summary with code previews:

```bash
cds search "sanitize" --format text
```

### Tree

Hierarchical graph visualization (traverse only):

```bash
cds traverse entity_root --format tree
```

## Configuration

Edit `~/.config/cds/config.toml`:

```toml
[index]
graph_dir = "/path/to/graph_index"
bm25_dir = "/path/to/bm25_index"

[search]
default_limit = 10
bm25_threshold = 5

[output]
default_format = "json"
json_pretty = true
```

Or use environment variables:

```bash
export GRAPH_INDEX_DIR=/path/to/index
export CDS_SERVICE_URL=http://localhost:9001
```

## Troubleshooting

### "Index not found"

Run `cds init <repo>` to create an index.

### "Service not running"

Ensure `cds-indexd` is running: `cds-indexd --port 9001`

### "No results found"

Try broader search terms or check index is up-to-date.

---

**Acceptance**:

- [ ] User guide covers installation, quick start, common tasks
- [ ] Examples are tested and work
- [ ] Troubleshooting section addresses common errors

---

Task 5: Hybrid Retrieval Cookbook

```markdown
<!-- cds-cli/docs/HYBRID_RETRIEVAL.md -->

# Hybrid Retrieval Cookbook

Combine CDS-Tools with standard Unix tools for powerful code analysis.

## Example 1: Find XSS Vulnerabilities

```bash
# 1. Find sanitization functions
cds search "sanitize XSS" --type function --format json > sanitize_funcs.json

# 2. Extract entity IDs
cat sanitize_funcs.json | jq -r '.results[].entity_id' > entity_ids.txt

# 3. Find callers (backward traversal)
cat entity_ids.txt | while read id; do
  cds traverse $id --direction backward --depth 2 --relations invoke
done > call_graph.json

# 4. Filter for route handlers
cat call_graph.json | jq '.nodes[] | select(.file | test("routes|views"))' > entry_points.json

# 5. Retrieve full code
cat entry_points.json | jq -r '.id' | xargs cds retrieve --format code
```

## Example 2: Dead Code Detection

```bash
# Find all functions
cds search "*" --type function --format json > all_funcs.json

# Find functions never called
cat all_funcs.json | jq -r '.results[].entity_id' | while read id; do
  callers=$(cds traverse $id --direction backward --relations invoke | jq '.nodes | length')
  if [ "$callers" -eq 1 ]; then  # Only the function itself
    cds retrieve $id --format text
  fi
done
```

## Example 3: Dependency Analysis

```bash
# Find all imports from a module
cds search "import requests" --format json | \
  jq -r '.results[].file_path' | \
  sort -u > files_using_requests.txt

# Analyze import patterns
cat files_using_requests.txt | xargs rg "from requests import"
```

## Example 4: Code Search with ripgrep

```bash
# Find files with TODOs in specific entities
cds search "utils" --format json | \
  jq -r '.results[].file_path' | \
  xargs rg "TODO"

# Find usage of deprecated functions
cds search "deprecated" --format json | \
  jq -r '.results[].name' | \
  while read func; do
    rg "\b$func\(" --files-with-matches
  done
```

## Example 5: ast-grep Integration

```bash
# Find error handling patterns
cds search "validate" --type function --format json | \
  jq -r '.results[].file_path' | \
  xargs ast-grep --pattern 'if $COND: raise $ERR'
```

**Acceptance**:

- [ ] Cookbook includes 5+ real-world examples
- [ ] Examples are tested and work end-to-end
- [ ] Covers cds + jq + rg + ast-grep workflows

---

### Week 5, Day 4-5: README & Polish

Task 6: CLI README

```markdown
<!-- cds-cli/README.md -->

# CDS-Tools CLI

Unified command-line interface for Code Discovery System (CDS).

## Features

- **Search**: Find code entities using hierarchical indexing (name/ID + BM25)
- **Traverse**: Explore code graph via BFS with type/relation filters
- **Retrieve**: Fetch full code and metadata for entities
- **Unix-friendly**: Pipe, compose, and integrate with standard tools

## Installation

```bash
cargo install cds-cli
```

## QUICK START

```bash
# Index a repository
cds init /path/to/repo

# Search for functions
cds search "authenticate" --type function

# Traverse dependencies
cds traverse entity_abc --depth 2 --relations invoke

# Retrieve full code
cds retrieve entity_xyz
```

## Documentation

- [User Guide](docs/USER_GUIDE.md) - Comprehensive usage guide
- [Hybrid Retrieval Cookbook](docs/HYBRID_RETRIEVAL.md) - Advanced examples
- [Configuration Reference](docs/CONFIG.md) - Config file and env vars

## OUTPUT FORMATS

- **JSON**: Agent-friendly, pipeable (default)
- **Text**: Human-readable summaries
- **Tree**: Graph visualization (traverse only)
- **Fold/Preview/Full**: Code snippet modes

## Examples

```bash
# Find all classes in a module
cds search "module_name" --type class

# Find who calls a function
cds traverse entity_abc --direction backward --relations invoke

# Combine with jq
cds search "utils" | jq '.results[].entity_id' | xargs cds retrieve
```

## License

Apache 2.0

---

**Acceptance**:

- [ ] README is clear and concise
- [ ] Links to comprehensive docs
- [ ] Examples work out of the box

---

## Acceptance Criteria (from PRD-03 §3.2, §8)

### Must-Have

- [ ] All commands have help text with examples
- [ ] User guide covers installation, quick start, common tasks
- [ ] Hybrid retrieval cookbook with 5+ examples
- [ ] README is clear and links to docs
- [ ] Troubleshooting guide addresses common errors
- [ ] Documentation reviewed for clarity and accuracy

### Quality

- [ ] Help text is consistent across commands
- [ ] Examples are tested and work
- [ ] Documentation is up-to-date with implementation

---

## Testing Strategy

### Documentation Tests

```rust
// Use cargo's built-in doc tests
/// # Examples
///
/// ```
/// use cds_cli::commands::search;
///
/// let result = search::run(SearchArgs {
///     query: "sanitize".to_string(),
///     entity_type: Some("function".to_string()),
///     limit: 5,
///     ..Default::default()
/// });
/// assert!(result.is_ok());
/// ```
```

### Example Validation

```bash
# tests/docs/validate_examples.sh

#!/bin/bash
set -e

echo "Validating documentation examples..."

# Test examples from USER_GUIDE.md
cds search "authenticate" --type function > /dev/null
cds traverse entity_abc --depth 2 > /dev/null

# Test cookbook examples
cds search "sanitize" --format json | jq '.results[].entity_id' > /dev/null

echo "All examples validated!"
```

---

## Related Issues

- **Parent**: [00-overview.md](00-overview.md)
- **References**: [01-command-impl.md](01-command-impl.md), [02-output-format.md](02-output-format.md)

---

## Next Steps

1. [ ] Add help text annotations to all commands (Week 5, Day 1)
2. [ ] Write user guide (Week 5, Day 2)
3. [ ] Write hybrid retrieval cookbook (Week 5, Day 3)
4. [ ] Write README and polish (Week 5, Day 4)
5. [ ] Validate all examples work (Week 5, Day 5)
6. [ ] Review documentation for clarity

---

**Status Updates**:

- *2025-10-19*: Sub-issue created, awaiting command implementation completion
