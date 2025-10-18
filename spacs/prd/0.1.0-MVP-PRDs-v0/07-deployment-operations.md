# PRD-07: Deployment & Operations

**Version:** 1.0 (Round 1 - Concise)
**Date:** 2025-10-18
**Status:** Draft
**Parent:** PRD-01 System Architecture

---

## 1. Document Overview

### 1.1 Purpose

Define deployment models, operational procedures, configuration management, and monitoring for CDSAgent components in local and production environments.

### 1.2 Scope

- Deployment architectures (local daemon, containerized service)
- Configuration management (env vars, config files)
- Index management and incremental updates
- Monitoring, logging, and observability
- Resource requirements and scaling

---

## 2. Deployment Models

### 2.1 Model A: Local CLI (Simplest)

**Architecture**:

```text
User → cds CLI → Direct library calls → CDS-Index crates → Index files
```

**Characteristics**:

- No separate service process
- `cds` binary links directly to Rust crates
- Index loaded on each CLI invocation
- Suitable for: Developer laptops, single-user workflows

**Pros**: Simple, no network overhead, easy debugging
**Cons**: Slow startup (index load each time), no shared index across processes

**Deployment Steps**:

1. Install `cds` binary (`cargo install cds-cli`)
2. Set `GRAPH_INDEX_DIR` environment variable
3. Run `cds init <repo>` to build index
4. Use `cds search`, `cds traverse`, etc.

**Resource Requirements**:

- CPU: 2+ cores (for parallel indexing)
- Memory: 1-2GB per 10K files
- Disk: ~100MB per 1K files (index size)

### 2.2 Model B: Local Daemon (Recommended)

**Architecture**:

```text
User → cds CLI → JSON-RPC → cds-indexd (daemon) → Index files
         ↓                      ↑
    Claude Agent ──────────────┘
```

**Characteristics**:

- `cds-indexd` runs as background process
- Index loaded once, shared across clients
- HTTP/JSON-RPC interface on `localhost:9876`
- Suitable for: Interactive agent sessions, multi-process access

**Pros**: Fast queries (index in memory), shared state
**Cons**: Requires daemon management

**Deployment Steps**:

1. Start daemon: `cds-indexd --index-dir /path/to/index --port 9876 &`
2. Verify: `curl http://localhost:9876/health`
3. CLI auto-connects to daemon if running
4. Stop: `cds-indexd --stop` or `kill $(cat /tmp/cds-indexd.pid)`

**Daemon Management**:

```bash
# Systemd service (Linux)
[Unit]
Description=CDS Index Service
After=network.target

[Service]
Type=simple
ExecStart=/usr/local/bin/cds-indexd --config /etc/cds/config.toml
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

**Resource Requirements**:

- CPU: 1-2 cores (idle), 4+ (during indexing)
- Memory: 2-4GB (index in RAM)
- Disk: Same as Model A

### 2.3 Model C: Containerized Service (Production)

**Architecture**:

```text
Clients → Load Balancer → [cds-indexd containers] → Shared index volume
```

**Characteristics**:

- Docker/Podman containers
- Horizontal scaling (multiple instances)
- Shared index via mounted volume
- Suitable for: Team deployments, CI/CD integration

**Docker Compose Example**:

```yaml
version: '3.8'

services:
  cds-indexd:
    image: cdsagent/indexd:latest
    ports:
      - "9876:9876"
    volumes:
      - ./graph_index:/data/index:ro
    environment:
      - GRAPH_INDEX_DIR=/data/index
      - RUST_LOG=info
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:9876/health"]
      interval: 30s
      timeout: 10s
      retries: 3
```

**Kubernetes Deployment**:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: cds-indexd
spec:
  replicas: 3
  selector:
    matchLabels:
      app: cds-indexd
  template:
    spec:
      containers:
      - name: indexd
        image: cdsagent/indexd:1.0
        ports:
        - containerPort: 9876
        resources:
          requests:
            memory: "2Gi"
            cpu: "1"
          limits:
            memory: "4Gi"
            cpu: "2"
        volumeMounts:
        - name: index-volume
          mountPath: /data/index
      volumes:
      - name: index-volume
        persistentVolumeClaim:
          claimName: cds-index-pvc
```

**Pros**: Scalable, isolated, easy rollback
**Cons**: Complexity, requires container infrastructure

---

## 3. Configuration Management

### 3.1 Configuration File

**Location**: `~/.config/cds/config.toml` or `/etc/cds/config.toml`

**Schema**:

```toml
[index]
# Path to graph index (overridden by GRAPH_INDEX_DIR env var)
graph_dir = "/data/graph_index_v2.3"

# Path to BM25 index
bm25_dir = "/data/BM25_index"

# Auto-refresh interval (0 = manual only)
refresh_interval_secs = 0

[service]
# Service mode (daemon vs direct)
mode = "daemon"  # or "direct"

# Daemon host and port
host = "127.0.0.1"
port = 9876

# Max concurrent clients
max_clients = 10

[search]
# Default search result limit
default_limit = 10

# BM25 fallback threshold
bm25_threshold = 5

[performance]
# Number of indexing threads
indexing_threads = 0  # 0 = num_cpus

# Enable memory mapping for large indices
use_mmap = true

# Cache size (MB)
cache_size_mb = 512

[logging]
# Log level: trace, debug, info, warn, error
level = "info"

# Log file path (empty = stdout)
file = ""

# Log format: json or text
format = "text"
```

### 3.2 Environment Variables

**Priority**: Environment variables > Config file > Defaults

| Variable | Purpose | Example |
|----------|---------|---------|
| `GRAPH_INDEX_DIR` | Graph index path | `/data/graph_index` |
| `BM25_INDEX_DIR` | BM25 index path | `/data/BM25_index` |
| `CDS_CONFIG` | Config file path | `/etc/cds/config.toml` |
| `CDS_SERVICE_URL` | Daemon URL | `http://localhost:9876` |
| `RUST_LOG` | Logging level | `info`, `debug` |
| `CDS_CACHE_SIZE` | Cache size (MB) | `1024` |

### 3.3 CLI Configuration Commands

```bash
# View current config
cds config list

# Get specific value
cds config get index.graph_dir

# Set value (writes to config file)
cds config set search.default_limit 20

# Validate config
cds config validate
```

---

## 4. Index Management

### 4.1 Initial Indexing

```bash
# Index a repository
cds init /path/to/repo --languages python,typescript --output /data/index

# Options:
#   --incremental    : Update existing index (skip unchanged files)
#   --jobs N         : Parallel workers (default: num_cpus)
#   --exclude PATTERN: Ignore files (e.g., "test_*,*_test.py")
```

**Output**:

```text
Indexing repository: /path/to/repo
Languages: python, typescript
Output directory: /data/index

[====================] 1234/1234 files (100%)

Summary:
  Files indexed: 1234
  Entities found:
    Directories: 89
    Files: 1234
    Classes: 456
    Functions: 3210
  Edges created:
    Contain: 4500
    Import: 890
    Invoke: 6700
    Inherit: 234
  Index size: 123 MB
  Build time: 8.5s
```

### 4.2 Incremental Updates

**Trigger**: File changes detected (manual or via file watcher)

```bash
# Update index after code changes
cds update /path/to/repo

# Automatically watch for changes (daemon mode)
cds-indexd --watch --index-dir /data/index
```

**Process**:

1. Detect changed files (via git status, mtime, or inotify)
2. Re-parse changed files only
3. Update affected nodes/edges in graph
4. Invalidate BM25 index entries for changed files
5. Rebuild BM25 for changed files only

**Performance**: <500ms for single file update

### 4.3 Index Versioning

**Version in metadata**:

```json
{
  "version": "2.3",
  "created_at": "2025-10-18T10:30:00Z",
  "repo_path": "/path/to/repo",
  "languages": ["python"],
  "stats": { /* ... */ }
}
```

**Compatibility Check**:

```rust
// cds_storage/src/loader.rs
fn load_index(path: &Path) -> Result<Index> {
    let metadata = read_metadata(path)?;

    if metadata.version != CURRENT_VERSION {
        return Err(Error::IncompatibleVersion(metadata.version));
    }

    // Load index...
}
```

**Migration**:

```bash
# Migrate old index to new version
cds migrate-index --from v2.2 --to v2.3 /data/old_index /data/new_index
```

---

## 5. Monitoring and Observability

### 5.1 Health Check Endpoint

**URL**: `GET /health`

**Response**:

```json
{
  "status": "healthy",
  "uptime_secs": 3600,
  "index_loaded": true,
  "index_stats": {
    "total_nodes": 5000,
    "total_edges": 12000,
    "last_updated": "2025-10-18T10:30:00Z"
  },
  "memory_usage_mb": 1024,
  "active_clients": 3
}
```

**Status Codes**:

- 200: Healthy
- 503: Index not loaded or corrupted

### 5.2 Metrics Endpoint

**URL**: `GET /metrics` (Prometheus format)

**Metrics**:

```prometheus
# HELP cds_search_requests_total Total search requests
# TYPE cds_search_requests_total counter
cds_search_requests_total 1234

# HELP cds_search_latency_seconds Search query latency
# TYPE cds_search_latency_seconds histogram
cds_search_latency_seconds_bucket{le="0.1"} 800
cds_search_latency_seconds_bucket{le="0.5"} 1200
cds_search_latency_seconds_sum 456.7
cds_search_latency_seconds_count 1234

# HELP cds_index_size_bytes Index size in bytes
# TYPE cds_index_size_bytes gauge
cds_index_size_bytes 123456789
```

### 5.3 Logging

**Structured Logging** (JSON format):

```json
{
  "timestamp": "2025-10-18T10:30:00Z",
  "level": "info",
  "message": "Search query completed",
  "query": "sanitize input",
  "results_count": 5,
  "latency_ms": 120,
  "client_id": "agent_session_123"
}
```

**Log Levels**:

- `trace`: Detailed execution (AST parsing steps)
- `debug`: Tool calls, cache hits/misses
- `info`: Requests, index updates
- `warn`: Deprecated features, slow queries
- `error`: Failures, exceptions

**Log Rotation** (using `tracing-appender`):

```toml
[logging]
file = "/var/log/cds/indexd.log"
max_size_mb = 100
max_files = 10
```

---

## 6. Resource Requirements

### 6.1 Index Storage

| Repository Size | Graph Index | BM25 Index | Total |
|----------------|-------------|------------|-------|
| 1K files | ~10 MB | ~20 MB | ~30 MB |
| 10K files | ~100 MB | ~200 MB | ~300 MB |
| 50K files | ~500 MB | ~1 GB | ~1.5 GB |

### 6.2 Runtime Memory

| Component | Baseline | Per 10K Files | Notes |
|-----------|----------|---------------|-------|
| Graph (in-memory) | 50 MB | +100 MB | Nodes/edges |
| BM25 index (mmap) | 10 MB | +50 MB | Inverted index |
| Search cache | 100 MB | +100 MB | Configurable |
| Total (daemon) | 160 MB | +250 MB | |

### 6.3 CPU Usage

| Operation | CPU (cores) | Duration |
|-----------|-------------|----------|
| Initial indexing (1K files) | 4 | 5s |
| Search query | 0.5 | <100ms |
| Traverse 2-hop | 0.5 | <200ms |
| Incremental update | 1 | <500ms |

---

## 7. Security Considerations

### 7.1 Local Daemon Security

- **Bind to localhost only**: Prevent external access
- **No authentication** (local trust model)
- **Read-only index**: Daemon cannot modify source code
- **Sandboxing**: Run daemon as unprivileged user

### 7.2 Production Security

- **TLS/HTTPS**: Encrypt daemon communication
- **API Key Authentication**: Token-based access
- **Rate Limiting**: Prevent DoS
- **Network Policies**: Restrict ingress/egress

---

## 8. Operational Procedures

### 8.1 Startup Checklist

- [ ] Verify index directory exists and is readable
- [ ] Check config file validity (`cds config validate`)
- [ ] Start daemon (if using Model B/C)
- [ ] Verify health endpoint returns 200
- [ ] Test sample query (`cds search "test"`)

### 8.2 Troubleshooting

| Issue | Diagnosis | Solution |
|-------|-----------|----------|
| "Index not found" | `GRAPH_INDEX_DIR` not set | Set env var or config |
| Slow queries | Index too large for memory | Enable mmap, increase cache |
| Daemon won't start | Port already in use | Change port or kill process |
| Search returns no results | Index out of date | Run `cds update` |

### 8.3 Backup and Recovery

**Backup**:

```bash
# Backup index
tar -czf cds-index-backup-$(date +%Y%m%d).tar.gz /data/graph_index
```

**Recovery**:

```bash
# Restore index
tar -xzf cds-index-backup-20251018.tar.gz -C /data/
```

---

## 9. Acceptance Criteria

- [ ] Can deploy in all 3 models (local CLI, daemon, containerized)
- [ ] Config file and env vars work correctly
- [ ] Incremental index updates functional
- [ ] Health/metrics endpoints return valid data
- [ ] Logs are structured and configurable
- [ ] Resource usage within targets (§6)

---

## 10. Open Questions

1. **Auto-indexing**: Should daemon auto-detect code changes via inotify? (v1.1 feature)
2. **Multi-repo**: Support indexing multiple repos in one daemon? (Future)
3. **Cloud Deployment**: AWS ECS, GCP Cloud Run support? (Post-v1.0)

---

**Status**: Ready for deployment testing. Requires finalized CDS-Index and CDS-Tools (PRD-02, PRD-03).
