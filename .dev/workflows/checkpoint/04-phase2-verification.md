# Phase 2: Consistency Verification

**Part of**: [Work Session Checkpoint Workflow](./../WORK_SESSION_CHECKPOINT_WORKFLOW.md)

---

**Objective**: Create a consistency matrix comparing git actual vs documented values

**Time Estimate**: 5-10 minutes

**Mode**: Analysis only (no file modifications)

---

## Step 2.1: Create Consistency Matrix

**Template**:

| Metric | Git Actual | metadata.yaml | work-summary | commit-log | notes | action-log | Status |
|--------|-----------|---------------|--------------|------------|-------|------------|--------|
| Commit Hash | [from git log] | [from yaml] | N/A | [from md] | N/A | N/A | ✅/❌ |
| Files Changed | [from git show] | [from yaml] | [from md] | [from md] | N/A | [from txt] | ✅/❌ |
| Lines Added | [from git show] | [from yaml] | [from md] | [from md] | N/A | [from txt] | ✅/❌ |
| Lines Deleted | [from git show] | [from yaml] | [from md] | [from md] | N/A | [from txt] | ✅/❌ |
| Tests Added | [from code] | [from yaml] | [from md] | N/A | [from md] | [from txt] | ✅/❌ |
| Import Parity | [from test output] | N/A | [from md] | N/A | [from md] | [from txt] | ✅/❌ |
| Invoke Variance | [from test output] | N/A | [from md] | N/A | [from md] | [from txt] | ✅/❌ |

**Example (T-02-01 Day 3)**:

| Metric | Git Actual | metadata.yaml | work-summary | commit-log | notes | action-log | Status |
|--------|-----------|---------------|--------------|------------|-------|------------|--------|
| Commit Hash | 3083e00 | 3083e00 | ✅ | 3083e00 | N/A | N/A | ✅ |
| Files Changed | 8 | 8 | ✅ | 8 | N/A | 8 | ✅ |
| Lines Added | +1,361 | +817 | ✅ | ✅ | N/A | +1,361 | ⚠️ |
| Lines Deleted | -64 | -52 | ✅ | ✅ | N/A | -64 | ⚠️ |
| Tests Added | 3 | 3 | ✅ | ✅ | ✅ | 3 | ✅ |
| Import Parity | 0% (218/218) | ✅ | ✅ | N/A | ✅ | ✅ | ✅ |
| Invoke Variance | +1.9% | ✅ | ✅ | N/A | ✅ | ✅ | ✅ |

**Interpretation**:

- ✅ = Value matches git actual or is documented correctly
- ⚠️ = Value differs slightly but not critical
- ❌ = Value significantly wrong or missing

---

## Step 2.2: Identify Discrepancies

**Priority Levels**:

**Priority 1: CRITICAL** (Must fix before checkpoint)

- ❌ Commit hash = "PENDING" or wrong
- ❌ Files changed count significantly wrong (off by >2)
- ❌ Template placeholders still present (e.g., `{COMMIT_HASH_SHORT}`)
- ❌ Worklog files empty or not filled out

**Priority 2: IMPORTANT** (Should fix for accuracy)

- ⚠️ Lines added/deleted slightly wrong (off by <20%)
- ⚠️ Test count wrong (said "two" but actually three)
- ⚠️ Missing Statistics section in action log
- ⚠️ Notes field in metadata incomplete

**Priority 3: MINOR** (Nice to fix but not blocking)

- Description wording could be clearer
- Additional context could be added
- Minor typos in documentation
- Cross-references could be improved

**Example Discrepancy List** (T-02-01 Day 3, after first review):

**Critical**:

- ❌ metadata.yaml: `hash: "PENDING"` → Should be `"3083e00"`
- ❌ commit-log.md: Template placeholders → Should be filled with actual commit details
- ❌ notes.md: Template placeholders → Should be filled with technical details

**Important**:

- ⚠️ metadata.yaml: `files_changed: 3` → Should be `8`
- ⚠️ action log: Said "two" unit tests → Should say "three"
- ⚠️ action log: Missing Statistics section → Should add complete breakdown

**Minor**:

- None identified

---

## Step 2.3: Calculate Consistency Score

**Formula**:

```text
Consistency Score = (Correct Metrics / Total Metrics) × 100%
```

**Example**:

- Total metrics checked: 14
- Correct: 8
- Slightly off: 2
- Wrong: 4

**Calculation**:

```text
Consistency Score = (8 + 2×0.5) / 14 × 100% = 64.3%
```

**Target**: **100% consistency** before checkpoint completion

**Thresholds**:

- **100%**: Perfect ✅ - Ready for checkpoint
- **90-99%**: Good ⚠️ - Minor fixes needed
- **75-89%**: Needs work ❌ - Multiple fixes required
- **<75%**: Critical ⛔ - Major discrepancies, significant work needed

---

**Navigation**:

- [← Back to Phase 1](03-phase1-review.md)
- [← Back to Main Index](./../WORK_SESSION_CHECKPOINT_WORKFLOW.md)
- [→ Next: Phase 3 - Update Artifacts](05-phase3-update.md)
