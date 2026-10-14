//! # NeuraCode Core
//!
//! The next-generation AI Agent cognitive enhancement system.
//! NeuraCode gives AI agents a "second brain" - deep codebase understanding,
//! predictive context preparation, and continuous learning capabilities.

pub mod cache;
pub mod code_brain;
pub mod collab_reasoning;
pub mod error;
pub mod learn_engine;
pub mod metrics;
pub mod multi_agent;
pub mod multi_modal;
pub mod predict_engine;
pub mod types;
pub mod utils;

pub use code_brain::CodeBrain;
pub use collab_reasoning::CollabReasoning;
pub use error::{NeuraCodeError, Result};
pub use learn_engine::LearnEngine;
pub use multi_agent::MultiAgentSupport;
pub use multi_modal::MultiModalEngine;
pub use predict_engine::PredictEngine;
pub use types::*;

// Re-export types that were moved
pub use types::{
    AgentInstallResult, ContextPackage, ImageUnderstanding, IndexReport, SearchResult, Session,
};

use tracing::info;

/// NeuraCode - The AI Agent Cognitive Enhancement System
///
/// # Philosophy
///
/// "Not fewer tokens for agents, but more understanding"
///
/// Traditional approaches: compress → restrict → may reduce capability
/// NeuraCode approach: enhance → expand → elevate capability boundaries
pub struct NeuraCode {
    /// Code understanding engine
    pub code_brain: CodeBrain,

    /// Predictive context engine
    pub predict_engine: PredictEngine,

    /// Continuous learning engine
    pub learn_engine: LearnEngine,

    /// Multi-modal understanding
    pub multi_modal: MultiModalEngine,

    /// Collaborative reasoning
    pub collab_reasoning: CollabReasoning,

    /// Multi-agent support
    pub multi_agent: MultiAgentSupport,
}

impl NeuraCode {
    /// Create a new NeuraCode instance
    pub async fn new(config: NeuraCodeConfig) -> Result<Self> {
        info!("Initializing NeuraCode v{}", env!("CARGO_PKG_VERSION"));

        let code_brain = CodeBrain::new(&config).await?;
        let predict_engine = PredictEngine::new(&config);
        let learn_engine = LearnEngine::new(&config).await?;
        let multi_modal = MultiModalEngine::new(&config);
        let collab_reasoning = CollabReasoning::new(&config);
        let multi_agent = MultiAgentSupport::new(&config);

        info!("NeuraCode initialized successfully");

        Ok(Self {
            code_brain,
            predict_engine,
            learn_engine,
            multi_modal,
            collab_reasoning,
            multi_agent,
        })
    }

    /// Index a codebase
    pub async fn index_codebase(&self, path: &std::path::Path) -> Result<IndexReport> {
        info!("Indexing codebase at: {}", path.display());
        self.code_brain.index(path).await
    }

    /// Semantic search across the codebase
    pub async fn search(&self, query: &str) -> Result<Vec<SearchResult>> {
        self.code_brain.semantic_search(query).await
    }

    /// Predict context for a task
    pub async fn predict_context(&self, task: &str) -> Result<ContextPackage> {
        self.predict_engine.predict(task).await
    }

    /// Learn from a session
    pub async fn learn(&self, session: Session) -> Result<()> {
        self.learn_engine.learn_from_session(&session).await
    }

    /// Understand an image (diagram, architecture, whiteboard)
    pub async fn understand_image(
        &self,
        image_path: &std::path::Path,
    ) -> Result<ImageUnderstanding> {
        self.multi_modal.understand_image(image_path).await
    }

    /// Install for all supported agents
    pub async fn install_all_agents(&self) -> Result<Vec<AgentInstallResult>> {
        self.multi_agent.install_all().await
    }
}

/// NeuraCode configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NeuraCodeConfig {
    /// Database path
    pub db_path: std::path::PathBuf,

    /// Cache size in MB
    pub cache_size_mb: usize,

    /// Enable predictive features
    pub enable_prediction: bool,

    /// Enable learning features
    pub enable_learning: bool,

    /// Enable multi-modal features
    pub enable_multimodal: bool,

    /// Supported languages
    pub languages: Vec<Language>,

    /// Maximum file size to index (in bytes)
    pub max_file_size: u64,

    /// Ignore patterns
    pub ignore_patterns: Vec<String>,
}

impl Default for NeuraCodeConfig {
    fn default() -> Self {
        Self {
            db_path: dirs::cache_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join("neuracode"),
            cache_size_mb: 512,
            enable_prediction: true,
            enable_learning: true,
            enable_multimodal: true,
            languages: vec![
                Language::Rust,
                Language::JavaScript,
                Language::TypeScript,
                Language::Python,
                Language::Go,
                Language::Java,
                Language::C,
                Language::Cpp,
            ],
            max_file_size: 1024 * 1024, // 1MB
            ignore_patterns: vec![
                "node_modules".to_string(),
                ".git".to_string(),
                "target".to_string(),
                "dist".to_string(),
                "build".to_string(),
                "__pycache__".to_string(),
                ".venv".to_string(),
                // Secrets — never indexed (see docs/privacy.md)
                ".env".to_string(),
                ".env.".to_string(),
                "id_rsa".to_string(),
                "id_ed25519".to_string(),
                ".pem".to_string(),
                ".key".to_string(),
                "credentials.json".to_string(),
                "secrets.yaml".to_string(),
            ],
        }
    }
}

impl NeuraCodeConfig {
    /// Load config from explicit path, `.neuracode/config.toml` in cwd, or default.
    /// Never panics; falls back to Default on any error.
    pub fn load_or_default(explicit: Option<String>) -> Self {
        // 1. Explicit --config path
        if let Some(p) = explicit {
            let pb = std::path::PathBuf::from(shellexpand_or(&p));
            if let Ok(content) = std::fs::read_to_string(&pb) {
                if let Ok(cfg) = toml::from_str::<NeuraCodeConfig>(&content) {
                    return cfg;
                }
            }
            return Self::default();
        }
        // 2. Project-local .neuracode/config.toml (TOML with subset of fields).
        //    Initialized projects get an isolated DB under ./.neuracode/db
        //    so multiple checkouts never share one global database.
        let local = std::path::Path::new(".neuracode/config.toml");
        if local.exists() {
            if let Ok(content) = std::fs::read_to_string(local) {
                // Parse leniently: only override known simple keys
                if let Ok(v) = toml::from_str::<toml::Value>(&content) {
                    let mut cfg = NeuraCodeConfig {
                        db_path: std::path::PathBuf::from(".neuracode/db"),
                        ..Self::default()
                    };
                    if let Some(mb) = v.get("cache_size_mb").and_then(|x| x.as_integer()) {
                        cfg.cache_size_mb = mb.max(0) as usize;
                    }
                    if let Some(mf) = v.get("max_file_size").and_then(|x| x.as_integer()) {
                        cfg.max_file_size = mf.max(0) as u64;
                    }
                    for key in ["enable_prediction", "enable_learning", "enable_multimodal"] {
                        if let Some(b) = v.get(key).and_then(|x| x.as_bool()) {
                            match key {
                                "enable_prediction" => cfg.enable_prediction = b,
                                "enable_learning" => cfg.enable_learning = b,
                                "enable_multimodal" => cfg.enable_multimodal = b,
                                _ => {}
                            }
                        }
                    }
                    return cfg;
                }
            }
        }
        Self::default()
    }
}

fn shellexpand_or(p: &str) -> String {
    if p.starts_with('~') {
        if let Some(home) = dirs::home_dir() {
            return p.replacen('~', &home.to_string_lossy(), 1);
        }
    }
    p.to_string()
}
