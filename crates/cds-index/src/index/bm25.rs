use std::{collections::HashSet, fs, path::Path};

use anyhow::{bail, Context, Result};
use tantivy::schema::Value;
use tantivy::tokenizer::{TextAnalyzer, TokenizerManager};
use tantivy::TantivyDocument;
use tantivy::{
    collector::TopDocs,
    query::QueryParser,
    schema::{
        Field, IndexRecordOption, Schema, SchemaBuilder, TextFieldIndexing, TextOptions, STORED,
        STRING,
    },
    Index, IndexReader, ReloadPolicy,
};

use crate::graph::NodeKind;

use super::tokenizer::{TantivyCodeTokenizer, Tokenizer};

pub const CODE_ANALYZER_NAME: &str = "cds_code";
const DEFAULT_WRITER_HEAP_SIZE: usize = 50 * 1024 * 1024; // 50 MiB

#[derive(Clone, Debug, Default)]
pub struct AnalyzerConfig {
    pub stop_words: Option<HashSet<String>>,
}

impl AnalyzerConfig {
    fn build_tokenizer(&self) -> Tokenizer {
        match &self.stop_words {
            Some(words) if !words.is_empty() => Tokenizer::new(words.iter().cloned()),
            _ => Tokenizer::with_default_stop_words(),
        }
    }

    pub fn text_analyzer(&self) -> TextAnalyzer {
        TextAnalyzer::from(TantivyCodeTokenizer::new(self.build_tokenizer()))
    }
}

pub fn register_code_analyzer(manager: &TokenizerManager, config: &AnalyzerConfig) {
    manager.register(
        CODE_ANALYZER_NAME,
        TantivyCodeTokenizer::new(config.build_tokenizer()),
    );
}

#[derive(Clone, Debug)]
pub struct Bm25Document<'a> {
    pub entity_id: &'a str,
    pub name: Option<&'a str>,
    pub path: &'a str,
    pub kind: NodeKind,
    pub content: &'a str,
}

#[derive(Clone, Debug)]
pub struct SearchResult {
    pub entity_id: String,
    pub name: Option<String>,
    pub path: String,
    pub kind: NodeKind,
    pub score: f32,
    pub matched_terms: Vec<String>,
}

#[derive(Clone, Copy)]
struct Fields {
    entity_id: Field,
    name: Field,
    path: Field,
    kind: Field,
    content: Field,
}

pub struct Bm25Index {
    index: Index,
    reader: IndexReader,
    schema: Schema,
    fields: Fields,
    analyzer_config: AnalyzerConfig,
    tokenizer: Tokenizer,
}

impl Clone for Bm25Index {
    fn clone(&self) -> Self {
        Self {
            index: self.index.clone(),
            reader: self.reader.clone(),
            schema: self.schema.clone(),
            fields: self.fields,
            analyzer_config: self.analyzer_config.clone(),
            tokenizer: self.tokenizer.clone(),
        }
    }
}

impl Bm25Index {
    /// Creates a brand-new BM25 index in the provided directory.
    /// Fails if the directory already contains data.
    pub fn create_in_dir(path: impl AsRef<Path>, config: AnalyzerConfig) -> Result<Self> {
        let path = path.as_ref();
        if path.exists() {
            if path.read_dir()?.next().is_some() {
                bail!(
                    "BM25 index directory {:?} already exists and is not empty",
                    path
                );
            }
        } else {
            fs::create_dir_all(path)
                .with_context(|| format!("failed to create BM25 index directory at {:?}", path))?;
        }

        let schema = build_schema();
        let index =
            Index::create_in_dir(path, schema.clone()).context("failed to create tantivy index")?;
        Self::from_index(index, schema, config)
    }

    /// Opens an existing BM25 index from disk.
    pub fn open(path: impl AsRef<Path>, config: AnalyzerConfig) -> Result<Self> {
        let path = path.as_ref();
        let index = Index::open_in_dir(path)
            .with_context(|| format!("failed to open index at {:?}", path))?;
        let schema = index.schema();
        Self::from_index(index, schema, config)
    }

    /// Opens an existing index or creates a new one if the directory is empty.
    pub fn open_or_create(path: impl AsRef<Path>, config: AnalyzerConfig) -> Result<Self> {
        let path = path.as_ref();
        match Index::open_in_dir(path) {
            Ok(index) => {
                let schema = index.schema();
                Self::from_index(index, schema, config)
            }
            Err(_) => Self::create_in_dir(path, config),
        }
    }

    fn from_index(index: Index, schema: Schema, config: AnalyzerConfig) -> Result<Self> {
        let fields = resolve_fields(&schema)?;
        register_code_analyzer(index.tokenizers(), &config);
        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::Manual)
            .try_into()
            .context("failed to create index reader")?;

        Ok(Self {
            index,
            reader,
            schema,
            fields,
            tokenizer: config.build_tokenizer(),
            analyzer_config: config,
        })
    }

    /// Adds (or rebuilds) the BM25 index with the provided documents.
    /// Existing content is cleared before new documents are indexed.
    pub fn replace_documents<'a>(
        &self,
        docs: impl IntoIterator<Item = Bm25Document<'a>>,
    ) -> Result<()> {
        let mut writer = self
            .index
            .writer(DEFAULT_WRITER_HEAP_SIZE)
            .context("failed to create index writer")?;

        writer
            .delete_all_documents()
            .context("failed to clear index")?;
        for doc in docs {
            writer
                .add_document(self.build_document(&doc))
                .context("failed to add document to BM25 index")?;
        }
        writer.commit().context("failed to commit BM25 index")?;
        self.reader
            .reload()
            .context("failed to reload BM25 index reader")?;
        Ok(())
    }

    /// Executes a BM25 search and returns scored results.
    pub fn search(
        &self,
        query: &str,
        limit: usize,
        kind_filter: Option<&[NodeKind]>,
    ) -> Result<Vec<SearchResult>> {
        if limit == 0 {
            return Ok(Vec::new());
        }
        let query_tokens = self.tokenizer.tokenize(query);
        if query_tokens.is_empty() {
            return Ok(Vec::new());
        }

        let query_string = query_tokens.join(" ");
        if query_string.is_empty() {
            return Ok(Vec::new());
        }

        let searcher = self.reader.searcher();
        let mut parser = QueryParser::for_index(&self.index, vec![self.fields.content]);
        parser.set_conjunction_by_default();
        let parsed_query = parser
            .parse_query(&query_string)
            .context("failed to parse BM25 query")?;

        let oversample = (limit * 4).max(limit + 5);
        let top_docs = searcher.search(&parsed_query, &TopDocs::with_limit(oversample))?;

        let kind_filter: Option<HashSet<NodeKind>> =
            kind_filter.map(|kinds| kinds.iter().copied().collect());

        let mut results = Vec::new();
        for (score, address) in top_docs {
            let doc: TantivyDocument = searcher
                .doc(address)
                .context("failed to fetch document from BM25 index")?;

            let entity_id = doc_value(&doc, self.fields.entity_id)?.to_string();
            let path = doc_value(&doc, self.fields.path)?.to_string();
            let kind = doc_value(&doc, self.fields.kind)?;
            let kind =
                node_kind_from_str(kind).context("invalid node kind stored in BM25 index")?;

            if let Some(filter) = &kind_filter {
                if !filter.contains(&kind) {
                    continue;
                }
            }

            let name = doc
                .get_first(self.fields.name)
                .and_then(|value| value.as_str())
                .map(str::to_string);

            let content = doc_value(&doc, self.fields.content)?;
            let doc_tokens: HashSet<String> =
                self.tokenizer.tokenize(content).into_iter().collect();

            let mut matched = Vec::new();
            let mut seen = HashSet::new();
            for token in &query_tokens {
                if seen.insert(token.as_str()) && doc_tokens.contains(token) {
                    matched.push(token.clone());
                }
            }

            results.push(SearchResult {
                entity_id,
                name,
                path,
                kind,
                score,
                matched_terms: matched,
            });

            if results.len() >= limit {
                break;
            }
        }

        Ok(results)
    }

    fn build_document(&self, doc: &Bm25Document<'_>) -> TantivyDocument {
        let mut document = TantivyDocument::default();
        document.add_text(self.fields.entity_id, doc.entity_id);
        document.add_text(self.fields.path, doc.path);
        document.add_text(self.fields.kind, node_kind_to_str(doc.kind));
        document.add_text(self.fields.content, doc.content);
        if let Some(name) = doc.name {
            document.add_text(self.fields.name, name);
        }
        document
    }

    /// Returns the underlying tantivy index (useful for advanced operations).
    pub fn index(&self) -> &Index {
        &self.index
    }

    /// Returns the schema used by the index.
    pub fn schema(&self) -> &Schema {
        &self.schema
    }
}

fn build_schema() -> Schema {
    let mut builder: SchemaBuilder = Schema::builder();
    builder.add_text_field("entity_id", STRING | STORED);
    builder.add_text_field("path", STRING | STORED);
    builder.add_text_field("kind", STRING | STORED);
    let name_options = TextOptions::default().set_stored();
    builder.add_text_field("name", name_options);

    let content_indexing = TextFieldIndexing::default()
        .set_tokenizer(CODE_ANALYZER_NAME)
        .set_index_option(IndexRecordOption::WithFreqsAndPositions);
    let content_options = TextOptions::default()
        .set_indexing_options(content_indexing)
        .set_stored();
    builder.add_text_field("content", content_options);

    builder.build()
}

fn resolve_fields(schema: &Schema) -> Result<Fields> {
    let entity_id = schema
        .get_field("entity_id")
        .context("schema missing entity_id field")?;
    let name = schema
        .get_field("name")
        .context("schema missing name field")?;
    let path = schema
        .get_field("path")
        .context("schema missing path field")?;
    let kind = schema
        .get_field("kind")
        .context("schema missing kind field")?;
    let content = schema
        .get_field("content")
        .context("schema missing content field")?;

    Ok(Fields {
        entity_id,
        name,
        path,
        kind,
        content,
    })
}

fn node_kind_to_str(kind: NodeKind) -> &'static str {
    match kind {
        NodeKind::Directory => "directory",
        NodeKind::File => "file",
        NodeKind::Class => "class",
        NodeKind::Function => "function",
    }
}

fn node_kind_from_str(kind: &str) -> Option<NodeKind> {
    match kind {
        "directory" => Some(NodeKind::Directory),
        "file" => Some(NodeKind::File),
        "class" => Some(NodeKind::Class),
        "function" => Some(NodeKind::Function),
        _ => None,
    }
}

fn doc_value(doc: &TantivyDocument, field: Field) -> Result<&str> {
    doc.get_first(field)
        .and_then(|value| value.as_str())
        .context("document missing required stored field")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::NodeKind;
    use tempfile::tempdir;

    #[test]
    fn bm25_index_creates_and_searches() -> Result<()> {
        let dir = tempdir()?;
        let index = Bm25Index::create_in_dir(dir.path(), AnalyzerConfig::default())?;
        index.replace_documents([
            Bm25Document {
                entity_id: "repo::module::sanitize_input",
                name: Some("sanitize_input"),
                path: "src/security.py",
                kind: NodeKind::Function,
                content: r#"
def sanitize_input(user_input):
    cleaned = user_input.strip()
    return cleaned.lower()
"#,
            },
            Bm25Document {
                entity_id: "repo::module::hash_password",
                name: Some("hash_password"),
                path: "src/security.py",
                kind: NodeKind::Function,
                content: r#"
def hash_password(password):
    return sha256(password.encode()).hexdigest()
"#,
            },
        ])?;

        let results = index.search("sanitize USER input", 5, None)?;
        assert_eq!(results.len(), 1);
        let result = &results[0];
        assert_eq!(result.entity_id, "repo::module::sanitize_input");
        assert!(result.matched_terms.iter().any(|term| term == "sanit"));
        assert!(result.matched_terms.iter().any(|term| term == "input"));
        Ok(())
    }

    #[test]
    fn bm25_respects_kind_filter() -> Result<()> {
        let dir = tempdir()?;
        let index = Bm25Index::create_in_dir(dir.path(), AnalyzerConfig::default())?;
        index.replace_documents([
            Bm25Document {
                entity_id: "repo::models::User",
                name: Some("User"),
                path: "src/models/user.py",
                kind: NodeKind::Class,
                content: "class User:\n    def save(self):\n        pass\n",
            },
            Bm25Document {
                entity_id: "repo::models::save_user",
                name: Some("save_user"),
                path: "src/models/user.py",
                kind: NodeKind::Function,
                content: "def save_user(user):\n    user.save()\n",
            },
        ])?;

        let class_only = index.search("save", 5, Some(&[NodeKind::Class]))?;
        assert_eq!(class_only.len(), 1);
        assert_eq!(class_only[0].kind, NodeKind::Class);

        let function_only = index.search("save", 5, Some(&[NodeKind::Function]))?;
        assert_eq!(function_only.len(), 1);
        assert_eq!(function_only[0].kind, NodeKind::Function);

        Ok(())
    }
}
