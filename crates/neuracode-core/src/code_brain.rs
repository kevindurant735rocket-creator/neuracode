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

        let brain = Self {
            config: config.clone(),
            graph: Arc::new(RwLock::new(CodeGraph::default())),
            file_index: Arc::new(DashMap::new()),
            name_index: Arc::new(DashMap::new()),
            db: Arc::new(parking_lot::Mutex::new(conn)),
        };

        // Load existing data from database
        brain.load_from_database()?;

        Ok(brain)
    }

    /// Load existing data from database
    fn load_from_database(&self) -> Result<()> {
        let db = self.db.lock();

        // Load nodes
        let mut stmt = db.prepare(
            "SELECT id, name, kind, file_path, start_line, start_column, end_line, end_column,
             language, signature, documentation, visibility, metadata FROM nodes",
        )?;

        let node_iter = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let kind: String = row.get(2)?;
            let file_path: String = row.get(3)?;
            let start_line: i64 = row.get(4)?;
            let start_column: i64 = row.get(5)?;
            let end_line: i64 = row.get(6)?;
            let end_column: i64 = row.get(7)?;
            let language: String = row.get(8)?;
            let signature: Option<String> = row.get(9)?;
            let documentation: Option<String> = row.get(10)?;
            let visibility: String = row.get(11)?;
            let metadata: Option<String> = row.get(12)?;

            Ok((
                id,
                name,
                kind,
                file_path,
                start_line,
                start_column,
                end_line,
                end_column,
                language,
                signature,
                documentation,
                visibility,
                metadata,
            ))
        })?;

        let mut nodes = Vec::new();
        for node in node_iter {
            let (
                id,
                name,
                kind,
                file_path,
                start_line,
                start_column,
                end_line,
                end_column,
                language,
                signature,
                documentation,
                visibility,
                metadata_str,
            ) = node?;

            let kind = NodeKind::from_name(&kind).unwrap_or(NodeKind::File);
            let language = Language::from_name(&language);
            let visibility = match visibility.as_str() {
                "Public" => Visibility::Public,
                "Private" => Visibility::Private,
                "Protected" => Visibility::Protected,
                _ => Visibility::Public,
            };

            let metadata: NodeMetadata = metadata_str
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default();

            let uid = match Uuid::parse_str(&id) {
                Ok(u) => u,
                Err(_) => {
                    tracing::warn!("Skipping node with invalid UUID: {}", id);
                    continue;
                }
            };
            let node = CodeNode {
                id: uid,
                name,
                kind,
                file_path: Self::normalize_path(Path::new(&file_path)),
                location: SourceLocation {
                    start_line: start_line as usize,
                    start_column: start_column as usize,
                    end_line: end_line as usize,
                    end_column: end_column as usize,
                },
                language,
                signature,
                documentation,
                visibility,
                metadata,
                embedding: None,
            };

            nodes.push(node);
        }

        // Load edges
        let mut stmt =
            db.prepare("SELECT id, source, target, kind, weight, metadata FROM edges")?;

        let edge_iter = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let source: String = row.get(1)?;
            let target: String = row.get(2)?;
            let kind: String = row.get(3)?;
            let weight: f64 = row.get(4)?;
            let metadata: Option<String> = row.get(5)?;

            Ok((id, source, target, kind, weight, metadata))
        })?;

        let mut edges = Vec::new();
        for edge in edge_iter {
            let (id, source, target, kind, weight, metadata_str) = edge?;

            let kind = EdgeKind::from_name(&kind).unwrap_or(EdgeKind::References);
            let metadata: EdgeMetadata = metadata_str
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default();

            let (eid, esrc, etgt) = match (
                Uuid::parse_str(&id),
                Uuid::parse_str(&source),
                Uuid::parse_str(&target),
            ) {
                (Ok(a), Ok(b), Ok(c)) => (a, b, c),
                _ => {
                    tracing::warn!("Skipping edge with invalid UUID: {} -> {}", source, target);
                    continue;
                }
            };
            let edge = CodeEdge {
                id: eid,
                source: esrc,
                target: etgt,
                kind,
                weight: weight as f32,
                metadata,
            };

            edges.push(edge);
        }

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

        info!(
            "Loaded {} nodes and {} edges from database",
            nodes.len(),
            edges.len()
        );

        Ok(())
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
        let raw_nodes = files
            .par_iter()
            .filter_map(|file| self.parse_file(file).ok())
            .flatten()
            .collect::<Vec<_>>();

        // Deduplicate within single batch by (file, name, kind, location)
        let mut seen = HashSet::new();
        let mut nodes = Vec::with_capacity(raw_nodes.len());
        for n in raw_nodes {
            let key = (
                n.file_path.clone(),
                n.name.clone(),
                n.kind,
                n.location.start_line,
                n.location.start_column,
                n.location.end_line,
            );
            if seen.insert(key) {
                nodes.push(n);
            }
        }

        info!("Created {} nodes", nodes.len());

        // Build edges
        let edges = self.build_edges(&nodes);
        info!("Created {} edges", edges.len());

        // Remove stale nodes for re-indexed files (deduplicate by file_path+name+location)
        {
            let mut graph = self.graph.write();
            let reindexed: HashSet<PathBuf> = files.iter().cloned().collect();
            // Collect stale ids
            let stale_ids: Vec<Uuid> = graph
                .nodes
                .values()
                .filter(|n| reindexed.contains(&n.file_path))
                .map(|n| n.id)
                .collect();
            let stale_set: HashSet<Uuid> = stale_ids.iter().cloned().collect();
            for sid in &stale_ids {
                graph.nodes.remove(sid);
            }
            // Drop edges touching stale nodes (will be replaced by fresh build)
            graph
                .edges
                .retain(|e| !stale_set.contains(&e.source) && !stale_set.contains(&e.target));
            for node in &nodes {
                graph.nodes.insert(node.id, node.clone());
            }
            // Merge edges: keep old edges for untouched files + new edges
            graph.edges.extend(edges.clone());
        }

        // Update indexes — rebuild entries for re-indexed files to avoid duplicates
        for f in &files {
            self.file_index.remove(f);
        }
        // Rebuild name index entries for affected names
        let affected_names: HashSet<String> = nodes.iter().map(|n| n.name.clone()).collect();
        for name in &affected_names {
            if let Some(mut entry) = self.name_index.get_mut(name) {
                entry.retain(|id| {
                    // keep ids that still exist in graph
                    self.graph.read().nodes.contains_key(id)
                });
            }
        }
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

        // Persist to database — first purge stale rows for re-indexed files.
        // Normalize paths so `./crates/x` and `crates/x` don't create parallel universes.
        let normalized_files: Vec<PathBuf> =
            files.iter().map(|f| Self::normalize_path(f)).collect();
        self.purge_files_from_db(&normalized_files)?;
        // Store normalized paths in memory graph as well (nodes already carry them via parse_file)
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
                                files.push(Self::normalize_path(path));
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
        let norm_path = Self::normalize_path(path);
        let content =
            std::fs::read_to_string(&norm_path).or_else(|_| std::fs::read_to_string(path))?;
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
                    self.extract_nodes(&mut cursor, &content, &norm_path, language, &mut nodes);
                }
            }
        }

        // Always add file node
        nodes.push(CodeNode {
            id: Uuid::new_v4(),
            name: norm_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string(),
            kind: NodeKind::File,
            file_path: norm_path.clone(),
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

    /// Build edges between nodes — file contents cached once, O(F*N) not O(N*M*IO)
    fn build_edges(&self, nodes: &[CodeNode]) -> Vec<CodeEdge> {
        let mut edges = Vec::new();
        let mut node_map: HashMap<String, Vec<&CodeNode>> = HashMap::new();

        // Group nodes by name (skip very short / file nodes to cut noise)
        for node in nodes {
            if node.kind == NodeKind::File || node.name.len() < 3 {
                continue;
            }
            node_map.entry(node.name.clone()).or_default().push(node);
        }
        if node_map.is_empty() {
            return edges;
        }

        // Map file -> nodes in that file (only non-file nodes)
        let mut file_nodes: HashMap<PathBuf, Vec<&CodeNode>> = HashMap::new();
        for node in nodes {
            if node.kind == NodeKind::File {
                continue;
            }
            file_nodes
                .entry(node.file_path.clone())
                .or_default()
                .push(node);
        }
        // Cache file contents once
        let mut file_cache: HashMap<PathBuf, String> = HashMap::new();
        for fp in file_nodes.keys() {
            if let Ok(c) = std::fs::read_to_string(fp) {
                file_cache.insert(fp.clone(), c);
            }
        }

        // For each file content, check which names appear (single scan per file)
        for (fp, content) in &file_cache {
            let nodelist = match file_nodes.get(fp) {
                Some(v) => v,
                None => continue,
            };
            for (name, targets) in &node_map {
                if name.len() < 3 || !content.contains(name.as_str()) {
                    continue;
                }
                for src in nodelist {
                    if src.name == *name {
                        continue;
                    }
                    for target in targets.iter().take(3) {
                        edges.push(CodeEdge {
                            id: Uuid::new_v4(),
                            source: src.id,
                            target: target.id,
                            kind: EdgeKind::References,
                            weight: 1.0,
                            metadata: EdgeMetadata {
                                confidence: 0.5,
                                ..Default::default()
                            },
                        });
                        if edges.len() >= 10000 {
                            return edges;
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

    /// Normalize a file path for stable identity: prefer absolute canonical path,
    /// fallback to `./`-stripped relative.
    fn normalize_path(p: &Path) -> PathBuf {
        if let Ok(canon) = std::fs::canonicalize(p) {
            return canon;
        }
        let s = p.to_string_lossy();
        let stripped = s.strip_prefix("./").unwrap_or(&s);
        PathBuf::from(stripped)
    }

    /// Persist nodes to database
    fn purge_files_from_db(&self, files: &[PathBuf]) -> Result<()> {
        if files.is_empty() {
            return Ok(());
        }
        let db = self.db.lock();
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        // Delete nodes for these files; match absolute + relative + `./`-prefixed legacy rows.
        // NOTE: edges must go first — schema has FK edges->nodes, deleting nodes first
        // fails with FOREIGN KEY constraint and silently keeps duplicates.
        for f in files {
            let abs = f.to_string_lossy().to_string();
            let mut candidates = vec![abs.clone()];
            if let Ok(rel) = f.strip_prefix(&cwd) {
                let r = rel.to_string_lossy().to_string();
                candidates.push(r.clone());
                candidates.push(format!("./{}", r));
            }
            if let Some(pos) = abs.find("/crates/") {
                candidates.push(abs[pos + 1..].to_string());
                candidates.push(format!("./{}", &abs[pos + 1..]));
            } else if let Some(pos) = abs.find("/tests/") {
                candidates.push(abs[pos + 1..].to_string());
                candidates.push(format!("./{}", &abs[pos + 1..]));
            }
            // Build one IN-list for both deletes
            let placeholders = candidates.iter().map(|_| "?").collect::<Vec<_>>().join(",");
            let sql_edges = format!(
                "DELETE FROM edges WHERE source IN (SELECT id FROM nodes WHERE file_path IN ({})) OR target IN (SELECT id FROM nodes WHERE file_path IN ({}))",
                placeholders, placeholders
            );
            let mut par: Vec<&dyn rusqlite::ToSql> = Vec::new();
            for c in &candidates {
                par.push(c);
            }
            for c in &candidates {
                par.push(c);
            }
            if let Err(e) = db.execute(&sql_edges, par.as_slice()) {
                tracing::warn!("purge edges failed: {}", e);
            }
            let sql_nodes = format!("DELETE FROM nodes WHERE file_path IN ({})", placeholders);
            let par2: Vec<&dyn rusqlite::ToSql> = candidates
                .iter()
                .map(|c| c as &dyn rusqlite::ToSql)
                .collect();
            if let Err(e) = db.execute(&sql_nodes, par2.as_slice()) {
                tracing::warn!("purge nodes failed: {}", e);
            }
        }
        // Remove orphan edges whose endpoints no longer exist
        let _ = db.execute(
            "DELETE FROM edges WHERE source NOT IN (SELECT id FROM nodes) OR target NOT IN (SELECT id FROM nodes)",
            [],
        );
        Ok(())
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

    /// Semantic search across the codebase — scores under lock, reads files after unlock
    #[instrument(skip(self))]
    pub async fn semantic_search(&self, query: &str) -> Result<Vec<SearchResult>> {
        debug!("Searching for: {}", query);

        // Phase 1: score under short read lock (no IO)
        let scored: Vec<(CodeNode, f32)> = {
            let graph = self.graph.read();
            let mut scored = Vec::new();

            // Enhanced query processing
            let query_lower = query.to_lowercase();
            let query_terms: Vec<String> = query_lower
                .split_whitespace()
                .map(|s| s.to_string())
                .filter(|s| s.len() > 1) // Filter out single chars
                .collect();

            // Remove common stop words
            let stop_words: HashSet<&str> = [
                "the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for", "of", "with",
                "by", "from", "as", "is", "was", "are", "were", "be", "been", "being", "have",
                "has", "had", "do", "does", "did", "will", "would", "could", "should", "may",
                "might", "must", "can", "this", "that", "these", "those", "i", "you", "he", "she",
                "it", "we", "they",
            ]
            .iter()
            .cloned()
            .collect();

            let filtered_terms: Vec<String> = query_terms
                .iter()
                .filter(|t| !stop_words.contains(t.as_str()))
                .cloned()
                .collect();

            for node in graph.nodes.values() {
                let name_lower = node.name.to_lowercase();
                let mut score = 0.0;
                let mut matched_terms = 0;

                // Exact phrase match (highest priority)
                if name_lower.contains(&query_lower) {
                    score += 20.0;
                    matched_terms = filtered_terms.len();
                }

                // Term matching with fuzzy logic
                for term in &filtered_terms {
                    // Exact term match in name
                    if name_lower.contains(term.as_str()) {
                        score += 5.0;
                        matched_terms += 1;
                    }
                    // Prefix match
                    else if name_lower.starts_with(term.as_str()) {
                        score += 3.0;
                        matched_terms += 1;
                    }
                    // Suffix match
                    else if name_lower.ends_with(term.as_str()) {
                        score += 2.0;
                        matched_terms += 1;
                    }
                    // Stem match (authentication <-> authenticate_user): strip common suffixes
                    else if Self::stem_contains(&name_lower, term.as_str()) {
                        score += 2.5;
                        matched_terms += 1;
                    }
                    // Fuzzy match (Levenshtein-like)
                    else if self.fuzzy_match(&name_lower, term.as_str(), 2) {
                        score += 1.5;
                        matched_terms += 1;
                    }
                }

                // Signature match
                if let Some(ref sig) = node.signature {
                    let sig_lower = sig.to_lowercase();
                    for term in &filtered_terms {
                        if sig_lower.contains(term.as_str()) {
                            score += 2.0;
                        }
                    }
                }

                // Documentation match
                if let Some(ref doc) = node.documentation {
                    let doc_lower = doc.to_lowercase();
                    for term in &filtered_terms {
                        if doc_lower.contains(term.as_str()) {
                            score += 1.0;
                        }
                    }
                }

                // Kind-based boosting
                match node.kind {
                    NodeKind::Function | NodeKind::Method => score *= 1.2,
                    NodeKind::Class | NodeKind::Struct => score *= 1.1,
                    _ => {}
                }

                // Visibility boosting (public APIs more important)
                if node.visibility == Visibility::Public {
                    score *= 1.1;
                }

                // Complexity-based boosting (complex functions more important)
                if let Some(complexity) = node.metadata.complexity {
                    if complexity > 5 {
                        score *= 1.0 + (complexity as f32 * 0.01);
                    }
                }

                // Coverage bonus (matched terms / total terms)
                if !filtered_terms.is_empty() {
                    let coverage = matched_terms as f32 / filtered_terms.len() as f32;
                    score *= 1.0 + coverage;
                }

                if score > 0.0 {
                    scored.push((node.clone(), score));
                }
            }
            scored
        };

        // Phase 2: sort + truncate before IO, then attach context without holding lock
        let mut scored = scored;
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(20);
        let mut results = Vec::with_capacity(scored.len());
        for (node, score) in scored {
            let context = self.get_node_context(&node);
            results.push(SearchResult {
                node,
                score,
                context,
            });
        }

        Ok(results)
    }

    /// Fuzzy string matching with max edit distance (char-based, UTF-8 safe)
    fn stem_of(s: &str) -> String {
        let mut stem = s.to_string();
        for suffix in ["ication", "ation", "tion", "ion", "ing", "ed", "es", "s"] {
            if stem.len() > suffix.len() + 3 && stem.ends_with(suffix) {
                stem.truncate(stem.len() - suffix.len());
                break;
            }
        }
        // Also split snake_case and check each token's stem
        stem
    }

    fn stem_contains(name_lower: &str, term: &str) -> bool {
        if term.len() < 5 || name_lower.len() < 5 {
            return false;
        }
        let term_stem = Self::stem_of(term);
        if term_stem.len() < 4 {
            return false;
        }
        if name_lower.contains(&term_stem) {
            return true;
        }
        // Token-wise: authenticate_user -> [authenticate, user], authentication -> authenticat
        for tok in name_lower.split('_') {
            if tok.contains(&term_stem) || term.contains(tok) && tok.len() >= 4 {
                return true;
            }
            let tok_stem = Self::stem_of(tok);
            if tok_stem.len() >= 4
                && (term_stem.starts_with(&tok_stem) || tok_stem.starts_with(&term_stem))
            {
                // Require meaningful overlap (>=6 chars or 70% of shorter)
                let overlap = term_stem
                    .chars()
                    .zip(tok_stem.chars())
                    .take_while(|(a, b)| a == b)
                    .count();
                if overlap >= 6 || overlap * 10 >= term_stem.len() * 7.min(tok_stem.len()) {
                    return true;
                }
            }
        }
        false
    }

    /// Fuzzy string matching with max edit distance (char-based, UTF-8 safe)
    fn fuzzy_match(&self, text: &str, pattern: &str, max_distance: usize) -> bool {
        if pattern.chars().count() < 3 || !text.is_ascii() || !pattern.is_ascii() {
            // Fall back to simple contains for non-ASCII to avoid byte slicing issues
            if !text.is_ascii() || !pattern.is_ascii() {
                return false;
            }
            if pattern.len() < 3 {
                return false;
            }
        }

        // Simple sliding window fuzzy match
        let text_bytes = text.as_bytes();
        let pattern_bytes = pattern.as_bytes();

        if text_bytes.len() < pattern_bytes.len() {
            return false;
        }

        for i in 0..=text_bytes.len() - pattern_bytes.len() {
            let mut distance = 0;
            for j in 0..pattern_bytes.len() {
                if text_bytes[i + j] != pattern_bytes[j] {
                    distance += 1;
                    if distance > max_distance {
                        break;
                    }
                }
            }
            if distance <= max_distance {
                return true;
            }
        }

        false
    }

    /// Get context for a node — bounds-checked, never panics on stale locations
    fn get_node_context(&self, node: &CodeNode) -> String {
        if let Ok(content) = std::fs::read_to_string(&node.file_path) {
            let lines: Vec<&str> = content.lines().collect();
            if lines.is_empty() {
                return String::new();
            }
            let start = node.location.start_line.saturating_sub(3).min(lines.len());
            let end = (node.location.end_line + 3).min(lines.len());
            if start >= end || start >= lines.len() {
                return String::new();
            }

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

        // Build reverse adjacency once: target -> sources (O(E))
        let mut rev_adj: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
        for e in &graph.edges {
            rev_adj.entry(e.target).or_default().push(e.source);
        }
        // Find direct dependents
        let direct_dependents: Vec<CodeNode> = rev_adj
            .get(&node_id)
            .map(|srcs| {
                srcs.iter()
                    .filter_map(|s| graph.nodes.get(s))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();

        // Find transitive dependents (BFS over adjacency)
        let mut transitive_dependents = Vec::new();
        let mut visited = HashSet::new();
        let mut queue: Vec<Uuid> = direct_dependents.iter().map(|n| n.id).collect();

        while let Some(current) = queue.pop() {
            if !visited.insert(current) {
                continue;
            }

            if let Some(sources) = rev_adj.get(&current) {
                for src in sources {
                    if !visited.contains(src) {
                        if let Some(n) = graph.nodes.get(src) {
                            transitive_dependents.push(n.clone());
                            queue.push(*src);
                        }
                    }
                }
            }
        }

        // Find affected tests — index test edges once
        let mut test_sources: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
        for e in &graph.edges {
            test_sources.entry(e.source).or_default().push(e.target);
        }

        // Find affected tests
        let affected_tests: Vec<CodeNode> = graph
            .nodes
            .values()
            .filter(|n| n.metadata.is_test)
            .filter(|n| {
                test_sources
                    .get(&n.id)
                    .map(|tgts| tgts.iter().any(|t| *t == node_id || visited.contains(t)))
                    .unwrap_or(false)
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

        // Calculate multiple centrality metrics
        let mut in_degree: HashMap<Uuid, usize> = HashMap::new();
        let mut out_degree: HashMap<Uuid, usize> = HashMap::new();
        let mut total_degree: HashMap<Uuid, usize> = HashMap::new();

        for edge in &graph.edges {
            *in_degree.entry(edge.target).or_default() += 1;
            *out_degree.entry(edge.source).or_default() += 1;
            *total_degree.entry(edge.target).or_default() += 1;
            *total_degree.entry(edge.source).or_default() += 1;
        }

        // Calculate PageRank-like score (sparse adjacency, O(E) per iteration)
        let damping_factor = 0.85;
        let node_count = graph.nodes.len() as f32;
        // Scale iterations with graph size to bound cost
        let iterations = if node_count > 2000.0 { 5 } else { 10 };

        let mut pagerank: HashMap<Uuid, f32> = graph
            .nodes
            .keys()
            .map(|id| (*id, 1.0 / node_count.max(1.0)))
            .collect();
        // Pre-build incoming adjacency once
        let mut incoming: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
        for edge in &graph.edges {
            incoming.entry(edge.target).or_default().push(edge.source);
        }

        for _ in 0..iterations {
            let mut new_pagerank: HashMap<Uuid, f32> = HashMap::with_capacity(graph.nodes.len());

            for id in graph.nodes.keys() {
                let mut rank = (1.0 - damping_factor) / node_count.max(1.0);

                // Sum contributions from incoming edges (sparse)
                if let Some(sources) = incoming.get(id) {
                    for src in sources {
                        let out_deg = out_degree.get(src).copied().unwrap_or(1).max(1);
                        rank += damping_factor * pagerank.get(src).unwrap_or(&0.0) / out_deg as f32;
                    }
                }

                new_pagerank.insert(*id, rank);
            }

            pagerank = new_pagerank;
        }

        // Find high-centrality nodes
        for (id, node) in &graph.nodes {
            let in_deg = in_degree.get(id).copied().unwrap_or(0);
            let out_deg = out_degree.get(id).copied().unwrap_or(0);
            let total_deg = total_degree.get(id).copied().unwrap_or(0);
            let pr = pagerank.get(id).copied().unwrap_or(0.0);

            // Calculate composite score
            let mut score = 0.0;
            let mut reasons = Vec::new();

            // Degree centrality
            if total_deg > 5 {
                score += total_deg as f32 * 2.0;
                reasons.push(format!("High connectivity: {} connections", total_deg));
            }

            // In-degree centrality (many dependencies)
            if in_deg > 3 {
                score += in_deg as f32 * 3.0;
                reasons.push(format!("High fan-in: {} dependencies", in_deg));
            }

            // Out-degree centrality (depends on many)
            if out_deg > 3 {
                score += out_deg as f32 * 1.5;
                reasons.push(format!("High fan-out: {} dependencies", out_deg));
            }

            // PageRank score
            if pr > 0.01 {
                score += pr * 100.0;
                reasons.push(format!("High importance: PageRank {:.4}", pr));
            }

            // Complexity factor
            if let Some(complexity) = node.metadata.complexity {
                if complexity > 10 {
                    score += complexity as f32 * 2.0;
                    reasons.push(format!("High complexity: {}", complexity));
                } else if complexity > 5 {
                    score += complexity as f32;
                    reasons.push(format!("Medium complexity: {}", complexity));
                }
            }

            // Lines of code factor
            if let Some(loc) = node.metadata.lines_of_code {
                if loc > 100 {
                    score += loc as f32 * 0.1;
                    reasons.push(format!("Large size: {} lines", loc));
                }
            }

            // Kind-based adjustments
            match node.kind {
                NodeKind::Function | NodeKind::Method => {
                    score *= 1.2;
                }
                NodeKind::Class | NodeKind::Struct => {
                    score *= 1.1;
                }
                _ => {}
            }

            if score > 5.0 {
                hotspots.push(Hotspot {
                    node: node.clone(),
                    score,
                    reasons,
                });
            }
        }

        // Sort by score (descending)
        hotspots.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        hotspots.truncate(10);
        hotspots
    }

    /// Graph counts (nodes, edges) — for stats/MCP without cloning graph
    pub fn counts(&self) -> (usize, usize) {
        let g = self.graph.read();
        (g.nodes.len(), g.edges.len())
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
