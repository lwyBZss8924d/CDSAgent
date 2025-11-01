use std::collections::HashSet;

pub fn load_stop_words_fixture() -> HashSet<String> {
    include_str!("../fixtures/parity/tokenizer/stop_words.txt")
        .lines()
        .map(|line| line.trim().to_ascii_lowercase())
        .filter(|line| !line.is_empty())
        .collect()
}
