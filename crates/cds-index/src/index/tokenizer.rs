use crate::index::stop_words::DEFAULT_STOP_WORDS;
use rust_stemmers::{Algorithm, Stemmer};
use std::collections::HashSet;
use std::ops::Range;
use std::sync::Arc;
use tantivy::tokenizer::{
    TextAnalyzer, Token, TokenStream as TantivyTokenStreamTrait, Tokenizer as TantivyTokenizerTrait,
};
use unicode_normalization::{char::is_combining_mark, UnicodeNormalization};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenizedToken {
    pub text: String,
    pub offset_from: usize,
    pub offset_to: usize,
}

#[derive(Debug, Clone)]
struct IdentifierToken {
    text: String,
    offset: Range<usize>,
}

/// Tokenizer that mirrors LocAgent's BM25 preprocessing pipeline.
pub struct Tokenizer {
    stop_words: Arc<HashSet<String>>,
    stemmer: Stemmer,
}

impl Clone for Tokenizer {
    fn clone(&self) -> Self {
        Self {
            stop_words: Arc::clone(&self.stop_words),
            stemmer: Stemmer::create(Algorithm::English),
        }
    }
}

impl Tokenizer {
    /// Creates a tokenizer with the provided stop-word list.
    pub fn new(stop_words: impl IntoIterator<Item = String>) -> Self {
        let normalized_stop_words = stop_words
            .into_iter()
            .map(|word| word.to_ascii_lowercase())
            .collect::<HashSet<_>>();

        Self {
            stop_words: Arc::new(normalized_stop_words),
            stemmer: Stemmer::create(Algorithm::English),
        }
    }

    /// Tokenizes the provided input string into normalized, stemmed tokens.
    pub fn tokenize(&self, input: &str) -> Vec<String> {
        self.tokenize_with_offsets(input)
            .into_iter()
            .map(|token| token.text)
            .collect()
    }

    /// Tokenizes the string and returns tokens with offsets aligned to the normalized text.
    pub fn tokenize_with_offsets(&self, input: &str) -> Vec<TokenizedToken> {
        let mut output = Vec::new();

        for token in tokenize_identifiers_raw(input) {
            let lower = token.text.to_ascii_lowercase();
            if lower.is_empty() || self.stop_words.contains(&lower) {
                continue;
            }

            let stemmed = self.stemmer.stem(&lower).into_owned();
            if stemmed.is_empty() || self.stop_words.contains(&stemmed) {
                continue;
            }

            output.push(TokenizedToken {
                text: stemmed,
                offset_from: token.offset.start,
                offset_to: token.offset.end,
            });
        }

        output
    }

    /// Creates a tokenizer configured with the default English stop-word set.
    pub fn with_default_stop_words() -> Self {
        let stop_words = DEFAULT_STOP_WORDS
            .iter()
            .map(|word| word.to_string())
            .collect::<Vec<_>>();
        Self::new(stop_words)
    }

    /// Builds a Tantivy `TextAnalyzer` that reuses the CDS tokenizer.
    pub fn to_text_analyzer(&self) -> TextAnalyzer {
        TextAnalyzer::from(TantivyCodeTokenizer::new(self.clone()))
    }
}

#[derive(Clone)]
pub struct TantivyCodeTokenizer {
    inner: Tokenizer,
}

impl TantivyCodeTokenizer {
    pub fn new(inner: Tokenizer) -> Self {
        Self { inner }
    }
}

impl TantivyTokenizerTrait for TantivyCodeTokenizer {
    type TokenStream<'a> = CodeTokenStream;

    fn token_stream<'a>(&'a mut self, text: &'a str) -> Self::TokenStream<'a> {
        CodeTokenStream::new(self.inner.tokenize_with_offsets(text))
    }
}

pub struct CodeTokenStream {
    tokens: Vec<TokenizedToken>,
    index: usize,
    current: Token,
}

impl CodeTokenStream {
    fn new(tokens: Vec<TokenizedToken>) -> Self {
        Self {
            tokens,
            index: 0,
            current: Token::default(),
        }
    }
}

impl TantivyTokenStreamTrait for CodeTokenStream {
    fn advance(&mut self) -> bool {
        if self.index >= self.tokens.len() {
            return false;
        }

        let token = &self.tokens[self.index];
        self.current.offset_from = token.offset_from;
        self.current.offset_to = token.offset_to;
        self.current.position = self.index;
        self.current.text.clear();
        self.current.text.push_str(&token.text);
        self.current.position_length = 1;

        self.index += 1;
        true
    }

    fn token(&self) -> &Token {
        &self.current
    }

    fn token_mut(&mut self) -> &mut Token {
        &mut self.current
    }
}

/// Normalizes the input (NFKD, ASCII fold, punctuation → spaces) and returns
/// raw identifier tokens before stop-word filtering.
#[cfg(test)]
fn tokenize_identifiers(input: &str) -> Vec<String> {
    tokenize_identifiers_raw(input)
        .into_iter()
        .map(|token| token.text)
        .collect()
}

fn tokenize_identifiers_raw(input: &str) -> Vec<IdentifierToken> {
    let normalized = normalize(input);
    let mut tokens = Vec::new();

    let mut fragment_start: Option<usize> = None;
    for (idx, ch) in normalized.char_indices() {
        if ch.is_ascii_whitespace() {
            if let Some(start) = fragment_start.take() {
                push_fragment_tokens(&normalized, start..idx, &mut tokens);
            }
        } else if fragment_start.is_none() {
            fragment_start = Some(idx);
        }
    }

    if let Some(start) = fragment_start {
        push_fragment_tokens(&normalized, start..normalized.len(), &mut tokens);
    }

    tokens
}

fn normalize(input: &str) -> String {
    let decomposed = input.nfkd().filter(|&c| !is_combining_mark(c));

    let mut normalized = String::with_capacity(input.len());
    for ch in decomposed {
        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
            normalized.push(ch);
        } else if ch.is_ascii_whitespace() || ch.is_ascii_punctuation() {
            normalized.push(' ');
        } else if ch.is_ascii() {
            normalized.push(ch);
        }
        // Non-ASCII characters are dropped to match LocAgent's ASCII folding.
    }

    normalized
}

fn push_fragment_tokens(
    normalized: &str,
    fragment: Range<usize>,
    tokens: &mut Vec<IdentifierToken>,
) {
    if fragment.is_empty() {
        return;
    }

    let mut part_start = fragment.start;
    let slice = &normalized[fragment.clone()];
    for (rel_idx, ch) in slice.char_indices() {
        if ch == '_' || ch == '-' {
            let absolute = fragment.start + rel_idx;
            if part_start < absolute {
                split_camel_segment(normalized, part_start..absolute, tokens);
            }
            part_start = absolute + ch.len_utf8();
        }
    }

    if part_start < fragment.end {
        split_camel_segment(normalized, part_start..fragment.end, tokens);
    }
}

fn split_camel_segment(normalized: &str, segment: Range<usize>, tokens: &mut Vec<IdentifierToken>) {
    if segment.is_empty() {
        return;
    }

    let chars = normalized[segment.clone()]
        .char_indices()
        .map(|(rel_idx, ch)| (segment.start + rel_idx, ch))
        .collect::<Vec<_>>();

    if chars.is_empty() {
        return;
    }

    let mut start = segment.start;
    for idx in 1..chars.len() {
        let (_, prev) = chars[idx - 1];
        let (curr_idx, curr) = chars[idx];
        let next = chars.get(idx + 1).map(|(_, ch)| *ch);

        if is_boundary(prev, curr, next) {
            if curr_idx > start {
                push_identifier(normalized, start..curr_idx, tokens);
            }
            start = curr_idx;
        }
    }

    if start < segment.end {
        push_identifier(normalized, start..segment.end, tokens);
    }
}

fn push_identifier(normalized: &str, range: Range<usize>, tokens: &mut Vec<IdentifierToken>) {
    if range.is_empty() {
        return;
    }

    let text = normalized[range.clone()].to_string();
    if text.is_empty() {
        return;
    }

    tokens.push(IdentifierToken {
        text,
        offset: range,
    });
}

fn is_boundary(prev: char, curr: char, next: Option<char>) -> bool {
    if prev.is_ascii_digit() != curr.is_ascii_digit() {
        return true;
    }

    if prev.is_ascii_lowercase() && curr.is_ascii_uppercase() {
        return true;
    }

    if prev.is_ascii_uppercase() && curr.is_ascii_uppercase() {
        if let Some(next_char) = next {
            if next_char.is_ascii_lowercase() {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn tokenizer_with_stop_words(words: &[&str]) -> Tokenizer {
        let stop_words = words.iter().map(|w| w.to_string()).collect::<HashSet<_>>();
        Tokenizer::new(stop_words)
    }

    #[test]
    fn tokenizes_camel_and_snake_case() {
        let tokenizer = Tokenizer::with_default_stop_words();
        let tokens = tokenizer.tokenize("AuthServiceFactory parse_ast_node HTTP2Request");

        assert_eq!(
            tokens,
            vec!["auth", "servic", "factori", "pars", "ast", "node", "http", "2", "request"]
        );
    }

    #[test]
    fn removes_stop_words_and_stems() {
        let tokenizer = tokenizer_with_stop_words(&["the", "and"]);
        let tokens = tokenizer.tokenize("Running the tests and reading codes");

        assert_eq!(tokens, vec!["run", "test", "read", "code"]);
    }

    #[test]
    fn normalizes_unicode_and_punctuation() {
        let tokenizer = Tokenizer::with_default_stop_words();
        let tokens = tokenizer.tokenize("Café-util — sanitize_input()");

        assert_eq!(tokens, vec!["cafe", "util", "sanit", "input"]);
    }

    #[test]
    fn splits_uppercase_sequences_with_digits() {
        let tokens = tokenize_identifiers("HTTP2Request");

        assert_eq!(tokens, vec!["HTTP", "2", "Request"]);
    }

    #[test]
    fn filters_empty_results() {
        let tokenizer = tokenizer_with_stop_words(&["the", "and", "of"]);
        let tokens = tokenizer.tokenize("THE and OF");

        assert!(tokens.is_empty());
    }

    #[test]
    fn tokenize_with_offsets_preserves_ranges() {
        let tokenizer = Tokenizer::with_default_stop_words();
        let tokens = tokenizer.tokenize_with_offsets("parseHTTP2Request");

        let texts: Vec<_> = tokens.iter().map(|t| t.text.as_str()).collect();
        assert_eq!(texts, vec!["pars", "http", "2", "request"]);

        // Ensure offsets are monotonically increasing and non-empty.
        for window in tokens.windows(2) {
            assert!(window[0].offset_to <= window[1].offset_from);
        }
    }

    #[test]
    fn tantivy_stream_matches_tokenizer() {
        let tokenizer = Tokenizer::with_default_stop_words();
        let mut adapter = TantivyCodeTokenizer::new(tokenizer.clone());
        let mut stream = adapter.token_stream("GraphBuilder::from_repo");

        let mut collected = Vec::new();
        while stream.advance() {
            collected.push(stream.token().text.clone());
        }

        assert_eq!(collected, tokenizer.tokenize("GraphBuilder::from_repo"));
    }
}
