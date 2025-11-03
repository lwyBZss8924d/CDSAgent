#![allow(dead_code)]

use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

/// Golden LocAgent query loaded from parity fixtures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoldenSearchQuery {
    pub query: String,
    pub top10_files: Vec<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct RawSearchQuery {
    #[allow(dead_code)]
    repo: String,
    query: String,
    #[serde(rename = "top_10")]
    top10: Vec<RawSearchHit>,
    #[allow(dead_code)]
    total_results: usize,
}

#[derive(Debug, Deserialize)]
struct RawSearchHit {
    file: String,
}

/// Load LocAgent golden search queries from a JSONL fixture file.
pub fn load_locagent_search_queries(path: impl AsRef<Path>) -> Vec<GoldenSearchQuery> {
    let file = File::open(path.as_ref()).expect("unable to open search_queries.jsonl");
    let reader = BufReader::new(file);

    reader
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let line = line.expect("failed to read search query line");
            let raw: RawSearchQuery = serde_json::from_str(&line)
                .unwrap_or_else(|err| panic!("invalid JSON on line {}: {}", idx + 1, err));

            GoldenSearchQuery {
                query: raw.query,
                top10_files: raw
                    .top10
                    .into_iter()
                    .map(|hit| PathBuf::from(hit.file))
                    .collect(),
            }
        })
        .collect()
}
