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
pub use predict_engine::PredictEngine;
pub use learn_engine::LearnEngine;
pub use multi_modal::MultiModalEngine;
pub use collab_reasoning::CollabReasoning;
pub use multi_agent::MultiAgentSupport;
pub use types::*;
pub use error::{NeuraCodeError, Result};

// Re-export types that were moved
pub use types::{
    AgentInstallResult,
    ContextPackage,
    ImageUnderstanding,
    IndexReport,
    SearchResult,
    Session,
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
    pub async fn understand_image(&self, image_path: &std::path::Path) -> Result<ImageUnderstanding> {
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
            ],
        }
    }
}
