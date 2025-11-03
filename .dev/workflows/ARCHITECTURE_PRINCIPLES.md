# CDSAgent Architecture Principles

**Version**: 1.0
**Date**: 2025-11-03
**Status**: Living Document
**Last Updated**: 2025-11-03 (Post-Thread 06 Overfitting Fix)

---

## Table of Contents

1. [Introduction](#introduction)
2. [Core Principles](#core-principles)
3. [Anti-Patterns](#anti-patterns)
4. [Design Decisions](#design-decisions)
5. [Validation Strategy](#validation-strategy)
6. [Regression Prevention](#regression-prevention)
7. [References](#references)

---

## Introduction

This document codifies the fundamental architectural principles for CDSAgent development, with emphasis on **generalizability** and **algorithmic parity** with the LocAgent paper methodology.

### Purpose

- Prevent overfitting to specific repositories or use cases
- Maintain alignment with LocAgent's research methodology
- Ensure production-ready code that works across diverse codebases
- Establish clear guidelines for future development

### Scope

This document applies to:

- All index components (graph builder, sparse index, BM25 search)
- Retrieval algorithms and ranking functions
- Cross-repository validation and testing
- Production deployment configurations

---

## Core Principles

### 1. Generality Over Single-Repository Metrics

**Principle**: Prioritize cross-repository performance over optimizing for any single codebase.

**Rationale**:

- LocAgent paper evaluated across 6+ diverse repositories (Section 5.1)
- Production systems must handle unknown codebases
- Overfitting to training data undermines research validity

**In Practice**:

```rust
// ❌ BAD: Repository-specific tuning
const DJANGO_BOOST: f32 = 2.5;
const SKLEARN_BOOST: f32 = 1.8;

// ✅ GOOD: Generic algorithm
let score = bm25_rank(query, document, k1=1.5, b=0.75);
```

**Metrics**:

- Target: ≥75% average overlap@10 across ≥3 diverse repos
- NOT: 90%+ on LocAgent repo alone

### 2. Zero Hardcoded Repository-Specific Rules

**Principle**: No custom phrases, synonym tables, or file-type-specific boosts.

**Rationale**:

- LocAgent uses **standard BM25Retriever** with **English stemmer only** (no custom rules)
- Hardcoded rules = technical debt + maintenance burden
- Innovation comes from graph-based navigation, NOT BM25 tuning

**Reference Implementation** (LocAgent Python):

```python
# tmp/LocAgent/plugins/location_tools/retriever/bm25_retriever.py
retriever = BM25Retriever.from_defaults(
    nodes=prepared_nodes,
    similarity_top_k=similarity_top_k,
    stemmer=Stemmer.Stemmer("english"),  # ONLY THIS
    language="english",
)
# NO custom phrases, NO synonyms, NO repository-specific tuning
```

**Violations Detected** (Thread 05, removed in Thread 06):

- 36 file-specific phrase lists (`CUSTOM_FILE_PHRASES`)
- 11 domain-specific synonym mappings (`SYNONYM_TABLE`)
- 7 multi-word phrase rules (`PHRASE_TABLE`)

**Enforcement**:

```bash
# Regression test (add to CI)
! rg "const CUSTOM_FILE_PHRASES|const SYNONYM_TABLE|const PHRASE_TABLE" crates/cds-index/src/
```

### 3. Algorithmic Parity vs Output Parity

**Principle**: Match LocAgent's **algorithm design**, not exact outputs.

**Rationale**:

- Implementation differences (Rust vs Python, Tantivy vs llama-index) cause output variance
- Target: Same algorithmic approach (BM25 + graph traversal)
- Accept: 75-85% overlap (algorithmic parity), NOT 90%+ (output parity)

**Distinction**:

| Aspect | Algorithmic Parity | Output Parity |
|--------|-------------------|---------------|
| **Goal** | Same algorithm design | Exact result match |
| **Acceptable Variance** | 15-25% | <10% |
| **Example** | BM25 k1=1.5, b=0.75 (both implementations) | Top-10 results identical |
| **CDSAgent Target** | ✅ YES | ❌ NO |

**Evidence**:

- LocAgent paper: "BM25 retrieval" (Section 4.2)
- No mention of custom tuning or repository-specific rules
- Focus: Graph-based multi-hop reasoning (novelty)

### 4. Standard Industry Parameters

**Principle**: Use well-established defaults from academic literature.

**BM25 Configuration**:

```rust
// ✅ Okapi BM25 standard parameters (Robertson et al., 1995)
pub const K1: f32 = 1.5;  // Term frequency saturation
pub const B: f32 = 0.75;   // Length normalization
```

**Tokenization**:

- English stop words (bm25s library standard, 180 words)
- Porter stemmer (Tantivy default)
- Unicode normalization (NFD)

**Field Boosting**:

```rust
// ✅ Generic semantic importance (NOT file-specific)
name_field: 3.0,   // Entity names most important
path_field: 2.5,   // File paths next
content_field: 1.0 // Source code baseline
```

### 5. Multi-Repository Validation

**Principle**: Test across ≥3 diverse codebases before claiming success.

**Required Test Repositories**:

1. LocAgent (small, graph-focused, 658 nodes)
2. Django (large framework, 6,876 nodes)
3. scikit-learn (ML library, 5,000+ nodes)
4. pytest (testing framework, 3,000+ nodes)
5. matplotlib (visualization, 4,500+ nodes)
6. requests (HTTP library, 1,200+ nodes)

**Validation Metrics**:

```yaml
acceptance_criteria:
  - metric: "Graph parity variance"
    target: "≤2%"
    repos: "All 6"
  - metric: "Search overlap@10 (algorithmic parity)"
    target: "≥75% average"
    repos: "≥3 diverse repos"
  - metric: "Index build time"
    target: "<5s for 1K files"
    repos: "All 6"
```

**Smoke Test Infrastructure**:

```bash
# Multi-repo validation (smoke_multi_repo.rs)
SMOKE_REPO_PATHS=/path/to/django,/path/to/sklearn \
  cargo test -p cds-index smoke_sparse_index_builds_for_external_repos -- --ignored
```

### 6. Regression Testing for Overfitting

**Principle**: Continuous monitoring to detect hardcoded rules creeping back in.

**CI Checks** (add to `.github/workflows/ci.yml`):

```yaml
- name: Check for hardcoded repository rules
  run: |
    if rg "const CUSTOM_FILE_PHRASES|const SYNONYM_TABLE|const PHRASE_TABLE" crates/cds-index/src/; then
      echo "❌ FAILED: Hardcoded repository-specific rules detected"
      echo "See .dev/workflows/ARCHITECTURE_PRINCIPLES.md #2"
      exit 1
    fi
    echo "✅ PASSED: No hardcoded rules found"

- name: Validate multi-repo parity
  run: |
    cargo test -p cds-index search_parity_tests -- --nocapture
    # Should show ≥75% average overlap across repos
```

**Code Review Checklist**:

- [ ] No new `const` tables with file extensions or repo-specific terms
- [ ] No `if repo_name == "django"` conditional logic
- [ ] No file-type-specific boost factors
- [ ] Changes tested on ≥2 repositories

---

## Anti-Patterns

### ❌ Anti-Pattern 1: Single-Repo Optimization

**Bad Example** (removed in Thread 06):

```rust
const CUSTOM_FILE_PHRASES: &[(&str, &[&str])] = &[
    ("py", &["class", "def", "import", "function"]),
    ("rs", &["fn", "struct", "impl", "trait"]),
    // ... 36 entries
];
```

**Why Bad**:

- Hardcodes assumptions about file content
- Breaks on new languages or coding styles
- Overfits to training data

**Good Alternative**:

```rust
// Generic chunking (language-agnostic)
pub fn chunk_source(content: &str) -> Vec<String> {
    content.lines()
        .collect::<Vec<_>>()
        .chunks(80)  // 80-line chunks
        .map(|chunk| chunk.join("\n"))
        .collect()
}
```

### ❌ Anti-Pattern 2: Synonym Injection

**Bad Example** (removed in Thread 06):

```rust
const SYNONYM_TABLE: &[(&str, &[&str])] = &[
    ("func", &["function", "method", "def"]),
    ("class", &["type", "interface", "struct"]),
    // ... 11 entries
];
```

**Why Bad**:

- Language-specific assumptions (Python/JS bias)
- Breaks cross-language parity
- Not in LocAgent reference implementation

**Good Alternative**:

```rust
// Standard tokenization (no synonyms)
pub fn tokenize(text: &str) -> Vec<Token> {
    text.unicode_words()  // Unicode-aware splitting
        .map(|word| stem(normalize(word.to_lowercase())))
        .collect()
}
```

### ❌ Anti-Pattern 3: File-Type-Specific Boosting

**Bad Example**:

```rust
let boost = match file_ext {
    "models.py" => 3.0,
    "views.py" => 2.5,
    "tests.py" => 1.0,
    _ => 1.0,
};
```

**Why Bad**:

- Django-specific (assumes MVC pattern)
- Breaks on non-web frameworks
- Not generalizable

**Good Alternative**:

```rust
// Generic semantic field boosting
let boost = match field {
    "name" => 3.0,   // Entity names (generic)
    "path" => 2.5,   // File paths (generic)
    "content" => 1.0, // Source code (baseline)
};
```

---

## Design Decisions

### 1. BM25 vs TF-IDF

**Decision**: Use BM25 (Okapi BM25, k1=1.5, b=0.75)

**Rationale**:

- LocAgent uses BM25Retriever (Section 4.2, Figure 3)
- BM25 handles term frequency saturation better than TF-IDF
- Industry standard for code search (GitHub Search, Sourcegraph)

**Trade-offs**:

- More complex than TF-IDF (2 hyperparameters)
- Requires document length normalization (b parameter)

### 2. Hierarchical Search (NameIndex + BM25)

**Decision**: Two-tier architecture (exact/prefix → BM25 fallback)

**Rationale**:

- Fast path for exact matches (68 ns latency)
- BM25 only for fuzzy/semantic queries
- Matches LocAgent's entity searcher design

**Implementation**:

```rust
pub struct SparseIndex {
    name_index: NameIndex,   // Upper tier (HashMap)
    bm25_index: BM25Index,   // Lower tier (Tantivy)
}

impl SparseIndex {
    pub fn search(&self, query: &str, limit: usize) -> Vec<EntityId> {
        // Try exact/prefix match first
        if let Some(results) = self.name_index.exact_match(query) {
            return results;
        }
        // Fall back to BM25 semantic search
        self.bm25_index.search(query, limit)
    }
}
```

### 3. Generic Chunking (80-line blocks)

**Decision**: Fixed-size chunking (80 lines, 20-line overlap)

**Rationale**:

- Language-agnostic (no AST parsing required)
- Preserves local context (overlap prevents boundary loss)
- LocAgent uses similar approach (Section 4.3)

**Parameters**:

```rust
const CHUNK_SIZE: usize = 80;   // ~1 screen of code
const OVERLAP: usize = 20;      // 25% overlap
```

**Rejected Alternative**: AST-based function extraction

- Reason: Language-specific, complex, overfits to well-structured code

---

## Validation Strategy

### Phase 1: Single-Repo Parity (Baseline)

**Objective**: Establish LocAgent repo as reference implementation

**Metrics**:

- Graph parity: ≤2% variance (nodes, edges, FQNs)
- Search overlap@10: Baseline measurement (NOT target)

**Status**: ✅ COMPLETE (T-02-01, Thread 06)

### Phase 2: Multi-Repo Validation

**Objective**: Prove generalizability across diverse codebases

**Test Repositories**:

```yaml
repos:
  - name: "LocAgent"
    size: small
    domain: "graph-focused"
    nodes: 658
  - name: "Django"
    size: large
    domain: "web framework"
    nodes: 6876
  - name: "scikit-learn"
    size: medium
    domain: "ML library"
    nodes: 5000
```

**Target Metrics**:

- Average overlap@10: ≥75% (algorithmic parity)
- Index build: <5s per 1K files
- Search latency: <500ms p95

**Status**: ⏳ PENDING (requires external repo clones)

### Phase 3: Production Readiness

**Objective**: Stress testing and edge case handling

**Scenarios**:

- Empty repositories
- Binary files (images, PDFs)
- Non-Python codebases (Rust, TypeScript, Go)
- Extremely large repos (>10K files)

**Status**: ⏳ PENDING

---

## Regression Prevention

### CI Pipeline Checks

Add to `.github/workflows/ci.yml`:

```yaml
name: Architecture Compliance

on: [push, pull_request]

jobs:
  check-hardcoded-rules:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Check for hardcoded repository rules
        run: |
          # Fail if hardcoded rule constants found
          if rg "const CUSTOM_FILE_PHRASES|const SYNONYM_TABLE|const PHRASE_TABLE" crates/cds-index/src/; then
            echo "❌ ARCHITECTURE VIOLATION: Hardcoded repository-specific rules detected"
            echo "See .dev/workflows/ARCHITECTURE_PRINCIPLES.md Section 2"
            exit 1
          fi

  multi-repo-parity:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run multi-repo smoke tests
        env:
          SMOKE_REPO_PATHS: /path/to/test/repos
        run: |
          cargo test -p cds-index smoke_sparse_index_builds_for_external_repos -- --ignored
```

### Code Review Checklist

For PRs touching `crates/cds-index/src/index/`:

- [ ] No new `const` tables with file extensions or language keywords
- [ ] No conditional logic based on repository name or file paths
- [ ] No file-type-specific boost factors beyond generic fields (name, path, content)
- [ ] Changes tested on ≥2 diverse repositories (not just LocAgent)
- [ ] Search overlap@10 measured across all test repos (not degraded)
- [ ] Documentation updated if algorithmic parameters changed

### Pre-Commit Hooks

Add to `.git/hooks/pre-commit`:

```bash
#!/bin/bash
# Check for architecture violations before commit

if git diff --cached --name-only | grep -q "crates/cds-index/src/index/"; then
    echo "🔍 Checking for hardcoded repository rules..."
    if git diff --cached | rg "const CUSTOM_FILE_PHRASES|const SYNONYM_TABLE|const PHRASE_TABLE"; then
        echo "❌ COMMIT BLOCKED: Hardcoded repository-specific rules detected"
        echo "See .dev/workflows/ARCHITECTURE_PRINCIPLES.md"
        exit 1
    fi
fi
```

---

## References

### Academic Papers

1. **LocAgent** (arXiv:2503.09089v2)
   - Section 4.2: "BM25 retrieval for entity search"
   - Section 5.1: "Evaluation across 6+ diverse repositories"
   - Key finding: Standard BM25 + graph navigation (NO custom rules)

2. **Okapi BM25** (Robertson et al., 1995)
   - "Simple BM25 extension to multiple weighted fields"
   - Parameters: k1=1.5, b=0.75 (standard)

### Implementation References

1. **LocAgent Python** (`tmp/LocAgent/`)
   - `plugins/location_tools/retriever/bm25_retriever.py` (lines 15-22)
   - `dependency_graph/entity_searcher.py` (entity search design)

2. **llama-index BM25Retriever**
   - Standard configuration (English stemmer only)
   - No custom phrase/synonym injection

3. **Tantivy** (Rust full-text search)
   - BM25 scoring built-in
   - Custom tokenizer support (used for code-specific splitting)

### Internal Documentation

1. **CRITICAL_ISSUE_OVERFITTING.md**
   - Documents Thread 05-06 overfitting discovery and fix
   - Lists all 71+ removed hardcoded rules

2. **WORK-SESSIONS-05-THREADS-01-05-SUMMARY-2025-11-02.txt**
   - Thread 05: BM25 tuning iterations (documents "what not to do")
   - Thread 06: Critical fix and restoration

3. **T-02-02-sparse-index metadata.yaml**
   - Acceptance criteria: "75-85% overlap (algorithmic parity)"
   - Session 05 Thread 06 work tracking

---

## Version History

| Version | Date | Changes | Author |
|---------|------|---------|--------|
| 1.0 | 2025-11-03 | Initial document created post-Thread 06 overfitting fix | Claude & Codex |

---

## Acknowledgments

This document synthesizes lessons learned from:

- **Thread 05**: BM25 overlap tuning (~4.3h, 7 iterations, 71+ rules added)
- **Thread 06**: Critical overfitting fix (~0.6h, all rules removed)
- **LocAgent Paper**: Research methodology and evaluation standards
- **Community feedback**: User's critical review caught overfitting violation

**Key Learning**: "Generality > single-repo metrics" is not just a principle, it's a requirement for production-ready code retrieval systems.

---

**Maintainers**: CDSAgent Development Team
**Review Frequency**: Quarterly or after major architectural changes
**Enforcement**: CI pipeline + code review checklist

---

END OF ARCHITECTURE PRINCIPLES
