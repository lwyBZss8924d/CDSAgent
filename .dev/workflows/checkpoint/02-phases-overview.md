# Work Session Checkpoint - Phases Overview

**Part of**: [Work Session Checkpoint Workflow](./../WORK_SESSION_CHECKPOINT_WORKFLOW.md)

---

## QUICK DECISION TREE

```text
START: End of Work Session
  ↓
┌─────────────────────────────────────────┐
│ 1. Review Raw Action Logs               │
│    • What was actually done?            │
│    • What commits were made?            │
└─────────┬───────────────────────────────┘
          ↓
┌─────────────────────────────────────────┐
│ 2. Check Git Operations                 │
│    • git status | log | show | notes    │
│    • Verify commit hashes               │
└─────────┬───────────────────────────────┘
          ↓
┌─────────────────────────────────────────┐
│ 3. Read All Artifact Files              │
│    • metadata.yaml                      │
│    • work-summary.md, commit-log.md     │
│    • notes.md, action logs              │
└─────────┬───────────────────────────────┘
          ↓
┌─────────────────────────────────────────┐
│ 4. Run Consistency Check                │
│    • Create consistency matrix          │
│    • Compare git actual vs documented   │
└─────────┬───────────────────────────────┘
          ↓
      ┌───┴───┐
      │ Issues│ YES → Fix Issues
      │ Found?│ ↓
      └───┬───┘ ┌─────────────────────────┐
          │     │ 5. Update Artifacts     │
          │     │    • Fix metadata       │
          │     │    • Update worklogs    │
          │     │    • Add statistics     │
          │     └──────────┬──────────────┘
          │                ↓
          │     ┌─────────────────────────┐
          │     │ 6. Git Operations       │
          │     │    • git add artifacts  │
          │     │    • git commit         │
          │     │    • git push           │
          │     └──────────┬──────────────┘
          │                ↓
          NO               ↓
          ↓                ↓
┌─────────────────────────────────────────┐
│ 7. Final Verification                   │
│    • Re-run consistency check           │
│    • Confirm 100% accuracy              │
│    • Document checkpoint complete       │
└─────────┬───────────────────────────────┘
          ↓
      ✅ CHECKPOINT COMPLETE
          ↓
    Ready for Next Session
```

**Key Principle**: **Git operations are the source of truth.** All artifacts must match git reality.

---

## The 5 Phases

### [Phase 1: Review & Data Collection](03-phase1-review.md)

**Objective**: Gather all information needed for consistency verification

**Time Estimate**: 5-10 minutes

**Mode**: Read-only (no file modifications)

**What You'll Do**:

- Read raw action logs
- Check git operations (status, log, show)
- Read all artifact files (metadata.yaml, worklogs)
- Extract key metrics from each source

### [Phase 2: Consistency Verification](04-phase2-verification.md)

**Objective**: Create a consistency matrix comparing git actual vs documented values

**Time Estimate**: 5-10 minutes

**Mode**: Analysis only (no file modifications)

**What You'll Do**:

- Create consistency matrix (git vs artifacts)
- Identify discrepancies (critical, important, minor)
- Calculate consistency score
- Determine action plan

### [Phase 3: Update Artifacts](05-phase3-update.md)

**Objective**: Fix all identified discrepancies to achieve 100% consistency

**Time Estimate**: 10-20 minutes (depending on issues)

**Mode**: Edit files to match git reality

**What You'll Do**:

- Fix metadata.yaml (hash, files_changed, metrics)
- Update action logs (test counts, statistics)
- Complete worklog files (fill templates)
- Cross-verify all fixes

### [Phase 4: Git Operations](06-phase4-git.md)

**Objective**: Commit artifact updates and push to remote

**Time Estimate**: 2-5 minutes

**Mode**: Git operations only (no code changes)

**What You'll Do**:

- Stage artifact changes only
- Commit with descriptive message
- Add git notes (optional)
- Push to remote

### [Phase 5: Final Verification](07-phase5-final.md)

**Objective**: Confirm 100% consistency and document checkpoint completion

**Time Estimate**: 5 minutes

**Mode**: Read-only verification

**What You'll Do**:

- Re-run git check
- Re-run consistency check
- Document checkpoint completion
- Record next session plan

---

## Quick Summary

**Total Time**: 25-50 minutes depending on issues found

**Core Process**:

1. **Collect** data from action logs, git, artifacts
2. **Compare** git actual vs documented (consistency matrix)
3. **Fix** all discrepancies to reach 100% consistency
4. **Commit** artifact updates
5. **Verify** and document completion

**Success Criteria**: 100% consistency score (all metrics match git reality)

---

**Navigation**:

- [← Back to Overview](01-overview.md)
- [← Back to Main Index](./../WORK_SESSION_CHECKPOINT_WORKFLOW.md)
- [→ Next: Phase 1 - Review & Data Collection](03-phase1-review.md)
