# Golden Fixture Playbook for External Smoke Repos

This note documents how we will produce LocAgent-derived golden search queries for
non-LocAgent smoke repositories (e.g., `django`, `matplotlib`, `pytest`). The goal is to
compare CDSAgent’s sparse index against the original LocAgent demo so we can set
realistic acceptance targets per repo, instead of extrapolating from the LocAgent repo
alone.

## Overview

1. **Build the reference indices** using the Python LocAgent repo (`tmp/LocAgent`).
2. **Run `auto_search_main.py`** on the target repository (or SWE-bench fixture) to
   capture the LocAgent top-10 results for each query.
3. **Export the query/results pairs** into a JSONL file that mirrors
   `tests/fixtures/parity/golden_outputs/search_queries.jsonl`.
4. **Drop the JSONL file** into
   `tests/fixtures/parity/golden_outputs/{repo}.search_queries.jsonl`.
5. **Re-run** `SMOKE_OVERLAP_DIAG=1 SMOKE_REPO_PATHS=".../repo" cargo test -p cds-index    smoke_sparse_index_overlap_report -- --ignored --nocapture` to compare CDSAgent vs.
   LocAgent for the new repo.

## Detailed Steps

### 1. Prep the LocAgent environment

```bash
cd <workspace>/tmp/LocAgent
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
```

Set the dataset variables so the LocAgent scripts can locate the repository-under-test.
For example, to analyze the local `tmp/smoke/django` checkout:

```bash
export GRAPH_INDEX_DIR="$(pwd)/indices/django/graph"
export BM25_INDEX_DIR="$(pwd)/indices/django/bm25"
```

### 2. Build the reference indices

```bash
python dependency_graph/build_graph.py --repo-root /path/to/tmp/smoke/django
python build_bm25_index.py --repo-root /path/to/tmp/smoke/django
```

These commands mirror CDSAgent’s graph + sparse index pipeline, ensuring parity.

### 3. Run LocAgent to harvest top-10 results

Create a query file (one query per line) that reflects the smoke scenarios you care
about (reuse `tests/fixtures/parity/golden_outputs/search_queries.jsonl` as a template).
Then run LocAgent:

```bash
python auto_search_main.py   --repo-root /path/to/tmp/smoke/django   --queries /path/to/django_queries.jsonl   --output /tmp/django_locagent_results.jsonl   --model open-source-placeholder
```

The output JSONL must include `repo`, `query`, `top_10`, and `total_results` fields. The
`top_10` entries should contain absolute file paths so that `smoke_sparse_index_overlap`
can relativize them back to the repo root.

### 4. Place the golden file where the Rust smoke test expects it

Move (or copy) the JSONL file to:

```
tests/fixtures/parity/golden_outputs/django.search_queries.jsonl
```

Repeat the process for `matplotlib`, `pytest`, `scikit-learn`, etc. Use the
filename convention `{repo}.search_queries.jsonl` (lowercase, spaces replaced with `_`).

### 5. Compare CDSAgent vs. LocAgent

Run the smoke overlap harness pointing at the local repo:

```bash
SMOKE_OVERLAP_DIAG=1 SMOKE_REPO_PATHS="$(pwd)/tmp/smoke/django" cargo test -p cds-index smoke_sparse_index_overlap_report -- --ignored --nocapture
```

When the golden file exists, the harness now reports repo-level overlap percentages and
contributes them to the global average. Use these numbers to tune acceptance thresholds
per repo.

## Next Steps

- Automate step (3) with a helper script that shells out to LocAgent and writes the
  JSONL directly into `tests/fixtures/parity/golden_outputs/`.
- Store the generated JSONL files under version control once vetted, so CI can enforce
  cross-repo overlap without needing to re-run LocAgent.
