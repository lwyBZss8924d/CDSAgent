use std::collections::HashSet;

pub mod parity_loader;

#[allow(unused_imports)]
pub use parity_loader::{load_locagent_search_queries, GoldenSearchQuery};

#[allow(dead_code)]
pub fn load_stop_words_fixture() -> HashSet<String> {
    include_str!("../fixtures/parity/tokenizer/stop_words.txt")
        .lines()
        .map(|line| line.trim().to_ascii_lowercase())
        .filter(|line| !line.is_empty())
        .collect()
}
