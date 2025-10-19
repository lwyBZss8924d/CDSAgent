# Spec-Tasks Worklog Templates

This directory contains templates for tracking task development progress.

## Structure

```tree
.artifacts/spec-tasks-templates/
├── README.md                      # This file
├── metadata.template.yaml         # Task metadata template
└── worklogs/
    ├── work-summary.template.md   # Daily work summary
    ├── commit-log.template.md     # Git commit tracking
    └── notes.template.md          # Development notes
```

## Usage

### Creating a New Task Worklog

When starting a new task (e.g., T-05-01-jsonrpc-schema):

```bash
# 1. Create task artifact directory
mkdir -p .artifacts/spec-tasks-T-05-01-jsonrpc-schema/worklogs

# 2. Copy metadata template
cp .artifacts/spec-tasks-templates/metadata.template.yaml \
   .artifacts/spec-tasks-T-05-01-jsonrpc-schema/metadata.yaml

# 3. Fill in task-specific information
# Edit .artifacts/spec-tasks-T-05-01-jsonrpc-schema/metadata.yaml
# Replace placeholders: {TASK_ID}, {TASK_TITLE}, {DEVELOPER_NAME}, etc.

# 4. Create daily worklog (each day of development)
DATE=$(date +%Y-%m-%d)
cp .artifacts/spec-tasks-templates/worklogs/work-summary.template.md \
   .artifacts/spec-tasks-T-05-01-jsonrpc-schema/worklogs/${DATE}-work-summary.md
cp .artifacts/spec-tasks-templates/worklogs/commit-log.template.md \
   .artifacts/spec-tasks-T-05-01-jsonrpc-schema/worklogs/${DATE}-commit-log.md
cp .artifacts/spec-tasks-templates/worklogs/notes.template.md \
   .artifacts/spec-tasks-T-05-01-jsonrpc-schema/worklogs/${DATE}-notes.md

# 5. Update placeholders in daily worklogs
```

### Helper Script

Use the provided script to automate worklog creation:

```bash
# Create new task worklog
./scripts/create-task-worklog.sh T-05-01-jsonrpc-schema "JSON-RPC Schema Definition" "Developer Name"

# Create daily worklog entry
./scripts/create-daily-worklog.sh T-05-01-jsonrpc-schema
```

## Template Placeholders

Replace these placeholders when creating task worklogs:

| Placeholder | Description | Example |
|-------------|-------------|---------|
| `{TASK_ID}` | Task identifier | T-05-01-jsonrpc-schema |
| `{TASK_TITLE}` | Full task name | JSON-RPC Schema Definition & Validation |
| `{DATE}` | Current date | 2025-10-19 |
| `{DEVELOPER_NAME}` | Developer name | Rust Dev 1 |
| `{BRANCH_NAME}` | Git branch | feat/task/T-05-01-jsonrpc-schema |
| `{MILESTONE_ID}` | Milestone | M1 |
| `{PRD_FILE}` | PRD filename | 05-api-specifications |
| `{ISSUE_FILE}` | Issue filename | 05-api-contracts |
| `{TASK_DIR}` | Task directory | 05-api-contracts |
| `{TASK_FILE}` | Task filename | T-05-01-jsonrpc-schema |
| `{COMMIT_HASH_SHORT}` | Short git hash | abc1234 |
| `{COMMIT_MESSAGE}` | Commit message | feat(api): implement JSON-RPC schema |

## Worklog Best Practices

### Daily Work Summary

- Fill out at **end of each day**
- List completed objectives vs planned
- Document key decisions and rationale
- Note blockers and challenges
- Track progress against acceptance criteria

### Commit Log

- Update after **each commit** or at EOD
- Include commit hash, message, files changed
- Add context notes for non-obvious commits
- Link to relevant issues/PRs

### Development Notes

- Capture technical decisions **as they happen**
- Document research findings
- Record learning and insights
- Note refactoring opportunities
- Track TODO items

### Metadata

- Update **status** as task progresses
- Record **git commits** with hashes
- Track **metrics** (hours, lines of code, test coverage)
- Add **comments** for significant events
- Link **PR** when created

## Integration with TODO.yaml

The worklog metadata feeds into the central TODO.yaml:

```yaml
# In TODO.yaml
tasks:
  T-05-01-jsonrpc-schema:
    worklog_path: .artifacts/spec-tasks-T-05-01/worklogs/
    git_commits:
      - abc1234: "feat(api): implement JSON-RPC schema"
    pr: "https://github.com/user/repo/pull/123"
    status: completed
```

## Example Worklog Structure

```tree
.artifacts/spec-tasks-T-05-01-jsonrpc-schema/
├── metadata.yaml
├── git-refs.txt
└── worklogs/
    ├── 2025-10-19-work-summary.md
    ├── 2025-10-19-commit-log.md
    ├── 2025-10-19-notes.md
    ├── 2025-10-20-work-summary.md
    ├── 2025-10-20-commit-log.md
    ├── 2025-10-20-notes.md
    ├── 2025-10-21-work-summary.md
    ├── 2025-10-21-commit-log.md
    └── 2025-10-21-notes.md
```

## Automation Scripts

### create-task-worklog.sh

Creates initial task worklog structure:

```bash
#!/bin/bash
TASK_ID=$1
TASK_TITLE=$2
DEVELOPER=$3

mkdir -p .artifacts/spec-tasks-${TASK_ID}/worklogs
cp .artifacts/spec-tasks-templates/metadata.template.yaml \
   .artifacts/spec-tasks-${TASK_ID}/metadata.yaml

# Replace placeholders
sed -i '' "s/{TASK_ID}/${TASK_ID}/g" \
   .artifacts/spec-tasks-${TASK_ID}/metadata.yaml
sed -i '' "s/{TASK_TITLE}/${TASK_TITLE}/g" \
   .artifacts/spec-tasks-${TASK_ID}/metadata.yaml
sed -i '' "s/{DEVELOPER_NAME}/${DEVELOPER}/g" \
   .artifacts/spec-tasks-${TASK_ID}/metadata.yaml
```

### create-daily-worklog.sh

Creates daily worklog entries:

```bash
#!/bin/bash
TASK_ID=$1
DATE=$(date +%Y-%m-%d)
WORKLOG_DIR=.artifacts/spec-tasks-${TASK_ID}/worklogs

for template in work-summary commit-log notes; do
  cp .artifacts/spec-tasks-templates/worklogs/${template}.template.md \
     ${WORKLOG_DIR}/${DATE}-${template}.md

  # Replace date placeholder
  sed -i '' "s/{DATE}/${DATE}/g" \
     ${WORKLOG_DIR}/${DATE}-${template}.md
done
```

---

**See Also**:

- [TODO.yaml](../../spacs/tasks/0.1.0-mvp/TODO.yaml) - Central task registry
- [Task README](../../spacs/tasks/0.1.0-mvp/README.md) - Task organization
- [DEVELOPMENT_STATUS.md](../../DEVELOPMENT_STATUS.md) - Project status
