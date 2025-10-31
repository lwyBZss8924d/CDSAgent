#!/usr/bin/env bash
# Verify git notes exist on commits
# Usage: ./scripts/git-notes-check.sh [base_commit]
#
# This script checks that all commits since the base commit have git notes attached.
# Git notes are REQUIRED for checkpoint workflow per docs/checkpoint/06-phase4-git.md

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Get base commit (default: origin/main)
BASE_COMMIT="${1:-origin/main}"

# Get current branch
CURRENT_BRANCH=$(git branch --show-current)

echo -e "${BLUE}Git Notes Verification${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Branch: ${CURRENT_BRANCH}"
echo "Base:   ${BASE_COMMIT}"
echo ""

# Get list of commits since base
COMMITS=$(git log --format="%H" "${BASE_COMMIT}..HEAD" 2>/dev/null || echo "")

if [ -z "$COMMITS" ]; then
    echo -e "${GREEN}✓ No commits to check (branch is up to date with base)${NC}"
    exit 0
fi

# Count total commits
TOTAL_COMMITS=$(echo "$COMMITS" | wc -l | tr -d ' ')
echo "Checking $TOTAL_COMMITS commit(s)..."
echo ""

# Check each commit for notes
MISSING_NOTES=()
INVALID_FORMAT=()

while IFS= read -r commit; do
    # Get commit short hash and message
    SHORT_HASH=$(git log -1 --format="%h" "$commit")
    COMMIT_MSG=$(git log -1 --format="%s" "$commit")

    # Check if notes exist
    NOTES=$(git notes show "$commit" 2>/dev/null || echo "")

    if [ -z "$NOTES" ]; then
        MISSING_NOTES+=("$SHORT_HASH: $COMMIT_MSG")
        echo -e "${RED}✗${NC} $SHORT_HASH - Missing notes"
    else
        # Validate note format (should contain spec-tasks/)
        if echo "$NOTES" | grep -q "spec-tasks/"; then
            echo -e "${GREEN}✓${NC} $SHORT_HASH - Notes present"
        else
            INVALID_FORMAT+=("$SHORT_HASH: $COMMIT_MSG")
            echo -e "${YELLOW}⚠${NC} $SHORT_HASH - Notes exist but invalid format"
        fi
    fi
done <<< "$COMMITS"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Report results
if [ ${#MISSING_NOTES[@]} -eq 0 ] && [ ${#INVALID_FORMAT[@]} -eq 0 ]; then
    echo -e "${GREEN}✓ All commits have valid git notes!${NC}"
    exit 0
else
    echo -e "${RED}✗ Git notes check FAILED${NC}"
    echo ""

    if [ ${#MISSING_NOTES[@]} -gt 0 ]; then
        echo -e "${RED}Commits missing notes (${#MISSING_NOTES[@]}):${NC}"
        for item in "${MISSING_NOTES[@]}"; do
            echo "  - $item"
        done
        echo ""
    fi

    if [ ${#INVALID_FORMAT[@]} -gt 0 ]; then
        echo -e "${YELLOW}Commits with invalid note format (${#INVALID_FORMAT[@]}):${NC}"
        for item in "${INVALID_FORMAT[@]}"; do
            echo "  - $item"
        done
        echo ""
    fi

    echo "How to fix:"
    echo "1. Add git notes to each commit:"
    echo "   git notes add -m \"spec-tasks/T-XX-XX-task-name"
    echo "   Day: X"
    echo "   Date: YYYY-MM-DD"
    echo "   Sessions: X-XX to X-XX (HH:MM-HH:MM UTC)"
    echo "   Duration: Xh"
    echo "   Worklog: .artifacts/spec-tasks-T-XX-XX/worklogs/YYYY-MM-DD-*"
    echo "   Status: [summary]"
    echo "   Files: X code files (+XXX/-XXX lines)\" <commit-hash>"
    echo ""
    echo "2. Push notes to remote:"
    echo "   git push origin refs/notes/commits"
    echo ""
    echo "See: docs/checkpoint/06-phase4-git.md for details"

    exit 1
fi
