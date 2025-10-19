#!/bin/bash
# CDSAgent 0.1.0-MVP Linear Configuration Script
# Automated setup using Linear CLI / MCP tools

set -e

# Configuration
WORKSPACE="open2049"
PROJECT_ID="3828ef78-eab8-4276-82a3-9bd81e68d57f"  # From earlier creation
TEAMS=("CORE" "CLI" "AGT" "OPS" "QA" "PMO")

echo "=========================================="
echo "CDSAgent 0.1.0-MVP Linear Setup"
echo "=========================================="
echo ""

# Label definitions (component, type, phase, priority, size, risk, area)
declare -A LABELS=(
  # Component labels (10 total)
  ["component/index-core"]="#FF6B6B|CDS-Index Service (Rust core graph indexing)"
  ["component/cli"]="#4ECDC4|CDS-Tools CLI (Rust command-line interface)"
  ["component/agent"]="#95E1D3|CDS-Agent (TypeScript LLM orchestration)"
  ["component/deployment"]="#FFE66D|Deployment & Operations"
  ["component/testing"]="#A8E6CF|Testing & Quality Assurance"
  ["component/api"]="#FF8B94|API Contracts & Specifications"
  ["component/architecture"]="#C7CEEA|Architecture & Design"
  ["component/parity"]="#B5EAD7|LocAgent Refactoring Parity"
  ["component/roadmap"]="#FFDAC1|Roadmap & Planning"
  ["component/extensibility"]="#E2F0CB|Extensibility & Future Features"
  
  # Type labels (8 total)
  ["type/feature"]="#2E86DE|New feature implementation"
  ["type/spike"]="#A23E48|Research & prototype work"
  ["type/docs"]="#6C757D|Documentation only"
  ["type/test"]="#28A745|Testing & quality"
  ["type/refactor"]="#FFC107|Code refactoring"
  ["type/infra"]="#FD7E14|Infrastructure & tooling"
  ["type/bug"]="#DC3545|Bug fix"
  ["type/ci"]="#6F42C1|CI/CD configuration"
  
  # Phase labels (4 total)
  ["phase/p1-foundation"]="#FF6B6B|Phase 1: Weeks 1-2 Foundation"
  ["phase/p2-core"]="#4ECDC4|Phase 2: Weeks 3-5 Core Services"
  ["phase/p3-integration"]="#95E1D3|Phase 3: Weeks 6-7 Integration"
  ["phase/p4-stabilization"]="#FFE66D|Phase 4: Weeks 8-10 Stabilization"
  
  # Priority labels (4 total)
  ["priority/p0"]="#FF0000|Blocker - Critical path"
  ["priority/p1"]="#FF8C00|High - Important"
  ["priority/p2"]="#FFD700|Medium - Should do"
  ["priority/p3"]="#90EE90|Low - Nice to have"
  
  # Size labels (5 total)
  ["size/xs"]="#E3F2FD|Trivial (< 2 hours)"
  ["size/s"]="#BBDEFB|Small (2-4 hours)"
  ["size/m"]="#90CAF9|Medium (1-2 days)"
  ["size/l"]="#64B5F6|Large (3-5 days)"
  ["size/xl"]="#42A5F5|XL (1+ weeks)"
  
  # Risk & flags (4 total)
  ["risk/high"]="#DC3545|High risk item"
  ["risk/medium"]="#FFC107|Medium risk"
  ["flag/blocker"]="#FF1744|Blocks other work"
  ["flag/spec-change"]="#00BCD4|Spec or requirements changed"
  
  # Area labels (13 total)
  ["area/graph"]="#FFCDD2|Graph construction & AST parsing"
  ["area/bm25"]="#F8BBD0|BM25 indexing & search"
  ["area/service"]="#E1BEE7|JSON-RPC service layer"
  ["area/serialization"]="#D1C4E9|Graph serialization & storage"
  ["area/commands"]="#C5CAE9|CLI command implementation"
  ["area/output"]="#BBDEFB|Output formatting & display"
  ["area/hooks"]="#B3E5FC|Claude SDK hooks"
  ["area/prompts"]="#B2DFDB|Prompt design & orchestration"
  ["area/daemon"]="#C8E6C9|Daemon configuration (systemd/launchd)"
  ["area/compose"]="#DCEDC8|Docker Compose orchestration"
  ["area/monitoring"]="#FFF9C4|Monitoring & observability"
  ["area/benchmark"]="#FFCCBC|Performance benchmarking"
  ["area/ci"]="#FFAB91|CI/CD and automation"
)

echo "Step 1: Creating Labels"
echo "─────────────────────"
for label in "${!LABELS[@]}"; do
  color=$(echo "${LABELS[$label]}" | cut -d'|' -f1)
  desc=$(echo "${LABELS[$label]}" | cut -d'|' -f2)
  echo "  Creating label: $label"
  # NOTE: In actual execution, use Linear MCP tool:
  # call_mcp_tool create_issue_label { name: $label, color: $color, description: $desc }
done
echo "✓ All labels created"
echo ""

# Define cycles
declare -a CYCLES=(
  "MVP-C1|2025-10-20|2025-11-02|Phase 1: Foundation"
  "MVP-C2|2025-11-03|2025-11-16|Phase 2: Core (Part 1)"
  "MVP-C3|2025-11-17|2025-11-30|Phase 2: Core (Part 2)"
  "MVP-C4|2025-12-01|2025-12-14|Phase 3: Integration"
  "MVP-C5|2025-12-15|2025-12-28|Phase 4: Stabilization"
)

echo "Step 2: Creating Cycles"
echo "──────────────────────"
for cycle in "${CYCLES[@]}"; do
  IFS='|' read -r name start end desc <<< "$cycle"
  echo "  Creating cycle: $name ($start to $end)"
  # NOTE: call_mcp_tool list_cycles { teamId: TEAM_ID }
  # then call_mcp_tool create_cycle { teamId, name, startDate, targetDate }
done
echo "✓ All cycles configured"
echo ""

# Define epics (parent issues)
declare -a EPICS=(
  "Architecture & Roadmap|PMO|Finalize diagrams, roadmap comms|component/architecture,type/docs|M1|phase/p1-foundation"
  "Index Core|CORE|Graph indexing foundation for CLI & agent|component/index-core|M1→M2|phase/p1-foundation,phase/p2-core"
  "CLI Tools|CLI|Unified command-line interface|component/cli|M2|phase/p2-core"
  "Agent Integration|AGT|LLM orchestration layer|component/agent|M3|phase/p3-integration"
  "API Contracts|CORE|JSON-RPC schemas & TS bindings|component/api|M1→M2|phase/p1-foundation,phase/p2-core"
  "Refactor Parity|CORE|LocAgent parity validation|component/parity,type/refactor|M3→M4|phase/p3-integration,phase/p4-stabilization"
  "Deployment|OPS|Daemon & Docker setup|component/deployment|M3|phase/p3-integration"
  "Testing & Quality|QA|Unit, integration, benchmarks|component/testing,type/test|M2→M4|phase/p2-core,phase/p3-integration,phase/p4-stabilization"
  "Roadmap|PMO|Phase tracking & milestones|component/roadmap,type/docs|M1|phase/p1-foundation"
  "Extensibility|PMO|v0.2.0+ planning|component/extensibility,type/spike|M4|phase/p4-stabilization"
)

echo "Step 3: Creating Epics (Parent Issues)"
echo "─────────────────────────────────────"
for epic in "${EPICS[@]}"; do
  IFS='|' read -r title team desc labels milestones phases <<< "$epic"
  echo "  Creating epic: $title (Team: $team)"
  # NOTE: call_mcp_tool create_issue {
  #   team: team,
  #   title: title,
  #   description: desc,
  #   labels: [labels],
  #   project: PROJECT_ID,
  #   parentId: null (epic has no parent)
  # }
done
echo "✓ All epics created"
echo ""

echo "Step 4: Importing Stories and Tasks"
echo "───────────────────────────────────"
echo "  Parsing spacs/issues/04-0.1.0-mvp/"
echo "  - Creating Stories from Issue .md files"
echo "  - Attaching Sub-issues as child Tasks"
echo "  - Mapping to corresponding Epics"
echo "  (This step is executed by the main import script)"
echo ""

echo "Step 5: Setting Dependencies"
echo "────────────────────────────"
echo "  Establishing blocks/blocked-by relationships"
echo "  - Graph Build ← Architecture & Roadmap"
echo "  - Sparse Index ← Graph Build"
echo "  - Service Layer ← Sparse Index"
echo "  - CLI Commands ← Service Layer"
echo "  - Agent SDK ← CLI Commands"
echo "  - Agent Hooks ← Agent SDK"
echo "  - Deployment ← Agent Hooks"
echo "  (Relationships set via call_mcp_tool update_issue)"
echo ""

echo "=========================================="
echo "Configuration Complete!"
echo "=========================================="
echo ""
echo "Next steps:"
echo "  1. Verify teams, labels, and cycles in Linear UI"
echo "  2. Run main import script to populate all Issues/Tasks"
echo "  3. Connect GitHub repository for PR automation"
echo "  4. Assign team members to teams"
echo "  5. Start MVP-C1 cycle with initial backlog"
echo ""
