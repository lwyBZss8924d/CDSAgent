#!/bin/bash

# create-session-worklog.sh
# Purpose: Initialize all session worklog files with proper placeholders replaced
# Usage: ./scripts/create-session-worklog.sh <TASK_ID> <SESSION_NUM> "<DESCRIPTION>" ["<DEVELOPER_NAME>"]
#
# Example:
#   ./scripts/create-session-worklog.sh T-02-02-sparse-index 05 "Phase 2 Testing" "Rust Dev 2"
#
# Creates:
#   - {date}-S{NN}-work-summary.md
#   - {date}-S{NN}-commit-log.md
#   - {date}-S{NN}-notes.md
#   - {date}-S{NN}-codereview.md

set -e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Parse arguments
TASK_ID="$1"
SESSION_NUM="$2"
DESCRIPTION="$3"
DEVELOPER_NAME="${4:-Developer}"

# Validate arguments
if [[ -z "$TASK_ID" || -z "$SESSION_NUM" || -z "$DESCRIPTION" ]]; then
    echo -e "${RED}Error: Missing required arguments${NC}"
    echo ""
    echo "Usage: $0 <TASK_ID> <SESSION_NUM> \"<DESCRIPTION>\" [\"<DEVELOPER_NAME>\"]"
    echo ""
    echo "Example:"
    echo "  $0 T-02-02-sparse-index 05 \"Phase 2 Testing\" \"Rust Dev 2\""
    exit 1
fi

# Validate SESSION_NUM is a number
if ! [[ "$SESSION_NUM" =~ ^[0-9]+$ ]]; then
    echo -e "${RED}Error: SESSION_NUM must be a number (got: $SESSION_NUM)${NC}"
    exit 1
fi

# Format session number with leading zero
SESSION_NUM_PADDED=$(printf "%02d" "$SESSION_NUM")

# Get current date
DATE=$(date +%Y-%m-%d)

# Determine paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
WORKLOG_DIR="${REPO_ROOT}/.artifacts/spec-tasks-${TASK_ID}/worklogs"
TEMPLATE_DIR="${REPO_ROOT}/.dev/templates/worklogs"

# Fallback to old template location if new one doesn't exist
if [[ ! -d "$TEMPLATE_DIR" ]]; then
    TEMPLATE_DIR="${REPO_ROOT}/.artifacts/spec-tasks-templates/worklogs"
fi

# Validate worklog directory exists
if [[ ! -d "$WORKLOG_DIR" ]]; then
    echo -e "${RED}Error: Worklog directory not found: $WORKLOG_DIR${NC}"
    echo "Have you initialized the task with create-task-worklog.sh?"
    exit 1
fi

# Validate template directory exists
if [[ ! -d "$TEMPLATE_DIR" ]]; then
    echo -e "${RED}Error: Template directory not found: $TEMPLATE_DIR${NC}"
    exit 1
fi

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}   Session Worklog Initialization${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "Task:        $TASK_ID"
echo "Session:     $SESSION_NUM_PADDED"
echo "Date:        $DATE"
echo "Description: $DESCRIPTION"
echo "Developer:   $DEVELOPER_NAME"
echo ""

# Read task title from metadata.yaml if available
METADATA_FILE="${REPO_ROOT}/.artifacts/spec-tasks-${TASK_ID}/metadata.yaml"
TASK_TITLE="Task Title"
if [[ -f "$METADATA_FILE" ]]; then
    TASK_TITLE=$(grep "^  title:" "$METADATA_FILE" | sed 's/^  title: "\(.*\)"$/\1/')
    if [[ -z "$TASK_TITLE" ]]; then
        TASK_TITLE=$(grep "^  title:" "$METADATA_FILE" | sed "s/^  title: '\(.*\)'$/\1/")
    fi
    if [[ -z "$TASK_TITLE" ]]; then
        TASK_TITLE=$(grep "^  title:" "$METADATA_FILE" | sed 's/^  title: //')
    fi
fi

# Get branch name
BRANCH_NAME=$(git branch --show-current 2>/dev/null || echo "feat/task/${TASK_ID}")

# Create session files
FILES_CREATED=0

for template_name in work-summary commit-log notes codereview; do
    TEMPLATE_FILE="${TEMPLATE_DIR}/${template_name}.template.md"
    OUTPUT_FILE="${WORKLOG_DIR}/${DATE}-S${SESSION_NUM_PADDED}-${template_name}.md"

    # Check if template exists
    if [[ ! -f "$TEMPLATE_FILE" ]]; then
        echo -e "${YELLOW}⚠ Template not found: ${template_name}.template.md (skipping)${NC}"
        continue
    fi

    # Check if file already exists
    if [[ -f "$OUTPUT_FILE" ]]; then
        echo -e "${YELLOW}⚠ File already exists: ${DATE}-S${SESSION_NUM_PADDED}-${template_name}.md (skipping)${NC}"
        continue
    fi

    # Copy template
    cp "$TEMPLATE_FILE" "$OUTPUT_FILE"

    # Replace placeholders using | separator (handles "/" in values)
    sed -i.bak "s|{DATE}|${DATE}|g" "$OUTPUT_FILE"
    sed -i.bak "s|{SESSION}|${SESSION_NUM_PADDED}|g" "$OUTPUT_FILE"
    sed -i.bak "s|{TASK_ID}|${TASK_ID}|g" "$OUTPUT_FILE"
    sed -i.bak "s|{TASK_TITLE}|${TASK_TITLE}|g" "$OUTPUT_FILE"
    sed -i.bak "s|{DEVELOPER_NAME}|${DEVELOPER_NAME}|g" "$OUTPUT_FILE"
    sed -i.bak "s|{BRANCH_NAME}|${BRANCH_NAME}|g" "$OUTPUT_FILE"
    sed -i.bak "s|{DESCRIPTION}|${DESCRIPTION}|g" "$OUTPUT_FILE"

    # Remove backup file
    rm "${OUTPUT_FILE}.bak"

    echo -e "${GREEN}✓ Created: ${DATE}-S${SESSION_NUM_PADDED}-${template_name}.md${NC}"
    FILES_CREATED=$((FILES_CREATED + 1))
done

echo ""
if [[ $FILES_CREATED -eq 0 ]]; then
    echo -e "${YELLOW}⚠ No files created (all already exist or templates missing)${NC}"
    exit 0
fi

echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✓ Session $SESSION_NUM_PADDED worklog initialized${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "Files created: $FILES_CREATED"
echo "Location: .artifacts/spec-tasks-${TASK_ID}/worklogs/"
echo ""
echo "Next steps:"
echo "  1. Fill out ${DATE}-S${SESSION_NUM_PADDED}-work-summary.md with session objectives"
echo "  2. Begin development (Thread 01)"
echo "  3. Update files continuously during session"
echo "  4. After session completes, create RAW log with create-raw-log.sh"
echo ""
