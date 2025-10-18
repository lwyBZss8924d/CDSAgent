# Sub-Issue 07.02: Environment Configuration - .env Templates & Validation

**Priority**: P1
**Owner**: DevOps Lead
**Timing**: Phase 4, Week 8
**PRD Reference**: [PRD-07 §2.2, §4.2](../../../prd/0.1.0-MVP-PRDs-v0/07-deployment-operations.md)

## Objective

Provide environment-based configuration with .env templates, validation, and support for dev/staging/prod profiles.

## Key Implementations

### .env.example Template

```bash
# .env.example - Copy to .env and configure

# ===== Environment Profile =====
ENV_PROFILE=dev  # dev, staging, prod

# ===== Index Service Configuration =====
GRAPH_INDEX_DIR=/data/graph_index
BM25_INDEX_DIR=/data/bm25_index
INDEX_SERVICE_PORT=3030
INDEX_SERVICE_HOST=0.0.0.0
RUST_LOG=info  # debug, info, warn, error

# ===== CLI Tools Configuration =====
INDEX_SERVICE_URL=http://index-service:3030
CLI_OUTPUT_FORMAT=json  # json, text, tree
CLI_MAX_RESULTS=100

# ===== Agent Configuration =====
# Required: Anthropic API key for Claude
ANTHROPIC_API_KEY=sk-ant-...
# Optional: OpenAI API key (if using GPT models)
OPENAI_API_KEY=sk-...

# Agent behavior
AGENT_MAX_ITERATIONS=10
AGENT_LOG_DIR=/app/logs
AGENT_STREAM_MODE=true

# ===== Monitoring (Prometheus) =====
PROMETHEUS_ENABLED=true
PROMETHEUS_PORT=9090
METRICS_ENDPOINT=/metrics

# ===== Feature Flags =====
ENABLE_HEALTH_CHECKS=true
ENABLE_REQUEST_LOGGING=true

# ===== Volumes (Docker-specific) =====
# These paths are for volume mounts
GRAPH_INDEX_VOLUME=./volumes/graph_index
BM25_INDEX_VOLUME=./volumes/bm25_index
AGENT_LOGS_VOLUME=./volumes/agent_logs
```

### Configuration Validation (Rust)

```rust
// cds-index/src/config.rs
use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct IndexServiceConfig {
    pub graph_index_dir: PathBuf,
    pub bm25_index_dir: PathBuf,
    pub port: u16,
    pub host: String,
    pub log_level: String,
}

impl IndexServiceConfig {
    pub fn from_env() -> Result<Self> {
        let graph_index_dir = env::var("GRAPH_INDEX_DIR")
            .context("GRAPH_INDEX_DIR not set")?
            .into();

        let bm25_index_dir = env::var("BM25_INDEX_DIR")
            .context("BM25_INDEX_DIR not set")?
            .into();

        let port = env::var("INDEX_SERVICE_PORT")
            .unwrap_or_else(|_| "3030".to_string())
            .parse()
            .context("Invalid INDEX_SERVICE_PORT")?;

        let host = env::var("INDEX_SERVICE_HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string());

        let log_level = env::var("RUST_LOG")
            .unwrap_or_else(|_| "info".to_string());

        Ok(Self {
            graph_index_dir,
            bm25_index_dir,
            port,
            host,
            log_level,
        })
    }

    pub fn validate(&self) -> Result<()> {
        // Validate directories exist (or can be created)
        if !self.graph_index_dir.exists() {
            std::fs::create_dir_all(&self.graph_index_dir)
                .context("Failed to create GRAPH_INDEX_DIR")?;
        }

        if !self.bm25_index_dir.exists() {
            std::fs::create_dir_all(&self.bm25_index_dir)
                .context("Failed to create BM25_INDEX_DIR")?;
        }

        // Validate port range
        if self.port < 1024 {
            anyhow::bail!("INDEX_SERVICE_PORT must be >= 1024");
        }

        Ok(())
    }
}
```

### Configuration Validation (TypeScript/Agent)

```typescript
// cds-agent/src/config.ts
import { z } from 'zod';

const AgentConfigSchema = z.object({
  anthropicApiKey: z.string().min(1, "ANTHROPIC_API_KEY required"),
  openaiApiKey: z.string().optional(),
  maxIterations: z.number().int().min(1).max(50).default(10),
  logDir: z.string().default('/app/logs'),
  streamMode: z.boolean().default(true),
});

export type AgentConfig = z.infer<typeof AgentConfigSchema>;

export function loadAgentConfig(): AgentConfig {
  const raw = {
    anthropicApiKey: process.env.ANTHROPIC_API_KEY,
    openaiApiKey: process.env.OPENAI_API_KEY,
    maxIterations: parseInt(process.env.AGENT_MAX_ITERATIONS || '10', 10),
    logDir: process.env.AGENT_LOG_DIR,
    streamMode: process.env.AGENT_STREAM_MODE === 'true',
  };

  try {
    return AgentConfigSchema.parse(raw);
  } catch (error) {
    console.error("Configuration validation failed:", error);
    process.exit(1);
  }
}
```

### Profile-Based Configuration

```bash
# scripts/load-profile.sh
#!/bin/bash

PROFILE=${ENV_PROFILE:-dev}

case $PROFILE in
  dev)
    export RUST_LOG=debug
    export CLI_OUTPUT_FORMAT=text
    export AGENT_STREAM_MODE=true
    ;;
  staging)
    export RUST_LOG=info
    export CLI_OUTPUT_FORMAT=json
    export AGENT_STREAM_MODE=false
    ;;
  prod)
    export RUST_LOG=warn
    export CLI_OUTPUT_FORMAT=json
    export AGENT_STREAM_MODE=false
    export PROMETHEUS_ENABLED=true
    ;;
  *)
    echo "Unknown profile: $PROFILE"
    exit 1
    ;;
esac

echo "Loaded $PROFILE profile"
```

## Gitignore Rules

```gitignore
# .gitignore additions
.env
.env.local
.env.*.local
volumes/
*.log
```

## Acceptance Criteria

- [ ] .env.example documents all required/optional variables
- [ ] Services fail-fast with clear error if required vars missing
- [ ] Configuration validation runs on startup
- [ ] .env files excluded from git
- [ ] Profile switching works (dev/staging/prod)
- [ ] API keys never committed to repo

**Related**: [00-overview.md](00-overview.md), [01-docker-compose.md](01-docker-compose.md), [04-docs.md](04-docs.md)
