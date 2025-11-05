#!/bin/bash
# LLM Re-Ranker Wrapper Script for CDSAgent
# Invokes ast-graph-index-ranker sub-agent via Claude Code CLI headless mode
#
# Usage:
#   echo '{"query": "...", "bm25_results": [...]}' | ./scripts/llm_reranker.sh
#
# Environment Variables:
#   LLM_RERANKER_MODEL - Model to use (default: haiku)
#   LLM_RERANKER_TIMEOUT - Timeout in seconds (default: 30)
#   LLM_RERANKER_DEBUG - Enable debug output (default: 0)

set -euo pipefail

# Configuration
MODEL="${LLM_RERANKER_MODEL:-haiku}"
TIMEOUT="${LLM_RERANKER_TIMEOUT:-30}"
DEBUG="${LLM_RERANKER_DEBUG:-0}"

# Read input JSON from stdin
QUERY_DATA="$(cat)"

# Debug: Show input
if [[ "$DEBUG" == "1" ]]; then
  echo "[DEBUG] Input JSON:" >&2
  echo "$QUERY_DATA" | jq '.' >&2
fi

# Build prompt for Claude CLI
PROMPT="You are the ast-graph-index-ranker sub-agent. Re-rank the BM25 search results provided below.

Input data (JSON):
$QUERY_DATA

Remember:
1. Output ONLY valid JSON matching the schema in your instructions
2. Focus on semantic relevance, not just keyword matching
3. Use graph context if provided to identify central files
4. Be conservative: only adjust scores when confidence >= 0.7
5. Target latency: <2 seconds

Begin your response with valid JSON:"

# Invoke Claude Code CLI in headless mode
# - Use --print (-p) for non-interactive mode
# - Use --output-format json for structured response
# - Use --model to specify haiku (fast, low-cost)
# - Suppress stderr to avoid noise (redirect to /dev/null)
# - Extract the "result" field which contains the sub-agent's JSON output

if [[ "$DEBUG" == "1" ]]; then
  echo "[DEBUG] Invoking Claude CLI with model=$MODEL timeout=$TIMEOUT" >&2
fi

# Run with timeout
RESPONSE=$(timeout "$TIMEOUT" claude -p "$PROMPT" \
  --output-format json \
  --model "$MODEL" \
  2>/dev/null || echo '{"error": "claude_cli_timeout_or_error"}')

# Debug: Show raw response
if [[ "$DEBUG" == "1" ]]; then
  echo "[DEBUG] Claude CLI response:" >&2
  echo "$RESPONSE" | jq '.' >&2
fi

# Extract the "result" field from the last message (type="result")
# Claude CLI returns an array of messages, we need the last one with type="result"
RESULT=$(echo "$RESPONSE" | jq -r '.[] | select(.type == "result") | .result // empty')

if [[ -z "$RESULT" ]]; then
  # Error case: Claude CLI failed or returned empty result
  echo '{"error": "empty_result_from_claude_cli", "raw_response": '"$RESPONSE"'}' >&2
  exit 1
fi

# Debug: Show extracted result
if [[ "$DEBUG" == "1" ]]; then
  echo "[DEBUG] Extracted result:" >&2
  echo "$RESULT" | jq '.' >&2
fi

# Strip markdown code fences if present (```json ... ```)
# Claude often wraps JSON in markdown, we need raw JSON
CLEAN_RESULT=$(echo "$RESULT" | sed -e 's/^```json//' -e 's/```$//' | sed -e 's/^```//' -e 's/```$//' | sed '/^$/d')

# Debug: Show cleaned result
if [[ "$DEBUG" == "1" ]]; then
  echo "[DEBUG] Cleaned result (markdown stripped):" >&2
  echo "$CLEAN_RESULT" | jq '.' >&2 2>/dev/null || echo "$CLEAN_RESULT" >&2
fi

# Output the sub-agent's JSON response to stdout
echo "$CLEAN_RESULT"

# Exit with success
exit 0
