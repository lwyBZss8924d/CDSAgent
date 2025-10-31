use std::{collections::HashMap, sync::Arc};

use dashmap::DashMap;

use crate::graph::{DependencyGraph, GraphNode, GraphNodeIndex, NodeKind};

/// Entry stored in the upper dictionary index.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameEntry {
    pub name: Arc<str>,
    pub qualified_name: Option<Arc<str>>,
    pub node_id: GraphNodeIndex,
    pub kind: NodeKind,
}

/// Aggregated statistics about the index contents.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NameIndexStats {
    pub unique_names: usize,
    pub total_entities: usize,
}

/// Immutable upper dictionary index used for exact and prefix lookups.
#[derive(Clone)]
pub struct NameIndex {
    lookup: HashMap<Arc<str>, Arc<[NameEntry]>>,
    sorted_keys: Vec<Arc<str>>,
    stats: NameIndexStats,
}

impl Default for NameIndex {
    fn default() -> Self {
        Self {
            lookup: HashMap::new(),
            sorted_keys: Vec::new(),
            stats: NameIndexStats::default(),
        }
    }
}

impl std::fmt::Debug for NameIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NameIndex")
            .field("unique_names", &self.stats.unique_names)
            .field("total_entities", &self.stats.total_entities)
            .finish()
    }
}

impl NameIndex {
    /// Creates an empty index.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a builder that supports concurrent ingestion.
    pub fn builder() -> NameIndexBuilder {
        NameIndexBuilder::new()
    }

    /// Builds an index directly from a dependency graph.
    pub fn from_graph(graph: &DependencyGraph) -> Self {
        let builder = NameIndexBuilder::with_capacity(graph.node_count());

        for node_idx in graph.graph().node_indices() {
            if let Some(node) = graph.node(node_idx) {
                builder.insert_graph_node(node_idx, node);
            }
        }

        builder.finish()
    }

    /// Returns aggregate statistics about the index.
    pub fn stats(&self) -> NameIndexStats {
        self.stats
    }

    /// Returns the total number of entities stored in the index.
    pub fn len(&self) -> usize {
        self.stats.total_entities
    }

    /// Returns `true` when the index does not contain any entries.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Performs an exact match lookup, optionally filtering by [`NodeKind`].
    pub fn exact_match(&self, query: &str, kind: Option<NodeKind>, limit: usize) -> Vec<NameEntry> {
        if limit == 0 {
            return Vec::new();
        }

        let normalized = normalize_key(query);
        let Some(entries) = self.lookup.get(normalized.as_str()) else {
            return Vec::new();
        };

        collect_filtered(entries.as_ref(), kind, limit)
    }

    /// Performs a prefix lookup with optional [`NodeKind`] filtering.
    pub fn prefix_match(
        &self,
        prefix: &str,
        kind: Option<NodeKind>,
        limit: usize,
    ) -> Vec<NameEntry> {
        if limit == 0 || self.sorted_keys.is_empty() {
            return Vec::new();
        }

        let normalized_prefix = normalize_key(prefix);
        let start = lower_bound(&self.sorted_keys, normalized_prefix.as_str());

        let mut results = Vec::new();
        for key in self.sorted_keys.iter().skip(start) {
            if !key.starts_with(normalized_prefix.as_str()) {
                break;
            }

            if let Some(entries) = self.lookup.get(key.as_ref()) {
                append_filtered(entries.as_ref(), kind, limit, &mut results);
            }

            if results.len() >= limit {
                break;
            }
        }

        results.truncate(limit);
        results
    }

    /// Returns the immutable entry slice for the given (raw) name if present.
    pub fn entries_for(&self, name: &str) -> Option<&[NameEntry]> {
        let normalized = normalize_key(name);
        self.lookup
            .get(normalized.as_str())
            .map(|entries| entries.as_ref())
    }
}

/// Concurrent-friendly builder that feeds the immutable [`NameIndex`].
pub struct NameIndexBuilder {
    entries: DashMap<String, Vec<PendingEntry>>,
}

impl NameIndexBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self {
            entries: DashMap::new(),
        }
    }

    /// Creates a builder with a pre-allocated number of buckets.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: DashMap::with_capacity(capacity),
        }
    }

    /// Inserts a graph node into the builder.
    pub fn insert_graph_node(&self, node_id: GraphNodeIndex, node: &GraphNode) {
        if node.display_name.is_empty() {
            return;
        }

        self.insert(
            node.display_name.clone(),
            node_id,
            node.kind,
            Some(node.id.clone()),
        );
    }

    /// Inserts a standalone entry into the builder.
    pub fn insert(
        &self,
        name: impl Into<String>,
        node_id: GraphNodeIndex,
        kind: NodeKind,
        qualified_name: Option<String>,
    ) {
        let display_name = name.into();
        if display_name.trim().is_empty() {
            return;
        }

        let normalized = normalize_key(&display_name);
        let entry = PendingEntry {
            node_id,
            kind,
            name: to_arc_str(display_name),
            qualified_name: qualified_name.map(to_arc_str),
        };

        self.entries.entry(normalized).or_default().push(entry);
    }

    /// Finalises the builder and produces an immutable [`NameIndex`].
    pub fn finish(self) -> NameIndex {
        let mut pairs: Vec<(Arc<str>, Vec<NameEntry>)> = Vec::with_capacity(self.entries.len());
        let mut total_entities = 0usize;

        for (normalized, pending) in self.entries.into_iter() {
            let mut values: Vec<NameEntry> = pending
                .into_iter()
                .map(|entry| NameEntry {
                    name: entry.name,
                    qualified_name: entry.qualified_name,
                    node_id: entry.node_id,
                    kind: entry.kind,
                })
                .collect();

            values.sort_by(|a, b| {
                a.name
                    .cmp(&b.name)
                    .then_with(|| a.node_id.index().cmp(&b.node_id.index()))
            });
            values.dedup_by_key(|entry| entry.node_id);

            total_entities += values.len();
            pairs.push((to_arc_str(normalized), values));
        }

        pairs.sort_by(|a, b| a.0.as_ref().cmp(b.0.as_ref()));

        let mut lookup = HashMap::with_capacity(pairs.len());
        let mut sorted_keys = Vec::with_capacity(pairs.len());

        for (key, values) in pairs.into_iter() {
            sorted_keys.push(key.clone());
            let storage = Arc::<[NameEntry]>::from(values.into_boxed_slice());
            lookup.insert(key, storage);
        }

        let unique_names = sorted_keys.len();

        NameIndex {
            lookup,
            sorted_keys,
            stats: NameIndexStats {
                unique_names,
                total_entities,
            },
        }
    }
}

impl Default for NameIndexBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
struct PendingEntry {
    node_id: GraphNodeIndex,
    kind: NodeKind,
    name: Arc<str>,
    qualified_name: Option<Arc<str>>,
}

fn normalize_key(value: &str) -> String {
    value.trim().to_lowercase()
}

fn to_arc_str(value: String) -> Arc<str> {
    Arc::from(value.into_boxed_str())
}

fn collect_filtered(entries: &[NameEntry], kind: Option<NodeKind>, limit: usize) -> Vec<NameEntry> {
    let mut results = Vec::new();
    append_filtered(entries, kind, limit, &mut results);
    results
}

fn append_filtered(
    entries: &[NameEntry],
    kind: Option<NodeKind>,
    limit: usize,
    results: &mut Vec<NameEntry>,
) {
    for entry in entries {
        if results.len() >= limit {
            break;
        }

        if kind.map_or(true, |expected| entry.kind == expected) {
            results.push(entry.clone());
        }
    }
}

fn lower_bound(keys: &[Arc<str>], target: &str) -> usize {
    let mut low = 0usize;
    let mut high = keys.len();

    while low < high {
        let mid = (low + high) / 2;
        if keys[mid].as_ref() < target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    low
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{DependencyGraph, GraphNode, NodeKind};
    use std::path::PathBuf;

    #[test]
    fn normalize_lowercases_and_trims() {
        assert_eq!(normalize_key("  AuthService  "), "authservice");
    }

    #[test]
    fn lower_bound_locates_first_not_less_than_target() {
        let keys = ["alpha", "beta", "gamma"]
            .into_iter()
            .map(|s| to_arc_str(s.to_string()))
            .collect::<Vec<_>>();
        assert_eq!(lower_bound(&keys, "alpha"), 0);
        assert_eq!(lower_bound(&keys, "beta"), 1);
        assert_eq!(lower_bound(&keys, "delta"), 2);
        assert_eq!(lower_bound(&keys, "omega"), 3);
    }

    #[test]
    fn builder_ignores_empty_names_and_deduplicates() {
        let builder = NameIndexBuilder::new();
        let idx = GraphNodeIndex::new(1);

        builder.insert("", idx, NodeKind::File, None);
        builder.insert("  ", idx, NodeKind::File, None);
        builder.insert("Service", idx, NodeKind::File, None);
        builder.insert("Service", idx, NodeKind::File, None);

        let index = builder.finish();
        assert_eq!(index.stats().unique_names, 1);
        assert_eq!(index.stats().total_entities, 1);
    }

    #[test]
    fn exact_match_is_case_insensitive() {
        let builder = NameIndexBuilder::new();
        let idx_a = GraphNodeIndex::new(1);
        let idx_b = GraphNodeIndex::new(2);

        builder.insert(
            "AuthService",
            idx_a,
            NodeKind::Class,
            Some("pkg.AuthService".into()),
        );
        builder.insert(
            "authservice",
            idx_b,
            NodeKind::Function,
            Some("pkg.authservice".into()),
        );

        let index = builder.finish();
        let results = index.exact_match("AUTHservice", None, 10);
        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|entry| entry.node_id == idx_a));
        assert!(results.iter().any(|entry| entry.node_id == idx_b));
    }

    #[test]
    fn prefix_match_respects_limit_and_kind_filter() {
        let builder = NameIndexBuilder::new();
        let dir = GraphNodeIndex::new(1);
        let file = GraphNodeIndex::new(2);
        let func = GraphNodeIndex::new(3);

        builder.insert("api", dir, NodeKind::Directory, Some("api".into()));
        builder.insert(
            "api_router",
            file,
            NodeKind::File,
            Some("api/router.py".into()),
        );
        builder.insert(
            "api_route",
            func,
            NodeKind::Function,
            Some("api.route".into()),
        );

        let index = builder.finish();

        let filtered = index.prefix_match("api", Some(NodeKind::Function), 5);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].node_id, func);

        let limited = index.prefix_match("api", None, 2);
        assert_eq!(limited.len(), 2);
    }

    #[test]
    fn from_graph_ingests_display_names() {
        let mut graph = DependencyGraph::new();

        let dir = graph.add_node(GraphNode::directory("pkg".into(), "pkg".into(), None));
        let file = graph.add_node(GraphNode::file(
            "pkg::mod".into(),
            "mod.py".into(),
            PathBuf::from("pkg/mod.py"),
        ));
        let empty = graph.add_node(GraphNode::directory("ignored".into(), "".into(), None));

        let index = NameIndex::from_graph(&graph);
        assert_eq!(index.stats().unique_names, 2);
        assert_eq!(index.stats().total_entities, 2);
        assert!(index
            .exact_match("pkg", None, 10)
            .iter()
            .any(|entry| entry.node_id == dir));
        assert!(index
            .exact_match("mod.py", None, 10)
            .iter()
            .any(|entry| entry.node_id == file));
        assert!(index.exact_match("ignored", None, 10).is_empty());
        assert!(graph
            .node(empty)
            .map(|node| node.display_name.is_empty())
            .unwrap());
    }

    #[test]
    fn new_index_is_empty() {
        let index = NameIndex::new();
        assert!(index.is_empty());
        assert_eq!(index.len(), 0);
        assert_eq!(index.stats().unique_names, 0);
        assert!(index.entries_for("anything").is_none());
    }

    #[test]
    fn entries_for_exposes_underlying_entries() {
        let builder = NameIndex::builder();
        let idx = GraphNodeIndex::new(7);
        builder.insert(
            "Handler",
            idx,
            NodeKind::Function,
            Some("pkg::Handler".into()),
        );

        let index = builder.finish();
        let entries = index.entries_for("handler").expect("entries");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].node_id, idx);
        assert_eq!(entries[0].qualified_name.as_deref(), Some("pkg::Handler"));
    }

    #[test]
    fn zero_limit_short_circuits_queries() {
        let builder = NameIndexBuilder::new();
        let idx = GraphNodeIndex::new(3);
        builder.insert("Widget", idx, NodeKind::Class, Some("pkg::Widget".into()));
        let index = builder.finish();

        assert!(index.exact_match("widget", None, 0).is_empty());
        assert!(index.prefix_match("wid", None, 0).is_empty());
    }
}
