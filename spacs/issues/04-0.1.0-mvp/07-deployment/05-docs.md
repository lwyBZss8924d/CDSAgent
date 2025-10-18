# Sub-Issue 07.04: Deployment Documentation - Setup Guides & Troubleshooting

**Priority**: P1
**Owner**: DevOps Lead + Technical Writer
**Timing**: Phase 4, Week 10
**PRD Reference**: [PRD-07 §2.4, §5](../../../prd/0.1.0-MVP-PRDs-v0/07-deployment-operations.md)

## Objective

Provide comprehensive deployment documentation including quick start guide, troubleshooting, production checklist, and API key setup.

## Key Deliverables

### Quick Start Guide (deployment/README.md)

```markdown
# CDSAgent Deployment - Quick Start Guide

Get CDSAgent running in under 10 minutes.

## Prerequisites

- Docker 20.10+ and Docker Compose 2.0+
- 4GB RAM minimum (8GB recommended)
- 10GB disk space for indexes
- API keys: Anthropic Claude (required), OpenAI (optional)

## 1. Clone and Configure

```bash
git clone https://github.com/your-org/CDSAgent.git
cd CDSAgent/deployment

# Copy environment template
cp config/.env.example config/.env

# Edit .env and set required variables
nano config/.env
```

**Required configuration**:

- `ANTHROPIC_API_KEY`: Your Claude API key from console.anthropic.com

## 2. Build and Deploy

```bash
# Build all Docker images (first time only, ~5 minutes)
./scripts/build-all.sh

# Start all services
docker-compose up -d

# Verify services are healthy
./scripts/health-check.sh
```

## 3. Verify Installation

```bash
# Check service status
docker-compose ps

# Test Index Service health endpoint
curl http://localhost:3030/health

# Test CLI (inside container)
docker-compose exec cli-tools cds search "function" --limit 5

# View agent logs
docker-compose logs -f agent
```

## 4. Build Your First Index

```bash
# Copy your codebase to the index volume
cp -r /path/to/your/repo ./volumes/graph_index/my_repo

# Rebuild graph index
docker-compose exec index-service cds-index-build \
  --repo-path /data/graph_index/my_repo \
  --language python

# Verify index was created
docker-compose exec cli-tools cds search "MyClass" --type class
```

## Next Steps

- Read [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for common issues
- See [PRODUCTION.md](PRODUCTION.md) for production deployment
- Explore [../docs/CLI.md](../docs/CLI.md) for CLI usage

## Stopping Services

```bash
# Stop all services (preserves data)
docker-compose down

# Stop and remove volumes (deletes indexes)
docker-compose down -v
```

### Troubleshooting Guide (deployment/TROUBLESHOOTING.md)

```markdown
# CDSAgent Troubleshooting Guide

Common issues and solutions.

## Services Won't Start

### Error: "Port 3030 already in use"

**Cause**: Another service is using port 3030.

**Solution**:
```bash
# Find process using port 3030
lsof -i :3030

# Kill the process or change INDEX_SERVICE_PORT in .env
export INDEX_SERVICE_PORT=3031
docker-compose up -d
```

### Error: "ANTHROPIC_API_KEY not set"

**Cause**: Missing required API key in .env file.

**Solution**:

```bash
# Edit .env and add your key
echo "ANTHROPIC_API_KEY=sk-ant-..." >> config/.env

# Restart agent service
docker-compose restart agent
```

## Health Checks Failing

### Error: "Health check failed for index-service"

**Symptoms**: `docker-compose ps` shows index-service as unhealthy.

**Diagnosis**:

```bash
# Check service logs
docker-compose logs index-service

# Check health endpoint manually
curl http://localhost:3030/health
```

**Common causes**:

1. **Index directories not mounted**: Verify volumes in docker-compose.yml
2. **Insufficient memory**: Increase Docker memory limit to 4GB+
3. **Index corruption**: Remove volumes and rebuild

**Solution**:

```bash
# Recreate volumes
docker-compose down -v
docker-compose up -d
```

## Index Build Failures

### Error: "tree-sitter parse failed"

**Cause**: Unsupported language or malformed source file.

**Solution**:

```bash
# Check supported languages (v0.1.0: Python only)
# For multi-language support, wait for v0.2.0 or file a feature request
```

### Error: "Permission denied: /data/graph_index"

**Cause**: Volume mount permissions issue.

**Solution**:

```bash
# Fix permissions on host
chmod -R 755 volumes/graph_index

# Or run container as your user (edit docker-compose.yml)
user: "${UID}:${GID}"
```

## Agent Issues

### Error: "Claude API rate limit exceeded"

**Cause**: Too many requests to Anthropic API.

**Solution**:

```bash
# Reduce agent max iterations in .env
echo "AGENT_MAX_ITERATIONS=5" >> config/.env
docker-compose restart agent

# Or upgrade your Anthropic plan
```

### Error: "CLI command timed out"

**Cause**: Large codebase causing slow searches.

**Diagnosis**:

```bash
# Check search performance
docker-compose exec cli-tools time cds search "function" --limit 10
```

**Solution**:

- Reduce `CLI_MAX_RESULTS` in .env
- Optimize index with BM25 pruning (see PRD-02)
- Consider upgrading server resources

## Docker Issues

### Error: "Cannot connect to Docker daemon"

**Cause**: Docker not running or insufficient permissions.

**Solution**:

```bash
# Start Docker daemon
sudo systemctl start docker

# Add user to docker group (Linux)
sudo usermod -aG docker $USER
newgrp docker
```

### Error: "Build failed: no space left on device"

**Cause**: Insufficient disk space.

**Solution**:

```bash
# Clean up Docker resources
docker system prune -a --volumes

# Free up space on host
df -h  # Check disk usage
```

## Performance Issues

### Symptom: "Searches take >10 seconds"

**Diagnosis**:

```bash
# Check Prometheus metrics
curl http://localhost:3030/metrics | grep cds_search_duration

# Profile with verbose logging
docker-compose exec index-service cds-index-service --log-level debug
```

**Solutions**:

1. Increase Docker memory allocation
2. Reduce index size (prune unused files)
3. Switch to faster storage (SSD)

## Getting More Help

- Check logs: `docker-compose logs -f <service-name>`
- File issues: <https://github.com/your-org/CDSAgent/issues>
- Join Discord: <https://discord.gg/cdsagent>

### Production Checklist (deployment/PRODUCTION.md)

```markdown
# Production Deployment Checklist

Pre-launch validation for production environments.

## Security

- [ ] API keys stored in secure secret manager (not .env files)
- [ ] Docker secrets configured (`docker secret create`)
- [ ] .env files added to .gitignore
- [ ] TLS/HTTPS enabled for Index Service
- [ ] Network policies restrict container communication
- [ ] Docker images scanned for vulnerabilities (`trivy` or `snyk`)

## Configuration

- [ ] ENV_PROFILE set to `prod`
- [ ] RUST_LOG set to `warn` or `error`
- [ ] Health checks configured with appropriate intervals
- [ ] Resource limits defined (CPU, memory)
- [ ] Volume backups configured
- [ ] Log rotation enabled

## Monitoring

- [ ] Prometheus scraping Index Service metrics
- [ ] Grafana dashboards configured
- [ ] Alerts configured for:
  - Service down (>1 minute)
  - High search latency (p95 >1s)
  - High error rate (>10%)
- [ ] Log aggregation enabled (ELK/Loki)
- [ ] Uptime monitoring configured (PagerDuty, Opsgenie)

## Performance

- [ ] Index pre-built before deployment (not built on startup)
- [ ] Docker image layers cached for fast rebuilds
- [ ] Resource requests/limits tuned based on load testing
- [ ] BM25 index optimized (pruned if >10GB)
- [ ] Load tested with expected query volume

## Backup & Recovery

- [ ] Index volumes backed up regularly
- [ ] Backup restoration tested
- [ ] Disaster recovery plan documented
- [ ] RTO/RPO defined and validated

## Documentation

- [ ] Deployment runbook created
- [ ] On-call playbook created
- [ ] API documentation published
- [ ] User training completed

## Validation

- [ ] Integration tests pass in production environment
- [ ] Smoke tests pass after deployment
- [ ] Performance benchmarks meet SLA (search <500ms p95)
- [ ] Security scan passed

## Rollback Plan

- [ ] Previous Docker images tagged and retained
- [ ] Rollback procedure documented and tested
- [ ] Downtime window communicated to stakeholders

### API Key Setup Guide (deployment/docs/API_KEYS.md)

```markdown
# API Key Setup Guide

## Anthropic Claude (Required)

1. **Create Account**: Visit <https://console.anthropic.com>
2. **Generate API Key**:
   - Navigate to Settings → API Keys
   - Click "Create Key"
   - Name it "CDSAgent Production"
   - Copy the key (starts with `sk-ant-`)
3. **Set in .env**:
   ```bash
   ANTHROPIC_API_KEY=sk-ant-api03-...
   ```

**Rate Limits** (as of 2025):

- Claude 4.5 Sonnet: 100 requests/min

## OpenAI (Optional)

1. **Create Account**: Visit <https://platform.openai.com>
2. **Generate API Key**:
   - Navigate to API Keys
   - Click "Create new secret key"
   - Copy the key (starts with `sk-`)
3. **Set in .env**:

   ```bash
   OPENAI_API_KEY=sk-...
   ```

**Usage**: Only required if using GPT models via multi-SDK adapter (v0.2.0 feature).

## Securing API Keys

### Development

Store in `.env` file (gitignored):

```bash
cp config/.env.example config/.env
# Edit .env with your keys
```

### Production

Use Docker secrets:

```bash
# Create secrets
echo "sk-ant-..." | docker secret create anthropic_api_key -

# Reference in docker-compose.yml
services:
  agent:
    secrets:
      - anthropic_api_key
    environment:
      - ANTHROPIC_API_KEY_FILE=/run/secrets/anthropic_api_key
```

Or use external secret managers:

- **AWS**: Secrets Manager + ECS task definitions
- **GCP**: Secret Manager + Cloud Run
- **Azure**: Key Vault + AKS
- **HashiCorp Vault**: Dynamic secrets

## Rotating Keys

1. Generate new key in provider console
2. Update .env or secret manager
3. Restart affected services:

   ```bash
   docker-compose restart agent
   ```

4. Revoke old key after validation

## Troubleshooting

Error: "API key invalid"

- Verify key format (starts with `sk-ant-` for Claude)
- Check for trailing whitespace in .env
- Confirm key is active in provider console

Error: "Rate limit exceeded"

- Reduce `AGENT_MAX_ITERATIONS`
- Implement request throttling
- Upgrade provider plan

## Acceptance Criteria

- [ ] README.md provides quick start in <10 minutes
- [ ] TROUBLESHOOTING.md covers 5+ common issues
- [ ] PRODUCTION.md includes security and monitoring checklist
- [ ] API_KEYS.md documents setup for Anthropic and OpenAI
- [ ] All docs tested by new user (usability validation)
- [ ] Docs include examples for Docker commands

**Related**: [00-overview.md](00-overview.md), [01-docker-compose.md](01-docker-compose.md), [02-env-config.md](02-env-config.md), [03-monitoring.md](03-monitoring.md)
