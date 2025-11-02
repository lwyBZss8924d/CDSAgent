#!/usr/bin/env bash
# Sync all worktrees with main branch
# Usage: ./scripts/sync-worktrees.sh

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORKTREE_DIR="${REPO_ROOT}/.worktrees"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
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

header() {
    echo -e "${BLUE}==>${NC} $*"
}

cd "$REPO_ROOT"

header "Syncing all worktrees with main branch"
echo ""

# Ensure we're on main and up to date
info "Updating main branch..."
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
    git checkout main
fi
git pull origin main
echo ""

# Update each worktree
SUCCESS_COUNT=0
FAIL_COUNT=0
TOTAL_COUNT=0

if [ ! -d "$WORKTREE_DIR" ]; then
    error "Worktree directory $WORKTREE_DIR does not exist"
    exit 1
fi

for worktree_path in "$WORKTREE_DIR"/*; do
    if [ ! -d "$worktree_path" ]; then
        continue
    fi

    TOTAL_COUNT=$((TOTAL_COUNT + 1))
    task_name=$(basename "$worktree_path")

    header "Syncing $task_name..."

    cd "$worktree_path"

    # Check if there are uncommitted changes
    if [ -n "$(git status --porcelain)" ]; then
        warn "$task_name has uncommitted changes, skipping..."
        FAIL_COUNT=$((FAIL_COUNT + 1))
        cd "$REPO_ROOT"
        continue
    fi

    # Rebase on main
    if git rebase main > /dev/null 2>&1; then
        info "✅ $task_name synced successfully"
        SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
    else
        error "❌ $task_name failed to rebase (conflicts detected)"
        git rebase --abort
        FAIL_COUNT=$((FAIL_COUNT + 1))
    fi

    cd "$REPO_ROOT"
    echo ""
done

# Summary
echo ""
header "Sync Summary"
echo "Total worktrees: $TOTAL_COUNT"
echo -e "${GREEN}Successfully synced: $SUCCESS_COUNT${NC}"
if [ $FAIL_COUNT -gt 0 ]; then
    echo -e "${RED}Failed to sync: $FAIL_COUNT${NC}"
fi
echo ""

# Show final status
info "Final worktree status:"
git worktree list

# Return to original branch if needed
if [ "$CURRENT_BRANCH" != "main" ] && git show-ref --verify --quiet "refs/heads/$CURRENT_BRANCH"; then
    git checkout "$CURRENT_BRANCH" > /dev/null 2>&1
fi

echo ""
info "✨ Worktree sync complete!"
