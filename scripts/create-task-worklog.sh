#!/usr/bin/env bash
# Create initial task worklog structure
# Usage: ./scripts/create-task-worklog.sh T-05-01-jsonrpc-schema "JSON-RPC Schema" "Developer Name"

set -euo pipefail

if [ $# -lt 3 ]; then
    echo "Usage: $0 <TASK_ID> <TASK_TITLE> <DEVELOPER_NAME>"
    echo "Example: $0 T-05-01-jsonrpc-schema \"JSON-RPC Schema Definition\" \"Rust Dev 1\""
    exit 1
fi

TASK_ID="$1"
TASK_TITLE="$2"
DEVELOPER="$3"

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TASK_DIR="${REPO_ROOT}/.artifacts/spec-tasks-${TASK_ID}"
TEMPLATE_DIR="${REPO_ROOT}/.artifacts/spec-tasks-templates"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}Creating worklog structure for ${TASK_ID}...${NC}"

# Create directories
mkdir -p "${TASK_DIR}/worklogs"

# Copy metadata template
cp "${TEMPLATE_DIR}/metadata.template.yaml" "${TASK_DIR}/metadata.yaml"

# Replace placeholders in metadata
sed -i.bak "s/{TASK_ID}/${TASK_ID}/g" "${TASK_DIR}/metadata.yaml"
sed -i.bak "s/{TASK_TITLE}/${TASK_TITLE}/g" "${TASK_DIR}/metadata.yaml"
sed -i.bak "s/{DEVELOPER_NAME}/${DEVELOPER}/g" "${TASK_DIR}/metadata.yaml"
sed -i.bak "s|{TASK_DIR}|${TASK_ID}|g" "${TASK_DIR}/metadata.yaml"
rm "${TASK_DIR}/metadata.yaml.bak"

# Create git-refs.txt
cat > "${TASK_DIR}/git-refs.txt" << EOF
# Git References for ${TASK_ID}
# Auto-generated on $(date +%Y-%m-%d)

Branch: feat/task/${TASK_ID}
Worktree: .worktrees/${TASK_ID}
Symlink: ~/dev-space/CDSAgent-${TASK_ID}

# Commits will be listed here as they are made
EOF

echo -e "${GREEN}✓ Created task worklog structure${NC}"
echo ""
echo "Structure:"
echo "  ${TASK_DIR}/"
echo "  ├── metadata.yaml"
echo "  ├── git-refs.txt"
echo "  └── worklogs/"
echo ""
echo "Next steps:"
echo "1. Review and customize metadata.yaml"
echo "2. Create daily worklog: ./scripts/create-daily-worklog.sh ${TASK_ID}"
echo "3. Start development in: ~/dev-space/CDSAgent-${TASK_ID}"
