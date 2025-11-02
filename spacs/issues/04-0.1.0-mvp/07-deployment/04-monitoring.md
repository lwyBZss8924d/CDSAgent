# Sub-Issue 07.03: Monitoring & Health Checks - Prometheus Metrics

**Priority**: P1
**Owner**: DevOps Lead
**Timing**: Phase 4, Week 9
**PRD Reference**: [PRD-07 §2.3, §4.3](../../../prd/0.1.0-MVP-PRDs-v0/07-deployment-operations.md)

## Objective

Implement health check endpoints and Prometheus metrics for Index Service with basic alerts and Docker health checks.

## Key Implementations

### Health Check Endpoint (Rust)

```rust
// cds-index/src/routes/health.rs
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({
        "status": "healthy",
        "service": "cds-index-service",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

pub async fn readiness_check(
    graph_index: &GraphIndex,
    bm25_index: &BM25Index
) -> impl IntoResponse {
    let graph_loaded = graph_index.is_loaded();
    let bm25_loaded = bm25_index.is_loaded();

    if graph_loaded && bm25_loaded {
        (StatusCode::OK, Json(json!({
            "status": "ready",
            "graph_index": "loaded",
            "bm25_index": "loaded"
        })))
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, Json(json!({
            "status": "not_ready",
            "graph_index": if graph_loaded { "loaded" } else { "not_loaded" },
            "bm25_index": if bm25_loaded { "loaded" } else { "not_loaded" }
        })))
    }
}
```

### Prometheus Metrics (Rust)

```rust
// cds-index/src/metrics.rs
use prometheus::{
    register_histogram_vec, register_int_counter_vec, register_int_gauge,
    HistogramVec, IntCounterVec, IntGauge, Encoder, TextEncoder,
};
use lazy_static::lazy_static;

lazy_static! {
    // Request metrics
    pub static ref HTTP_REQUESTS_TOTAL: IntCounterVec = register_int_counter_vec!(
        "cds_http_requests_total",
        "Total HTTP requests by method and endpoint",
        &["method", "endpoint", "status"]
    ).unwrap();

    pub static ref HTTP_REQUEST_DURATION: HistogramVec = register_histogram_vec!(
        "cds_http_request_duration_seconds",
        "HTTP request latency in seconds",
        &["method", "endpoint"],
        vec![0.001, 0.01, 0.05, 0.1, 0.5, 1.0, 5.0]
    ).unwrap();

    // Index metrics
    pub static ref GRAPH_NODE_COUNT: IntGauge = register_int_gauge!(
        "cds_graph_node_count",
        "Total number of nodes in graph index"
    ).unwrap();

    pub static ref GRAPH_EDGE_COUNT: IntGauge = register_int_gauge!(
        "cds_graph_edge_count",
        "Total number of edges in graph index"
    ).unwrap();

    pub static ref BM25_DOCUMENT_COUNT: IntGauge = register_int_gauge!(
        "cds_bm25_document_count",
        "Total number of documents in BM25 index"
    ).unwrap();

    // Search metrics
    pub static ref SEARCH_QUERIES_TOTAL: IntCounterVec = register_int_counter_vec!(
        "cds_search_queries_total",
        "Total search queries by type",
        &["query_type"]
    ).unwrap();

    pub static ref SEARCH_DURATION: HistogramVec = register_histogram_vec!(
        "cds_search_duration_seconds",
        "Search query duration in seconds",
        &["query_type"],
        vec![0.001, 0.01, 0.05, 0.1, 0.5, 1.0]
    ).unwrap();
}

pub async fn metrics_handler() -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    (StatusCode::OK, buffer)
}
```

### Metrics Middleware

```rust
// cds-index/src/middleware/metrics.rs
use axum::{
    middleware::Next,
    response::Response,
    extract::Request,
};
use std::time::Instant;

pub async fn metrics_middleware(
    req: Request,
    next: Next,
) -> Response {
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let start = Instant::now();

    let response = next.run(req).await;

    let status = response.status().as_u16().to_string();
    let duration = start.elapsed().as_secs_f64();

    // Record metrics
    HTTP_REQUESTS_TOTAL
        .with_label_values(&[&method, &path, &status])
        .inc();

    HTTP_REQUEST_DURATION
        .with_label_values(&[&method, &path])
        .observe(duration);

    response
}
```

### Prometheus Configuration

```yaml
# config/prometheus.yml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'cds-index-service'
    static_configs:
      - targets: ['index-service:3030']
    metrics_path: '/metrics'
    scrape_interval: 10s

  # Future: Add CLI and Agent metrics
  # - job_name: 'cds-agent'
  #   static_configs:
  #     - targets: ['agent:8080']

alerting:
  alertmanagers:
    - static_configs:
        - targets: []

rule_files:
  - 'alerts.yml'
```

### Alert Rules

```yaml
# config/alerts.yml
groups:
  - name: cds_alerts
    interval: 30s
    rules:
      - alert: IndexServiceDown
        expr: up{job="cds-index-service"} == 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "CDS Index Service is down"
          description: "Index Service has been down for more than 1 minute"

      - alert: HighSearchLatency
        expr: histogram_quantile(0.95, cds_search_duration_seconds) > 1.0
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High search latency detected"
          description: "95th percentile search latency is above 1 second"

      - alert: HighErrorRate
        expr: rate(cds_http_requests_total{status=~"5.."}[5m]) > 0.1
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High error rate detected"
          description: "More than 10% of requests are failing"
```

### Docker Health Check Integration

```yaml
# docker-compose.yml health check section
services:
  index-service:
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3030/health"]
      interval: 30s
      timeout: 3s
      retries: 3
      start_period: 10s
    restart: unless-stopped
```

### Health Check Script

```shell
#!/bin/bash
# scripts/health-check.sh

set -e

SERVICES=("index-service" "cli-tools" "agent")
FAILED=0

for service in "${SERVICES[@]}"; do
  echo "Checking $service..."

  if docker-compose ps | grep "$service" | grep -q "Up"; then
    echo "✓ $service is running"
  else
    echo "✗ $service is NOT running"
    FAILED=1
  fi
done

# Check Index Service health endpoint
echo "Checking Index Service health endpoint..."
if curl -f http://localhost:3030/health > /dev/null 2>&1; then
  echo "✓ Health endpoint responding"
else
  echo "✗ Health endpoint NOT responding"
  FAILED=1
fi

# Check Prometheus metrics
echo "Checking Prometheus metrics..."
if curl -f http://localhost:3030/metrics > /dev/null 2>&1; then
  echo "✓ Metrics endpoint responding"
else
  echo "✗ Metrics endpoint NOT responding"
  FAILED=1
fi

if [ $FAILED -eq 0 ]; then
  echo "All health checks passed!"
  exit 0
else
  echo "Some health checks failed"
  exit 1
fi
```

## Sample Grafana Dashboard (Optional)

```json
{
  "dashboard": {
    "title": "CDSAgent Monitoring",
    "panels": [
      {
        "title": "Request Rate",
        "targets": [
          {
            "expr": "rate(cds_http_requests_total[5m])"
          }
        ]
      },
      {
        "title": "Search Latency (p95)",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, cds_search_duration_seconds)"
          }
        ]
      },
      {
        "title": "Index Size",
        "targets": [
          {
            "expr": "cds_graph_node_count"
          },
          {
            "expr": "cds_bm25_document_count"
          }
        ]
      }
    ]
  }
}
```

## Acceptance Criteria

- [ ] /health endpoint returns 200 when service ready
- [ ] /metrics endpoint exposes Prometheus metrics
- [ ] Docker health checks restart failed services
- [ ] Prometheus scrapes metrics every 10s
- [ ] Alert rules defined for service down, high latency, high error rate
- [ ] Manual health check script validates all services

**Related**: [00-overview.md](00-overview.md), [01-docker-compose.md](01-docker-compose.md), [../08-testing/02-integration.md](../08-testing/02-integration.md)
