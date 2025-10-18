# CDSAgent Development Justfile
# Install just: https://github.com/casey/just

# Default recipe - show available commands
default:
    @just --list

# Rust targets
# ============

# Build all Rust crates
build:
    cargo build --all

# Build in release mode
build-release:
    cargo build --all --release

# Run Index Service (dev mode)
run-service:
    GRAPH_INDEX_DIR=./data/graph_index BM25_INDEX_DIR=./data/bm25_index cargo run --bin cds-index-service

# Run CLI tool
run-cli *ARGS:
    cargo run --bin cds -- {{ARGS}}

# Run all Rust tests
test:
    cargo test --all --lib --bins --tests

# Run Rust benchmarks
bench:
    cargo bench --all

# Check all code (fast compile check)
check:
    cargo check --all --all-targets

# Format Rust code
fmt:
    cargo fmt --all

# Check if Rust code is formatted
fmt-check:
    cargo fmt --all -- --check

# Lint Rust code with clippy
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Clean build artifacts
clean:
    cargo clean
    rm -rf target/

# TypeScript/Agent targets
# =========================

# Install agent dependencies
agent-install:
    cd cds-agent && bun install

# Run agent in dev mode
agent-dev:
    cd cds-agent && bun run dev

# Test agent
agent-test:
    cd cds-agent && bun test

# Lint agent code
agent-lint:
    cd cds-agent && bun run lint

# Format agent code
agent-fmt:
    cd cds-agent && bun run fmt

# Combined targets
# ================

# Install all dependencies (Rust + Bun)
install: agent-install
    @echo "Dependencies installed"

# Format all code (Rust + TypeScript)
fmt-all: fmt agent-fmt
    @echo "All code formatted"

# Lint all code (Rust + TypeScript)
lint-all: clippy agent-lint
    @echo "All code linted"

# Test all code (Rust + TypeScript)
test-all: test agent-test
    @echo "All tests passed"

# Full CI check (format, lint, test)
ci: fmt-check lint-all test-all
    @echo "CI checks passed"

# Development environment
# =======================

# Initialize development environment
dev-setup:
    @echo "Setting up development environment..."
    mkdir -p data/{graph_index,bm25_index}
    mkdir -p cds-agent/logs
    cp cds-agent/.env.example cds-agent/.env || true
    @echo "✓ Development environment ready"
    @echo "  1. Edit cds-agent/.env with your API keys"
    @echo "  2. Run 'just run-service' to start Index Service"
    @echo "  3. Run 'just agent-dev' to start Agent"

# Show project status
status:
    @echo "=== CDSAgent Development Status ==="
    @echo ""
    @echo "Rust workspace:"
    @cargo --version
    @rustc --version
    @echo ""
    @echo "Bun/TypeScript:"
    @cd cds-agent && bun --version || echo "Bun not found"
    @echo ""
    @echo "Git status:"
    @git status --short || echo "Not a git repository"
