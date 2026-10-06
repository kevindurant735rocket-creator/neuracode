//! Core types for NeuraCode

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// Supported programming languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum Language {
    Rust,
    JavaScript,
    TypeScript,
    Python,
    Go,
    Java,
    C,
    Cpp,
    CSharp,
    Ruby,
    PHP,
    Swift,
    Kotlin,
    Scala,
    Shell,
    SQL,
    HTML,
    CSS,
    YAML,
    JSON,
    Markdown,
    Other,
}

impl<'de> serde::Deserialize<'de> for Language {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Language::from_name(&s))
    }
}

impl Language {
    pub fn from_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "rust" => Self::Rust,
            "javascript" => Self::JavaScript,
            "typescript" => Self::TypeScript,
            "python" => Self::Python,
            "go" => Self::Go,
            "java" => Self::Java,
            "c" => Self::C,
            "c++" | "cpp" => Self::Cpp,
            "c#" | "csharp" => Self::CSharp,
            "ruby" => Self::Ruby,
            "php" => Self::PHP,
            "swift" => Self::Swift,
            "kotlin" => Self::Kotlin,
            "scala" => Self::Scala,
            "shell" | "bash" => Self::Shell,
            "sql" => Self::SQL,
            "html" => Self::HTML,
            "css" => Self::CSS,
            "yaml" => Self::YAML,
            "json" => Self::JSON,
            "markdown" => Self::Markdown,
            _ => Self::Other,
        }
    }
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            "rs" => Some(Self::Rust),
            "js" | "jsx" | "mjs" | "cjs" => Some(Self::JavaScript),
            "ts" | "tsx" | "mts" | "cts" => Some(Self::TypeScript),
            "py" | "pyw" | "pyi" => Some(Self::Python),
            "go" => Some(Self::Go),
            "java" => Some(Self::Java),
            "c" | "h" => Some(Self::C),
            "cpp" | "cc" | "cxx" | "hpp" | "hxx" | "cu" => Some(Self::Cpp),
            "cs" => Some(Self::CSharp),
            "rb" => Some(Self::Ruby),
            "php" => Some(Self::PHP),
            "swift" => Some(Self::Swift),
            "kt" | "kts" => Some(Self::Kotlin),
            "scala" | "sc" => Some(Self::Scala),
            "sh" | "bash" | "zsh" | "fish" => Some(Self::Shell),
            "sql" => Some(Self::SQL),
            "html" | "htm" | "vue" | "svelte" => Some(Self::HTML),
            "css" | "scss" | "sass" | "less" | "styl" => Some(Self::CSS),
            "yaml" | "yml" => Some(Self::YAML),
            "json" | "jsonc" | "json5" => Some(Self::JSON),
            "md" | "markdown" | "mdx" => Some(Self::Markdown),
            "xml" | "svg" => Some(Self::Other),
            "toml" => Some(Self::Other),
            "ini" | "cfg" | "conf" => Some(Self::Other),
            "dockerfile" => Some(Self::Other),
            "makefile" | "mk" => Some(Self::Other),
            "proto" => Some(Self::Other),
            "graphql" | "gql" => Some(Self::Other),
            "tf" => Some(Self::Other),
            "lua" => Some(Self::Other),
            "r" => Some(Self::Other),
            "jl" => Some(Self::Other),
            "ex" | "exs" => Some(Self::Other),
            "erl" | "hrl" => Some(Self::Other),
            "hs" => Some(Self::Other),
            "ml" | "mli" => Some(Self::Other),
            "clj" | "cljs" => Some(Self::Other),
            "dart" => Some(Self::Other),
            "zig" => Some(Self::Other),
            "nim" => Some(Self::Other),
            "v" => Some(Self::Other),
            "cr" => Some(Self::Other),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Rust => "Rust",
            Self::JavaScript => "JavaScript",
            Self::TypeScript => "TypeScript",
            Self::Python => "Python",
            Self::Go => "Go",
            Self::Java => "Java",
            Self::C => "C",
            Self::Cpp => "C++",
            Self::CSharp => "C#",
            Self::Ruby => "Ruby",
            Self::PHP => "PHP",
            Self::Swift => "Swift",
            Self::Kotlin => "Kotlin",
            Self::Scala => "Scala",
            Self::Shell => "Shell",
            Self::SQL => "SQL",
            Self::HTML => "HTML",
            Self::CSS => "CSS",
            Self::YAML => "YAML",
            Self::JSON => "JSON",
            Self::Markdown => "Markdown",
            Self::Other => "Other",
        }
    }

    /// Get file extensions for this language
    pub fn extensions(&self) -> &'static [&'static str] {
        match self {
            Self::Rust => &["rs"],
            Self::JavaScript => &["js", "jsx", "mjs", "cjs"],
            Self::TypeScript => &["ts", "tsx", "mts", "cts"],
            Self::Python => &["py", "pyw", "pyi"],
            Self::Go => &["go"],
            Self::Java => &["java"],
            Self::C => &["c", "h"],
            Self::Cpp => &["cpp", "cc", "cxx", "hpp", "hxx", "cu"],
            Self::CSharp => &["cs"],
            Self::Ruby => &["rb"],
            Self::PHP => &["php"],
            Self::Swift => &["swift"],
            Self::Kotlin => &["kt", "kts"],
            Self::Scala => &["scala", "sc"],
            Self::Shell => &["sh", "bash", "zsh", "fish"],
            Self::SQL => &["sql"],
            Self::HTML => &["html", "htm", "vue", "svelte"],
            Self::CSS => &["css", "scss", "sass", "less", "styl"],
            Self::YAML => &["yaml", "yml"],
            Self::JSON => &["json", "jsonc", "json5"],
            Self::Markdown => &["md", "markdown", "mdx"],
            Self::Other => &[],
        }
    }

    /// Check if this is a programming language (not markup/config)
    pub fn is_programming_language(&self) -> bool {
        !matches!(
            self,
            Self::HTML | Self::CSS | Self::YAML | Self::JSON | Self::Markdown | Self::Other
        )
    }
}

/// Node kinds in the code graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeKind {
    Function,
    Method,
    Class,
    Struct,
    Enum,
    Trait,
    Interface,
    Module,
    File,
    Variable,
    Constant,
    Type,
    Macro,
    Namespace,
    Package,
}

impl NodeKind {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Function => "function",
            Self::Method => "method",
            Self::Class => "class",
            Self::Struct => "struct",
            Self::Enum => "enum",
            Self::Trait => "trait",
            Self::Interface => "interface",
            Self::Module => "module",
            Self::File => "file",
            Self::Variable => "variable",
            Self::Constant => "constant",
            Self::Type => "type",
            Self::Macro => "macro",
            Self::Namespace => "namespace",
            Self::Package => "package",
        }
    }
}

/// A node in the code graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeNode {
    pub id: Uuid,
    pub name: String,
    pub kind: NodeKind,
    pub file_path: PathBuf,
    pub location: SourceLocation,
    pub language: Language,
    pub signature: Option<String>,
    pub documentation: Option<String>,
    pub visibility: Visibility,
    pub metadata: NodeMetadata,
    pub embedding: Option<Vec<f32>>,
}

/// Source location
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SourceLocation {
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

/// Visibility
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Internal,
    Package,
}

/// Node metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NodeMetadata {
    pub is_async: bool,
    pub is_static: bool,
    pub is_abstract: bool,
    pub is_test: bool,
    pub is_deprecated: bool,
    pub complexity: Option<u32>,
    pub lines_of_code: Option<usize>,
    pub tags: Vec<String>,
}

/// Edge kinds in the code graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EdgeKind {
    Calls,
    Imports,
    Inherits,
    Implements,
    Contains,
    References,
    DependsOn,
    Tests,
    Documents,
    Configures,
}

impl EdgeKind {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Calls => "calls",
            Self::Imports => "imports",
            Self::Inherits => "inherits",
            Self::Implements => "implements",
            Self::Contains => "contains",
            Self::References => "references",
            Self::DependsOn => "depends_on",
            Self::Tests => "tests",
            Self::Documents => "documents",
            Self::Configures => "configures",
        }
    }
}

/// An edge in the code graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeEdge {
    pub id: Uuid,
    pub source: Uuid,
    pub target: Uuid,
    pub kind: EdgeKind,
    pub weight: f32,
    pub metadata: EdgeMetadata,
}

/// Edge metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EdgeMetadata {
    pub is_conditional: bool,
    pub is_dynamic: bool,
    pub call_count: Option<u32>,
    pub confidence: f32,
}

/// Task types for prediction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TaskType {
    BugFix,
    Refactor,
    NewFeature,
    CodeReview,
    PerformanceOptimization,
    SecurityFix,
    Documentation,
    Testing,
    Architecture,
    General,
}

impl TaskType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::BugFix => "bug_fix",
            Self::Refactor => "refactor",
            Self::NewFeature => "new_feature",
            Self::CodeReview => "code_review",
            Self::PerformanceOptimization => "performance_optimization",
            Self::SecurityFix => "security_fix",
            Self::Documentation => "documentation",
            Self::Testing => "testing",
            Self::Architecture => "architecture",
            Self::General => "general",
        }
    }
}

/// Commit information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub files_changed: Vec<PathBuf>,
}

/// Fix information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixInfo {
    pub commit_hash: String,
    pub description: String,
    pub files_changed: Vec<PathBuf>,
    pub diff: String,
}

/// Architecture information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureInfo {
    pub pattern: String,
    pub layers: Vec<LayerInfo>,
    pub components: Vec<ComponentInfo>,
    pub data_flow: Vec<DataFlowInfo>,
}

/// Layer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerInfo {
    pub name: String,
    pub description: String,
    pub modules: Vec<String>,
}

/// Component information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentInfo {
    pub name: String,
    pub kind: String,
    pub dependencies: Vec<String>,
    pub interfaces: Vec<String>,
}

/// Data flow information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFlowInfo {
    pub from: String,
    pub to: String,
    pub data_type: String,
    pub protocol: String,
}

/// Dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyInfo {
    pub name: String,
    pub version: String,
    pub kind: DependencyKind,
    pub is_direct: bool,
}

/// Dependency kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DependencyKind {
    Runtime,
    Development,
    Build,
    Test,
    Optional,
}

/// Interaction in a session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub kind: InteractionKind,
    pub content: String,
    pub response: Option<String>,
}

/// Interaction kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteractionKind {
    Query,
    Command,
    Edit,
    Search,
    Navigation,
    Error,
    Success,
}

/// Code change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeChange {
    pub file_path: PathBuf,
    pub change_type: ChangeType,
    pub old_content: Option<String>,
    pub new_content: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Change type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeType {
    Create,
    Modify,
    Delete,
    Rename,
}

/// Pattern learned from sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub id: Uuid,
    pub kind: PatternKind,
    pub name: String,
    pub description: String,
    pub frequency: u32,
    pub confidence: f32,
    pub examples: Vec<String>,
}

/// Pattern kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatternKind {
    NamingConvention,
    CodeStructure,
    ErrorHandling,
    TestingStyle,
    DocumentationStyle,
    ArchitecturePattern,
    Workflow,
    Preference,
}

/// Image types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageType {
    ArchitectureDiagram,
    Flowchart,
    SequenceDiagram,
    ClassDiagram,
    Whiteboard,
    Screenshot,
    Other,
}

/// Diagram information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagramInfo {
    pub nodes: Vec<DiagramNode>,
    pub edges: Vec<DiagramEdge>,
    pub labels: Vec<String>,
}

/// Diagram node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagramNode {
    pub id: String,
    pub label: String,
    pub kind: String,
    pub position: Option<(f32, f32)>,
}

/// Diagram edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagramEdge {
    pub source: String,
    pub target: String,
    pub label: Option<String>,
}

/// Supported agent types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentType {
    ClaudeCode,
    Cursor,
    Codex,
    GeminiCLI,
    OpenCode,
    Copilot,
    Windsurf,
    Cline,
    Aider,
    Continue,
}

impl AgentType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::ClaudeCode => "Claude Code",
            Self::Cursor => "Cursor",
            Self::Codex => "Codex",
            Self::GeminiCLI => "Gemini CLI",
            Self::OpenCode => "OpenCode",
            Self::Copilot => "GitHub Copilot",
            Self::Windsurf => "Windsurf",
            Self::Cline => "Cline",
            Self::Aider => "Aider",
            Self::Continue => "Continue",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::ClaudeCode,
            Self::Cursor,
            Self::Codex,
            Self::GeminiCLI,
            Self::OpenCode,
            Self::Copilot,
            Self::Windsurf,
            Self::Cline,
            Self::Aider,
            Self::Continue,
        ]
    }
}

/// Hotspot in the codebase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hotspot {
    pub node: CodeNode,
    pub score: f32,
    pub reasons: Vec<String>,
}

/// Community in the code graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Community {
    pub id: Uuid,
    pub name: String,
    pub nodes: Vec<Uuid>,
    pub description: String,
}

/// Impact report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactReport {
    pub target: CodeNode,
    pub direct_dependents: Vec<CodeNode>,
    pub transitive_dependents: Vec<CodeNode>,
    pub affected_tests: Vec<CodeNode>,
    pub risk_level: RiskLevel,
    pub recommendations: Vec<String>,
}

/// Risk level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl RiskLevel {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }
}

/// Learned knowledge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Knowledge {
    pub id: Uuid,
    pub kind: KnowledgeKind,
    pub content: String,
    pub source: String,
    pub confidence: f32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub tags: Vec<String>,
    pub related: Vec<Uuid>,
}

/// Knowledge kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KnowledgeKind {
    CodePattern,
    ArchitectureDecision,
    BugFix,
    UserPreference,
    ProjectContext,
    BestPractice,
    Workflow,
}

// ============================================================================
// Additional types for NeuraCode
// ============================================================================

/// Index report
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexReport {
    pub files_indexed: usize,
    pub nodes_created: usize,
    pub edges_created: usize,
    pub duration_ms: u64,
    pub languages: Vec<Language>,
}

/// Search result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchResult {
    pub node: CodeNode,
    pub score: f32,
    pub context: String,
}

/// Context package for a task
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContextPackage {
    pub task_type: TaskType,
    pub relevant_files: Vec<std::path::PathBuf>,
    pub test_files: Vec<std::path::PathBuf>,
    pub recent_commits: Vec<CommitInfo>,
    pub error_logs: Vec<String>,
    pub similar_fixes: Vec<FixInfo>,
    pub architecture: Option<ArchitectureInfo>,
    pub dependencies: Vec<DependencyInfo>,
}

/// Session information for learning
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub interactions: Vec<Interaction>,
    pub code_changes: Vec<CodeChange>,
    pub patterns: Vec<Pattern>,
}

/// Image understanding result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImageUnderstanding {
    pub image_type: ImageType,
    pub content: String,
    pub extracted_diagram: Option<DiagramInfo>,
    pub confidence: f32,
}

/// Agent installation result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AgentInstallResult {
    pub agent: AgentType,
    pub success: bool,
    pub message: String,
}
