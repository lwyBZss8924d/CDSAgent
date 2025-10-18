# CDSAgent 0.1.0-MVP Issue Tracking

This directory contains all issues for the CDSAgent 0.1.0 MVP release, organized by component and dependency order.

## Quick Navigation

### Priority P0 (Critical Path - Start First)

- [] [09 - Implementation Roadmap](09-roadmap.md) - Planning reference
- [] [06 - Refactor Parity](06-refactor-parity.md) - Cross-cutting validation
- [] [02 - CDS-Index Core](02-index-core/) - Graph indexing foundation
- [] [05 - API Contracts](05-api-contracts.md) - Interface definitions

### Priority P1 (Core Implementation)

- [] [03 - CDS-Tools CLI](03-cli-tools/) - Command-line interface
- [] [04 - CDS-Agent Integration](04-agent-integration/) - Claude Agent SDK
- [] [07 - Deployment & Operations](07-deployment/) - Daemon & containerization
- [] [08 - Testing & Quality](08-testing/) - Validation & benchmarks

### Priority P2 (Integration & Polish)

- [] [01 - Architecture & Roadmap](01-architecture-and-roadmap.md) - Documentation synthesis
- [] [10 - Extensibility & Future](10-extensibility.md) - v0.2.0+ planning

## Issue Structure

### Single-File Issues

- **01-architecture-and-roadmap.md**: System-level documentation synthesis
- **05-api-contracts.md**: JSON-RPC schemas and TypeScript types
- **06-refactor-parity.md**: LocAgent parity validation plan
- **09-roadmap.md**: Phase tracking and milestone management
- **10-extensibility.md**: Future work backlog (v0.2.0+)

### Directory-Based Issues (with sub-issues)

- **02-index-core/**: Graph construction, sparse indexing, JSON-RPC service
- **03-cli-tools/**: CLI commands, output formatting, integration tests
- **04-agent-integration/**: Claude SDK setup, prompts, hooks
- **07-deployment/**: Daemon configuration, Docker, deployment docs
- **08-testing/**: Unit tests, integration tests, performance benchmarks

## Dependency Flow

```text
09-roadmap.md (Planning) ──────────────────────┐
                                               ↓
06-refactor-parity.md ─────────────────────────┤
                                               ↓
05-api-contracts.md ───────────────────────────┤
                                               ↓
02-index-core/ (Foundation) ───────────────────┤
    ├─ 01-graph-build.md                       │
    ├─ 02-sparse-index.md                      │
    ├─ 03-service-layer.md ────────────────────┤
    └─ 04-serialization-fixtures.md            │
                                               ↓
03-cli-tools/ (Depends on 02) ─────────────────┤
    ├─ 01-command-impl.md                      │
    ├─ 02-output-format.md                     │
    ├─ 03-integration-tests.md                 │
    └─ 04-docs.md                              │
                                               ↓
04-agent-integration/ (Depends on 02+03) ──────┤
    ├─ 01-sdk-bootstrap.md                     │
    ├─ 02-prompt-design.md                     │
    ├─ 03-hooks.md                             │
    └─ 04-sample-transcripts.md                │
                                               ↓
07-deployment/ (Depends on 02+03) ─────────────┤
08-testing/ (Parallel with all) ───────────────┤
                                               ↓
01-architecture-and-roadmap.md (Synthesis) ────┘
```

## Key Principles

- **Traceability**: Every issue links back to specific PRD sections
- **Acceptance Criteria**: Copied directly from PRDs for consistency
- **Dependency-Aware**: Issues ordered by critical path dependencies
- **Phased Approach**: Aligned with PRD-09's 4-phase implementation plan

## Related Documents

- **PRDs**: `spacs/prd/0.1.0-MVP-PRDs-v0/` (01-10)
- **Planning Backlog**: `spacs/plan/0.1.0-mvp-backlog.md`
- **Tasks**: `spacs/tasks/0.1.0-mvp/` (implementation tracking)
- **Original Requirements**: `spacs/issues/01-CDSAgent-MVP-definition.md`
- **Architecture Plan**: `spacs/issues/02-CDSAgent-Tech-Architecture-Plan.md`

## Status Legend

- ☐ Not started
- 🏗 In progress
- ✅ Completed
- ⏸ Blocked

Last Updated: 2025-10-19
