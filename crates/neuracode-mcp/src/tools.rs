//! MCP Tools

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tool input/output types for MCP

// Search tool
#[derive(Debug, Deserialize)]
pub struct SearchInput {
    pub query: String,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct SearchOutput {
    pub results: Vec<SearchResult>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub name: String,
    pub kind: String,
    pub file_path: String,
    pub location: String,
    pub score: f32,
    pub context: String,
}

// Predict tool
#[derive(Debug, Deserialize)]
pub struct PredictInput {
    pub task: String,
}

#[derive(Debug, Serialize)]
pub struct PredictOutput {
    pub task_type: String,
    pub relevant_files: Vec<String>,
    pub test_files: Vec<String>,
    pub suggestions: Vec<String>,
}

// Impact tool
#[derive(Debug, Deserialize)]
pub struct ImpactInput {
    pub target: String,
}

#[derive(Debug, Serialize)]
pub struct ImpactOutput {
    pub target: String,
    pub risk_level: String,
    pub direct_dependents: Vec<String>,
    pub transitive_dependents: Vec<String>,
    pub affected_tests: Vec<String>,
    pub recommendations: Vec<String>,
}

// Architecture tool
#[derive(Debug, Serialize)]
pub struct ArchitectureOutput {
    pub pattern: String,
    pub layers: Vec<LayerInfo>,
    pub components: Vec<ComponentInfo>,
    pub data_flow: Vec<DataFlowInfo>,
}

#[derive(Debug, Serialize)]
pub struct LayerInfo {
    pub name: String,
    pub description: String,
    pub modules: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ComponentInfo {
    pub name: String,
    pub kind: String,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct DataFlowInfo {
    pub from: String,
    pub to: String,
    pub data_type: String,
}

// Hotspots tool
#[derive(Debug, Serialize)]
pub struct HotspotsOutput {
    pub hotspots: Vec<HotspotInfo>,
}

#[derive(Debug, Serialize)]
pub struct HotspotInfo {
    pub name: String,
    pub file_path: String,
    pub score: f32,
    pub reasons: Vec<String>,
}

// Analyze tool
#[derive(Debug, Deserialize)]
pub struct AnalyzeInput {
    pub path: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AnalyzeOutput {
    pub total_files: usize,
    pub total_functions: usize,
    pub total_classes: usize,
    pub languages: Vec<String>,
    pub issues: Vec<IssueInfo>,
}

#[derive(Debug, Serialize)]
pub struct IssueInfo {
    pub severity: String,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<usize>,
}

// Suggest tool
#[derive(Debug, Deserialize)]
pub struct SuggestInput {
    pub context: String,
}

#[derive(Debug, Serialize)]
pub struct SuggestOutput {
    pub task_type: String,
    pub suggestions: Vec<SuggestionItem>,
    pub related_files: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct SuggestionItem {
    pub title: String,
    pub description: String,
    pub priority: i32,
}

// Stats tool
#[derive(Debug, Serialize)]
pub struct StatsOutput {
    pub version: String,
    pub indexed_files: usize,
    pub total_nodes: usize,
    pub total_edges: usize,
    pub languages: Vec<String>,
    pub cache_size_mb: usize,
}

// Compare tool
#[derive(Debug, Deserialize)]
pub struct CompareInput {
    pub path1: String,
    pub path2: String,
}

#[derive(Debug, Serialize)]
pub struct CompareOutput {
    pub path1: PathStats,
    pub path2: PathStats,
    pub differences: Differences,
}

#[derive(Debug, Serialize)]
pub struct PathStats {
    pub files: usize,
    pub nodes: usize,
    pub edges: usize,
}

#[derive(Debug, Serialize)]
pub struct Differences {
    pub files_diff: i64,
    pub nodes_diff: i64,
    pub edges_diff: i64,
}

// Export tool
#[derive(Debug, Deserialize)]
pub struct ExportInput {
    pub format: String,  // json, dot, mermaid
}

#[derive(Debug, Serialize)]
pub struct ExportOutput {
    pub format: String,
    pub data: String,
    pub file_path: String,
}

// Tool registry
pub struct ToolRegistry {
    tools: HashMap<String, ToolInfo>,
}

#[derive(Debug, Clone)]
pub struct ToolInfo {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

impl ToolRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            tools: HashMap::new(),
        };
        registry.register_tools();
        registry
    }
    
    fn register_tools(&mut self) {
        self.tools.insert(
            "neuracode_search".to_string(),
            ToolInfo {
                name: "neuracode_search".to_string(),
                description: "Semantic search across the codebase".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string",
                            "description": "Search query"
                        },
                        "limit": {
                            "type": "number",
                            "description": "Maximum results"
                        }
                    },
                    "required": ["query"]
                }),
            },
        );
        
        self.tools.insert(
            "neuracode_predict".to_string(),
            ToolInfo {
                name: "neuracode_predict".to_string(),
                description: "Predict context for a task".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "task": {
                            "type": "string",
                            "description": "Task description"
                        }
                    },
                    "required": ["task"]
                }),
            },
        );
        
        self.tools.insert(
            "neuracode_impact".to_string(),
            ToolInfo {
                name: "neuracode_impact".to_string(),
                description: "Analyze impact of changes".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "target": {
                            "type": "string",
                            "description": "File or function to analyze"
                        }
                    },
                    "required": ["target"]
                }),
            },
        );
        
        self.tools.insert(
            "neuracode_architecture".to_string(),
            ToolInfo {
                name: "neuracode_architecture".to_string(),
                description: "Get architecture information".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {}
                }),
            },
        );
        
        self.tools.insert(
            "neuracode_hotspots".to_string(),
            ToolInfo {
                name: "neuracode_hotspots".to_string(),
                description: "Identify codebase hotspots".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {}
                }),
            },
        );
        
        self.tools.insert(
            "neuracode_analyze".to_string(),
            ToolInfo {
                name: "neuracode_analyze".to_string(),
                description: "Deep code analysis".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to analyze"
                        }
                    }
                }),
            },
        );
        
        self.tools.insert(
            "neuracode_suggest".to_string(),
            ToolInfo {
                name: "neuracode_suggest".to_string(),
                description: "Get AI-powered suggestions".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "context": {
                            "type": "string",
                            "description": "Task or context"
                        }
                    },
                    "required": ["context"]
                }),
            },
        );
        
        self.tools.insert(
            "neuracode_stats".to_string(),
            ToolInfo {
                name: "neuracode_stats".to_string(),
                description: "Show statistics".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {}
                }),
            },
        );
        
        self.tools.insert(
            "neuracode_compare".to_string(),
            ToolInfo {
                name: "neuracode_compare".to_string(),
                description: "Compare two codebases".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path1": {
                            "type": "string",
                            "description": "First path"
                        },
                        "path2": {
                            "type": "string",
                            "description": "Second path"
                        }
                    },
                    "required": ["path1", "path2"]
                }),
            },
        );
        
        self.tools.insert(
            "neuracode_export".to_string(),
            ToolInfo {
                name: "neuracode_export".to_string(),
                description: "Export code graph data".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "format": {
                            "type": "string",
                            "description": "Export format (json, dot, mermaid)"
                        }
                    },
                    "required": ["format"]
                }),
            },
        );
    }
    
    pub fn list_tools(&self) -> Vec<ToolInfo> {
        self.tools.values().cloned().collect()
    }
    
    pub fn get_tool(&self, name: &str) -> Option<&ToolInfo> {
        self.tools.get(name)
    }
}
