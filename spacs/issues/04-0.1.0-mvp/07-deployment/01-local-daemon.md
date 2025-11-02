# Sub-Issue 07.01: Local Daemon - systemd/launchd Configuration

**Priority**: P1
**Owner**: DevOps Lead
**Timing**: Phase 4, Week 8
**PRD Reference**: [PRD-07 §2.1, §4.1](../../../prd/0.1.0-MVP-PRDs-v0/07-deployment-operations.md)

## Objective

Configure CDS-Index Service to run as a local system daemon using systemd (Linux) or launchd (macOS) with automatic restart, health monitoring, and user-mode operation.

## Key Implementations

### systemd Service Unit (Linux)

```ini
# /etc/systemd/system/cds-index.service
[Unit]
Description=CDSAgent Index Service
Documentation=https://github.com/your-org/CDSAgent
After=network.target
Wants=network-online.target

[Service]
Type=notify
User=cdsagent
Group=cdsagent
WorkingDirectory=/opt/cdsagent

# Environment variables
Environment="GRAPH_INDEX_DIR=/var/lib/cdsagent/graph_index"
Environment="BM25_INDEX_DIR=/var/lib/cdsagent/bm25_index"
Environment="RUST_LOG=info"
Environment="INDEX_SERVICE_PORT=3030"
Environment="INDEX_SERVICE_HOST=127.0.0.1"

# Security hardening
PrivateTmp=true
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/cdsagent /var/log/cdsagent

# Process management
ExecStart=/usr/local/bin/cds-index-service
Restart=on-failure
RestartSec=5s
TimeoutStartSec=30s
TimeoutStopSec=30s

# Resource limits
LimitNOFILE=65536
MemoryMax=4G

# Logging
StandardOutput=journal
StandardError=journal
SyslogIdentifier=cds-index

[Install]
WantedBy=multi-user.target
```

### launchd Property List (macOS)

```xml
<!-- ~/Library/LaunchAgents/com.cdsagent.index.plist -->
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.cdsagent.index</string>

    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/cds-index-service</string>
    </array>

    <key>EnvironmentVariables</key>
    <dict>
        <key>GRAPH_INDEX_DIR</key>
        <string>/Users/YOUR_USER/Library/Application Support/cdsagent/graph_index</string>
        <key>BM25_INDEX_DIR</key>
        <string>/Users/YOUR_USER/Library/Application Support/cdsagent/bm25_index</string>
        <key>RUST_LOG</key>
        <string>info</string>
        <key>INDEX_SERVICE_PORT</key>
        <string>3030</string>
        <key>INDEX_SERVICE_HOST</key>
        <string>127.0.0.1</string>
    </dict>

    <key>WorkingDirectory</key>
    <string>/usr/local/opt/cdsagent</string>

    <key>StandardOutPath</key>
    <string>/Users/YOUR_USER/Library/Logs/cdsagent/stdout.log</string>

    <key>StandardErrorPath</key>
    <string>/Users/YOUR_USER/Library/Logs/cdsagent/stderr.log</string>

    <key>RunAtLoad</key>
    <true/>

    <key>KeepAlive</key>
    <dict>
        <key>SuccessfulExit</key>
        <false/>
        <key>Crashed</key>
        <true/>
    </dict>

    <key>ThrottleInterval</key>
    <integer>10</integer>

    <key>ProcessType</key>
    <string>Interactive</string>

    <key>Nice</key>
    <integer>0</integer>
</dict>
</plist>
```

### Installation Scripts

#### Linux (systemd)

```shell
#!/bin/bash
# scripts/install-daemon-linux.sh

set -e

echo "Installing CDS-Index Service daemon (systemd)..."

# Check for root
if [ "$EUID" -ne 0 ]; then
  echo "Please run as root (sudo)"
  exit 1
fi

# Create service user
if ! id -u cdsagent >/dev/null 2>&1; then
  useradd -r -s /bin/false -d /opt/cdsagent -m cdsagent
  echo "Created cdsagent user"
fi

# Create directories
mkdir -p /opt/cdsagent
mkdir -p /var/lib/cdsagent/{graph_index,bm25_index}
mkdir -p /var/log/cdsagent

# Set ownership
chown -R cdsagent:cdsagent /opt/cdsagent /var/lib/cdsagent /var/log/cdsagent

# Install binary
cp target/release/cds-index-service /usr/local/bin/
chmod 755 /usr/local/bin/cds-index-service

# Install systemd unit
cp deployment/systemd/cds-index.service /etc/systemd/system/
chmod 644 /etc/systemd/system/cds-index.service

# Reload systemd
systemctl daemon-reload

# Enable and start service
systemctl enable cds-index.service
systemctl start cds-index.service

# Check status
systemctl status cds-index.service

echo "✓ CDS-Index Service installed and started"
echo "  Check logs: journalctl -u cds-index -f"
echo "  Stop: systemctl stop cds-index"
echo "  Restart: systemctl restart cds-index"
```

#### macOS (launchd)

```shell
#!/bin/bash
# scripts/install-daemon-macos.sh

set -e

echo "Installing CDS-Index Service daemon (launchd)..."

# Determine user
USER=$(whoami)
PLIST_PATH="$HOME/Library/LaunchAgents/com.cdsagent.index.plist"

# Create directories
mkdir -p "$HOME/Library/Application Support/cdsagent"/{graph_index,bm25_index}
mkdir -p "$HOME/Library/Logs/cdsagent"

# Install binary
sudo cp target/release/cds-index-service /usr/local/bin/
sudo chmod 755 /usr/local/bin/cds-index-service

# Customize plist with actual username
sed "s/YOUR_USER/$USER/g" deployment/launchd/com.cdsagent.index.plist > "$PLIST_PATH"

# Load service
launchctl unload "$PLIST_PATH" 2>/dev/null || true
launchctl load "$PLIST_PATH"

# Verify
sleep 2
if launchctl list | grep -q com.cdsagent.index; then
  echo "✓ CDS-Index Service installed and started"
  echo "  Check logs: tail -f $HOME/Library/Logs/cdsagent/stdout.log"
  echo "  Stop: launchctl unload $PLIST_PATH"
  echo "  Restart: launchctl kickstart -k gui/$(id -u)/com.cdsagent.index"
else
  echo "✗ Service failed to start"
  exit 1
fi
```

### Health Check Integration

```rust
// cds-index/src/bin/cds-index-service.rs
use systemd_daemon::{self, State};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = IndexServiceConfig::from_env()?;
    config.validate()?;

    // Build indexes
    let graph_index = load_or_build_graph_index(&config.graph_index_dir).await?;
    let bm25_index = load_or_build_bm25_index(&config.bm25_index_dir).await?;

    // Start HTTP server
    let app = build_router(graph_index, bm25_index);
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));

    // Notify systemd that we're ready (if running under systemd)
    if systemd_daemon::booted() {
        systemd_daemon::notify(false, &[State::Ready])?;
        tracing::info!("Notified systemd: service ready");
    }

    tracing::info!("CDS-Index Service listening on {}", addr);

    // Run server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    // Notify systemd we're stopping
    if systemd_daemon::booted() {
        systemd_daemon::notify(false, &[State::Stopping])?;
    }

    Ok(())
}

async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutdown signal received, stopping server...");
}
```

### Configuration Defaults (PRD-04 FR-HOOK-1 Alignment)

```toml
# /opt/cdsagent/config.toml (Linux)
# or ~/Library/Application Support/cdsagent/config.toml (macOS)

[service]
host = "127.0.0.1"
port = 3030
log_level = "info"

[indexes]
graph_dir = "/var/lib/cdsagent/graph_index"  # Linux
# graph_dir = "~/Library/Application Support/cdsagent/graph_index"  # macOS
bm25_dir = "/var/lib/cdsagent/bm25_index"

[performance]
max_connections = 100
request_timeout_secs = 30

[monitoring]
health_check_path = "/health"
metrics_path = "/metrics"
```

### Management Scripts

```shell
#!/bin/bash
# scripts/daemon-ctl.sh - Unified daemon control script

set -e

PLATFORM=$(uname -s)

case "$PLATFORM" in
  Linux)
    SERVICE="cds-index.service"
    case "$1" in
      start)   sudo systemctl start $SERVICE ;;
      stop)    sudo systemctl stop $SERVICE ;;
      restart) sudo systemctl restart $SERVICE ;;
      status)  systemctl status $SERVICE ;;
      logs)    journalctl -u $SERVICE -f ;;
      enable)  sudo systemctl enable $SERVICE ;;
      disable) sudo systemctl disable $SERVICE ;;
      *)       echo "Usage: $0 {start|stop|restart|status|logs|enable|disable}" ;;
    esac
    ;;
  Darwin)
    PLIST="$HOME/Library/LaunchAgents/com.cdsagent.index.plist"
    LABEL="com.cdsagent.index"
    case "$1" in
      start)   launchctl load "$PLIST" ;;
      stop)    launchctl unload "$PLIST" ;;
      restart) launchctl kickstart -k "gui/$(id -u)/$LABEL" ;;
      status)  launchctl list | grep "$LABEL" || echo "Service not running" ;;
      logs)    tail -f "$HOME/Library/Logs/cdsagent/stdout.log" ;;
      enable)  launchctl load "$PLIST" ;;
      disable) launchctl unload "$PLIST" ;;
      *)       echo "Usage: $0 {start|stop|restart|status|logs|enable|disable}" ;;
    esac
    ;;
  *)
    echo "Unsupported platform: $PLATFORM"
    exit 1
    ;;
esac
```

## Acceptance Criteria

- [ ] systemd service unit (Linux) starts service automatically on boot
- [ ] launchd plist (macOS) runs service in user mode
- [ ] Service restarts automatically on failure (systemd: Restart=on-failure, launchd: KeepAlive)
- [ ] Health checks integrated with systemd Type=notify
- [ ] Installation scripts work on Ubuntu 22.04+ and macOS 13+
- [ ] Configuration defaults align with PRD-04 hook requirements (GRAPH_INDEX_DIR, BM25_INDEX_DIR)
- [ ] Logs accessible via journalctl (Linux) or ~/Library/Logs (macOS)
- [ ] daemon-ctl.sh provides unified management interface

**Related**: [00-overview.md](00-overview.md), [02-docker-compose.md](02-docker-compose.md), [../04-agent-integration/03-hooks.md](../04-agent-integration/03-hooks.md)
