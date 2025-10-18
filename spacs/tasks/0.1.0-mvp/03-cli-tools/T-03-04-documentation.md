# Task T-03-04: CLI Documentation & Guides

**Issue**: [Sub-Issue 03.04 – CLI Docs](../../issues/04-0.1.0-mvp/03-cli-tools/04-docs.md)

**PRD References**: [PRD-03 §6](../../prd/0.1.0-MVP-PRDs-v0/03-cds-tools-cli.md)

**Owners**: Technical Writer (primary), Rust Dev 2 (review)

**Status**: ☐ Not Started | **Week**: 5

---

## Objective

Produce user-facing documentation that enables developers and agents to leverage `cds` commands effectively, including examples, troubleshooting, and integration tips.

## Deliverables

- `docs/CLI.md` (comprehensive guide)
- `cds-tools/README.md` (quickstart + reference)
- Man page stub `docs/man/cds.1.md`
- Troubleshooting section covering common errors

## Implementation Steps

1. Gather command flag descriptions and sample outputs (JSON/text/tree).
2. Document environment variables (`INDEX_SERVICE_URL`, `GRAPH_INDEX_DIR`, etc.).
3. Provide step-by-step tutorial aligning with SWE-bench issue workflow.
4. Review with QA and agent team to ensure clarity and accuracy.

## Acceptance Criteria

- [ ] Guides reviewed by CLI maintainer and QA lead.
- [ ] Examples verified against latest CLI behavior.
- [ ] Troubleshooting section includes at least five frequent failure modes.
- [ ] Documentation linked from root `README.md` and `docs/README.md`.

## Dependencies

- **Prerequisite**: [T-03-01](T-03-01-core-commands.md), [T-03-02](T-03-02-output-format.md) (need finalized behavior).
- **Blocks**: none, but feeds into onboarding tasks and release notes.

## Notes

- Plan to translate key sections into automation prompts for agent tool-use instructions.
