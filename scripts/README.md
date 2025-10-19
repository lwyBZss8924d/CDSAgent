# CDSAgent Development Scripts

This directory contains helper scripts for CDSAgent development workflows.

## Available Scripts

### `worktree-symlink.sh`

Manages git worktree symlinks for parallel task development.

**Usage:**

```bash
# Create symlinks for all worktrees in ~/dev-space/
./scripts/worktree-symlink.sh create

# List all worktrees and their symlinks
./scripts/worktree-symlink.sh list

# Remove all CDSAgent worktree symlinks
./scripts/worktree-symlink.sh remove

# Show help
./scripts/worktree-symlink.sh help
```

**Features:**

- Automatically creates symlinks for all task worktrees
- Symlinks pattern: `~/dev-space/CDSAgent-T-XX-XX-task-name`
- Makes worktrees easier to access in IDEs
- Color-coded output for better visibility

**Examples:**

```bash
# After creating new worktrees
git worktree add .worktrees/T-08-01-unit-tests -b feat/task/T-08-01-unit-tests main
./scripts/worktree-symlink.sh create

# Access via symlink in IDE
code ~/dev-space/CDSAgent-T-08-01-unit-tests
```

## Future Scripts

Planned development scripts:

- `test-runner.sh` - Run tests across all worktrees
- `parity-check.sh` - Validate LocAgent parity for graph/index
- `benchmark.sh` - Run performance benchmarks
- `setup-dev.sh` - One-command development environment setup
- `fixture-gen.sh` - Generate test fixtures from sample repos

## Contributing

When adding new scripts:

1. Follow the naming convention: `kebab-case.sh`
2. Add execute permissions: `chmod +x scripts/new-script.sh`
3. Include help/usage documentation
4. Update this README with script description
5. Test on macOS and Linux if possible

---

**See Also:**

- [Worktree Workflow Documentation](../docs/WORKTREE_WORKFLOW.md)
- [Development Guide](../README.md#development)
