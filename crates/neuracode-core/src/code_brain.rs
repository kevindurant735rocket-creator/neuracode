//! Code Brain - Deep codebase understanding engine
//!
//! This module provides the core code intelligence capabilities:
//! - Code graph construction and management
//! - Semantic search
//! - Impact analysis
//! - Architecture detection
//! - Hotspot identification

use crate::error::{NeuraCodeError, Result};
use crate::types::*;
use crate::NeuraCodeConfig;
use dashmap::DashMap;
use ignore::WalkBuilder;
use parking_lot::RwLock;
use rayon::prelude::*;
use rusqlite::{params, Connection};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::{debug, info, instrument};
use tree_sitter::Parser;
use uuid::Uuid;

/// Code Brain - The core code understanding engine
pub struct CodeBrain {
    /// Configuration
    config: NeuraCodeConfig,

    /// In-memory code graph
    graph: Arc<RwLock<CodeGraph>>,

    /// Node index by file
    file_index: Arc<DashMap<PathBuf, Vec<Uuid>>>,

    /// Node index by name
    name_index: Arc<DashMap<String, Vec<Uuid>>>,

    /// SQLite connection for persistence
    db: Arc<parking_lot::Mutex<Connection>>,
}

/// In-memory code graph
#[derive(Debug, Default)]
pub struct CodeGraph {
    pub nodes: HashMap<Uuid, CodeNode>,
    pub edges: Vec<CodeEdge>,
    pub communities: Vec<Community>,
}

impl CodeBrain {
    /// Create a new CodeBrain instance
    pub async fn new(config: &NeuraCodeConfig) -> Result<Self> {
        info!("Initializing CodeBrain");

        // Ensure database directory exists
        std::fs::create_dir_all(&config.db_path)?;

        // Open database
        let db_path = config.db_path.join("codebase.db");
        let conn = Connection::open(&db_path)?;

        // Initialize schema
        Self::init_database(&conn)?;

        Ok(Self {
            config: config.clone(),
            graph: Arc::new(RwLock::new(CodeGraph::default())),
            file_index: Arc::new(DashMap::new()),
            name_index: Arc::new(DashMap::new()),
            db: Arc::new(parking_lot::Mutex::new(conn)),
        })
    }

    /// Get tree-sitter language for a given Language
    fn get_tree_sitter_language(language: &Language) -> Option<tree_sitter::Language> {
        match language {
            Language::Rust => Some(tree_sitter_rust::language()),
            Language::JavaScript => Some(tree_sitter_javascript::language()),
            Language::TypeScript => Some(tree_sitter_typescript::language_typescript()),
            Language::Python => Some(tree_sitter_python::language()),
            Language::Go => Some(tree_sitter_go::language()),
            Language::Java => Some(tree_sitter_java::language()),
            Language::C => Some(tree_sitter_c::language()),
            Language::Cpp => Some(tree_sitter_cpp::language()),
            _ => None,
        }
    }

    /// Initialize database schema
    fn init_database(conn: &Connection) -> Result<()> {
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS nodes (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                kind TEXT NOT NULL,
                file_path TEXT NOT NULL,
                start_line INTEGER NOT NULL,
                start_column INTEGER NOT NULL,
                end_line INTEGER NOT NULL,
                end_column INTEGER NOT NULL,
                language TEXT NOT NULL,
                signature TEXT,
                documentation TEXT,
                visibility TEXT NOT NULL,
                metadata TEXT,
                embedding BLOB,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            
            CREATE TABLE IF NOT EXISTS edges (
                id TEXT PRIMARY KEY,
                source TEXT NOT NULL,
                target TEXT NOT NULL,
                kind TEXT NOT NULL,
                weight REAL NOT NULL,
                metadata TEXT,
                FOREIGN KEY (source) REFERENCES nodes(id),
                FOREIGN KEY (target) REFERENCES nodes(id)
            );
            
            CREATE TABLE IF NOT EXISTS communities (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                nodes TEXT NOT NULL
            );
            
            CREATE INDEX IF NOT EXISTS idx_nodes_name ON nodes(name);
            CREATE INDEX IF NOT EXISTS idx_nodes_file ON nodes(file_path);
            CREATE INDEX IF NOT EXISTS idx_nodes_language ON nodes(language);
            CREATE INDEX IF NOT EXISTS idx_edges_source ON edges(source);
            CREATE INDEX IF NOT EXISTS idx_edges_target ON edges(target);
            CREATE INDEX IF NOT EXISTS idx_edges_kind ON edges(kind);
            ",
        )?;

        Ok(())
    }

    /// Index a codebase
    #[instrument(skip(self))]
    pub async fn index(&self, path: &Path) -> Result<IndexReport> {
        let start_time = std::time::Instant::now();
        info!("Starting codebase indexing at: {}", path.display());

        // Collect all source files
        let files = self.collect_files(path)?;
        info!("Found {} files to index", files.len());

        // Parse files in parallel
        let nodes = files
            .par_iter()
            .filter_map(|file| self.parse_file(file).ok())
            .flatten()
            .collect::<Vec<_>>();

        info!("Created {} nodes", nodes.len());

        // Build edges
        let edges = self.build_edges(&nodes);
        info!("Created {} edges", edges.len());

        // Update in-memory graph
        {
            let mut graph = self.graph.write();
            for node in &nodes {
                graph.nodes.insert(node.id, node.clone());
            }
            graph.edges = edges.clone();
        }

        // Update indexes
        for node in &nodes {
            self.file_index
                .entry(node.file_path.clone())
                .or_default()
                .push(node.id);

            self.name_index
                .entry(node.name.clone())
                .or_default()
                .push(node.id);
        }

        // Persist to database
        self.persist_nodes(&nodes)?;
        self.persist_edges(&edges)?;

        // Detect communities
        let communities = self.detect_communities();
        {
            let mut graph = self.graph.write();
            graph.communities = communities;
        }

        let duration = start_time.elapsed();
        info!("Indexing completed in {}ms", duration.as_millis());

        Ok(IndexReport {
            files_indexed: files.len(),
            nodes_created: nodes.len(),
            edges_created: edges.len(),
            duration_ms: duration.as_millis() as u64,
            languages: self.detect_languages(&nodes),
        })
    }

    /// Collect all source files in a directory
    fn collect_files(&self, path: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        let walker = WalkBuilder::new(path)
            .hidden(true)
            .git_ignore(true)
            .git_exclude(true)
            .build();

        for entry in walker {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if Language::from_extension(ext).is_some() {
                        // Check file size
                        if let Ok(metadata) = entry.metadata() {
                            if metadata.len() <= self.config.max_file_size {
                                files.push(path.to_path_buf());
                            }
                        }
                    }
                }
            }
        }

        Ok(files)
    }

    /// Parse a single file
    fn parse_file(&self, path: &Path) -> Result<Vec<CodeNode>> {
        let content = std::fs::read_to_string(path)?;
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| NeuraCodeError::Parse("Invalid file extension".to_string()))?;

        let language = Language::from_extension(ext)
            .ok_or_else(|| NeuraCodeError::Parse(format!("Unsupported language: {}", ext)))?;

        let mut nodes = Vec::new();

        // Parse with tree-sitter - create a new parser for each file (thread-safe)
        if let Some(ts_lang) = Self::get_tree_sitter_language(&language) {
            let mut parser = Parser::new();
            if parser.set_language(ts_lang).is_ok() {
                if let Some(tree) = parser.parse(&content, None) {
                    let mut cursor = tree.walk();
                    self.extract_nodes(&mut cursor, &content, path, language, &mut nodes);
                }
            }
        }

        // Always add file node
        nodes.push(CodeNode {
            id: Uuid::new_v4(),
            name: path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string(),
            kind: NodeKind::File,
            file_path: path.to_path_buf(),
            location: SourceLocation {
                start_line: 0,
                start_column: 0,
                end_line: content.lines().count(),
                end_column: 0,
            },
            language,
            signature: None,
            documentation: None,
            visibility: Visibility::Public,
            metadata: NodeMetadata {
                lines_of_code: Some(content.lines().count()),
                ..Default::default()
            },
            embedding: None,
        });

        Ok(nodes)
    }

    /// Extract nodes from AST
    fn extract_nodes(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        content: &str,
        path: &Path,
        language: Language,
        nodes: &mut Vec<CodeNode>,
    ) {
        let node = cursor.node();
        let kind = node.kind();

        // Check if this is a declaration we care about
        let node_kind = self.map_node_kind(kind, language);

        if let Some(nk) = node_kind {
            let name = self.extract_name(cursor, content);
            if let Some(name) = name {
                let start = node.start_position();
                let end = node.end_position();

                nodes.push(CodeNode {
                    id: Uuid::new_v4(),
                    name,
                    kind: nk,
                    file_path: path.to_path_buf(),
                    location: SourceLocation {
                        start_line: start.row,
                        start_column: start.column,
                        end_line: end.row,
                        end_column: end.column,
                    },
                    language,
                    signature: self.extract_signature(cursor, content),
                    documentation: self.extract_documentation(cursor, content),
                    visibility: self.extract_visibility(cursor, content, language),
                    metadata: NodeMetadata {
                        is_async: self.is_async(cursor, content, language),
                        is_static: self.is_static(cursor, content, language),
                        complexity: Some(self.calculate_complexity(node)),
                        lines_of_code: Some(end.row - start.row + 1),
                        ..Default::default()
                    },
                    embedding: None,
                });
            }
        }

        // Recurse into children
        if cursor.goto_first_child() {
            loop {
                self.extract_nodes(cursor, content, path, language, nodes);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }
    }

    /// Map tree-sitter node kind to our NodeKind
    fn map_node_kind(&self, kind: &str, language: Language) -> Option<NodeKind> {
        match language {
            Language::Rust => match kind {
                "function_item" | "function_signature_item" => Some(NodeKind::Function),
                "struct_item" => Some(NodeKind::Struct),
                "enum_item" => Some(NodeKind::Enum),
                "trait_item" => Some(NodeKind::Trait),
                "impl_item" => Some(NodeKind::Class),
                "mod_item" => Some(NodeKind::Module),
                "macro_definition" => Some(NodeKind::Macro),
                "const_item" | "static_item" => Some(NodeKind::Constant),
                "type_item" => Some(NodeKind::Type),
                _ => None,
            },
            Language::JavaScript | Language::TypeScript => match kind {
                "function_declaration" | "function_expression" => Some(NodeKind::Function),
                "method_definition" => Some(NodeKind::Method),
                "class_declaration" | "class_expression" => Some(NodeKind::Class),
                "interface_declaration" => Some(NodeKind::Interface),
                "type_alias_declaration" => Some(NodeKind::Type),
                "enum_declaration" => Some(NodeKind::Enum),
                "module" => Some(NodeKind::Module),
                _ => None,
            },
            Language::Python => match kind {
                "function_definition" => Some(NodeKind::Function),
                "class_definition" => Some(NodeKind::Class),
                "decorated_definition" => Some(NodeKind::Function),
                _ => None,
            },
            Language::Go => match kind {
                "function_declaration" => Some(NodeKind::Function),
                "method_declaration" => Some(NodeKind::Method),
                "type_declaration" => Some(NodeKind::Type),
                "struct_type" => Some(NodeKind::Struct),
                "interface_type" => Some(NodeKind::Interface),
                _ => None,
            },
            Language::Java => match kind {
                "class_declaration" => Some(NodeKind::Class),
                "interface_declaration" => Some(NodeKind::Interface),
                "method_declaration" => Some(NodeKind::Method),
                "enum_declaration" => Some(NodeKind::Enum),
                _ => None,
            },
            _ => None,
        }
    }

    /// Extract name from node
    fn extract_name(&self, cursor: &mut tree_sitter::TreeCursor, content: &str) -> Option<String> {
        let _node = cursor.node();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "identifier" || child.kind() == "type_identifier" {
                    return child
                        .utf8_text(content.as_bytes())
                        .ok()
                        .map(|s| s.to_string());
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }
        None
    }

    /// Extract signature
    fn extract_signature(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        content: &str,
    ) -> Option<String> {
        let node = cursor.node();
        let start = node.start_position();
        let _end = node.end_position();

        let lines: Vec<&str> = content.lines().collect();
        if start.row < lines.len() {
            let line = lines[start.row];
            Some(line.trim().to_string())
        } else {
            None
        }
    }

    /// Extract documentation
    fn extract_documentation(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        content: &str,
    ) -> Option<String> {
        let node = cursor.node();
        let start = node.start_position();

        let lines: Vec<&str> = content.lines().collect();
        let mut docs = Vec::new();

        // Look for comments before the node
        let mut i = start.row;
        while i > 0 {
            i -= 1;
            if i >= lines.len() {
                break;
            }
            let line = lines[i].trim();
            if line.starts_with("///") || line.starts_with("/**") || line.starts_with("*") {
                docs.insert(0, line.to_string());
            } else if !line.is_empty() {
                break;
            }
        }

        if docs.is_empty() {
            None
        } else {
            Some(docs.join("\n"))
        }
    }

    /// Extract visibility
    fn extract_visibility(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        content: &str,
        language: Language,
    ) -> Visibility {
        let _node = cursor.node();

        match language {
            Language::Rust => {
                if cursor.goto_first_child() {
                    loop {
                        let child = cursor.node();
                        if child.kind() == "visibility_modifier" {
                            cursor.goto_parent();
                            return Visibility::Public;
                        }
                        if !cursor.goto_next_sibling() {
                            break;
                        }
                    }
                    cursor.goto_parent();
                }
                Visibility::Private
            }
            Language::Python => {
                let name = self.extract_name(cursor, content).unwrap_or_default();
                if name.starts_with('_') {
                    Visibility::Private
                } else {
                    Visibility::Public
                }
            }
            _ => Visibility::Public,
        }
    }

    /// Check if function is async
    fn is_async(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        content: &str,
        _language: Language,
    ) -> bool {
        let node = cursor.node();
        let text = node.utf8_text(content.as_bytes()).unwrap_or("");
        text.contains("async")
    }

    /// Check if function is static
    fn is_static(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        content: &str,
        _language: Language,
    ) -> bool {
        let node = cursor.node();
        let text = node.utf8_text(content.as_bytes()).unwrap_or("");
        text.contains("static")
    }

    /// Calculate cyclomatic complexity
    fn calculate_complexity(&self, node: tree_sitter::Node) -> u32 {
        let mut complexity = 1;
        let kind = node.kind();

        match kind {
            "if_expression" | "if_statement" | "while_expression" | "while_statement"
            | "for_expression" | "for_statement" | "match_expression" | "match_arm"
            | "catch_clause" | "ternary_expression" | "logical_expression" => {
                complexity += 1;
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            complexity += self.calculate_complexity(child) - 1;
        }

        complexity
    }

    /// Build edges between nodes
    fn build_edges(&self, nodes: &[CodeNode]) -> Vec<CodeEdge> {
        let mut edges = Vec::new();
        let mut node_map: HashMap<String, Vec<&CodeNode>> = HashMap::new();

        // Group nodes by name
        for node in nodes {
            node_map.entry(node.name.clone()).or_default().push(node);
        }

        // Build call edges
        for node in nodes {
            if node.kind == NodeKind::File {
                continue;
            }

            // Find references to other nodes
            for (name, targets) in &node_map {
                if name == &node.name {
                    continue;
                }

                // Check if this node references the target
                if let Ok(content) = std::fs::read_to_string(&node.file_path) {
                    if content.contains(name) {
                        for target in targets {
                            edges.push(CodeEdge {
                                id: Uuid::new_v4(),
                                source: node.id,
                                target: target.id,
                                kind: EdgeKind::References,
                                weight: 1.0,
                                metadata: EdgeMetadata {
                                    confidence: 0.5,
                                    ..Default::default()
                                },
                            });
                        }
                    }
                }
            }
        }

        edges
    }

    /// Detect communities using simple clustering
    fn detect_communities(&self) -> Vec<Community> {
        let graph = self.graph.read();
        let mut communities = Vec::new();

        // Group by directory
        let mut dir_groups: HashMap<String, Vec<Uuid>> = HashMap::new();

        for (id, node) in &graph.nodes {
            if let Some(parent) = node.file_path.parent() {
                let dir = parent.to_string_lossy().to_string();
                dir_groups.entry(dir).or_default().push(*id);
            }
        }

        for (dir, nodes) in dir_groups {
            if nodes.len() > 1 {
                communities.push(Community {
                    id: Uuid::new_v4(),
                    name: dir.clone(),
                    nodes,
                    description: format!("Module in {}", dir),
                });
            }
        }

        communities
    }

    /// Detect languages in the codebase
    fn detect_languages(&self, nodes: &[CodeNode]) -> Vec<Language> {
        let mut languages: HashSet<Language> = HashSet::new();
        for node in nodes {
            languages.insert(node.language);
        }
        languages.into_iter().collect()
    }

    /// Persist nodes to database
    fn persist_nodes(&self, nodes: &[CodeNode]) -> Result<()> {
        let db = self.db.lock();
        let tx = db.unchecked_transaction()?;

        for node in nodes {
            let metadata = serde_json::to_string(&node.metadata)?;
            let embedding = node
                .embedding
                .as_ref()
                .map(bincode::serialize)
                .transpose()
                .ok()
                .flatten();

            tx.execute(
                "INSERT OR REPLACE INTO nodes (
                    id, name, kind, file_path, start_line, start_column,
                    end_line, end_column, language, signature, documentation,
                    visibility, metadata, embedding, created_at, updated_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
                params![
                    node.id.to_string(),
                    node.name,
                    node.kind.name(),
                    node.file_path.to_string_lossy(),
                    node.location.start_line,
                    node.location.start_column,
                    node.location.end_line,
                    node.location.end_column,
                    node.language.name(),
                    node.signature,
                    node.documentation,
                    format!("{:?}", node.visibility),
                    metadata,
                    embedding,
                    chrono::Utc::now().to_rfc3339(),
                    chrono::Utc::now().to_rfc3339(),
                ],
            )?;
        }

        tx.commit()?;
        Ok(())
    }

    /// Persist edges to database
    fn persist_edges(&self, edges: &[CodeEdge]) -> Result<()> {
        let db = self.db.lock();
        let tx = db.unchecked_transaction()?;

        for edge in edges {
            let metadata = serde_json::to_string(&edge.metadata)?;

            tx.execute(
                "INSERT OR REPLACE INTO edges (id, source, target, kind, weight, metadata)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    edge.id.to_string(),
                    edge.source.to_string(),
                    edge.target.to_string(),
                    edge.kind.name(),
                    edge.weight,
                    metadata,
                ],
            )?;
        }

        tx.commit()?;
        Ok(())
    }

    /// Semantic search across the codebase
    #[instrument(skip(self))]
    pub async fn semantic_search(&self, query: &str) -> Result<Vec<SearchResult>> {
        debug!("Searching for: {}", query);

        let graph = self.graph.read();
        let mut results = Vec::new();

        // Simple keyword matching (in production, use embeddings)
        let query_lower = query.to_lowercase();
        let query_terms: Vec<&str> = query_lower.split_whitespace().collect();

        for node in graph.nodes.values() {
            let name_lower = node.name.to_lowercase();
            let mut score = 0.0;

            // Name match
            if name_lower.contains(&query_lower) {
                score += 10.0;
            }

            // Term match
            for term in &query_terms {
                if name_lower.contains(term) {
                    score += 2.0;
                }
            }

            // Signature match
            if let Some(ref sig) = node.signature {
                let sig_lower = sig.to_lowercase();
                for term in &query_terms {
                    if sig_lower.contains(term) {
                        score += 1.0;
                    }
                }
            }

            if score > 0.0 {
                // Get context
                let context = self.get_node_context(node);

                results.push(SearchResult {
                    node: node.clone(),
                    score,
                    context,
                });
            }
        }

        // Sort by score
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results.truncate(20);

        Ok(results)
    }

    /// Get context for a node
    fn get_node_context(&self, node: &CodeNode) -> String {
        if let Ok(content) = std::fs::read_to_string(&node.file_path) {
            let lines: Vec<&str> = content.lines().collect();
            let start = node.location.start_line.saturating_sub(3);
            let end = (node.location.end_line + 3).min(lines.len());

            lines[start..end].join("\n")
        } else {
            String::new()
        }
    }

    /// Analyze impact of changing a node
    pub async fn impact_analysis(&self, node_id: Uuid) -> Result<ImpactReport> {
        let graph = self.graph.read();

        let node = graph
            .nodes
            .get(&node_id)
            .ok_or_else(|| NeuraCodeError::NotFound(format!("Node {} not found", node_id)))?;

        // Find direct dependents
        let direct_dependents: Vec<CodeNode> = graph
            .edges
            .iter()
            .filter(|e| e.target == node_id)
            .filter_map(|e| graph.nodes.get(&e.source))
            .cloned()
            .collect();

        // Find transitive dependents (BFS)
        let mut transitive_dependents = Vec::new();
        let mut visited = HashSet::new();
        let mut queue: Vec<Uuid> = direct_dependents.iter().map(|n| n.id).collect();

        while let Some(current) = queue.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);

            for edge in &graph.edges {
                if edge.target == current && !visited.contains(&edge.source) {
                    if let Some(n) = graph.nodes.get(&edge.source) {
                        transitive_dependents.push(n.clone());
                        queue.push(edge.source);
                    }
                }
            }
        }

        // Find affected tests
        let affected_tests: Vec<CodeNode> = graph
            .nodes
            .values()
            .filter(|n| n.metadata.is_test)
            .filter(|n| {
                graph.edges.iter().any(|e| {
                    e.source == n.id && (e.target == node_id || visited.contains(&e.target))
                })
            })
            .cloned()
            .collect();

        // Calculate risk level
        let risk_level = if !affected_tests.is_empty() && direct_dependents.len() > 5 {
            RiskLevel::Critical
        } else if direct_dependents.len() > 3 {
            RiskLevel::High
        } else if !direct_dependents.is_empty() {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        };

        // Generate recommendations
        let mut recommendations = Vec::new();
        if risk_level == RiskLevel::Critical || risk_level == RiskLevel::High {
            recommendations.push("Consider writing tests before making changes".to_string());
            recommendations.push(format!(
                "Review {} affected test files",
                affected_tests.len()
            ));
        }
        if !transitive_dependents.is_empty() {
            recommendations.push(format!(
                "Be aware of {} transitive dependents",
                transitive_dependents.len()
            ));
        }

        Ok(ImpactReport {
            target: node.clone(),
            direct_dependents,
            transitive_dependents,
            affected_tests,
            risk_level,
            recommendations,
        })
    }

    /// Identify hotspots in the codebase
    pub async fn identify_hotspots(&self) -> Vec<Hotspot> {
        let graph = self.graph.read();
        let mut hotspots = Vec::new();

        // Calculate centrality (simple degree centrality)
        let mut degree: HashMap<Uuid, usize> = HashMap::new();
        for edge in &graph.edges {
            *degree.entry(edge.target).or_default() += 1;
            *degree.entry(edge.source).or_default() += 1;
        }

        // Find high-centrality nodes
        for (id, node) in &graph.nodes {
            let node_degree = degree.get(id).copied().unwrap_or(0);
            if node_degree > 5 {
                let mut reasons = Vec::new();
                reasons.push(format!("High connectivity: {} connections", node_degree));

                if let Some(complexity) = node.metadata.complexity {
                    if complexity > 10 {
                        reasons.push(format!("High complexity: {}", complexity));
                    }
                }

                hotspots.push(Hotspot {
                    node: node.clone(),
                    score: node_degree as f32,
                    reasons,
                });
            }
        }

        hotspots.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        hotspots.truncate(10);
        hotspots
    }

    /// Detect architecture pattern
    pub async fn detect_architecture(&self) -> Option<ArchitectureInfo> {
        let graph = self.graph.read();

        // Simple heuristics for common patterns
        let has_controllers = graph
            .nodes
            .values()
            .any(|n| n.name.to_lowercase().contains("controller"));
        let has_services = graph
            .nodes
            .values()
            .any(|n| n.name.to_lowercase().contains("service"));
        let has_repositories = graph.nodes.values().any(|n| {
            n.name.to_lowercase().contains("repository") || n.name.to_lowercase().contains("repo")
        });
        let has_models = graph.nodes.values().any(|n| {
            n.name.to_lowercase().contains("model") || n.name.to_lowercase().contains("entity")
        });

        let pattern = if has_controllers && has_services && has_repositories {
            "MVC / Layered Architecture"
        } else if has_services && has_models {
            "Service-Oriented Architecture"
        } else {
            "Modular Architecture"
        };

        Some(ArchitectureInfo {
            pattern: pattern.to_string(),
            layers: vec![],
            components: vec![],
            data_flow: vec![],
        })
    }
}
