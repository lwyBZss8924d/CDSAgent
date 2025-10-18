# Sub-Issue 07.01: Docker Compose - Multi-Service Orchestration

**Priority**: P1
**Owner**: DevOps Lead
**Timing**: Phase 4, Week 8
**PRD Reference**: [PRD-07 §2.1, §4.1](../../../prd/0.1.0-MVP-PRDs-v0/07-deployment-operations.md)

## Objective

Create Docker Compose configuration to orchestrate Index Service, CLI, and Agent with proper networking and volume persistence.

## Key Implementations

### Index Service Dockerfile

```dockerfile
# docker/index-service/Dockerfile
FROM rust:1.75 AS builder
WORKDIR /build
COPY cds-index/ ./
RUN cargo build --release --bin cds-index-service

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/target/release/cds-index-service /usr/local/bin/
EXPOSE 3030
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s \
  CMD curl -f http://localhost:3030/health || exit 1
CMD ["cds-index-service"]
```

### CLI Tools Dockerfile

```dockerfile
# docker/cli-tools/Dockerfile
FROM rust:1.75 AS builder
WORKDIR /build
COPY cds-tools/ ./
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates curl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/target/release/cds /usr/local/bin/
ENV INDEX_SERVICE_URL=http://index-service:3030
CMD ["bash"]
```

### Agent Dockerfile

```dockerfile
# docker/agent/Dockerfile
FROM oven/bun:1.1.34
WORKDIR /app
COPY cds-agent/package.json cds-agent/bun.lockb ./
RUN bun install
COPY cds-agent/ ./
ENV CLI_PATH=/usr/local/bin/cds
CMD ["bun", "run", "src/main.ts"]
```

### Docker Compose

```yaml
# docker-compose.yml
version: '3.8'

services:
  index-service:
    build:
      context: .
      dockerfile: docker/index-service/Dockerfile
    container_name: cds-index-service
    ports:
      - "3030:3030"
    volumes:
      - ./volumes/graph_index:/data/graph_index
      - ./volumes/bm25_index:/data/bm25_index
    environment:
      - GRAPH_INDEX_DIR=/data/graph_index
      - BM25_INDEX_DIR=/data/bm25_index
      - RUST_LOG=info
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3030/health"]
      interval: 30s
      timeout: 3s
      retries: 3
    networks:
      - cds-network

  cli-tools:
    build:
      context: .
      dockerfile: docker/cli-tools/Dockerfile
    container_name: cds-cli
    depends_on:
      index-service:
        condition: service_healthy
    environment:
      - INDEX_SERVICE_URL=http://index-service:3030
    volumes:
      - ./volumes/graph_index:/data/graph_index:ro
    networks:
      - cds-network
    command: tail -f /dev/null  # Keep container running for exec

  agent:
    build:
      context: .
      dockerfile: docker/agent/Dockerfile
    container_name: cds-agent
    depends_on:
      - cli-tools
    environment:
      - CLI_PATH=/usr/local/bin/cds
      - ANTHROPIC_API_KEY=${ANTHROPIC_API_KEY}
      - OPENAI_API_KEY=${OPENAI_API_KEY}
    volumes:
      - ./volumes/agent_logs:/app/logs
    networks:
      - cds-network

networks:
  cds-network:
    driver: bridge

volumes:
  graph_index:
  bm25_index:
  agent_logs:
```

## Build & Deploy Scripts

### build-all.sh

```bash
#!/bin/bash
set -e

echo "Building Index Service..."
docker build -t cds-index-service:latest -f docker/index-service/Dockerfile .

echo "Building CLI Tools..."
docker build -t cds-cli:latest -f docker/cli-tools/Dockerfile .

echo "Building Agent..."
docker build -t cds-agent:latest -f docker/agent/Dockerfile .

echo "All images built successfully!"
```

### deploy.sh

```bash
#!/bin/bash
set -e

# Validate .env exists
if [ ! -f .env ]; then
  echo "Error: .env file not found. Copy .env.example to .env and configure."
  exit 1
fi

# Start services
docker-compose up -d

# Wait for health checks
echo "Waiting for services to be healthy..."
timeout 60 bash -c 'until docker-compose ps | grep -q "healthy"; do sleep 2; done'

echo "Deployment complete! Services:"
docker-compose ps
```

## Acceptance Criteria

- [ ] All 3 services build successfully with Docker
- [ ] `docker-compose up` starts all services
- [ ] Health checks pass for Index Service
- [ ] CLI container can reach Index Service on cds-network
- [ ] Agent container can invoke CLI commands
- [ ] Volumes persist data across restarts

**Related**: [00-overview.md](00-overview.md), [02-env-config.md](02-env-config.md), [03-monitoring.md](03-monitoring.md)
