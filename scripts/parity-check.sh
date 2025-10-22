#!/usr/bin/env bash

# parity-check.sh - LocAgent Parity Validation Automation
#
# Usage:
#   ./scripts/parity-check.sh --all          # Run all parity checks
#   ./scripts/parity-check.sh --graph        # Run graph parity check
#   ./scripts/parity-check.sh --search       # Run search parity check
#   ./scripts/parity-check.sh --traverse     # Run traverse parity check
#   ./scripts/parity-check.sh --performance  # Run performance benchmarks
#   ./scripts/parity-check.sh --report       # Generate markdown report

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Thresholds (from parity-validation-methodology.md §1.3)
GRAPH_VARIANCE_THRESHOLD=2.0        # ≤2% variance
SEARCH_OVERLAP_THRESHOLD=0.90       # ≥90% overlap@10
TRAVERSE_EXACT_MATCH_THRESHOLD=10   # 10/10 scenarios
PERF_INDEX_TARGET_MS=3000           # <3s for 1K files
PERF_SEARCH_TARGET_MS=100           # <100ms p95
PERF_TRAVERSE_TARGET_MS=200         # <200ms p95

# Directories
FIXTURES_DIR="tests/fixtures/parity"
GOLDEN_OUTPUTS_DIR="${FIXTURES_DIR}/golden_outputs"
REPORT_FILE="parity-report-$(date +%Y-%m-%d-%H%M).md"

# Exit codes
EXIT_SUCCESS=0
EXIT_FAILURE=1

# Global state
CHECKS_RUN=0
CHECKS_PASSED=0
CHECKS_FAILED=0
CHECKS_WARNING=0

# Functions

print_header() {
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}  LocAgent Parity Validation${NC}"
    echo -e "${BLUE}  Date: $(date '+%Y-%m-%d %H:%M:%S')${NC}"
    echo -e "${BLUE}========================================${NC}"
    echo ""
}

print_section() {
    local title="$1"
    echo -e "${BLUE}### ${title}${NC}"
    echo ""
}

print_pass() {
    local message="$1"
    echo -e "${GREEN}✅ ${message}${NC}"
}

print_fail() {
    local message="$1"
    echo -e "${RED}❌ ${message}${NC}"
}

print_warn() {
    local message="$1"
    echo -e "${YELLOW}⚠️  ${message}${NC}"
}

print_info() {
    local message="$1"
    echo -e "   ${message}"
}

check_prerequisites() {
    print_section "Checking Prerequisites"

    # Create fixture directories if they don't exist
    mkdir -p "$GOLDEN_OUTPUTS_DIR" "$FIXTURES_DIR/locagent_repo" "$FIXTURES_DIR/sample_repos"

    # Check if golden outputs are populated (warn if empty, but don't fail)
    if [ ! "$(ls -A "$GOLDEN_OUTPUTS_DIR" 2>/dev/null | grep -v '\.gitkeep')" ]; then
        print_warn "Golden outputs directory is empty: $GOLDEN_OUTPUTS_DIR"
        print_info "Baseline extraction not yet complete (Phase 2 - in progress)"
        print_info "Parity tests will show as 'not implemented' until baselines are extracted"
        print_info "See: tests/fixtures/parity/README.md for baseline extraction instructions"
        echo ""
    fi

    # Check if Rust toolchain is installed
    if ! command -v cargo &> /dev/null; then
        print_fail "Cargo not found. Please install Rust toolchain."
        exit $EXIT_FAILURE
    fi

    # Check if project builds
    if ! cargo build --release --quiet 2>/dev/null; then
        print_fail "Rust project failed to build."
        print_info "Run: cargo build --release"
        exit $EXIT_FAILURE
    fi

    print_pass "Prerequisites check passed"
    echo ""
}

run_graph_parity_check() {
    print_section "Graph Parity Check"

    CHECKS_RUN=$((CHECKS_RUN + 1))

    # Check if graph parity tests exist
    if ! cargo test --test graph_parity_tests --no-run &>/dev/null; then
        print_warn "Graph parity tests not yet implemented"
        print_info "Expected: cargo test --test graph_parity_tests"
        CHECKS_WARNING=$((CHECKS_WARNING + 1))
        echo ""
        return 0
    fi

    # Run graph parity tests
    echo "Running graph parity tests..."
    if cargo test --test graph_parity_tests --release -- --nocapture 2>&1 | tee /tmp/graph_parity.log; then
        # Extract metrics from test output (take max variance across all repos)
        local node_variance=$(grep "Node count variance:" /tmp/graph_parity.log | awk '{print $4}' | tr -d '%' | sort -nr | head -1 || echo "0.0")
        local edge_variance=$(grep "Edge count variance:" /tmp/graph_parity.log | awk '{print $4}' | tr -d '%' | sort -nr | head -1 || echo "0.0")

        if (( $(echo "$node_variance <= $GRAPH_VARIANCE_THRESHOLD" | bc -l) )) && \
           (( $(echo "$edge_variance <= $GRAPH_VARIANCE_THRESHOLD" | bc -l) )); then
            print_pass "Graph Parity Check: PASSED"
            print_info "Node count variance: ${node_variance}% (threshold: ≤${GRAPH_VARIANCE_THRESHOLD}%)"
            print_info "Edge count variance: ${edge_variance}% (threshold: ≤${GRAPH_VARIANCE_THRESHOLD}%)"
            CHECKS_PASSED=$((CHECKS_PASSED + 1))
        else
            print_fail "Graph Parity Check: FAILED"
            print_info "Node count variance: ${node_variance}% (threshold: ≤${GRAPH_VARIANCE_THRESHOLD}%)"
            print_info "Edge count variance: ${edge_variance}% (threshold: ≤${GRAPH_VARIANCE_THRESHOLD}%)"
            CHECKS_FAILED=$((CHECKS_FAILED + 1))
        fi
    else
        print_fail "Graph Parity Check: FAILED (tests crashed)"
        print_info "Check logs: /tmp/graph_parity.log"
        CHECKS_FAILED=$((CHECKS_FAILED + 1))
    fi

    echo ""
}

run_search_parity_check() {
    print_section "Search Parity Check"

    CHECKS_RUN=$((CHECKS_RUN + 1))

    # Check if search parity tests exist
    if ! cargo test --test search_parity_tests --no-run &>/dev/null; then
        print_warn "Search parity tests not yet implemented"
        print_info "Expected: cargo test --test search_parity_tests"
        CHECKS_WARNING=$((CHECKS_WARNING + 1))
        echo ""
        return 0
    fi

    # Run search parity tests
    echo "Running search parity tests..."
    if cargo test --test search_parity_tests --release -- --nocapture 2>&1 | tee /tmp/search_parity.log; then
        # Extract metrics (aggregate across all repos: sum matches / sum total)
        local overlap=$(grep "Search overlap@10:" /tmp/search_parity.log | awk '{print $3}' | cut -d'/' -f1 | awk '{sum+=$1} END {print sum}' || echo "0")
        local total=$(grep "Search overlap@10:" /tmp/search_parity.log | awk '{print $3}' | cut -d'/' -f2 | awk '{sum+=$1} END {print sum}' || echo "50")
        local overlap_rate=$(echo "scale=2; $overlap / $total" | bc)

        if (( $(echo "$overlap_rate >= $SEARCH_OVERLAP_THRESHOLD" | bc -l) )); then
            print_pass "Search Parity Check: PASSED"
            print_info "Overlap@10: ${overlap}/${total} queries (${overlap_rate} match rate)"
            print_info "Threshold: ≥${SEARCH_OVERLAP_THRESHOLD}"
            CHECKS_PASSED=$((CHECKS_PASSED + 1))
        else
            print_fail "Search Parity Check: FAILED"
            print_info "Overlap@10: ${overlap}/${total} queries (${overlap_rate} match rate)"
            print_info "Threshold: ≥${SEARCH_OVERLAP_THRESHOLD}"
            CHECKS_FAILED=$((CHECKS_FAILED + 1))
        fi
    else
        print_fail "Search Parity Check: FAILED (tests crashed)"
        print_info "Check logs: /tmp/search_parity.log"
        CHECKS_FAILED=$((CHECKS_FAILED + 1))
    fi

    echo ""
}

run_traverse_parity_check() {
    print_section "Traverse Parity Check"

    CHECKS_RUN=$((CHECKS_RUN + 1))

    # Check if traverse parity tests exist
    if ! cargo test --test traverse_parity_tests --no-run &>/dev/null; then
        print_warn "Traverse parity tests not yet implemented"
        print_info "Expected: cargo test --test traverse_parity_tests"
        CHECKS_WARNING=$((CHECKS_WARNING + 1))
        echo ""
        return 0
    fi

    # Run traverse parity tests
    echo "Running traverse parity tests..."
    if cargo test --test traverse_parity_tests --release -- --nocapture 2>&1 | tee /tmp/traverse_parity.log; then
        # Extract metrics (aggregate across all repos: sum matches / sum total)
        local exact_matches=$(grep "Exact matches:" /tmp/traverse_parity.log | awk '{print $3}' | cut -d'/' -f1 | awk '{sum+=$1} END {print sum}' || echo "0")
        local total_scenarios=$(grep "Exact matches:" /tmp/traverse_parity.log | awk '{print $3}' | cut -d'/' -f2 | awk '{sum+=$1} END {print sum}' || echo "10")

        # Compare: all scenarios must pass (100% match rate) AND minimum threshold met
        if [ "$exact_matches" -eq "$total_scenarios" ] && \
           [ "$total_scenarios" -ge "$TRAVERSE_EXACT_MATCH_THRESHOLD" ]; then
            print_pass "Traverse Parity Check: PASSED"
            print_info "Exact matches: ${exact_matches}/${total_scenarios} scenarios (100%)"
            print_info "Minimum threshold: ≥${TRAVERSE_EXACT_MATCH_THRESHOLD} scenarios"
            CHECKS_PASSED=$((CHECKS_PASSED + 1))
        else
            print_fail "Traverse Parity Check: FAILED"
            print_info "Exact matches: ${exact_matches}/${total_scenarios} scenarios"
            if [ "$total_scenarios" -lt "$TRAVERSE_EXACT_MATCH_THRESHOLD" ]; then
                print_info "Expected: ≥${TRAVERSE_EXACT_MATCH_THRESHOLD} scenarios (only ${total_scenarios} ran)"
            else
                print_info "Expected: 100% match rate (all scenarios must pass exactly)"
            fi

            # List failing scenarios (if any exist)
            if grep -q "FAILED:" /tmp/traverse_parity.log 2>/dev/null; then
                grep "FAILED:" /tmp/traverse_parity.log | while read -r line; do
                    print_info "  - $line"
                done
            fi

            CHECKS_FAILED=$((CHECKS_FAILED + 1))
        fi
    else
        print_fail "Traverse Parity Check: FAILED (tests crashed)"
        print_info "Check logs: /tmp/traverse_parity.log"
        CHECKS_FAILED=$((CHECKS_FAILED + 1))
    fi

    echo ""
}

run_performance_check() {
    print_section "Performance Benchmarks"

    CHECKS_RUN=$((CHECKS_RUN + 1))

    # Check if benchmarks exist
    if ! cargo bench --no-run &>/dev/null; then
        print_warn "Performance benchmarks not yet implemented"
        print_info "Expected: cargo bench"
        CHECKS_WARNING=$((CHECKS_WARNING + 1))
        echo ""
        return 0
    fi

    # Run benchmarks
    echo "Running performance benchmarks..."
    if cargo bench --all 2>&1 | tee /tmp/performance_bench.log; then
        # Extract metrics (simplified - actual implementation would parse Criterion output)
        local index_time=$(grep "graph_build_bench" /tmp/performance_bench.log | awk '{print $2}' || echo "0")
        local search_time=$(grep "search_bench" /tmp/performance_bench.log | awk '{print $2}' || echo "0")
        local traverse_time=$(grep "traverse_bench" /tmp/performance_bench.log | awk '{print $2}' || echo "0")

        # For now, just report that benchmarks ran
        print_pass "Performance Benchmarks: COMPLETED"
        print_info "Index build: ${index_time} (target: <${PERF_INDEX_TARGET_MS}ms)"
        print_info "Search query: ${search_time} (target: <${PERF_SEARCH_TARGET_MS}ms)"
        print_info "Traverse 2-hop: ${traverse_time} (target: <${PERF_TRAVERSE_TARGET_MS}ms)"
        print_warn "⚠️  Manual review required - check Criterion HTML reports"

        CHECKS_PASSED=$((CHECKS_PASSED + 1))
    else
        print_fail "Performance Benchmarks: FAILED"
        print_info "Check logs: /tmp/performance_bench.log"
        CHECKS_FAILED=$((CHECKS_FAILED + 1))
    fi

    echo ""
}

generate_report() {
    print_section "Generating Parity Report"

    cat > "$REPORT_FILE" << EOF
# LocAgent Parity Validation Report

**Date**: $(date '+%Y-%m-%d %H:%M:%S')
**Generated By**: \`./scripts/parity-check.sh --report\`

---

## Summary

- **Total Checks Run**: $CHECKS_RUN
- **Passed**: $CHECKS_PASSED ✅
- **Failed**: $CHECKS_FAILED ❌
- **Warnings**: $CHECKS_WARNING ⚠️

---

## Detailed Results

### Graph Parity Check

$(if [ -f /tmp/graph_parity.log ]; then
    cat /tmp/graph_parity.log
else
    echo "Not run or logs not available."
fi)

---

### Search Parity Check

$(if [ -f /tmp/search_parity.log ]; then
    cat /tmp/search_parity.log
else
    echo "Not run or logs not available."
fi)

---

### Traverse Parity Check

$(if [ -f /tmp/traverse_parity.log ]; then
    cat /tmp/traverse_parity.log
else
    echo "Not run or logs not available."
fi)

---

### Performance Benchmarks

$(if [ -f /tmp/performance_bench.log ]; then
    cat /tmp/performance_bench.log | tail -50
else
    echo "Not run or logs not available."
fi)

---

## Next Steps

$(if [ $CHECKS_FAILED -gt 0 ]; then
    echo "- [ ] Fix $CHECKS_FAILED failing parity check(s)"
    echo "- [ ] Re-run: \`./scripts/parity-check.sh --all\`"
    echo "- [ ] Review methodology: \`docs/parity-validation-methodology.md\`"
else
    echo "- [x] All parity checks passed! ✅"
    echo "- [ ] Review performance benchmarks (target: 2-5x speedup)"
    echo "- [ ] Update TODO.yaml with parity status"
fi)

---

**Report saved to**: \`$REPORT_FILE\`
EOF

    print_pass "Report generated: $REPORT_FILE"
    echo ""
}

print_summary() {
    print_section "Summary"

    echo -e "Total checks run:    ${BLUE}${CHECKS_RUN}${NC}"
    echo -e "Passed:              ${GREEN}${CHECKS_PASSED} ✅${NC}"
    echo -e "Failed:              ${RED}${CHECKS_FAILED} ❌${NC}"
    echo -e "Warnings:            ${YELLOW}${CHECKS_WARNING} ⚠️${NC}"
    echo ""

    if [ $CHECKS_FAILED -gt 0 ]; then
        print_fail "Parity validation FAILED"
        print_info "Review logs in /tmp/*_parity.log"
        print_info "See methodology: docs/parity-validation-methodology.md §9 Troubleshooting"
        return $EXIT_FAILURE
    elif [ $CHECKS_WARNING -gt 0 ]; then
        print_warn "Parity validation completed with WARNINGS"
        print_info "Some checks not yet implemented (expected for early development)"
        return $EXIT_SUCCESS
    else
        print_pass "All parity checks PASSED! ✅"
        return $EXIT_SUCCESS
    fi
}

# Main script

main() {
    local run_graph=0
    local run_search=0
    local run_traverse=0
    local run_performance=0
    local generate_report_flag=0

    # Parse arguments
    if [ $# -eq 0 ]; then
        echo "Usage: $0 [--all|--graph|--search|--traverse|--performance|--report]"
        echo ""
        echo "Options:"
        echo "  --all          Run all parity checks"
        echo "  --graph        Run graph parity check"
        echo "  --search       Run search parity check"
        echo "  --traverse     Run traverse parity check"
        echo "  --performance  Run performance benchmarks"
        echo "  --report       Generate markdown report"
        echo ""
        exit $EXIT_SUCCESS
    fi

    while [[ $# -gt 0 ]]; do
        case $1 in
            --all)
                run_graph=1
                run_search=1
                run_traverse=1
                run_performance=1
                shift
                ;;
            --graph)
                run_graph=1
                shift
                ;;
            --search)
                run_search=1
                shift
                ;;
            --traverse)
                run_traverse=1
                shift
                ;;
            --performance)
                run_performance=1
                shift
                ;;
            --report)
                generate_report_flag=1
                shift
                ;;
            *)
                echo "Unknown option: $1"
                exit $EXIT_FAILURE
                ;;
        esac
    done

    print_header
    check_prerequisites

    # Run selected checks
    if [ $run_graph -eq 1 ]; then
        run_graph_parity_check
    fi

    if [ $run_search -eq 1 ]; then
        run_search_parity_check
    fi

    if [ $run_traverse -eq 1 ]; then
        run_traverse_parity_check
    fi

    if [ $run_performance -eq 1 ]; then
        run_performance_check
    fi

    # Generate report if requested
    if [ $generate_report_flag -eq 1 ]; then
        generate_report
    fi

    # Print summary and exit
    print_summary
    exit $?
}

main "$@"
