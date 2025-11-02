# Git Workflow Validation - Complete Reference

This document provides detailed information about CDSAgent's git workflow validation, checkpoint process, and git notes management.

## Table of Contents

- [Complete Checkpoint Workflow](#complete-checkpoint-workflow)
- [Git Notes Deep Dive](#git-notes-deep-dive)
- [Commit Message Standards](#commit-message-standards)
- [Advanced Git Notes Management](#advanced-git-notes-management)
- [CI/CD Integration](#cicd-integration)

---

## Complete Checkpoint Workflow

The checkpoint workflow ensures session artifacts are complete and validated before committing.

### 5-Phase Checkpoint Process

Detailed guide: `.dev/workflows/WORK_SESSION_CHECKPOINT_WORKFLOW.md`

#### Phase 1: Review Session Work

**Purpose**: Review what was accomplished in the session

**Checklist**:

- [ ] All threads completed
- [ ] Code changes compile and pass tests
- [ ] Session objectives met
- [ ] No uncommitted changes (except checkpoint artifacts)

**Chapter**: `.dev/workflows/checkpoint/03-phase1-review.md`

#### Phase 2: Verify Artifacts

**Purpose**: Ensure all session artifacts are created

**Required Artifacts**:

- [ ] Work summary: `.artifacts/spec-tasks-T-XX-XX/worklogs/{date}-S{NN}-work-summary.md`
- [ ] Commit log: `.artifacts/spec-tasks-T-XX-XX/worklogs/{date}-S{NN}-commit-log.md`
- [ ] Technical notes: `.artifacts/spec-tasks-T-XX-XX/worklogs/{date}-S{NN}-notes.md`
- [ ] RAW log: `.artifacts/spec-tasks-T-XX-XX/worklogs/raw/WORK-SESSIONS-{NN}-THREADS-{START}-{END}-SUMMARY-{date}.txt`
- [ ] Code review (optional): `.artifacts/spec-tasks-T-XX-XX/worklogs/{date}-S{NN}-codereview.md`

**Chapter**: `.dev/workflows/checkpoint/04-phase2-verification.md`

#### Phase 3: Update Metadata

**Purpose**: Update task metadata with session information

**Updates Required**:

- [ ] metadata.yaml: Add session entry
- [ ] metadata.yaml: Update `last_updated` timestamp
- [ ] metadata.yaml: Increment `actual_hours`
- [ ] metadata.yaml: Update metrics (lines_added, files_modified, etc.)
- [ ] metadata.yaml: Add git commit hashes
- [ ] metadata.yaml: Update acceptance criteria status

**Chapter**: `.dev/workflows/checkpoint/05-phase3-update.md`

#### Phase 4: Git Operations

**Purpose**: Validate git notes and prepare checkpoint commit

**Steps**:

1. **Validate Git Notes**:

   ```shell
   # Run validation script
   ./.dev/scripts/validation/git-notes-check.sh
   ```

2. **Add Missing Notes** (if validation fails):

   ```shell
   # Find commits without notes
   git log --all --oneline | while read hash msg; do
     git notes show $hash 2>/dev/null || echo "Missing: $hash $msg"
   done

   # Add notes
   git notes add <hash> -m "spec-tasks/T-XX-XX: Session NN Thread MM - description"
   ```

3. **Push Git Notes**:

   ```shell
   git push origin refs/notes/commits
   ```

4. **Create Checkpoint Commit**:

   ```shell
   # Stage checkpoint artifacts
   git add .artifacts/spec-tasks-T-XX-XX/

   # Create checkpoint commit
   git commit -m "checkpoint(worklog): T-XX-XX Session NN complete - description"

   # Add git note to checkpoint commit
   git notes add HEAD -m "spec-tasks/T-XX-XX: Session NN Checkpoint - description"
   ```

**Chapter**: `.dev/workflows/checkpoint/06-phase4-git.md`

#### Phase 5: Final Validation

**Purpose**: Verify checkpoint is complete and ready to push

**Final Checks**:

- [ ] All artifacts committed
- [ ] All commits have git notes
- [ ] Git notes pushed to origin
- [ ] metadata.yaml updated
- [ ] Checkpoint commit created
- [ ] Ready to push to origin

**Chapter**: `.dev/workflows/checkpoint/07-phase5-final.md`

### Checkpoint Helper Script

The `checkpoint-helper.sh` script automates the checkpoint workflow:

```shell
# Run from worktree
./.dev/scripts/validation/checkpoint-helper.sh T-XX-XX
```

**Interactive Prompts**:

1. Review session artifacts
2. Verify metadata.yaml
3. Check git notes
4. Validate commit messages
5. Approve checkpoint

**Exit Codes**:

- `0`: Checkpoint approved, ready to commit
- `1`: Validation failed, fix issues
- `2`: Warnings, review before proceeding

---

## Git Notes Deep Dive

### Purpose of Git Notes

Git notes provide metadata for commits without modifying commit history:

- **Link commits to tasks**: `spec-tasks/T-XX-XX`
- **Track sessions**: `Session NN`
- **Record threads**: `Thread MM`
- **Describe work**: Brief description

### Git Notes Format

**Standard Format**:

```text
spec-tasks/T-XX-XX: Session NN Thread MM - brief description
```

**Examples**:

```text
spec-tasks/T-02-02: Session 04 Thread 01 - Phase 2 planning
spec-tasks/T-02-02: Session 04 Thread 06 - BM25 index scaffold
spec-tasks/T-02-01: Session 03 Thread 04 - Coverage hardening
```

### Git Notes Storage

Notes are stored in a separate branch: `refs/notes/commits`

**View Notes**:

```shell
# Show notes for all commits
git log --show-notes

# Show notes for specific commit
git notes show <commit-hash>

# List all notes
git notes list
```

### Pushing Git Notes

**CRITICAL**: Git notes must be pushed explicitly:

```shell
# Push notes to origin
git push origin refs/notes/commits

# Fetch notes from origin
git fetch origin refs/notes/commits:refs/notes/commits
```

**Common Mistake**: Forgetting to push notes separately from commits.

### Git Notes Best Practices

1. **Immediate Addition**: Add notes right after committing
2. **Consistent Format**: Always use `spec-tasks/T-XX-XX: Session NN Thread MM - description`
3. **Session Tracking**: Reference correct session number (sequential across days)
4. **Thread Tracking**: Reference correct thread number (resets per session)
5. **Push with Commits**: Always push notes in the same PR as commits

---

## Commit Message Standards

CDSAgent follows Conventional Commits specification.

### Format

```text
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types (Detailed)

**feat**: New feature or capability

```text
feat(index): T-02-02 Phase 2 - tokenizer + BM25 scaffold
feat(graph): T-02-01 - implement traversal API
```

**fix**: Bug fix

```text
fix(index): T-02-02 - fix tokenizer offset preservation
fix(tests): T-02-01 - correct parity test fixture paths
```

**docs**: Documentation only changes

```text
docs(api): T-05-01 - update JSON-RPC schema documentation
docs(worklog): T-02-02 - complete Session 04 notes
```

**chore**: Maintenance tasks

```text
chore(pm): update TODO.yaml - T-02-02 Session 04 progress
chore(config): update Claude auto-approval settings
```

**refactor**: Code restructuring without functionality change

```text
refactor(graph): T-02-01 - modularize builder into components
```

**test**: Adding or modifying tests

```text
test(index): T-02-02 - add tokenizer unit tests
test(graph): T-02-01 - add parity validation tests
```

**checkpoint**: Session checkpoint commit

```text
checkpoint(worklog): T-02-02 Day 2 Session 04 complete - Phase 2 delivered
checkpoint(worklog): T-02-01 Day 6 complete
```

**init**: Session initialization

```text
init(session): T-02-02 Day 2 Session 04 - Phase 2 Planning & Prep kickoff
init(session): T-02-01 Day 1 Session 01 kickoff
```

### Scopes (Detailed)

**Task ID**: `T-XX-XX` format

```text
feat(T-02-02): implement sparse index
```

**Component**: System component

```text
feat(index): add BM25 ranking
feat(graph): implement node builder
feat(cli): add search command
```

**Phase**: Session phase

```text
checkpoint(worklog): Phase 2 complete
init(session): Phase 1 kickoff
```

### Body and Footer (Optional)

**Body**: Additional context (wrapped at 72 characters)

```text
feat(index): T-02-02 Phase 2 - tokenizer + BM25 scaffold

Implements custom tokenizer matching LocAgent behavior:
- Preserves character offsets
- Uses NLTK stop words
- Integrates with Tantivy analyzer

BM25 index scaffold includes:
- Tantivy backend setup
- Custom analyzer configuration
- Search API placeholder
```

**Footer**: Breaking changes, references

```text
BREAKING CHANGE: index API now requires builder pattern
Refs: #42, spacs/issues/04-0.1.0-mvp/02-index-core/02-sparse-index.md
```

---

## Advanced Git Notes Management

### View All Notes

```shell
# Show notes for all commits (most recent first)
git log --show-notes

# Show notes for specific commit
git notes show <commit-hash>

# List all notes with commit hashes
git notes list
```

### Edit Existing Notes

```shell
# Edit note in default editor
git notes edit <commit-hash>

# Append to existing note
git notes append <commit-hash> -m "Additional information"
```

### Copy Notes Between Commits

```shell
# Copy note from one commit to another
git notes copy <source-hash> <dest-hash>
```

### Remove Notes

```shell
# Remove note from specific commit
git notes remove <commit-hash>

# Remove all notes (DANGEROUS)
git notes remove --all
```

### Prune Invalid Notes

```shell
# Remove notes for non-existent commits
git notes prune
```

### Merge Notes

```shell
# Fetch notes from origin
git fetch origin refs/notes/commits:refs/notes/commits

# Merge notes (if conflicts)
git notes merge refs/notes/commits
```

### Notes for Multiple Refs

```shell
# Show notes for specific ref
git notes --ref=custom show <commit-hash>

# Use different notes ref
git notes --ref=custom add <commit-hash> -m "Custom note"
```

---

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Validate Git Workflow

on: [push, pull_request]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
        with:
          fetch-depth: 0  # Fetch all history for git notes

      - name: Fetch Git Notes
        run: |
          git fetch origin refs/notes/commits:refs/notes/commits

      - name: Validate Git Notes
        run: |
          ./.dev/scripts/validation/git-notes-check.sh
          if [ $? -ne 0 ]; then
            echo "❌ Missing git notes! Please add notes to all commits."
            echo "Format: spec-tasks/T-XX-XX: Session NN Thread MM - description"
            exit 1
          fi

      - name: Validate Commit Messages
        run: |
          # Check conventional commits format
          git log --format=%s origin/main..HEAD | while read msg; do
            if ! echo "$msg" | grep -qE '^(feat|fix|docs|chore|refactor|test|checkpoint|init)\(.*\):'; then
              echo "❌ Invalid commit message: $msg"
              echo "Expected format: <type>(<scope>): <description>"
              exit 1
            fi
          done
```

### Pre-Push Hook

```shell
#!/bin/bash
# .git/hooks/pre-push

echo "🔍 Validating git workflow before push..."

# Run git notes validation
./.dev/scripts/validation/git-notes-check.sh
if [ $? -ne 0 ]; then
  echo ""
  echo "❌ Push blocked: Missing git notes!"
  echo "Fix: Add notes to commits before pushing"
  echo ""
  exit 1
fi

echo "✅ Git workflow validation passed"
exit 0
```

---

## Troubleshooting

### Issue: Notes Not Syncing

**Problem**: Notes added locally don't appear on remote

**Solution**:

```shell
# Explicitly push notes
git push origin refs/notes/commits

# Verify notes on remote
git ls-remote origin refs/notes/commits
```

### Issue: Conflicting Notes

**Problem**: Different notes for same commit on local vs remote

**Solution**:

```shell
# Fetch remote notes
git fetch origin refs/notes/commits:refs/notes/commits

# Merge notes (manual resolution if conflict)
git notes merge refs/notes/commits
```

### Issue: Lost Notes After Rebase

**Problem**: Notes disappear after interactive rebase

**Solution**:

```shell
# Notes are tied to commit hashes
# After rebase, commit hashes change
# Use git notes copy to transfer notes

# Before rebase: note original hashes
git log --oneline --show-notes

# After rebase: copy notes to new hashes
git notes copy <old-hash> <new-hash>
```

---

## References

- **Primary Workflow**: `.dev/workflows/WORK_SESSION_CHECKPOINT_WORKFLOW.md`
- **Checkpoint Chapters**: `.dev/workflows/checkpoint/01-overview.md` through `11-commands.md`
- **Git Notes Official Documentation**: `https://git-scm.com/docs/git-notes`
- **Conventional Commits**: `https://www.conventionalcommits.org/`
- **Worktree SOP**: `.dev/workflows/WORKTREE_WORKFLOW.md`

**Important⚠️**: Wen Write [git-workflow-validation] job's commit message and git notes, MUST run (date -u '+%Y-%m-%dT%H:%M:%SZ') get UTC TIME NOW first!!!
