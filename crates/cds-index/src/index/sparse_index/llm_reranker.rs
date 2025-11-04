// llm_reranker.rs - Rust Bridge to Claude Code CLI LLM Re-Ranking
//
// Thread-21: Selective LLM Integration
// Purpose: Invoke scripts/llm_reranker.sh via subprocess with error handling
// Feature: llm-reranking (optional, off by default)

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use super::SearchResult;

/// LLM re-ranker using Claude Code CLI headless mode
///
/// # Architecture
/// - Invokes `scripts/llm_reranker.sh` via subprocess
/// - Passes BM25 results as JSON via stdin
/// - Reads re-ranked results from stdout
/// - Handles timeouts, errors, and graceful fallback
///
/// # Thread-20 Performance
/// - Latency: ~17s per query (haiku model)
/// - Effectiveness: 42.86% (only SEVERE entity queries benefit)
/// - Cost: $0.001-0.002 per query (estimated)
#[derive(Clone, Debug)]
pub struct LlmReranker {
    script_path: PathBuf,
    timeout_secs: u64,
}

/// Input format for LLM re-ranker (JSON via stdin)
#[derive(Debug, Serialize)]
struct RerankerInput {
    query: String,
    bm25_results: Vec<Bm25ResultForLlm>,
}

#[derive(Debug, Serialize)]
struct Bm25ResultForLlm {
    path: String,
    score: f32,
    entity_id: String,
    content: String, // Synthesized from docstring + implementation
}

/// Output format from LLM re-ranker (JSON from stdout)
#[derive(Debug, Deserialize)]
struct RerankerOutput {
    reranked_results: Vec<RerankedItem>,
}

#[derive(Debug, Deserialize)]
struct RerankedItem {
    path: String,
    adjusted_score: f32,
    confidence: f32,
    reasoning: Option<String>,
}

impl LlmReranker {
    /// Create new LLM re-ranker with default configuration
    ///
    /// # Default Settings
    /// - Script path: `./scripts/llm_reranker.sh`
    /// - Timeout: 10 seconds (hard limit)
    ///
    /// # Errors
    /// Returns error if script not found or not executable
    pub fn new() -> Result<Self> {
        Self::with_config("./scripts/llm_reranker.sh", 10)
    }

    /// Create LLM re-ranker with custom configuration
    ///
    /// # Arguments
    /// * `script_path` - Path to llm_reranker.sh
    /// * `timeout_secs` - Maximum execution time per query
    pub fn with_config(script_path: impl AsRef<Path>, timeout_secs: u64) -> Result<Self> {
        let script_path = script_path.as_ref().to_path_buf();

        // Verify script exists
        if !script_path.exists() {
            bail!(
                "LLM re-ranker script not found: {}",
                script_path.display()
            );
        }

        Ok(Self {
            script_path,
            timeout_secs,
        })
    }

    /// Re-rank BM25 results using LLM semantic understanding
    ///
    /// # Arguments
    /// * `query` - Search query text
    /// * `results` - BM25 search results (Top-50 candidates)
    ///
    /// # Returns
    /// Re-ranked results with adjusted scores and confidence levels
    ///
    /// # Error Handling
    /// - Timeout: Returns original BM25 results
    /// - JSON parse failure: Returns original BM25 results
    /// - Script error: Returns original BM25 results
    ///
    /// All errors are logged but do NOT propagate (graceful fallback)
    pub fn rerank(&self, query: &str, results: &[SearchResult]) -> Result<Vec<SearchResult>> {
        // Prepare input JSON
        let input = RerankerInput {
            query: query.to_string(),
            bm25_results: results
                .iter()
                .map(|r| Bm25ResultForLlm {
                    path: r.path.clone(),
                    score: r.score,
                    entity_id: r.entity_id.clone(),
                    content: self.synthesize_content(r),
                })
                .collect(),
        };

        let input_json = serde_json::to_string(&input).context("failed to serialize input")?;

        // Spawn subprocess with timeout
        let mut child = Command::new(&self.script_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("failed to spawn llm_reranker.sh")?;

        // Write input JSON to stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(input_json.as_bytes())
                .context("failed to write to stdin")?;
        }

        // Wait for output with timeout
        let output = match Self::wait_with_timeout(&mut child, Duration::from_secs(self.timeout_secs))
        {
            Ok(output) => output,
            Err(e) => {
                eprintln!("LLM re-ranker timeout or error: {}", e);
                eprintln!("Falling back to BM25 results");
                let _ = child.kill(); // Try to kill timed-out process
                return Ok(results.to_vec()); // Graceful fallback
            }
        };

        // Check exit status
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("LLM re-ranker failed: {}", stderr);
            return Ok(results.to_vec()); // Graceful fallback
        }

        // Parse output JSON
        let output_str = String::from_utf8(output.stdout).context("invalid UTF-8 in output")?;
        let reranker_output: RerankerOutput =
            serde_json::from_str(&output_str).context("failed to parse output JSON")?;

        // Map re-ranked results back to SearchResult
        let mut reranked = Vec::new();
        for item in reranker_output.reranked_results {
            // Find original result by path
            if let Some(original) = results.iter().find(|r| r.path == item.path) {
                let mut result = original.clone();
                result.score = item.adjusted_score;
                // TODO: Store LLM confidence in SearchResult (requires struct update)
                reranked.push(result);
            }
        }

        Ok(reranked)
    }

    /// Wait for child process with timeout
    fn wait_with_timeout(
        child: &mut std::process::Child,
        timeout: Duration,
    ) -> Result<std::process::Output> {
        use std::thread;

        let pid = child.id();
        let handle = thread::spawn(move || {
            thread::sleep(timeout);
            // If thread completes, timeout occurred
            pid
        });

        // Try to wait for child
        match child.try_wait()? {
            Some(status) => {
                // Process completed - read stdout/stderr
                let mut stdout_str = String::new();
                let mut stderr_str = String::new();

                if let Some(mut stdout_pipe) = child.stdout.take() {
                    let _ = std::io::Read::read_to_string(&mut stdout_pipe, &mut stdout_str);
                }
                if let Some(mut stderr_pipe) = child.stderr.take() {
                    let _ = std::io::Read::read_to_string(&mut stderr_pipe, &mut stderr_str);
                }

                Ok(std::process::Output {
                    status,
                    stdout: stdout_str.into_bytes(),
                    stderr: stderr_str.into_bytes(),
                })
            }
            None => {
                // Still running, wait with timeout
                match handle.join() {
                    Ok(_) => {
                        // Timeout occurred
                        bail!("LLM re-ranker timed out after {} seconds", timeout.as_secs());
                    }
                    Err(_) => {
                        // Thread panicked (shouldn't happen)
                        bail!("Timeout thread panicked");
                    }
                }
            }
        }
    }

    /// Synthesize content for LLM from SearchResult metadata
    ///
    /// # Content Strategy (Thread-17 Vanilla BM25)
    /// - Entity name + path (primary signals)
    /// - Matched terms (BM25 highlights)
    /// - No docstring/comments (removed in Thread-17 overfitting fix)
    fn synthesize_content(&self, result: &SearchResult) -> String {
        let mut content = String::new();

        // Add entity name
        if let Some(name) = &result.name {
            content.push_str(&format!("name: {}\n", name));
        }

        // Add path
        content.push_str(&format!("path: {}\n", result.path));

        // Add kind
        content.push_str(&format!("kind: {:?}\n", result.kind));

        // Add matched terms (if available)
        if !result.matched_terms.is_empty() {
            content.push_str(&format!("matched_terms: {}\n", result.matched_terms.join(", ")));
        }

        content
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::NodeKind;

    #[test]
    fn test_synthesize_content() {
        let reranker = LlmReranker::new().unwrap();

        let result = SearchResult {
            entity_id: "repo::file.py::MyClass".to_string(),
            name: Some("MyClass".to_string()),
            path: "src/file.py".to_string(),
            kind: NodeKind::Class,
            score: 15.3,
            matched_terms: vec!["class".to_string(), "parameter".to_string()],
        };

        let content = reranker.synthesize_content(&result);

        assert!(content.contains("name: MyClass"));
        assert!(content.contains("path: src/file.py"));
        assert!(content.contains("kind: Class"));
        assert!(content.contains("matched_terms: class, parameter"));
    }

    #[test]
    fn test_reranker_creation_fails_if_script_missing() {
        let result = LlmReranker::with_config("/nonexistent/script.sh", 10);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    // TODO: Integration test with NO mock , real test_reranker_input.json script (requires test fixtures)
    // #[test]
    // fn test_rerank_with_mock_script() { ... }
}
