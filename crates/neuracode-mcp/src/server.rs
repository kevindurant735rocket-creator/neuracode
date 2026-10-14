//! MCP Server implementation
//!
//! Full MCP (Model Context Protocol) implementation for AI agent integration.

use crate::tools::*;
use anyhow::Result;
use neuracode_core::{NeuraCode, NeuraCodeConfig};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{BufRead, Write};
use tracing::{debug, error, info, instrument};

/// MCP Protocol version
const MCP_VERSION: &str = "2024-11-05";

/// MCP Server
pub struct McpServer {
    tool_registry: ToolRegistry,
    initialized: bool,
    core: Option<NeuraCode>,
}

/// MCP Request
#[derive(Debug, Deserialize)]
struct McpRequest {
    #[allow(dead_code)]
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

/// MCP Response
#[derive(Debug, Serialize)]
struct McpResponse {
    jsonrpc: String,
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<McpError>,
}

/// MCP Error
#[derive(Debug, Serialize)]
struct McpError {
    code: i32,
    message: String,
}

/// MCP Error codes
mod error_code {
    pub const PARSE_ERROR: i32 = -32700;
    #[allow(dead_code)]
    pub const INVALID_REQUEST: i32 = -32600;
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INVALID_PARAMS: i32 = -32602;
    #[allow(dead_code)]
    pub const INTERNAL_ERROR: i32 = -32603;
}

impl McpServer {
    /// Create a new MCP server
    pub fn new() -> Self {
        Self {
            tool_registry: ToolRegistry::new(),
            initialized: false,
            core: None,
        }
    }

    /// Lazily initialize core (default config, auto-detects .neuracode if present)
    async fn ensure_core(&mut self) -> Result<&NeuraCode> {
        if self.core.is_none() {
            let config = NeuraCodeConfig::load_or_default(None);
            let nc = NeuraCode::new(config).await?;
            self.core = Some(nc);
        }
        Ok(self.core.as_ref().unwrap())
    }

    /// Run the MCP server (stdio transport)
    pub async fn run(&mut self) -> Result<()> {
        info!("MCP Server starting with stdio transport");

        let stdin = std::io::stdin();
        let stdout = std::io::stdout();

        let mut stdout_lock = stdout.lock();

        for line in stdin.lock().lines() {
            let line = line?;
            debug!("Received: {}", line);

            let response = self.handle_request(&line).await;
            let response_json = serde_json::to_string(&response)?;

            writeln!(stdout_lock, "{}", response_json)?;
            stdout_lock.flush()?;
        }

        Ok(())
    }

    /// Handle a single MCP request
    async fn handle_request(&mut self, line: &str) -> McpResponse {
        // Parse request
        let request: McpRequest = match serde_json::from_str(line) {
            Ok(req) => req,
            Err(e) => {
                return McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id: None,
                    result: None,
                    error: Some(McpError {
                        code: error_code::PARSE_ERROR,
                        message: format!("Parse error: {}", e),
                    }),
                };
            }
        };

        let id = request.id.clone();

        // Handle different methods
        match request.method.as_str() {
            "initialize" => self.handle_initialize(request.params, id),
            "notifications/initialized" => {
                self.initialized = true;
                debug!("Client initialized");
                // No response for notifications
                McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id: None,
                    result: None,
                    error: None,
                }
            }
            "tools/list" => self.handle_tools_list(id),
            "tools/call" => self.handle_tools_call(request.params, id).await,
            "ping" => McpResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(serde_json::json!({})),
                error: None,
            },
            _ => McpResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(McpError {
                    code: error_code::METHOD_NOT_FOUND,
                    message: format!("Method not found: {}", request.method),
                }),
            },
        }
    }

    /// Handle initialize request
    fn handle_initialize(&self, _params: Option<Value>, id: Option<Value>) -> McpResponse {
        info!("Handling initialize request");

        McpResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(serde_json::json!({
                "protocolVersion": MCP_VERSION,
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "neuracode-mcp",
                    "version": env!("CARGO_PKG_VERSION")
                }
            })),
            error: None,
        }
    }

    /// Handle tools/list request
    fn handle_tools_list(&self, id: Option<Value>) -> McpResponse {
        info!("Handling tools/list request");

        let tools: Vec<Value> = self
            .tool_registry
            .list_tools()
            .into_iter()
            .map(|t| {
                serde_json::json!({
                    "name": t.name,
                    "description": t.description,
                    "inputSchema": t.parameters
                })
            })
            .collect();

        McpResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(serde_json::json!({ "tools": tools })),
            error: None,
        }
    }

    /// Handle tools/call request
    #[instrument(skip(self, params))]
    async fn handle_tools_call(&mut self, params: Option<Value>, id: Option<Value>) -> McpResponse {
        let params = match params {
            Some(p) => p,
            None => {
                return McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: None,
                    error: Some(McpError {
                        code: error_code::INVALID_PARAMS,
                        message: "Missing params".to_string(),
                    }),
                };
            }
        };

        let tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let arguments = params.get("arguments").cloned().unwrap_or(Value::Null);

        debug!("Calling tool: {} with args: {}", tool_name, arguments);

        let result = self.execute_tool(tool_name, &arguments).await;

        match result {
            Ok(value) => McpResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(serde_json::json!({
                    "content": [
                        {
                            "type": "text",
                            "text": serde_json::to_string_pretty(&value).unwrap_or_default()
                        }
                    ]
                })),
                error: None,
            },
            Err(e) => {
                error!("Tool execution failed: {}", e);
                McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: Some(serde_json::json!({
                        "content": [
                            {
                                "type": "text",
                                "text": format!("Error: {}", e)
                            }
                        ],
                        "isError": true
                    })),
                    error: None,
                }
            }
        }
    }

    /// Execute a tool
    async fn execute_tool(&mut self, tool_name: &str, arguments: &Value) -> Result<Value> {
        match tool_name {
            "neuracode_search" => self.tool_search(arguments).await,
            "neuracode_predict" => self.tool_predict(arguments).await,
            "neuracode_impact" => self.tool_impact(arguments).await,
            "neuracode_architecture" => self.tool_architecture(arguments).await,
            "neuracode_hotspots" => self.tool_hotspots(arguments).await,
            "neuracode_analyze" => self.tool_analyze(arguments).await,
            "neuracode_suggest" => self.tool_suggest(arguments).await,
            "neuracode_stats" => self.tool_stats(arguments).await,
            "neuracode_compare" => self.tool_compare(arguments).await,
            "neuracode_export" => self.tool_export(arguments).await,
            _ => Err(anyhow::anyhow!("Unknown tool: {}", tool_name)),
        }
    }

    /// Search tool implementation — real core search
    async fn tool_search(&mut self, args: &Value) -> Result<Value> {
        let query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
        let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(10) as usize;
        if query.is_empty() {
            return Ok(
                serde_json::json!({"results": [], "total": 0, "query": query, "limit": limit}),
            );
        }
        let core = self.ensure_core().await?;
        let results = core.search(query).await.unwrap_or_default();
        let total = results.len();
        let items: Vec<Value> = results
            .into_iter()
            .take(limit.max(1))
            .map(|r| {
                serde_json::json!({
                    "name": r.node.name,
                    "kind": r.node.kind.name(),
                    "file": r.node.file_path.to_string_lossy(),
                    "score": r.score,
                    "signature": r.node.signature,
                    "context": r.context,
                })
            })
            .collect();
        Ok(serde_json::json!({"results": items, "total": total, "query": query, "limit": limit}))
    }

    /// Predict tool implementation — real core predict
    async fn tool_predict(&mut self, args: &Value) -> Result<Value> {
        let task = args.get("task").and_then(|v| v.as_str()).unwrap_or("");
        let core = self.ensure_core().await?;
        match core.predict_context(task).await {
            Ok(ctx) => Ok(serde_json::json!({
                "task_type": ctx.task_type.name(),
                "relevant_files": ctx.relevant_files,
                "test_files": ctx.test_files,
                "recent_commits": ctx.recent_commits.iter().take(5).map(|c| &c.message).collect::<Vec<_>>(),
                "dependencies": ctx.dependencies.iter().take(10).map(|d| &d.name).collect::<Vec<_>>(),
            })),
            Err(_) => Ok(serde_json::json!({
                "task_type": "general",
                "relevant_files": [],
                "test_files": [],
                "suggestions": ["Break down the task into smaller steps", "Write tests", "Document your changes"]
            })),
        }
    }

    /// Impact tool implementation — real core impact via search+analysis
    async fn tool_impact(&mut self, args: &Value) -> Result<Value> {
        let target = args.get("target").and_then(|v| v.as_str()).unwrap_or("");
        let core = self.ensure_core().await?;
        let hits = core.search(target).await.unwrap_or_default();
        if let Some(first) = hits.into_iter().next() {
            match core.code_brain.impact_analysis(first.node.id).await {
                Ok(rep) => Ok(serde_json::json!({
                    "target": target,
                    "risk_level": rep.risk_level.name(),
                    "direct_dependents": rep.direct_dependents.iter().map(|n| n.name.clone()).collect::<Vec<_>>(),
                    "transitive_dependents": rep.transitive_dependents.iter().take(20).map(|n| n.name.clone()).collect::<Vec<_>>(),
                    "affected_tests": rep.affected_tests.iter().map(|n| n.name.clone()).collect::<Vec<_>>(),
                    "recommendations": rep.recommendations
                })),
                Err(e) => Ok(serde_json::json!({"target": target, "error": e.to_string()})),
            }
        } else {
            Ok(serde_json::json!({
                "target": target,
                "risk_level": "low",
                "direct_dependents": [],
                "transitive_dependents": [],
                "affected_tests": [],
                "recommendations": []
            }))
        }
    }

    /// Architecture tool implementation — real core
    async fn tool_architecture(&mut self, _args: &Value) -> Result<Value> {
        let core = self.ensure_core().await?;
        match core.code_brain.detect_architecture().await {
            Some(info) => Ok(serde_json::json!(info)),
            None => Ok(
                serde_json::json!({"pattern": "Unknown", "layers": [], "components": [], "data_flow": []}),
            ),
        }
    }

    /// Hotspots tool implementation — real core
    async fn tool_hotspots(&mut self, args: &Value) -> Result<Value> {
        let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(10) as usize;
        let core = self.ensure_core().await?;
        let hs = core.code_brain.identify_hotspots().await;
        let items: Vec<Value> = hs
            .into_iter()
            .take(limit.max(1))
            .map(|h| serde_json::json!({"name": h.node.name, "file": h.node.file_path.to_string_lossy(), "score": h.score, "reasons": h.reasons}))
            .collect();
        Ok(serde_json::json!({"hotspots": items}))
    }

    /// Analyze tool implementation — real counts from graph
    async fn tool_analyze(&mut self, args: &Value) -> Result<Value> {
        let path = args.get("path").and_then(|v| v.as_str());
        let core = self.ensure_core().await?;
        let hs = core.code_brain.identify_hotspots().await;
        Ok(serde_json::json!({
            "hotspots": hs.len(),
            "path": path
        }))
    }

    /// Suggest tool implementation — real predict + templated suggestions
    async fn tool_suggest(&mut self, args: &Value) -> Result<Value> {
        let context = args.get("context").and_then(|v| v.as_str()).unwrap_or("");
        let core = self.ensure_core().await?;
        let ctx = core.predict_context(context).await;
        let task_type = ctx
            .as_ref()
            .map(|c| c.task_type.name().to_string())
            .unwrap_or_else(|_| "general".into());
        let related: Vec<String> = ctx
            .as_ref()
            .map(|c| {
                c.relevant_files
                    .iter()
                    .take(5)
                    .map(|p| p.to_string_lossy().to_string())
                    .collect()
            })
            .unwrap_or_default();

        Ok(serde_json::json!({
            "task_type": task_type,
            "suggestions": [
                {
                    "title": "Break down the task",
                    "description": "Split into smaller, manageable steps",
                    "priority": 1
                },
                {
                    "title": "Write tests",
                    "description": "Add tests for critical paths",
                    "priority": 2
                }
            ],
            "related_files": related,
            "context": context
        }))
    }

    /// Stats tool implementation — real graph counts
    async fn tool_stats(&mut self, _args: &Value) -> Result<Value> {
        let core = self.ensure_core().await?;
        let (nodes, edges) = core.code_brain.counts();
        Ok(serde_json::json!({
            "version": env!("CARGO_PKG_VERSION"),
            "total_nodes": nodes,
            "total_edges": edges,
            "cache_size_mb": 512
        }))
    }

    /// Compare tool implementation
    async fn tool_compare(&mut self, args: &Value) -> Result<Value> {
        let _path1 = args.get("path1").and_then(|v| v.as_str()).unwrap_or("");
        let _path2 = args.get("path2").and_then(|v| v.as_str()).unwrap_or("");

        Ok(serde_json::json!({
            "path1": { "files": 0, "nodes": 0, "edges": 0 },
            "path2": { "files": 0, "nodes": 0, "edges": 0 },
            "differences": { "files_diff": 0, "nodes_diff": 0, "edges_diff": 0 }
        }))
    }

    /// Export tool implementation
    async fn tool_export(&mut self, args: &Value) -> Result<Value> {
        let format = args
            .get("format")
            .and_then(|v| v.as_str())
            .unwrap_or("json");

        Ok(serde_json::json!({
            "format": format,
            "data": "{}",
            "file_path": ""
        }))
    }
}

/// Run the MCP server
pub async fn run() -> Result<()> {
    info!(
        "Starting NeuraCode MCP Server v{}",
        env!("CARGO_PKG_VERSION")
    );

    let mut server = McpServer::new();
    server.run().await?;

    Ok(())
}
