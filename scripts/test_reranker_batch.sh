#!/bin/bash
# Batch Test Harness for LLM Re-Ranking POC (Thread-20 Phase 2)
# Tests 5-10 diverse RANKING_ISSUE queries from diagnostic analysis
#
# Usage:
#   ./scripts/test_reranker_batch.sh [output_file.json]
#
# Environment Variables:
#   LLM_RERANKER_MODEL - Model to use (default: haiku)
#   LLM_RERANKER_TIMEOUT - Timeout per query (default: 60)
#   LLM_RERANKER_DEBUG - Enable debug output (default: 0)

set -euo pipefail

# Configuration
MODEL="${LLM_RERANKER_MODEL:-haiku}"
TIMEOUT="${LLM_RERANKER_TIMEOUT:-60}"
DEBUG="${LLM_RERANKER_DEBUG:-0}"
OUTPUT_FILE="${1:-.artifacts/spec-tasks-T-02-02-sparse-index/THREAD-20-BATCH-RESULTS.json}"

# Paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DIAG_DIR=".artifacts/spec-tasks-T-02-02-sparse-index/diag"
RERANKER_SCRIPT="$SCRIPT_DIR/llm_reranker.sh"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Logging
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
if [[ ! -f "$RERANKER_SCRIPT" ]]; then
    log_error "Reranker script not found: $RERANKER_SCRIPT"
    exit 1
fi

if [[ ! -d "$DIAG_DIR" ]]; then
    log_error "Diagnostic directory not found: $DIAG_DIR"
    exit 1
fi

# Select RANKING_ISSUE queries from diagnostic files
# Criteria: overlap@10 < 75%, overlap@20 - overlap@10 >= 15% (ranking issue detected)
#
# Query Selection Rationale (8 diverse queries):
# 1. Severity levels: SEVERE (20-30%), MODERATE (50-60%), MILD (>80%)
# 2. Repos: scikit-learn (3), matplotlib (2), django (1), pytest (2)
# 3. Query types: Feature, API, Config, Test
# 4. Ranking gap: 14-50% between @10 and @20
select_test_queries() {
    log_info "Selecting 8 diverse RANKING_ISSUE queries from diagnostic files..."

    # Query selection (8 queries covering severity spectrum and diverse repos)
    cat <<'EOF' > /tmp/test_queries.jsonl
{"repo": "scikit-learn", "query_id": 0, "query_text": "RidgeClassifierCV store_cv_values parameter", "baseline_overlap": 20.0, "gap_at_20": 20.0, "severity": "SEVERE"}
{"repo": "pytest", "query_id": 1, "query_text": "_pytest.rewrite detect docstring constant", "baseline_overlap": 20.0, "gap_at_20": 40.0, "severity": "SEVERE"}
{"repo": "scikit-learn", "query_id": 3, "query_text": "RidgeClassifierCV cross-validation logic", "baseline_overlap": 28.57, "gap_at_20": 14.29, "severity": "SEVERE"}
{"repo": "matplotlib", "query_id": 2, "query_text": "cbook get_versions helper", "baseline_overlap": 28.57, "gap_at_20": 14.29, "severity": "SEVERE"}
{"repo": "django", "query_id": 1, "query_text": "TemporaryUploadedFile applies FILE_UPLOAD_PERMISSION", "baseline_overlap": 57.14, "gap_at_20": 14.29, "severity": "MODERATE"}
{"repo": "matplotlib", "query_id": 4, "query_text": "build metadata for Matplotlib version", "baseline_overlap": 60.0, "gap_at_20": 30.0, "severity": "MODERATE"}
{"repo": "pytest", "query_id": 0, "query_text": "rewrite handles first expression numeric literal", "baseline_overlap": 83.33, "gap_at_20": 16.67, "severity": "MILD"}
{"repo": "scikit-learn", "query_id": "POC", "query_text": "linear_model ridge.py parameters", "baseline_overlap": 25.0, "gap_at_20": 50.0, "severity": "SEVERE", "note": "POC baseline"}
EOF

    log_info "Selected $(wc -l < /tmp/test_queries.jsonl) test queries (diverse severity and repos)"
    log_info "  - SEVERE: 4 queries (20-30% baseline)"
    log_info "  - MODERATE: 2 queries (50-60% baseline)"
    log_info "  - MILD: 1 query (>80% baseline)"
    log_info "  - POC baseline: 1 query (validation)"
}

# Extract BM25 results for a specific query from diagnostic file
extract_bm25_results() {
    local repo="$1"
    local query_id="$2"

    # Special handling for POC baseline query (from test_reranker_input.json)
    if [[ "$query_id" == "POC" ]]; then
        if [[ -f "$SCRIPT_DIR/test_reranker_input.json" ]]; then
            cat "$SCRIPT_DIR/test_reranker_input.json"
            return 0
        else
            log_error "POC baseline file not found: $SCRIPT_DIR/test_reranker_input.json"
            return 1
        fi
    fi

    # Regular diagnostic file extraction
    local diag_file="$DIAG_DIR/${repo}_query_diagnostics.json"

    if [[ ! -f "$diag_file" ]]; then
        log_error "Diagnostic file not found: $diag_file"
        return 1
    fi

    # Extract query data: query_text, cds_top10, cds_scores, expected_top10
    jq --arg qid "$query_id" '
        .[$qid | tonumber] |
        {
            query: .query_text,
            bm25_results: ([.cds_scores[] | {
                path: (.[0] | sub(".*/tmp/smoke/[^/]+/"; "") | sub(".*/tmp/LocAgent/"; "")),
                score: .[1],
                rank: 0
            }] | to_entries | map(.value + {rank: (.key + 1)}) | .[0:10]),
            ground_truth_top10: .expected_top10,
            baseline_overlap_at_10: (.overlap_at_10 | tostring + "%")
        }
    ' "$diag_file"
}

# Run LLM re-ranking on a single query
run_reranking() {
    local test_input="$1"
    local start_time=$(date +%s)

    # Invoke re-ranker
    local result=$(echo "$test_input" | "$RERANKER_SCRIPT" 2>&1)
    local exit_code=$?
    local end_time=$(date +%s)
    local latency=$((end_time - start_time))

    if [[ $exit_code -ne 0 ]]; then
        log_error "Re-ranking failed with exit code $exit_code"
        echo "{\"error\": \"reranking_failed\", \"exit_code\": $exit_code, \"latency_seconds\": $latency}"
        return 1
    fi

    # Return result with latency
    echo "$result" | jq --arg lat "$latency" '. + {latency_seconds: ($lat | tonumber)}'
}

# Calculate overlap improvement
calculate_overlap() {
    local reranked_results="$1"
    local ground_truth="$2"

    # Calculate overlap@10 for re-ranked results
    echo "$reranked_results" | jq --argjson gt "$ground_truth" '
        .reranked_results[0:10] |
        map(.path) as $reranked |
        ($gt | map(. as $item |
            if ($reranked | any(. == $item)) then 1 else 0 end
        ) | add) as $matches |
        {
            overlap_at_10: (($matches / ($gt | length)) * 100),
            matches: $matches,
            total: ($gt | length)
        }
    '
}

# Main batch test loop
run_batch_tests() {
    log_info "Starting batch re-ranking tests..."

    local total_queries=0
    local successful_queries=0
    local failed_queries=0
    local total_latency=0
    local results=[]

    while IFS= read -r query_line; do
        total_queries=$((total_queries + 1))

        local repo=$(echo "$query_line" | jq -r '.repo')
        local query_id=$(echo "$query_line" | jq -r '.query_id')
        local query_text=$(echo "$query_line" | jq -r '.query_text')
        local baseline_overlap=$(echo "$query_line" | jq -r '.baseline_overlap')

        log_info "[$total_queries] Testing: $repo / Query $query_id: $query_text"
        log_info "    Baseline overlap@10: $baseline_overlap%"

        # Extract BM25 results
        local test_input=$(extract_bm25_results "$repo" "$query_id")
        if [[ $? -ne 0 ]]; then
            log_error "Failed to extract BM25 results for $repo query $query_id"
            failed_queries=$((failed_queries + 1))
            continue
        fi

        # Run re-ranking
        local reranked_result=$(run_reranking "$test_input")
        if [[ $? -ne 0 ]]; then
            log_error "Re-ranking failed for $repo query $query_id"
            failed_queries=$((failed_queries + 1))
            continue
        fi

        # Calculate overlap improvement
        local ground_truth=$(echo "$test_input" | jq '.ground_truth_top10')
        local overlap_metrics=$(calculate_overlap "$reranked_result" "$ground_truth")
        local new_overlap=$(echo "$overlap_metrics" | jq -r '.overlap_at_10')
        local improvement=$(echo "$new_overlap - $baseline_overlap" | bc -l)

        log_info "    After re-ranking: ${new_overlap}% overlap@10 (Δ +${improvement}%)"

        # Extract latency
        local latency=$(echo "$reranked_result" | jq -r '.latency_seconds // 0')
        total_latency=$((total_latency + latency))

        # Store result
        local result=$(jq -n \
            --arg repo "$repo" \
            --argjson qid "$query_id" \
            --arg query "$query_text" \
            --argjson baseline "$baseline_overlap" \
            --argjson new "$new_overlap" \
            --argjson improvement "$improvement" \
            --argjson lat "$latency" \
            --argjson overlap "$overlap_metrics" \
            '{
                repo: $repo,
                query_id: $qid,
                query_text: $query,
                baseline_overlap_at_10: $baseline,
                reranked_overlap_at_10: $new,
                improvement_percentage: $improvement,
                latency_seconds: $lat,
                overlap_metrics: $overlap
            }')

        # Append to results array (store in temp file)
        echo "$result" >> /tmp/batch_results.jsonl

        successful_queries=$((successful_queries + 1))

    done < /tmp/test_queries.jsonl

    # Generate summary
    local avg_latency=0
    if [[ $successful_queries -gt 0 ]]; then
        avg_latency=$((total_latency / successful_queries))
    fi

    local summary=$(jq -n \
        --argjson total "$total_queries" \
        --argjson success "$successful_queries" \
        --argjson failed "$failed_queries" \
        --argjson avg_lat "$avg_latency" \
        '{
            total_queries: $total,
            successful_queries: $success,
            failed_queries: $failed,
            avg_latency_seconds: $avg_lat,
            model: env.LLM_RERANKER_MODEL,
            timestamp: (now | strftime("%Y-%m-%dT%H:%M:%SZ"))
        }')

    # Combine summary + results
    local all_results=$(cat /tmp/batch_results.jsonl | jq -s '.')
    jq -n \
        --argjson summary "$summary" \
        --argjson results "$all_results" \
        '{summary: $summary, results: $results}' > "$OUTPUT_FILE"

    log_info "Batch test complete!"
    log_info "  Total queries: $total_queries"
    log_info "  Successful: $successful_queries"
    log_info "  Failed: $failed_queries"
    log_info "  Avg latency: ${avg_latency}s"
    log_info "Results saved to: $OUTPUT_FILE"

    # Cleanup
    rm -f /tmp/batch_results.jsonl /tmp/test_queries.jsonl
}

# Main
main() {
    log_info "LLM Re-Ranking Batch Test Harness (Thread-20 Phase 2)"
    log_info "Model: $MODEL | Timeout: ${TIMEOUT}s | Debug: $DEBUG"

    select_test_queries
    run_batch_tests

    log_info "Done!"
}

main "$@"
