//! Learn Engine - Continuous learning and knowledge accumulation
//!
//! This module enables NeuraCode to learn from sessions and improve over time.

use crate::error::Result;
use crate::types::*;
use crate::NeuraCodeConfig;
use dashmap::DashMap;
use parking_lot::RwLock;
use rusqlite::{params, Connection};
use std::sync::Arc;
use tracing::{info, instrument};
use uuid::Uuid;

/// Learn Engine - Continuous learning from sessions
#[allow(dead_code)]
pub struct LearnEngine {
    /// Configuration
    config: NeuraCodeConfig,

    /// Learned patterns
    patterns: Arc<DashMap<Uuid, Pattern>>,

    /// User preferences
    preferences: Arc<RwLock<UserPreferences>>,

    /// Code style
    code_style: Arc<RwLock<CodeStyle>>,

    /// Knowledge base
    knowledge: Arc<DashMap<Uuid, Knowledge>>,

    /// SQLite connection
    db: Arc<parking_lot::Mutex<Connection>>,
}

/// User preferences learned over time
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct UserPreferences {
    /// Preferred naming convention
    pub naming_convention: Option<String>,

    /// Preferred code style
    pub code_style: Option<String>,

    /// Preferred test framework
    pub test_framework: Option<String>,

    /// Preferred documentation style
    pub documentation_style: Option<String>,

    /// Common patterns
    pub common_patterns: Vec<String>,

    /// Preferred languages
    pub preferred_languages: Vec<Language>,
}

/// Code style learned from changes
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CodeStyle {
    /// Indentation style
    pub indentation: IndentationStyle,

    /// Line ending style
    pub line_ending: LineEndingStyle,

    /// Brace style
    pub brace_style: BraceStyle,

    /// Maximum line length
    pub max_line_length: usize,

    /// Use semicolons
    pub use_semicolons: Option<bool>,

    /// Use trailing commas
    pub use_trailing_commas: Option<bool>,

    /// Quote style
    pub quote_style: QuoteStyle,
}

/// Indentation style
#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
pub enum IndentationStyle {
    #[default]
    Spaces,
    Tabs,
}

/// Line ending style
#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
pub enum LineEndingStyle {
    #[default]
    LF,
    CRLF,
}

/// Brace style
#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
pub enum BraceStyle {
    #[default]
    SameLine,
    NextLine,
}

/// Quote style
#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
pub enum QuoteStyle {
    #[default]
    Double,
    Single,
}

impl LearnEngine {
    /// Create a new LearnEngine
    pub async fn new(config: &NeuraCodeConfig) -> Result<Self> {
        info!("Initializing LearnEngine");

        // Ensure database directory exists
        std::fs::create_dir_all(&config.db_path)?;

        // Open database
        let db_path = config.db_path.join("learning.db");
        let conn = Connection::open(&db_path)?;

        // Initialize schema
        Self::init_database(&conn)?;

        // Load existing data
        let patterns = Self::load_patterns(&conn)?;
        let knowledge = Self::load_knowledge(&conn)?;

        Ok(Self {
            config: config.clone(),
            patterns: Arc::new(patterns),
            preferences: Arc::new(RwLock::new(UserPreferences::default())),
            code_style: Arc::new(RwLock::new(CodeStyle {
                indentation: IndentationStyle::Spaces,
                line_ending: LineEndingStyle::LF,
                brace_style: BraceStyle::SameLine,
                max_line_length: 100,
                use_semicolons: None,
                use_trailing_commas: None,
                quote_style: QuoteStyle::Double,
            })),
            knowledge: Arc::new(knowledge),
            db: Arc::new(parking_lot::Mutex::new(conn)),
        })
    }

    /// Initialize database schema
    fn init_database(conn: &Connection) -> Result<()> {
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS patterns (
                id TEXT PRIMARY KEY,
                kind TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                frequency INTEGER NOT NULL,
                confidence REAL NOT NULL,
                examples TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            
            CREATE TABLE IF NOT EXISTS knowledge (
                id TEXT PRIMARY KEY,
                kind TEXT NOT NULL,
                content TEXT NOT NULL,
                source TEXT,
                confidence REAL NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                tags TEXT,
                related TEXT
            );
            
            CREATE TABLE IF NOT EXISTS preferences (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            
            CREATE TABLE IF NOT EXISTS code_style (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            
            CREATE INDEX IF NOT EXISTS idx_patterns_kind ON patterns(kind);
            CREATE INDEX IF NOT EXISTS idx_patterns_name ON patterns(name);
            CREATE INDEX IF NOT EXISTS idx_knowledge_kind ON knowledge(kind);
            CREATE INDEX IF NOT EXISTS idx_knowledge_tags ON knowledge(tags);
            ",
        )?;

        Ok(())
    }

    /// Load patterns from database
    fn load_patterns(conn: &Connection) -> Result<DashMap<Uuid, Pattern>> {
        let patterns = DashMap::new();

        let mut stmt = conn.prepare("SELECT * FROM patterns")?;
        let rows = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let examples: String = row.get(6)?;

            Ok(Pattern {
                id: Uuid::parse_str(&id).unwrap_or_else(|_| Uuid::new_v4()),
                kind: Self::parse_pattern_kind(&row.get::<_, String>(1)?),
                name: row.get(2)?,
                description: row.get(3)?,
                frequency: row.get(4)?,
                confidence: row.get(5)?,
                examples: serde_json::from_str(&examples).unwrap_or_default(),
            })
        })?;

        for pattern in rows.flatten() {
            patterns.insert(pattern.id, pattern);
        }

        Ok(patterns)
    }

    /// Load knowledge from database
    fn load_knowledge(conn: &Connection) -> Result<DashMap<Uuid, Knowledge>> {
        let knowledge = DashMap::new();

        let mut stmt = conn.prepare("SELECT * FROM knowledge")?;
        let rows = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let tags: String = row.get(6)?;
            let related: String = row.get(7)?;

            Ok(Knowledge {
                id: Uuid::parse_str(&id).unwrap_or_else(|_| Uuid::new_v4()),
                kind: Self::parse_knowledge_kind(&row.get::<_, String>(1)?),
                content: row.get(2)?,
                source: row.get(3)?,
                confidence: row.get(4)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::Utc::now(),
                tags: serde_json::from_str(&tags).unwrap_or_default(),
                related: serde_json::from_str(&related).unwrap_or_default(),
            })
        })?;

        for k in rows.flatten() {
            knowledge.insert(k.id, k);
        }

        Ok(knowledge)
    }

    /// Parse pattern kind
    fn parse_pattern_kind(s: &str) -> PatternKind {
        match s {
            "naming_convention" => PatternKind::NamingConvention,
            "code_structure" => PatternKind::CodeStructure,
            "error_handling" => PatternKind::ErrorHandling,
            "testing_style" => PatternKind::TestingStyle,
            "documentation_style" => PatternKind::DocumentationStyle,
            "architecture_pattern" => PatternKind::ArchitecturePattern,
            "workflow" => PatternKind::Workflow,
            "preference" => PatternKind::Preference,
            _ => PatternKind::CodeStructure,
        }
    }

    /// Parse knowledge kind
    fn parse_knowledge_kind(s: &str) -> KnowledgeKind {
        match s {
            "code_pattern" => KnowledgeKind::CodePattern,
            "architecture_decision" => KnowledgeKind::ArchitectureDecision,
            "bug_fix" => KnowledgeKind::BugFix,
            "user_preference" => KnowledgeKind::UserPreference,
            "project_context" => KnowledgeKind::ProjectContext,
            "best_practice" => KnowledgeKind::BestPractice,
            "workflow" => KnowledgeKind::Workflow,
            _ => KnowledgeKind::ProjectContext,
        }
    }

    /// Learn from a session
    #[instrument(skip(self))]
    pub async fn learn_from_session(&self, session: &Session) -> Result<()> {
        info!("Learning from session: {}", session.id);

        // Learn from code changes
        self.learn_code_style(&session.code_changes).await?;

        // Learn from interactions
        self.learn_preferences(&session.interactions).await?;

        // Extract patterns
        self.extract_patterns(session).await?;

        // Accumulate knowledge
        self.accumulate_knowledge(session).await?;

        info!("Learning completed for session: {}", session.id);
        Ok(())
    }

    /// Learn code style from changes
    async fn learn_code_style(&self, changes: &[CodeChange]) -> Result<()> {
        // Update style under the lock, then drop it before any await
        let style_snapshot = {
            let mut style = self.code_style.write();

            for change in changes {
                if let Some(ref content) = change.new_content {
                    // Detect indentation
                    for line in content.lines() {
                        if line.starts_with("    ") {
                            style.indentation = IndentationStyle::Spaces;
                        } else if line.starts_with('\t') {
                            style.indentation = IndentationStyle::Tabs;
                        }
                    }

                    // Detect line length
                    for line in content.lines() {
                        if line.len() > style.max_line_length {
                            style.max_line_length = line.len();
                        }
                    }

                    // Detect semicolons
                    if content.contains(';') {
                        style.use_semicolons = Some(true);
                    }

                    // Detect trailing commas
                    if content.contains(",\n") || content.contains(",\r\n") {
                        style.use_trailing_commas = Some(true);
                    }

                    // Detect quote style
                    if content.contains("'") && !content.contains("\"") {
                        style.quote_style = QuoteStyle::Single;
                    } else if content.contains("\"") && !content.contains("'") {
                        style.quote_style = QuoteStyle::Double;
                    }
                }
            }

            style.clone()
        }; // guard dropped here

        // Persist to database
        self.persist_code_style(&style_snapshot).await?;

        Ok(())
    }

    /// Learn preferences from interactions
    async fn learn_preferences(&self, interactions: &[Interaction]) -> Result<()> {
        let mut prefs = self.preferences.write();

        for interaction in interactions {
            match interaction.kind {
                InteractionKind::Query => {
                    // Learn from queries
                    let query = interaction.content.to_lowercase();
                    if query.contains("test") {
                        prefs.common_patterns.push("testing".to_string());
                    }
                    if query.contains("error") || query.contains("bug") {
                        prefs.common_patterns.push("error_handling".to_string());
                    }
                }
                InteractionKind::Edit => {
                    // Learn from edits
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Extract patterns from session
    async fn extract_patterns(&self, session: &Session) -> Result<()> {
        // Extract naming conventions
        let naming_pattern = self.extract_naming_convention(&session.code_changes);
        if let Some(pattern) = naming_pattern {
            self.add_pattern(pattern).await?;
        }

        // Extract error handling patterns
        let error_pattern = self.extract_error_handling_pattern(&session.code_changes);
        if let Some(pattern) = error_pattern {
            self.add_pattern(pattern).await?;
        }

        // Extract testing patterns
        let testing_pattern = self.extract_testing_pattern(&session.code_changes);
        if let Some(pattern) = testing_pattern {
            self.add_pattern(pattern).await?;
        }

        Ok(())
    }

    /// Extract naming convention
    fn extract_naming_convention(&self, changes: &[CodeChange]) -> Option<Pattern> {
        // Simple heuristic: check if names follow camelCase, snake_case, etc.
        let mut camel_case_count = 0;
        let mut snake_case_count = 0;

        for change in changes {
            if let Some(ref content) = change.new_content {
                for word in content.split_whitespace() {
                    if word.contains('_') {
                        snake_case_count += 1;
                    } else if word.chars().any(|c| c.is_uppercase()) {
                        camel_case_count += 1;
                    }
                }
            }
        }

        if camel_case_count > snake_case_count {
            Some(Pattern {
                id: Uuid::new_v4(),
                kind: PatternKind::NamingConvention,
                name: "camelCase".to_string(),
                description: "Uses camelCase for naming".to_string(),
                frequency: camel_case_count as u32,
                confidence: 0.8,
                examples: vec![],
            })
        } else if snake_case_count > 0 {
            Some(Pattern {
                id: Uuid::new_v4(),
                kind: PatternKind::NamingConvention,
                name: "snake_case".to_string(),
                description: "Uses snake_case for naming".to_string(),
                frequency: snake_case_count as u32,
                confidence: 0.8,
                examples: vec![],
            })
        } else {
            None
        }
    }

    /// Extract error handling pattern
    fn extract_error_handling_pattern(&self, changes: &[CodeChange]) -> Option<Pattern> {
        let mut has_result = false;
        let mut has_try_catch = false;

        for change in changes {
            if let Some(ref content) = change.new_content {
                if content.contains("Result<") {
                    has_result = true;
                }
                if content.contains("try") && content.contains("catch") {
                    has_try_catch = true;
                }
            }
        }

        if has_result {
            Some(Pattern {
                id: Uuid::new_v4(),
                kind: PatternKind::ErrorHandling,
                name: "Result Type".to_string(),
                description: "Uses Result<T, E> for error handling".to_string(),
                frequency: 1,
                confidence: 0.9,
                examples: vec![],
            })
        } else if has_try_catch {
            Some(Pattern {
                id: Uuid::new_v4(),
                kind: PatternKind::ErrorHandling,
                name: "Try-Catch".to_string(),
                description: "Uses try-catch for error handling".to_string(),
                frequency: 1,
                confidence: 0.9,
                examples: vec![],
            })
        } else {
            None
        }
    }

    /// Extract testing pattern
    fn extract_testing_pattern(&self, changes: &[CodeChange]) -> Option<Pattern> {
        let mut has_tests = false;

        for change in changes {
            if change.file_path.to_string_lossy().contains("test") {
                has_tests = true;
                break;
            }
        }

        if has_tests {
            Some(Pattern {
                id: Uuid::new_v4(),
                kind: PatternKind::TestingStyle,
                name: "Test Files".to_string(),
                description: "Uses separate test files".to_string(),
                frequency: 1,
                confidence: 0.8,
                examples: vec![],
            })
        } else {
            None
        }
    }

    /// Add a pattern
    async fn add_pattern(&self, pattern: Pattern) -> Result<()> {
        self.patterns.insert(pattern.id, pattern.clone());

        // Persist to database
        let db = self.db.lock();
        let examples = serde_json::to_string(&pattern.examples)?;

        db.execute(
            "INSERT OR REPLACE INTO patterns (
                id, kind, name, description, frequency, confidence, examples, created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                pattern.id.to_string(),
                format!("{:?}", pattern.kind),
                pattern.name,
                pattern.description,
                pattern.frequency,
                pattern.confidence,
                examples,
                chrono::Utc::now().to_rfc3339(),
                chrono::Utc::now().to_rfc3339(),
            ],
        )?;

        Ok(())
    }

    /// Accumulate knowledge from session
    async fn accumulate_knowledge(&self, session: &Session) -> Result<()> {
        // Extract knowledge from interactions
        for interaction in &session.interactions {
            if interaction.kind == InteractionKind::Error {
                // Learn from errors
                let knowledge = Knowledge {
                    id: Uuid::new_v4(),
                    kind: KnowledgeKind::BugFix,
                    content: interaction.content.clone(),
                    source: format!("session:{}", session.id),
                    confidence: 0.7,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                    tags: vec!["error".to_string()],
                    related: vec![],
                };

                self.add_knowledge(knowledge).await?;
            }
        }

        Ok(())
    }

    /// Add knowledge
    async fn add_knowledge(&self, knowledge: Knowledge) -> Result<()> {
        self.knowledge.insert(knowledge.id, knowledge.clone());

        // Persist to database
        let db = self.db.lock();
        let tags = serde_json::to_string(&knowledge.tags)?;
        let related = serde_json::to_string(&knowledge.related)?;

        db.execute(
            "INSERT OR REPLACE INTO knowledge (
                id, kind, content, source, confidence, created_at, updated_at, tags, related
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                knowledge.id.to_string(),
                format!("{:?}", knowledge.kind),
                knowledge.content,
                knowledge.source,
                knowledge.confidence,
                knowledge.created_at.to_rfc3339(),
                knowledge.updated_at.to_rfc3339(),
                tags,
                related,
            ],
        )?;

        Ok(())
    }

    /// Persist code style
    async fn persist_code_style(&self, style: &CodeStyle) -> Result<()> {
        let db = self.db.lock();

        let values = vec![
            ("indentation", format!("{:?}", style.indentation)),
            ("line_ending", format!("{:?}", style.line_ending)),
            ("brace_style", format!("{:?}", style.brace_style)),
            ("max_line_length", style.max_line_length.to_string()),
            (
                "use_semicolons",
                style
                    .use_semicolons
                    .map(|b| b.to_string())
                    .unwrap_or_default(),
            ),
            (
                "use_trailing_commas",
                style
                    .use_trailing_commas
                    .map(|b| b.to_string())
                    .unwrap_or_default(),
            ),
            ("quote_style", format!("{:?}", style.quote_style)),
        ];

        for (key, value) in values {
            db.execute(
                "INSERT OR REPLACE INTO code_style (key, value, updated_at) VALUES (?1, ?2, ?3)",
                params![key, value, chrono::Utc::now().to_rfc3339()],
            )?;
        }

        Ok(())
    }

    /// Get learned patterns
    pub fn get_patterns(&self) -> Vec<Pattern> {
        self.patterns.iter().map(|p| p.value().clone()).collect()
    }

    /// Get code style
    pub fn get_code_style(&self) -> CodeStyle {
        self.code_style.read().clone()
    }

    /// Get knowledge base
    pub fn get_knowledge(&self) -> Vec<Knowledge> {
        self.knowledge.iter().map(|k| k.value().clone()).collect()
    }

    /// Search knowledge
    pub fn search_knowledge(&self, query: &str) -> Vec<Knowledge> {
        let query_lower = query.to_lowercase();

        self.knowledge
            .iter()
            .filter(|k| {
                let content = k.value().content.to_lowercase();
                content.contains(&query_lower)
            })
            .map(|k| k.value().clone())
            .collect()
    }
}
