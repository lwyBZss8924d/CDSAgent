#!/bin/bash

# create-raw-log.sh
# Purpose: Create RAW session log file with proper naming and placeholders
# Usage: ./scripts/create-raw-log.sh <TASK_ID> <SESSION_NUM> <THREAD_START> <THREAD_END> ["<DESCRIPTION>"]
#
# Example:
#   ./scripts/create-raw-log.sh T-02-02-sparse-index 05 01 04 "Phase 2 Testing"
#
# Creates:
#   worklogs/raw/WORK-SESSIONS-05-THREADS-01-04-SUMMARY-{date}.txt
#
# NOTE: This should be created AFTER session completes, not before!

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
THREAD_START="$3"
THREAD_END="$4"
DESCRIPTION="${5:-Session Summary}"

# Validate arguments
if [[ -z "$TASK_ID" || -z "$SESSION_NUM" || -z "$THREAD_START" || -z "$THREAD_END" ]]; then
    echo -e "${RED}Error: Missing required arguments${NC}"
    echo ""
    echo "Usage: $0 <TASK_ID> <SESSION_NUM> <THREAD_START> <THREAD_END> [\"<DESCRIPTION>\"]"
    echo ""
    echo "Example:"
    echo "  $0 T-02-02-sparse-index 05 01 04 \"Phase 2 Testing\""
    exit 1
fi

# Validate numbers
if ! [[ "$SESSION_NUM" =~ ^[0-9]+$ ]]; then
    echo -e "${RED}Error: SESSION_NUM must be a number (got: $SESSION_NUM)${NC}"
    exit 1
fi

if ! [[ "$THREAD_START" =~ ^[0-9]+$ ]]; then
    echo -e "${RED}Error: THREAD_START must be a number (got: $THREAD_START)${NC}"
    exit 1
fi

if ! [[ "$THREAD_END" =~ ^[0-9]+$ ]]; then
    echo -e "${RED}Error: THREAD_END must be a number (got: $THREAD_END)${NC}"
    exit 1
fi

# Format numbers with leading zeros
SESSION_NUM_PADDED=$(printf "%02d" "$SESSION_NUM")
THREAD_START_PADDED=$(printf "%02d" "$THREAD_START")
THREAD_END_PADDED=$(printf "%02d" "$THREAD_END")

# Get current date
DATE=$(date +%Y-%m-%d)

# Determine paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
RAW_DIR="${REPO_ROOT}/.artifacts/spec-tasks-${TASK_ID}/worklogs/raw"
TEMPLATE_FILE="${REPO_ROOT}/.dev/templates/worklogs/raw-session.template.txt"

# Fallback to old template location if new one doesn't exist
if [[ ! -f "$TEMPLATE_FILE" ]]; then
    echo -e "${YELLOW}⚠ New template not found, checking old location...${NC}"
    TEMPLATE_FILE="${REPO_ROOT}/.artifacts/spec-tasks-templates/worklogs/raw-session.template.txt"
fi

# Validate template exists
if [[ ! -f "$TEMPLATE_FILE" ]]; then
    echo -e "${RED}Error: RAW log template not found${NC}"
    echo "Expected: .dev/templates/worklogs/raw-session.template.txt"
    exit 1
fi

# Create raw directory if it doesn't exist
if [[ ! -d "$RAW_DIR" ]]; then
    echo -e "${YELLOW}Creating RAW log directory: $RAW_DIR${NC}"
    mkdir -p "$RAW_DIR"
fi

# Construct output filename
OUTPUT_FILE="${RAW_DIR}/WORK-SESSIONS-${SESSION_NUM_PADDED}-THREADS-${THREAD_START_PADDED}-${THREAD_END_PADDED}-SUMMARY-${DATE}.txt"

# Check if file already exists
if [[ -f "$OUTPUT_FILE" ]]; then
    echo -e "${YELLOW}⚠ Warning: RAW log file already exists${NC}"
    echo "File: ${OUTPUT_FILE}"
    echo ""
    read -p "Overwrite? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Aborted."
        exit 0
    fi
fi

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}   RAW Session Log Creation${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "Task:        $TASK_ID"
echo "Session:     $SESSION_NUM_PADDED"
echo "Threads:     $THREAD_START_PADDED-$THREAD_END_PADDED"
echo "Date:        $DATE"
echo "Description: $DESCRIPTION"
echo ""

# Read task title and phase from metadata.yaml if available
METADATA_FILE="${REPO_ROOT}/.artifacts/spec-tasks-${TASK_ID}/metadata.yaml"
TASK_TITLE="Task Title"
PHASE="Phase X"
DAY="X"

if [[ -f "$METADATA_FILE" ]]; then
    TASK_TITLE=$(grep "^  title:" "$METADATA_FILE" | sed 's/^  title: "\(.*\)"$/\1/')
    if [[ -z "$TASK_TITLE" ]]; then
        TASK_TITLE=$(grep "^  title:" "$METADATA_FILE" | sed "s/^  title: '\(.*\)'$/\1/")
    fi
    if [[ -z "$TASK_TITLE" ]]; then
        TASK_TITLE=$(grep "^  title:" "$METADATA_FILE" | sed 's/^  title: //')
    fi
fi

# Copy template
cp "$TEMPLATE_FILE" "$OUTPUT_FILE"

# Replace placeholders using | separator (handles "/" in values)
sed -i.bak "s|{SESSION}|${SESSION_NUM_PADDED}|g" "$OUTPUT_FILE"
sed -i.bak "s|{THREAD_START}|${THREAD_START_PADDED}|g" "$OUTPUT_FILE"
sed -i.bak "s|{THREAD_END}|${THREAD_END_PADDED}|g" "$OUTPUT_FILE"
sed -i.bak "s|{DATE}|${DATE}|g" "$OUTPUT_FILE"
sed -i.bak "s|{TASK_ID}|${TASK_ID}|g" "$OUTPUT_FILE"
sed -i.bak "s|{TASK_TITLE}|${TASK_TITLE}|g" "$OUTPUT_FILE"
sed -i.bak "s|{DAY}|${DAY}|g" "$OUTPUT_FILE"
sed -i.bak "s|{PHASE}|${PHASE}|g" "$OUTPUT_FILE"
sed -i.bak "s|{PHASE_DESCRIPTION}|${DESCRIPTION}|g" "$OUTPUT_FILE"
sed -i.bak "s|{DESCRIPTION}|${DESCRIPTION}|g" "$OUTPUT_FILE"
sed -i.bak "s|{DURATION}|X.Xh|g" "$OUTPUT_FILE"

# Remove backup file
rm "${OUTPUT_FILE}.bak"

echo -e "${GREEN}✓ RAW log created successfully${NC}"
echo ""
echo "File: WORK-SESSIONS-${SESSION_NUM_PADDED}-THREADS-${THREAD_START_PADDED}-${THREAD_END_PADDED}-SUMMARY-${DATE}.txt"
echo "Location: .artifacts/spec-tasks-${TASK_ID}/worklogs/raw/"
echo ""
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "Next steps:"
echo "  1. Open the RAW log file and fill in:"
echo "     - Day number (currently: $DAY)"
echo "     - Phase (currently: $PHASE)"
echo "     - Duration (currently: X.Xh)"
echo "     - Session overview"
echo "     - Thread-by-thread details"
echo "     - Session summary with commits and metrics"
echo ""
echo "  2. Update metadata.yaml with session entry:"
echo "     - Add to 'notes:' section"
echo "     - Add to 'worklog.entries:' with raw_log path"
echo ""
echo "  3. Run checkpoint workflow to commit all artifacts"
echo ""

# Open file in default editor (optional)
if command -v code &> /dev/null; then
    echo "Opening in VS Code..."
    code "$OUTPUT_FILE"
elif [[ -n "$EDITOR" ]]; then
    echo "Opening in $EDITOR..."
    $EDITOR "$OUTPUT_FILE"
fi
