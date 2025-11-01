mod support;

use cds_index::index::{register_code_analyzer, AnalyzerConfig, Tokenizer, CODE_ANALYZER_NAME};
use support::load_stop_words_fixture;
use tantivy::tokenizer::{TokenStream, TokenizerManager};

#[test]
fn fixture_stop_words_filter_properly() {
    let fixture_words = load_stop_words_fixture();
    let tokenizer = Tokenizer::new(fixture_words);

    let tokens = tokenizer.tokenize("Running the tests and reading code");
    assert_eq!(tokens, vec!["run", "test", "read", "code"]);

    let empty = tokenizer.tokenize("the and of to at");
    assert!(empty.is_empty());
}

#[test]
fn analyzer_config_produces_expected_tokens() {
    let stop_words = load_stop_words_fixture();
    let config = AnalyzerConfig {
        stop_words: Some(stop_words.clone()),
    };

    let mut analyzer = config.text_analyzer();
    let mut stream = analyzer.token_stream("Running the tests and reading code");

    let mut tokens = Vec::new();
    while let Some(token) = stream.next() {
        tokens.push(token.text.clone());
    }

    assert_eq!(tokens, vec!["run", "test", "read", "code"]);
}

#[test]
fn register_code_analyzer_registers_in_manager() {
    let manager = TokenizerManager::new();
    let config = AnalyzerConfig { stop_words: None };
    register_code_analyzer(&manager, &config);

    let mut analyzer = manager
        .get(CODE_ANALYZER_NAME)
        .expect("code analyzer should be registered");

    let mut stream = analyzer.token_stream("GraphBuilder::from_repo");
    let mut tokens = Vec::new();
    while let Some(token) = stream.next() {
        tokens.push(token.text.clone());
    }

    assert_eq!(
        tokens,
        Tokenizer::with_default_stop_words().tokenize("GraphBuilder::from_repo")
    );
}
