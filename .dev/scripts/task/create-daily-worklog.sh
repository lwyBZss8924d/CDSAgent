#!/usr/bin/env bash
# Create daily worklog entries for a task
# Usage: ./scripts/create-daily-worklog.sh T-05-01-jsonrpc-schema [YYYY-MM-DD]

set -euo pipefail

if [ $# -lt 1 ]; then
    echo "Usage: $0 <TASK_ID> [DATE]"
    echo "Example: $0 T-05-01-jsonrpc-schema"
    echo "Example: $0 T-05-01-jsonrpc-schema 2025-10-20"
    exit 1
fi

TASK_ID="$1"
DATE="${2:-$(date +%Y-%m-%d)}"

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TASK_DIR="${REPO_ROOT}/.artifacts/spec-tasks-${TASK_ID}"
WORKLOG_DIR="${TASK_DIR}/worklogs"
TEMPLATE_DIR="${REPO_ROOT}/.artifacts/spec-tasks-templates/worklogs"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Worktree context validation
CURRENT_DIR=$(pwd)
WORKTREE_EXPECTED="/.worktrees/${TASK_ID}"

if [[ ! "$CURRENT_DIR" =~ $WORKTREE_EXPECTED ]]; then
    echo -e "${YELLOW}⚠ Warning: Not running from worktree!${NC}"
    echo "Current directory: $CURRENT_DIR"
    echo "Expected: */.worktrees/${TASK_ID}"
    echo ""
    echo "Recommendation: Navigate to worktree first:"
    echo "  cd ~/dev-space/CDSAgent-${TASK_ID}"
    echo "  /Users/arthur/dev-space/CDSAgent/scripts/create-daily-worklog.sh ${TASK_ID}"
    echo ""
    echo "Continuing anyway, but artifacts may not be visible in worktree..."
fi

# Check if task directory exists
if [ ! -d "${TASK_DIR}" ]; then
    echo -e "${RED}✗ Task directory not found: ${TASK_DIR}${NC}"
    echo ""
    echo "Creating task directory structure..."
    mkdir -p "${WORKLOG_DIR}"
    echo -e "${GREEN}✓ Created ${TASK_DIR}${NC}"
    echo ""
    echo "Note: You should initialize task artifacts first:"
    echo "  ./scripts/create-task-worklog.sh ${TASK_ID} \"Task Title\" \"Developer\""
fi

echo -e "${BLUE}Creating daily worklog for ${TASK_ID} on ${DATE}...${NC}"

# Create worklog files from templates
for template in work-summary commit-log notes; do
    OUTPUT_FILE="${WORKLOG_DIR}/${DATE}-${template}.md"

    if [ -f "${OUTPUT_FILE}" ]; then
        echo -e "${YELLOW}Warning: ${OUTPUT_FILE} already exists, skipping...${NC}"
        continue
    fi

    cp "${TEMPLATE_DIR}/${template}.template.md" "${OUTPUT_FILE}"

    # Replace date placeholder
    # Use | as separator to handle / in values
    sed -i.bak "s|{DATE}|${DATE}|g" "${OUTPUT_FILE}"

    # Try to get task info from metadata if it exists
    if [ -f "${TASK_DIR}/metadata.yaml" ]; then
        TASK_TITLE=$(grep '^  title:' "${TASK_DIR}/metadata.yaml" | sed 's/.*: "\(.*\)"/\1/')
        DEVELOPER=$(grep '^  primary:' "${TASK_DIR}/metadata.yaml" | sed 's/.*: "\(.*\)"/\1/')
        BRANCH=$(grep '^  branch:' "${TASK_DIR}/metadata.yaml" | sed 's/.*: "\(.*\)"/\1/')

        # Use | as separator to handle / in TASK_TITLE
        sed -i.bak "s|{TASK_ID}|${TASK_ID}|g" "${OUTPUT_FILE}"
        sed -i.bak "s|{TASK_TITLE}|${TASK_TITLE}|g" "${OUTPUT_FILE}"
        sed -i.bak "s|{DEVELOPER_NAME}|${DEVELOPER}|g" "${OUTPUT_FILE}"
        sed -i.bak "s|{BRANCH_NAME}|${BRANCH}|g" "${OUTPUT_FILE}"
    fi

    rm "${OUTPUT_FILE}.bak"

    echo -e "${GREEN}✓ Created ${template}.md${NC}"
done

echo ""
echo "Daily worklog created:"
echo "  ${WORKLOG_DIR}/${DATE}-work-summary.md"
echo "  ${WORKLOG_DIR}/${DATE}-commit-log.md"
echo "  ${WORKLOG_DIR}/${DATE}-notes.md"
echo ""
echo "Fill out as you work today!"
