# Issue-03: CDSAgent Technical Architecture & 0.1.0-MVP-PRDs-v0 Alignment Analysis

## Objective

Ensure the newly drafted CDSAgent MVP technical PRDs (version 0.1.0, round 1) stay faithful to the original requirements and LocAgent sources, and identify any gaps, contradictions, or follow-up work needed before Round 2 PRD expansion.

## Primary References

1. Original requirement description: `spacs/issues/01-CDSAgent-MVP-definition.md`.
2. CDSAgent requirement analysis artifact (2025-10-18): `spacs/research/2025-10-18-cdsagent-requirement-analysis.md`.
3. LocAgent paper (local copy): `tmp/LocAgent/arXiv-2503.09089v2/` — [LocAgent: Graph-Guided LLM Agents for Code Localization](https://arxiv.org/html/2503.09089v2).
4. LocAgent reference implementation (Python): `tmp/LocAgent/`.
5. CDSAgent MVP PRDs v0 set: `spacs/prd/0.1.0-MVP-PRDs-v0/` (files 01–10).

## Alignment Workstreams (ULTRATHINK Plan)

- [x] **WS1 – Requirement Traceability:** Map each PRD section to explicit statements in Issue-01 and the research analysis; flag any PRD claims lacking a requirement source. (See `spacs/research/2025-10-18-cdsagent-prds-alignment-notes.md`).
- [x] **WS2 – LocAgent Parity Check:** For every PRD component, confirm the cited LocAgent mechanisms (graph build, sparse index, tool semantics) accurately reflect the paper and Python repo behavior; note discrepancies or missing implementation details. (Documented in alignment notes, WS1&WS2 section).
- [x] **WS3 – PRD Consistency Review:** Cross-compare the ten PRDs to ensure shared assumptions (e.g., transport protocol, CLI outputs, SDK hooks) do not conflict; highlight areas needing harmonized definitions or shared appendices. (Completed; see “Cross-PRD Consistency Review” in alignment notes).
- [x] **WS4 – Gap & Risk Log:** Produce a consolidated list of open questions, risks, and follow-up tasks required before promoting PRDs to v0.2 (e.g., undecided Rust↔TypeScript transport, benchmarking strategy). (Resolved items GR-1…GR-5 recorded in alignment notes and tracked in Issue-02 TODOs.)
- [x] **WS5 – Deliverable Packaging:** Prepare alignment summary notes under `spacs/research/` (new dated file) and update Issues-01/02 with cross-links once analysis is complete. (Notes saved, Issue-01/02 references updated.)

## Suggested Workflow

1. Read Issue-02 PRD intents alongside the v0 documents to capture author assumptions.
2. Iterate through PRDs layer-by-layer (01→10), performing WS1–WS3 checks; document findings per file.
3. Distill findings into the gap & risk log (WS4) and capture recommended adjustments for Round 2 drafts.
4. Commit alignment notes and mark completion status in this issue and upstream issues.

## Success Criteria

- Traceability matrix or notes linking PRD statements to requirements and LocAgent evidence.
- Annotated list of misalignments or missing coverage items with owner proposals.
- Updated issue checkboxes reflecting completed workstreams and references to produced artifacts.

## Notes

- Maintain bilingual context where relevant (original requirement text includes Mandarin excerpts).
- Prioritize accuracy over speed (“ULTRATHINK”); verify citations directly in the paper/repo before concluding alignment.
- Coordinate with PRD author on any structural changes before editing v0 documents.
