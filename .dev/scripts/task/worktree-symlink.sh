#!/usr/bin/env bash
# Worktree Symlink Manager for CDSAgent
# Creates symlinks for worktrees to make them easier to access in IDEs

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORKTREE_DIR="${REPO_ROOT}/.worktrees"
SYMLINK_BASE="${HOME}/dev-space"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

info() {
    echo -e "${GREEN}[INFO]${NC} $*"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $*"
}

error() {
    echo -e "${RED}[ERROR]${NC} $*"
}

create_symlinks() {
    info "Creating symlinks for all worktrees in ${SYMLINK_BASE}..."

    if [[ ! -d "${WORKTREE_DIR}" ]]; then
        error "Worktree directory ${WORKTREE_DIR} does not exist"
        exit 1
    fi

    for worktree in "${WORKTREE_DIR}"/*; do
        if [[ -d "${worktree}" ]]; then
            task_name=$(basename "${worktree}")
            symlink_path="${SYMLINK_BASE}/CDSAgent-${task_name}"

            if [[ -L "${symlink_path}" ]]; then
                warn "Symlink already exists: ${symlink_path}"
            else
                ln -s "${worktree}" "${symlink_path}"
                info "Created symlink: ${symlink_path} -> ${worktree}"
            fi
        fi
    done

    info "Symlink creation complete!"
}

remove_symlinks() {
    info "Removing all CDSAgent worktree symlinks from ${SYMLINK_BASE}..."

    for symlink in "${SYMLINK_BASE}"/CDSAgent-T-*; do
        if [[ -L "${symlink}" ]]; then
            rm "${symlink}"
            info "Removed symlink: ${symlink}"
        fi
    done

    info "Symlink removal complete!"
}

list_worktrees() {
    info "Current worktrees:"
    git -C "${REPO_ROOT}" worktree list
    echo ""
    info "Symlinks in ${SYMLINK_BASE}:"
    ls -la "${SYMLINK_BASE}"/CDSAgent-T-* 2>/dev/null || warn "No symlinks found"
}

show_help() {
    cat << EOF
Usage: $(basename "$0") [COMMAND]

Manage worktree symlinks for CDSAgent development.

Commands:
    create      Create symlinks for all worktrees
    remove      Remove all CDSAgent worktree symlinks
    list        List all worktrees and their symlinks
    help        Show this help message

Examples:
    # Create symlinks for IDE access
    $(basename "$0") create

    # List all worktrees
    $(basename "$0") list

    # Clean up symlinks
    $(basename "$0") remove

Notes:
    - Symlinks are created in ${SYMLINK_BASE}/
    - Pattern: CDSAgent-<task-name>
    - Worktrees are stored in ${WORKTREE_DIR}/

EOF
}

# Main command dispatch
case "${1:-help}" in
    create)
        create_symlinks
        ;;
    remove)
        remove_symlinks
        ;;
    list)
        list_worktrees
        ;;
    help|--help|-h)
        show_help
        ;;
    *)
        error "Unknown command: $1"
        show_help
        exit 1
        ;;
esac
