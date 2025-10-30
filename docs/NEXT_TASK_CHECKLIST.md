# Next Task Selection & Environment Setup Checklist

**Purpose**: Quick-reference checklist for selecting and preparing the next DEV-COOKING task
**Version**: 1.0
**Last Updated**: 2025-10-23
**Companion Document**: [WORKTREE_WORKFLOW.md](WORKTREE_WORKFLOW.md)

---

## QUICK DECISION TREE

```text
START: What task should I start next?
  ↓
[1] Check Milestone Status
  ↓
[2] List Not-Started Tasks (current milestone)
  ↓
[3] Verify Dependencies (all 'requires' merged?)
  ↓  YES
[4] Apply Priority Filter (P0 > P1 > P2)
  ↓
[5] Select Task
  ↓
[6] Prepare Worktree Environment
  ↓
[7] BEGIN DEVELOPMENT ✅
```

---

## Phase 1: Review & Analysis

### 1.1 Check Milestone Progress

```shell
# View current milestone status
cat spacs/tasks/0.1.0-mvp/README.md | grep -A 30 "Milestone M"

# Check milestone metadata
yq '.milestones' spacs/tasks/0.1.0-mvp/TODO.yaml
```

**Questions to answer**:

- [ ] Which milestone is active? (M0, M1, M2, M3, M4, M5)
- [ ] What is the target completion date?
- [ ] How many critical-path tasks remain?
- [ ] What is the milestone completion percentage?

### 1.2 Review Recently Completed Tasks

```shell
# Check last merged PRs
gh pr list --state merged --limit 10

# Check last commits to main
git log --oneline -10 main

# Check last worktree updates
git worktree list
```

**Questions to answer**:

- [ ] What tasks were recently merged?
- [ ] Are there new dependencies available?
- [ ] Is main branch synced with latest changes?

---

## Phase 2: Task Selection

### 2.1 List Candidate Tasks

```shell
# List all not-started tasks in current milestone
yq '.tasks | to_entries |
    map(select(.value.milestone == "M1" and .value.status == "not_started")) |
    .[].key' \
  spacs/tasks/0.1.0-mvp/TODO.yaml

# Check critical-path tasks
yq '.milestones.M1.critical_path' spacs/tasks/0.1.0-mvp/TODO.yaml
```

**Checklist**:

- [ ] Found at least one `not_started` task
- [ ] Identified critical-path tasks (priority)
- [ ] Reviewed task titles and estimates

### 2.2 Verify Dependencies

For each candidate task:

```shell
# Check dependencies
yq '.tasks.T-XX-XX.dependencies.requires' spacs/tasks/0.1.0-mvp/TODO.yaml

# Verify PRs merged
gh pr list --state merged --search "T-05-01"
```

**Dependency checklist**:

- [ ] All `requires` tasks have **MERGED** PRs (not just completed)
- [ ] No circular dependencies exist
- [ ] Baseline data available (for parity tasks)
- [ ] Test fixtures available (if needed)

### 2.3 Apply Priority Filter

**Decision matrix**:

| Task | Priority | Critical Path? | Deps Met? | Deadline | Score |
|------|----------|----------------|-----------|----------|-------|
| T-XX-XX | P0/P1/P2 | Yes/No | ✅/❌ | X days | ⭐⭐⭐ |

**Selection criteria** (in priority order):

1. ⭐⭐⭐ **P0 + Critical Path + Deps Met + Urgent Deadline**
2. ⭐⭐ **P0 + Critical Path + Deps Met**
3. ⭐⭐ **P1 + Deps Met + Can Parallelize**
4. ⭐ **P2 + Deps Met**

### 2.4 Final Verification

**Before proceeding**:

- [ ] Task specification exists: `spacs/tasks/0.1.0-mvp/{category}/T-XX-XX-{name}.md`
- [ ] All dependencies MERGED (check GitHub PRs)
- [ ] Main branch synced: `git pull origin main`
- [ ] Sufficient time until milestone deadline
- [ ] Required resources available (LocAgent, test repos, etc.)

**Selected Task**: `T-XX-XX-task-name` ✅

---

## Phase 3: Worktree Environment Preparation

### 3.1 Sync Main Branch

⚠️ **CRITICAL: Do this BEFORE creating worktree!**

```shell
# Navigate to main repo
cd ~/dev-space/CDSAgent

# Switch to main
git checkout main

# Pull latest changes
git pull origin main

# Verify latest commit
git log -1 --oneline
```

**Checklist**:

- [ ] Switched to main branch
- [ ] Pulled latest changes from origin
- [ ] Verified commit hash matches GitHub main

### 3.2 Create or Verify Worktree

**Check if worktree already exists** (from M0):

```shell
git worktree list | grep T-XX-XX
```

**If worktree doesn't exist, create it**:

```shell
# Create worktree from synced main
git worktree add .worktrees/T-XX-XX-task-name \
  -b feat/task/T-XX-XX-task-name main

# Example:
git worktree add .worktrees/T-05-02-typescript-bindings \
  -b feat/task/T-05-02-typescript-bindings main
```

**Checklist**:

- [ ] Worktree created successfully
- [ ] Branch name follows convention: `feat/task/T-XX-XX-task-name`
- [ ] Based on latest main commit

### 3.3 Create IDE Symlink

```shell
# Create symlink for easy access
./scripts/worktree-symlink.sh create T-XX-XX-task-name

# Or manually:
ln -s $(pwd)/.worktrees/T-XX-XX-task-name \
  ~/dev-space/CDSAgent-T-XX-XX-task-name

# Verify
ls -la ~/dev-space/CDSAgent-T-XX-XX-task-name
```

**Checklist**:

- [ ] Symlink created at `~/dev-space/CDSAgent-T-XX-XX-task-name`
- [ ] Symlink points to `.worktrees/T-XX-XX-task-name`
- [ ] Can navigate to worktree via symlink

### 3.4 Initialize Task Artifacts

⚠️ **CRITICAL: Run FROM worktree, NOT main!**

**Reason**: Worktrees have independent file systems. Running from main creates artifacts that worktrees can't see.

```shell
# Navigate to worktree FIRST
cd ~/dev-space/CDSAgent-T-XX-XX-task-name

# Run script with absolute path
/Users/arthur/dev-space/CDSAgent/scripts/create-task-worklog.sh \
  T-XX-XX-task-name "Task Title from Spec" "Your Name"

# Verify artifacts created
ls -la .artifacts/spec-tasks-T-XX-XX-task-name/
```

**Checklist**:

- [ ] Ran script FROM worktree (not main)
- [ ] `metadata.yaml` exists in worktree
- [ ] `git-refs.txt` exists in worktree
- [ ] `worklogs/` directory exists in worktree

**Troubleshooting**: If artifacts missing, see [WORKTREE_WORKFLOW.md - Phase 1: Step 5](WORKTREE_WORKFLOW.md#step-5-troubleshooting---if-artifacts-missing)

### 3.5 Update Task-Specific CLAUDE.md (Optional)

**Purpose**: Add task context for AI-assisted development

```shell
cd ~/dev-space/CDSAgent-T-XX-XX-task-name
vim CLAUDE.md
```

**What to add**:

- [ ] Current task section with objective and deliverables
- [ ] Task-specific development commands
- [ ] Implementation checklist (5-7 steps)
- [ ] API reference quick links (if applicable)
- [ ] Testing strategy with examples

**Example**: See `.worktrees/T-05-02-typescript-bindings/CLAUDE.md` (commit 0b4181d)

### 3.6 Create Daily Worklog

```shell
# From worktree
cd ~/dev-space/CDSAgent-T-XX-XX-task-name

# Create today's worklog
/Users/arthur/dev-space/CDSAgent/scripts/create-daily-worklog.sh T-XX-XX-task-name

# Verify
ls -la .artifacts/spec-tasks-T-XX-XX-task-name/worklogs/$(date +%Y-%m-%d)-*.md
```

**Checklist**:

- [ ] `YYYY-MM-DD-work-summary.md` created
- [ ] `YYYY-MM-DD-commit-log.md` created
- [ ] `YYYY-MM-DD-notes.md` created

### 3.7 Read Task Specification

```shell
# Read task spec
cat spacs/tasks/0.1.0-mvp/{category}/T-XX-XX-task-name.md

# Read related PRD
cat spacs/prd/0.1.0-MVP-PRDs-v0/{XX}-{prd-name}.md

# Read related Issue
cat spacs/issues/04-0.1.0-mvp/{XX}-{issue-name}.md
```

**Understanding checklist**:

- [ ] Read task deliverables
- [ ] Reviewed acceptance criteria
- [ ] Understood dependencies
- [ ] Reviewed estimated hours
- [ ] Read related PRD context
- [ ] Read related Issue technical breakdown

### 3.8 Final Environment Check

```shell
cd ~/dev-space/CDSAgent-T-XX-XX-task-name

# Verify branch
git branch --show-current  # Should show: feat/task/T-XX-XX-task-name

# Verify artifacts
ls -la .artifacts/spec-tasks-T-XX-XX-task-name/metadata.yaml

# Verify clean state
git status

# Open in IDE
code .
```

**Final checklist**:

- [ ] On correct branch: `feat/task/T-XX-XX-task-name`
- [ ] Artifacts initialized correctly
- [ ] Clean git state (no uncommitted changes)
- [ ] IDE opened to worktree
- [ ] Ready to code! ✅

---

## Phase 4: Start Development

**You are now ready to begin coding!**

**Next steps**:

1. Fill out "Today's Objectives" in `worklogs/YYYY-MM-DD-work-summary.md`
2. Start implementing per task specification
3. Commit frequently with meaningful messages
4. Update worklogs throughout the day
5. Follow [WORKTREE_WORKFLOW.md - Phase 2: Daily Development](WORKTREE_WORKFLOW.md#phase-2-daily-development)

---

## Common Scenarios

### Scenario 1: Multiple P0 Tasks Available

**Decision**: Choose based on:

1. **Critical path first** - Tasks that block others
2. **Shortest deadline** - Most urgent
3. **Smallest estimate** - Quick wins to unblock others
4. **Your expertise** - Fastest to complete

### Scenario 2: All Tasks Blocked

**Actions**:

1. Check if blockers are truly merged (verify PRs)
2. Consider P1 or P2 tasks (tests, docs, tooling)
3. Sync with team on Slack/GitHub
4. Update TODO.yaml with blocker comments

### Scenario 3: Worktree Already Exists from M0

**Actions**:

1. Verify worktree is synced with main:

   ```shell
   cd ~/dev-space/CDSAgent-T-XX-XX
   git fetch origin main
   git rebase origin/main
   ```

2. Skip worktree creation, go to artifacts initialization
3. Follow Phase 3 from Step 3.4 onwards

---

## Quick Commands Reference

```shell
# Check milestone
cat spacs/tasks/0.1.0-mvp/README.md | grep -A 30 "Milestone M"

# List not-started tasks
yq '.tasks | to_entries | map(select(.value.status == "not_started"))' \
  spacs/tasks/0.1.0-mvp/TODO.yaml

# Check dependencies
yq '.tasks.T-XX-XX.dependencies' spacs/tasks/0.1.0-mvp/TODO.yaml

# Verify PRs merged
gh pr list --state merged --limit 10

# Sync main
cd ~/dev-space/CDSAgent && git checkout main && git pull origin main

# Create worktree
git worktree add .worktrees/T-XX-XX-task-name -b feat/task/T-XX-XX-task-name main

# Create symlink
./scripts/worktree-symlink.sh create T-XX-XX-task-name

# Initialize artifacts (from worktree!)
cd ~/dev-space/CDSAgent-T-XX-XX-task-name
/Users/arthur/dev-space/CDSAgent/scripts/create-task-worklog.sh \
  T-XX-XX-task-name "Task Title" "Your Name"

# Create daily worklog
/Users/arthur/dev-space/CDSAgent/scripts/create-daily-worklog.sh T-XX-XX-task-name

# Read spec
cat spacs/tasks/0.1.0-mvp/{category}/T-XX-XX-task-name.md
```

---

## Troubleshooting

### Issue: Can't decide between multiple tasks

**Solution**: Use decision matrix (Section 2.3) or ask tech lead

### Issue: Artifacts not visible in worktree

**Solution**: See [WORKTREE_WORKFLOW.md - Troubleshooting - Artifacts not visible](WORKTREE_WORKFLOW.md#issue-artifacts-not-visible-in-worktree)

**Quick fix**:

```shell
cd ~/dev-space/CDSAgent-T-XX-XX
/Users/arthur/dev-space/CDSAgent/scripts/create-task-worklog.sh \
  T-XX-XX-task-name "Task Title" "Your Name"
```

### Issue: Don't understand task specification

**Solution**:

1. Read related PRD: `spacs/prd/0.1.0-MVP-PRDs-v0/`
2. Read related Issue: `spacs/issues/04-0.1.0-mvp/`
3. Check LocAgent reference: `tmp/LocAgent/`
4. Ask tech lead or create GitHub discussion

---

## Resources

- **Full Workflow SOP**: [WORKTREE_WORKFLOW.md](WORKTREE_WORKFLOW.md)
- **Task Registry**: [spacs/tasks/0.1.0-mvp/TODO.yaml](../spacs/tasks/0.1.0-mvp/TODO.yaml)
- **Milestone Overview**: [spacs/tasks/0.1.0-mvp/README.md](../spacs/tasks/0.1.0-mvp/README.md)
- **Development Status**: [DEVELOPMENT_STATUS.md](../DEVELOPMENT_STATUS.md)

---

**Version**: 1.0
**Created**: 2025-10-23
**Maintainer**: CDSAgent Tech Lead

---

END OF CHECKLIST
