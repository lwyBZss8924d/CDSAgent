#!/usr/bin/env bash
# Checkpoint Helper - Pre-checkpoint validation and assistance
# Usage: ./scripts/checkpoint-helper.sh [task_id]
#
# This script performs comprehensive pre-checkpoint checks to ensure:
# 1. Git status is clean or has only artifact changes
# 2. All commits have git notes attached
# 3. Metadata is consistent with git operations
# 4. Daily worklogs exist
#
# See: docs/WORK_SESSION_CHECKPOINT_WORKFLOW.md

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

# Get task ID from current worktree or argument
if [ $# -ge 1 ]; then
    TASK_ID="$1"
else
    # Try to infer from current directory
    CURRENT_DIR=$(pwd)
    if [[ "$CURRENT_DIR" =~ \.worktrees/([^/]+) ]]; then
        TASK_ID="${BASH_REMATCH[1]}"
    else
        echo -e "${RED}✗ Cannot determine task ID${NC}"
        echo "Usage: $0 <TASK_ID>"
        echo "  or run from worktree directory"
        exit 1
    fi
fi

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
TASK_DIR="${REPO_ROOT}/.artifacts/spec-tasks-${TASK_ID}"
TODAY=$(date +%Y-%m-%d)

echo -e "${BOLD}${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BOLD}${BLUE}   Checkpoint Helper - Pre-flight Checks${NC}"
echo -e "${BOLD}${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "Task: ${TASK_ID}"
echo "Date: ${TODAY}"
echo ""

# Track overall status
CHECKS_PASSED=0
CHECKS_FAILED=0
WARNINGS=0

# ============================================================================
# Check 1: Git Status
# ============================================================================
echo -e "${CYAN}[1/5] Checking git status...${NC}"

GIT_STATUS=$(git status --porcelain)

if [ -z "$GIT_STATUS" ]; then
    echo -e "${GREEN}  ✓ Working tree is clean${NC}"
    ((CHECKS_PASSED++))
else
    # Check if only artifacts are changed
    NON_ARTIFACT_CHANGES=$(echo "$GIT_STATUS" | grep -v "^.. .artifacts/" || true)

    if [ -z "$NON_ARTIFACT_CHANGES" ]; then
        echo -e "${GREEN}  ✓ Only artifact changes (expected)${NC}"
        ((CHECKS_PASSED++))
    else
        echo -e "${RED}  ✗ Uncommitted code changes detected${NC}"
        echo "$NON_ARTIFACT_CHANGES" | sed 's/^/    /'
        ((CHECKS_FAILED++))
        echo ""
        echo "  Action: Commit or stash code changes before checkpoint"
    fi
fi
echo ""

# ============================================================================
# Check 2: Git Notes
# ============================================================================
echo -e "${CYAN}[2/5] Checking git notes...${NC}"

# Get commits since origin/main
COMMITS=$(git log --format="%H" origin/main..HEAD 2>/dev/null || echo "")

if [ -z "$COMMITS" ]; then
    echo -e "${YELLOW}  ⚠ No commits to check${NC}"
    ((WARNINGS++))
else
    TOTAL_COMMITS=$(echo "$COMMITS" | wc -l | tr -d ' ')
    MISSING_NOTES=0

    while IFS= read -r commit; do
        NOTES=$(git notes show "$commit" 2>/dev/null || echo "")
        if [ -z "$NOTES" ]; then
            ((MISSING_NOTES++))
        fi
    done <<< "$COMMITS"

    if [ $MISSING_NOTES -eq 0 ]; then
        echo -e "${GREEN}  ✓ All $TOTAL_COMMITS commits have git notes${NC}"
        ((CHECKS_PASSED++))
    else
        echo -e "${RED}  ✗ $MISSING_NOTES out of $TOTAL_COMMITS commits missing notes${NC}"
        ((CHECKS_FAILED++))
        echo ""
        echo "  Action: Run ./scripts/git-notes-check.sh for details"
    fi
fi
echo ""

# ============================================================================
# Check 3: Daily Worklogs
# ============================================================================
echo -e "${CYAN}[3/5] Checking daily worklogs...${NC}"

WORKLOG_DIR="${TASK_DIR}/worklogs"

if [ ! -d "$WORKLOG_DIR" ]; then
    echo -e "${RED}  ✗ Worklog directory not found${NC}"
    ((CHECKS_FAILED++))
else
    REQUIRED_FILES=(
        "${WORKLOG_DIR}/${TODAY}-work-summary.md"
        "${WORKLOG_DIR}/${TODAY}-commit-log.md"
        "${WORKLOG_DIR}/${TODAY}-notes.md"
    )

    MISSING_WORKLOGS=()
    for file in "${REQUIRED_FILES[@]}"; do
        if [ ! -f "$file" ]; then
            MISSING_WORKLOGS+=("$(basename "$file")")
        fi
    done

    if [ ${#MISSING_WORKLOGS[@]} -eq 0 ]; then
        echo -e "${GREEN}  ✓ Today's worklogs exist${NC}"
        ((CHECKS_PASSED++))
    else
        echo -e "${YELLOW}  ⚠ Missing worklog files:${NC}"
        for file in "${MISSING_WORKLOGS[@]}"; do
            echo "    - $file"
        done
        ((WARNINGS++))
        echo ""
        echo "  Action: Run ./scripts/create-daily-worklog.sh ${TASK_ID}"
    fi
fi
echo ""

# ============================================================================
# Check 4: Metadata Consistency
# ============================================================================
echo -e "${CYAN}[4/5] Checking metadata consistency...${NC}"

METADATA_FILE="${TASK_DIR}/metadata.yaml"

if [ ! -f "$METADATA_FILE" ]; then
    echo -e "${RED}  ✗ metadata.yaml not found${NC}"
    ((CHECKS_FAILED++))
else
    # Check for PENDING placeholders
    PENDING_FIELDS=$(grep -c "PENDING\|TODO\|FIXME" "$METADATA_FILE" 2>/dev/null || echo "0")

    if [ "$PENDING_FIELDS" -eq 0 ]; then
        echo -e "${GREEN}  ✓ No PENDING fields in metadata${NC}"
        ((CHECKS_PASSED++))
    else
        echo -e "${YELLOW}  ⚠ Found $PENDING_FIELDS PENDING/TODO/FIXME markers${NC}"
        ((WARNINGS++))
        echo ""
        echo "  Action: Update metadata.yaml with actual values"
    fi

    # Check if git commits are listed
    COMMITS_IN_METADATA=$(grep -c "^    - hash:" "$METADATA_FILE" 2>/dev/null || echo "0")
    COMMITS_IN_GIT=$(git log --format="%H" origin/main..HEAD 2>/dev/null | wc -l | tr -d ' ')

    if [ "$COMMITS_IN_METADATA" -ge "$COMMITS_IN_GIT" ]; then
        echo -e "${GREEN}  ✓ Commit count consistent ($COMMITS_IN_METADATA in metadata, $COMMITS_IN_GIT in git)${NC}"
        ((CHECKS_PASSED++))
    else
        echo -e "${YELLOW}  ⚠ Commit count mismatch: $COMMITS_IN_METADATA in metadata, $COMMITS_IN_GIT in git${NC}"
        ((WARNINGS++))
        echo ""
        echo "  Action: Update git.commits section in metadata.yaml"
    fi
fi
echo ""

# ============================================================================
# Check 5: Artifact Completeness
# ============================================================================
echo -e "${CYAN}[5/5] Checking artifact completeness...${NC}"

REQUIRED_ARTIFACTS=(
    "${TASK_DIR}/metadata.yaml"
    "${TASK_DIR}/git-refs.txt"
    "${WORKLOG_DIR}"
)

MISSING_ARTIFACTS=()
for artifact in "${REQUIRED_ARTIFACTS[@]}"; do
    if [ ! -e "$artifact" ]; then
        MISSING_ARTIFACTS+=("$(basename "$artifact")")
    fi
done

if [ ${#MISSING_ARTIFACTS[@]} -eq 0 ]; then
    echo -e "${GREEN}  ✓ All required artifacts exist${NC}"
    ((CHECKS_PASSED++))
else
    echo -e "${RED}  ✗ Missing artifacts:${NC}"
    for artifact in "${MISSING_ARTIFACTS[@]}"; do
        echo "    - $artifact"
    done
    ((CHECKS_FAILED++))
    echo ""
    echo "  Action: Run ./scripts/create-task-worklog.sh ${TASK_ID} \"Title\" \"Developer\""
fi
echo ""

# ============================================================================
# Summary Report
# ============================================================================
echo -e "${BOLD}${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BOLD}${BLUE}   Summary Report${NC}"
echo -e "${BOLD}${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo -e "${GREEN}Passed:   $CHECKS_PASSED${NC}"
echo -e "${YELLOW}Warnings: $WARNINGS${NC}"
echo -e "${RED}Failed:   $CHECKS_FAILED${NC}"
echo ""

if [ $CHECKS_FAILED -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo -e "${GREEN}${BOLD}✓ Ready for checkpoint!${NC}"
    echo ""
    echo "Next steps:"
    echo "  1. Review action logs and git operations"
    echo "  2. Follow checkpoint workflow: docs/WORK_SESSION_CHECKPOINT_WORKFLOW.md"
    echo "  3. Run consistency check (Phase 2)"
    echo "  4. Update artifacts (Phase 3)"
    echo "  5. Commit artifacts (Phase 4)"
    exit 0
elif [ $CHECKS_FAILED -eq 0 ]; then
    echo -e "${YELLOW}${BOLD}⚠ Checkpoint possible with warnings${NC}"
    echo ""
    echo "Warnings should be addressed but not blocking."
    echo "Review actions above and proceed with caution."
    exit 0
else
    echo -e "${RED}${BOLD}✗ Not ready for checkpoint${NC}"
    echo ""
    echo "Please fix failed checks before proceeding."
    echo "Run this script again after fixes."
    exit 1
fi
