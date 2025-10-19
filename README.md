# CDSAgent

**CDSAgent**: A Fast, Graph-Based Code Retrieval System

CDSAgent is a graph-guided code localization agent designed to serve as a sub-agent for codebase information retrieval (IR) tasks in any coding agent system. Built with Rust for performance and TypeScript for LLM orchestration.

---

## Features

- **Graph-Based Code Indexing**: Directed heterogeneous graph with 4 node types (directory, file, class, function) and 4 edge types (contain, import, invoke, inherit)
- **Hierarchical Sparse Search**: Two-tier index (name/ID HashMap + BM25 content search) for fast, accurate retrieval
- **LLM Orchestration**: Claude Agent SDK integration with chain-of-thought reasoning
- **Multi-Language AST Parsing**: Tree-sitter-based parsing (v0.1.0: Python; v0.2.0+: TypeScript/JavaScript, Rust, Go)
- **JSON-RPC Service**: High-performance index service exposing graph and search endpoints
- **CLI Tools**: Command-line interface for search, traverse, and code retrieval operations

---

## Architecture

```text
┌──────────────────────────────────────────────────────────────┐
│                   CDS-Agent (TypeScript)                     │
│  Claude Agent SDK + Chain-of-Thought Prompting + Hooks       │
└───────────────────────────┬──────────────────────────────────┘
                            │ Bash Tool / MCP (v0.2.0)
                            │
┌───────────────────────────▼──────────────────────────────────┐
│                  CDS-Tools CLI (Rust)                        │
│   cds search | cds traverse | cds retrieve                   │
└───────────────────────────┬──────────────────────────────────┘
                            │ JSON-RPC Client
                            │
┌───────────────────────────▼──────────────────────────────────┐
│           CDS-Index Service (Rust, JSON-RPC)                 │
│  ┌─────────────────┐      ┌──────────────────────────────┐   │
│  │  Dependency     │      │  Hierarchical Sparse Index   │   │
│  │  Graph Builder  │      │  (Name Index + BM25 Search)  │   │
│  └─────────────────┘      └──────────────────────────────┘   │
└──────────────────────────────────────────────────────────────┘
```

---

## Quick Start (Development)

### Prerequisites

- **Rust 1.75+** (stable channel)
- **Bun 1.1+** (for TypeScript agent)
- **just** (optional, for task runner)

### 1. Clone the Repository

```bash
git clone https://github.com/lwyBZss8924d/CDSAgent
cd CDSAgent
```

### 2. Install Dependencies

```bash
# Install Rust dependencies
cargo build --all

# Install TypeScript/Bun dependencies
cd cds-agent && bun install && cd ..

# Or use just:
just install
```

### 3. Set Up Environment

```bash
# Initialize development environment
just dev-setup

# OR manually:
mkdir -p data/{graph_index,bm25_index}
mkdir -p cds-agent/logs
cp cds-agent/.env.example cds-agent/.env

# Edit cds-agent/.env with your API keys:
# ANTHROPIC_API_KEY=sk-ant-...
```

### 4. Run the Index Service

```bash
# Using just:
just run-service

# OR manually:
GRAPH_INDEX_DIR=./data/graph_index BM25_INDEX_DIR=./data/bm25_index cargo run --bin cds-index-service
```

### 5. Use the CLI

```bash
# Search for entities
just run-cli search "Calculator" --limit 10

# Traverse dependencies
just run-cli traverse "file.py::MyClass::my_function" --direction outgoing

# Retrieve code
just run-cli retrieve "file.py::MyClass" --context 5
```

### 6. Run the Agent

```bash
# In a separate terminal:
just agent-dev

# OR manually:
cd cds-agent && bun run dev
```

---

## Development Workflow

### Project Structure

```tree
CDSAgent/
├── crates/
│   ├── cds-index/          # Index Service (Rust)
│   │   ├── src/
│   │   │   ├── graph/      # AST parsing & graph building
│   │   │   ├── index/      # Name index + BM25 search
│   │   │   ├── service/    # JSON-RPC server
│   │   │   └── bin/        # cds-index-service binary
│   │   └── tests/          # Integration tests
│   └── cds-tools/          # CLI Tools (Rust)
│       ├── src/
│       │   ├── commands/   # search, traverse, retrieve
│       │   ├── client/     # JSON-RPC client
│       │   └── formatters/ # JSON, text, tree output
│       └── tests/          # CLI integration tests
├── cds-agent/              # LLM Orchestration (TypeScript/Bun)
│   ├── src/
│   │   ├── agent-config.ts # Claude SDK setup
│   │   ├── system-prompt.ts# Chain-of-thought prompt
│   │   └── hooks/          # PreToolUse, PostToolUse, SubagentStop
│   └── .claude/agents/     # Subagent configurations
├── deployment/             # systemd, launchd, Docker
├── spacs/                  # Specs, PRDs, issues, tasks
├── Cargo.toml              # Rust workspace
├── justfile                # Task runner (recommended)
└── README.md               # This file
```

### Common Development Tasks

```bash
# Build everything
just build

# Run all tests
just test-all

# Format all code
just fmt-all

# Lint all code
just lint-all

# Full CI check (format + lint + test)
just ci

# Clean build artifacts
just clean
```

### Running Tests

```bash
# Rust unit tests
cargo test --all

# Rust integration tests
cargo test --test '*'

# Rust benchmarks
just bench

# TypeScript tests
cd cds-agent && bun test

# OR use just:
just test-all
```

---

## Documentation

- **API Documentation**: `docs/api/` - JSON-RPC API specifications and schemas
  - [API Overview](docs/api/README.md) - Complete API documentation
  - [JSON-RPC Schema](docs/api/jsonrpc-schema.json) - Machine-readable schema
  - [Error Codes](docs/api/error-codes.md) - Error code catalogue
  - [Versioning](docs/api/versioning.md) - API versioning strategy
- **PRDs**: `spacs/prd/0.1.0-MVP-PRDs-v0/` - Product requirements (10 PRDs)
- **Issues**: `spacs/issues/04-0.1.0-mvp/` - Detailed issue breakdown
- **Tasks**: `spacs/tasks/0.1.0-mvp/` - Implementation task tracking
- **Architecture**: `spacs/issues/02-CDSAgent-Tech-Architecture-Plan.md` - System design
- **Backlog**: `spacs/plan/0.1.0-mvp-backlog.md` - Development plan

### Key Documents

- [API Documentation](docs/api/README.md) - JSON-RPC API reference
- [Tech Architecture Plan](spacs/issues/02-CDSAgent-Tech-Architecture-Plan.md)
- [0.1.0-MVP Backlog](spacs/plan/0.1.0-mvp-backlog.md)
- [LocAgent Parity Validation](spacs/issues/04-0.1.0-mvp/06-refactor-parity.md)

---

## Development Environment

### Required Tools

- **Rust**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Bun**: `curl -fsSL https://bun.sh/install | bash`
- **just** (optional): `cargo install just`

### IDE Setup

#### VS Code

Recommended extensions:

- `rust-lang.rust-analyzer` - Rust language server
- `oven.bun-vscode` - Bun runtime support
- `esbenp.prettier-vscode` - TypeScript formatting

### Environment Variables

#### Index Service (crates/cds-index)

```bash
GRAPH_INDEX_DIR=./data/graph_index   # Graph index storage
BM25_INDEX_DIR=./data/bm25_index     # BM25 index storage
INDEX_SERVICE_PORT=3030              # Service port
INDEX_SERVICE_HOST=127.0.0.1         # Bind address
RUST_LOG=info                        # Log level
```

#### Agent (cds-agent)

```bash
ANTHROPIC_API_KEY=sk-ant-...         # Required: Claude API key
OPENAI_API_KEY=sk-...                # Optional: OpenAI API key (v0.2.0)
AGENT_MAX_ITERATIONS=10              # Max tool call iterations
AGENT_LOG_DIR=./logs                 # Log directory
CLI_PATH=/usr/local/bin/cds          # Path to cds CLI binary
INDEX_SERVICE_URL=http://localhost:3030  # Index service endpoint
```

---

## Deployment

### Local Daemon (systemd/launchd)

See `spacs/issues/04-0.1.0-mvp/07-deployment/01-local-daemon.md` for detailed instructions.

#### Linux (systemd)

```bash
sudo scripts/install-daemon-linux.sh
sudo systemctl status cds-index
journalctl -u cds-index -f
```

#### macOS (launchd)

```bash
scripts/install-daemon-macos.sh
launchctl list | grep cds
tail -f ~/Library/Logs/cdsagent/stdout.log
```

### Docker Compose

```bash
cd deployment
cp config/.env.example config/.env
# Edit config/.env with your API keys

docker-compose up -d
./scripts/health-check.sh
```

---

## Contributing

1. Read the [Architecture Plan](spacs/issues/02-CDSAgent-Tech-Architecture-Plan.md)
2. Check [open issues](spacs/issues/04-0.1.0-mvp/)
3. Follow the [task structure](spacs/tasks/0.1.0-mvp/)
4. Run `just ci` before submitting PRs

---

## Roadmap

### v0.1.0-MVP (Current)

- [x] Rust workspace setup
- [x] TypeScript/Bun agent project
- [x] Development environment scaffolding
- [ ] Graph builder with tree-sitter (Python)
- [ ] Hierarchical sparse index (name + BM25)
- [ ] JSON-RPC service layer
- [ ] CLI tools (search, traverse, retrieve)
- [ ] Claude Agent SDK integration
- [ ] Chain-of-thought prompting
- [ ] Context management hooks
- [ ] SWE-bench Lite validation (File Acc@5 ≥75%)

### v0.2.0

- [ ] Incremental index updates
- [ ] Multi-language support (TypeScript/JavaScript + Rust)
- [ ] Native MCP tool wrappers
- [ ] Multi-SDK adapter (Claude + OpenAI)
- [ ] LanceDB evaluation for unified storage
- [ ] gRPC service (replaces JSON-RPC)
- [ ] Kubernetes deployment
- [ ] Full SWE-bench evaluation

---

## License

MIT OR Apache-2.0

---

## References

- **Paper**: [arXiv:2503.09089v2](https://arxiv.org/abs/2503.09089)
- **Claude Agent SDK**: [anthropics/anthropic-sdk-typescript](https://github.com/anthropics/anthropic-sdk-typescript)
- **Tree-sitter**: [tree-sitter/tree-sitter](https://github.com/tree-sitter/tree-sitter)
