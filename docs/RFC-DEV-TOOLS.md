# RFC: Unified Development Tools Architecture

**Version**: 1.0
**Date**: 2025-10-31
**Status**: Approved
**Authors**: CDSAgent Tech Lead

---

## Executive Summary

This RFC defines a unified development tools architecture for CDSAgent that optimizes for:

1. **AI Assistant Convenience** - Shell-executable, observable, fail-fast
2. **Developer Productivity** - Context-aware, automated checks, clear errors
3. **Accuracy** - Validation before execution, consistency checks
4. **Maintainability** - Shared libraries, tested, documented

---

## Quick Reference for AI Assistants (Claude/Codex)

### 🚀 Essential Commands

```bash
# Task Initialization (from worktree)
cd ~/dev-space/CDSAgent-T-XX-XX-task-name
/Users/arthur/dev-space/CDSAgent/scripts/create-task-worklog.sh \
  T-XX-XX-task-name "Task Title" "Developer Name"
/Users/arthur/dev-space/CDSAgent/scripts/create-daily-worklog.sh T-XX-XX-task-name

# Before Every Checkpoint
./scripts/checkpoint-helper.sh T-XX-XX-task-name

# Git Notes Verification
./scripts/git-notes-check.sh

# Daily Worklog Creation
./scripts/create-daily-worklog.sh T-XX-XX-task-name [YYYY-MM-DD]
```

### ✅ Success Patterns

```bash
# Pattern 1: Always run from worktree for artifact operations
cd ~/dev-space/CDSAgent-T-XX-XX  # ✅ CORRECT
./path/to/script.sh

# NOT from main repo!
cd ~/dev-space/CDSAgent  # ❌ WRONG
./scripts/script.sh      # Artifacts won't be visible in worktree

# Pattern 2: Verify before checkpoint
./scripts/checkpoint-helper.sh T-XX-XX
# Wait for ✓ Ready for checkpoint!
# Then proceed with Phase 4 git operations

# Pattern 3: Check git notes before push
./scripts/git-notes-check.sh
# Fix any missing notes BEFORE git push
```

### ⚠️ Common Pitfalls

| Problem | Detection | Prevention |
|---------|-----------|------------|
| Running from wrong directory | `create-daily-worklog.sh` shows warning | Always `cd` to worktree first |
| Missing git notes | `git-notes-check.sh` fails | Run check before every push |
| Uncommitted code changes | `checkpoint-helper.sh` reports | Commit code before checkpoint |
| "/" in task titles | Built-in fix (sed uses `\|`) | No action needed (auto-fixed) |

---

## Architecture Overview

### Current State (v1.0 - Implemented)

```tree
scripts/
├── create-task-worklog.sh      # ✅ Fixed: sed separator, handles "/"
├── create-daily-worklog.sh     # ✅ Enhanced: worktree validation
├── git-notes-check.sh          # ✅ New: automated notes verification
├── checkpoint-helper.sh        # ✅ New: pre-checkpoint validation
├── worktree-symlink.sh         # Existing: symlink management
└── sync-worktrees.sh           # Existing: worktree sync
```

### Future State (v2.0 - Planned)

```tree
tools/dev-workflow/
├── bin/                        # Wrapper commands
│   ├── task-init              # → calls planning + execution
│   ├── checkpoint             # → calls checkpoint-helper.sh
│   ├── git-notes-verify       # → calls git-notes-check.sh
│   └── daily-log              # → calls create-daily-worklog.sh
│
├── lib/                       # Shared libraries
│   ├── common.sh              # Colors, logging, errors
│   ├── validation.sh          # Input/context validation
│   ├── observability.sh       # Status displays
│   └── git-utils.sh           # Git operations
│
├── plans/                     # Lua planning (optional)
│   ├── task-analysis.lua      # Dependency analysis
│   └── workflow-plan.lua      # Multi-step planning
│
├── core/                      # Current scripts (refactored)
│   └── [existing *.sh scripts with lib/ imports]
│
└── tests/                     # Integration tests
    └── test-*.sh
```

---

## Design Principles

### 1. Claude-Optimized Execution

**Problem**: AI assistants need tools that are:

- Executable via single Bash command
- Observable (clear output to parse)
- Fail-fast (errors before damage)

**Solution**:

```shell
# Single command, clear output
./scripts/checkpoint-helper.sh T-02-02-sparse-index

# Colored, structured output
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   Checkpoint Helper - Pre-flight Checks
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[1/5] Checking git status...
  ✓ Only artifact changes (expected)

[2/5] Checking git notes...
  ✗ 2 out of 3 commits missing notes

Action: Run ./scripts/git-notes-check.sh for details
```

**Exit codes**:

- `0` = Success (ready to proceed)
- `1` = Failure (fix required)
- `2` = Warnings (optional fixes)

### 2. User-Friendly Interfaces

**Problem**: Developers need quick, memorable commands

**Solution**:

```shell
# Short names (future v2.0)
task-init T-02-02-sparse-index
checkpoint
daily-log

# Context inference (auto-detect task ID from directory)
cd ~/dev-space/CDSAgent-T-02-02-sparse-index
checkpoint  # No need to specify task ID

# Help everywhere
checkpoint --help
task-init --help
```

### 3. Fail-Fast Validation

**Problem**: Errors discovered late waste time

**Solution**: Validate before executing

```shell
# Pre-flight checks before checkpoint
./scripts/checkpoint-helper.sh T-XX-XX

# Checks:
# 1. Git status (clean or only artifacts)
# 2. Git notes (all commits)
# 3. Daily worklogs (exist)
# 4. Metadata (no PENDING)
# 5. Artifacts (complete)

# Exit 1 if any check fails
# → User fixes → Re-run → Exit 0 → Proceed
```

### 4. Observable Progress

**Problem**: Long operations lack feedback

**Solution**: Progress indicators and status

```shell
# Example: checkpoint-helper.sh
[1/5] Checking git status...       ✓ Done
[2/5] Checking git notes...        ✓ Done
[3/5] Checking daily worklogs...   ✓ Done
[4/5] Checking metadata...         ⚠ Warnings
[5/5] Checking artifacts...        ✓ Done

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Summary Report
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Passed:   4
Warnings: 1
Failed:   0
```

---

## Implementation Details

### Fixed Scripts (v1.0 - DONE)

#### 1. `create-task-worklog.sh`

**Fix**: Sed separator issue

```bash
# Before (BROKEN with "/" in title):
sed -i.bak "s/{TASK_TITLE}/${TASK_TITLE}/g" ...

# After (FIXED):
sed -i.bak "s|{TASK_TITLE}|${TASK_TITLE}|g" ...
```

**Usage**:

```shell
cd ~/dev-space/CDSAgent-T-02-02-sparse-index
/Users/arthur/dev-space/CDSAgent/scripts/create-task-worklog.sh \
  T-02-02-sparse-index "Sparse Index - Name/ID + BM25 Search" "Rust Dev 2"
```

**Success**: Creates `.artifacts/spec-tasks-T-02-02-sparse-index/`

---

#### 2. `create-daily-worklog.sh`

**Enhancement**: Worktree context validation

```shell
# Warns if not run from worktree
if [[ ! "$CURRENT_DIR" =~ /.worktrees/${TASK_ID} ]]; then
    echo "⚠ Warning: Not running from worktree!"
    echo "Recommendation: cd ~/dev-space/CDSAgent-${TASK_ID}"
fi
```

**Usage**:

```shell
cd ~/dev-space/CDSAgent-T-02-02-sparse-index
/Users/arthur/dev-space/CDSAgent/scripts/create-daily-worklog.sh T-02-02-sparse-index
```

**Success**: Creates 3 worklog files:

- `2025-10-31-work-summary.md`
- `2025-10-31-commit-log.md`
- `2025-10-31-notes.md`

---

#### 3. `git-notes-check.sh` (NEW)

**Purpose**: Verify all commits have git notes

**Algorithm**:

1. Get commits since `origin/main`
2. For each commit, check `git notes show`
3. Report missing/invalid notes
4. Exit 1 if any missing

**Usage**:

```shell
# Check all commits
./scripts/git-notes-check.sh

# Check from specific base
./scripts/git-notes-check.sh <base-commit>
```

**Output (failure)**:

```shell
✗ Git notes check FAILED

Commits missing notes (2):
  - a3f4d89: feat(index): implement BM25
  - 7e2c1b0: test(index): add BM25 tests

How to fix:
1. Add git notes to each commit:
   git notes add -m "spec-tasks/T-XX-XX
   Day: X
   Date: YYYY-MM-DD
   ..." <commit-hash>

2. Push notes:
   git push origin refs/notes/commits
```

---

#### 4. `checkpoint-helper.sh` (NEW)

**Purpose**: Pre-checkpoint comprehensive validation

**5 Checks**:

| Check | What | Why |
|-------|------|-----|
| 1. Git Status | Clean or only artifacts | No uncommitted code |
| 2. Git Notes | All commits have notes | Checkpoint requires notes |
| 3. Daily Worklogs | Today's files exist | Documentation complete |
| 4. Metadata | No PENDING/TODO/FIXME | Real values filled |
| 5. Artifacts | metadata.yaml, git-refs.txt, worklogs/ | Complete structure |

**Usage**:

```shell
# Before Phase 4 of checkpoint workflow
./scripts/checkpoint-helper.sh T-02-02-sparse-index
```

**Output (success)**:

```shell
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   Checkpoint Helper - Pre-flight Checks
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[1/5] Checking git status...
  ✓ Only artifact changes (expected)

[2/5] Checking git notes...
  ✓ All 3 commits have git notes

[3/5] Checking daily worklogs...
  ✓ Today's worklogs exist

[4/5] Checking metadata consistency...
  ✓ No PENDING fields in metadata
  ✓ Commit count consistent (3 in metadata, 3 in git)

[5/5] Checking artifact completeness...
  ✓ All required artifacts exist

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✓ Ready for checkpoint!

Next steps:
  1. Review action logs and git operations
  2. Follow checkpoint workflow: docs/WORK_SESSION_CHECKPOINT_WORKFLOW.md
  3. Run consistency check (Phase 2)
  4. Update artifacts (Phase 3)
  5. Commit artifacts (Phase 4)
```

---

## Integration with SOPs

### CLAUDE.md Reference (Minimal Prompt)

```markdown
## Development Tools Quick Reference

### Task Initialization
cd ~/dev-space/CDSAgent-T-XX-XX
/path/to/scripts/create-task-worklog.sh T-XX-XX "Title" "Developer"
/path/to/scripts/create-daily-worklog.sh T-XX-XX

### Before Checkpoint (REQUIRED)
./scripts/checkpoint-helper.sh T-XX-XX
./scripts/git-notes-check.sh

### Common Issues Prevention
- Sed error: Fixed automatically (uses | separator)
- Wrong directory: create-daily-worklog.sh warns
- Missing git notes: git-notes-check.sh detects
- Incomplete checkpoint: checkpoint-helper.sh validates
```

### AGENTS.md Reference (AI Context)

```markdown
## AI Assistant Development Tools

When working with CDSAgent tasks:

**BEFORE starting development**:
- Verify you're in worktree: `pwd` should contain `.worktrees/T-XX-XX`
- Initialize if needed: `scripts/create-task-worklog.sh`
- Create daily log: `scripts/create-daily-worklog.sh`

**BEFORE every checkpoint**:
- Run: `./scripts/checkpoint-helper.sh T-XX-XX`
- Wait for ✓ Ready for checkpoint!
- If ✗ Failed, fix issues and re-run

**BEFORE every git push**:
- Run: `./scripts/git-notes-check.sh`
- Add missing notes: `git notes add -m "..." <hash>`
- Push notes: `git push origin refs/notes/commits`

**Exit codes**:
- 0 = Success, proceed
- 1 = Failure, fix and retry
- 2 = Warnings, review and decide
```

---

## Testing Strategy

### Manual Testing (Current)

Test each script with real T-02-02 development:

1. **create-task-worklog.sh**: Test with "Sparse Index - Name/ID + BM25" title
2. **create-daily-worklog.sh**: Test from worktree vs main (should warn)
3. **git-notes-check.sh**: Test with missing notes (should detect)
4. **checkpoint-helper.sh**: Test before checkpoint (should validate)

### Automated Testing (Future v2.0)

```tree
tools/dev-workflow/tests/
├── test-task-init.sh          # Test task initialization
├── test-checkpoint.sh         # Test checkpoint helper
├── test-git-notes.sh          # Test git notes check
└── test-integration.sh        # End-to-end test
```

---

## Migration Path

### Phase 1 (v1.0 - DONE ✅)

- [x] Fix sed separator in create-task-worklog.sh
- [x] Enhance create-daily-worklog.sh with validation
- [x] Create git-notes-check.sh
- [x] Create checkpoint-helper.sh
- [x] Update documentation (checkpoint/, WORKTREE_WORKFLOW.md)

### Phase 2 (v1.1 - After T-02-02)

- [ ] Test all scripts during T-02-02 development
- [ ] Gather feedback and edge cases
- [ ] Fix any discovered issues
- [ ] Add more validation checks

### Phase 3 (v2.0 - M3 Planning Week)

- [ ] Refactor to tools/dev-workflow/ structure
- [ ] Extract common code to lib/*.sh
- [ ] Create wrapper commands in bin/
- [ ] Add Lua planning integration (optional)
- [ ] Write integration tests
- [ ] Create installation Makefile

---

## Appendix A: Complete Tool Reference

### Tool Matrix

| Tool | Purpose | When | Exit 0 = | Exit 1 = |
|------|---------|------|----------|----------|
| `create-task-worklog.sh` | Initialize artifacts | Once per task | Success | Error |
| `create-daily-worklog.sh` | Create daily logs | Every session | Success | Error |
| `git-notes-check.sh` | Verify notes | Before push | All have notes | Missing notes |
| `checkpoint-helper.sh` | Pre-checkpoint check | Before Phase 4 | Ready | Not ready |
| `worktree-symlink.sh` | Manage symlinks | Setup/cleanup | Success | Error |
| `sync-worktrees.sh` | Sync with main | After PR merge | Success | Error |

### Required Environment

```shell
# All scripts assume:
REPO_ROOT=/Users/arthur/dev-space/CDSAgent
WORKTREE_BASE=~/dev-space/CDSAgent-T-XX-XX
ARTIFACTS_BASE=.artifacts/spec-tasks-T-XX-XX

# Scripts use absolute paths internally
# Can be called from anywhere, but worktree context matters
```

---

## Appendix B: Error Messages & Solutions

### Error: "sed: bad flag in substitute command"

**Cause**: Task title contains "/" (e.g., "Name/ID + BM25")

**Solution**: ✅ Fixed in v1.0 - scripts now use `|` separator

**No action needed** - automatic fix

---

### Error: "Task directory not found"

**Cause**: `create-daily-worklog.sh` run before `create-task-worklog.sh`

**Solution**:

```shell
# Initialize task first
cd ~/dev-space/CDSAgent-T-XX-XX
/path/to/scripts/create-task-worklog.sh T-XX-XX "Title" "Developer"

# Then create daily log
/path/to/scripts/create-daily-worklog.sh T-XX-XX
```

---

### Warning: "Not running from worktree"

**Cause**: Script run from main repo instead of worktree

**Impact**: Artifacts created in main, not visible in worktree

**Solution**:

```shell
# Always navigate to worktree first
cd ~/dev-space/CDSAgent-T-XX-XX  # ✅ CORRECT
./path/to/script.sh
```

---

### Error: "Commits missing notes"

**Cause**: Git notes not added after code commits

**Solution**:

```shell
# Add notes to each commit
git notes add <commit-hash> -m "spec-tasks/T-XX-XX
Day: X
Date: YYYY-MM-DD
Sessions: X-XX to X-XX (HH:MM-HH:MM UTC)
Duration: Xh
Worklog: .artifacts/spec-tasks-T-XX-XX/worklogs/YYYY-MM-DD-*
Status: [summary]
Files: X code files (+XXX/-XXX lines)"

# Push notes
git push origin refs/notes/commits

# Verify
./scripts/git-notes-check.sh
```

---

## Appendix C: Future Enhancements

### Short-term (v1.1)

1. **Add verbose mode** (`-v` flag) for debugging
2. **Add dry-run mode** (`--dry-run`) to preview actions
3. **Add JSON output** (`--json`) for programmatic parsing
4. **Add quiet mode** (`-q`) for CI/CD

### Medium-term (v2.0)

1. **Wrapper commands** (bin/task-init, bin/checkpoint)
2. **Shared libraries** (lib/common.sh, lib/validation.sh)
3. **Lua integration** (plans/*.lua for complex workflows)
4. **Integration tests** (tests/*.sh)

### Long-term (v3.0)

1. **CI/CD integration** (GitHub Actions workflow)
2. **Metrics collection** (development velocity tracking)
3. **Auto-recovery** (rollback on errors)
4. **Multi-user coordination** (lock files, task claiming)

---

## References

- **WORKTREE_WORKFLOW.md**: Complete task development SOP
- **WORK_SESSION_CHECKPOINT_WORKFLOW.md**: Checkpoint process
- **docs/checkpoint/06-phase4-git.md**: Git notes workflow
- **spacs/tasks/0.1.0-mvp/TODO.yaml**: Task tracking registry

---

**Version History**:

- v1.0 (2025-10-31): Initial RFC with v1.0 implementation complete
- Future: v1.1 (after T-02-02), v2.0 (M3 planning)

**Maintainer**: CDSAgent Tech Lead
**Status**: ✅ Approved for Implementation

---

END OF RFC
