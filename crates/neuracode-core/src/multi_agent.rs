//! Multi-Agent Support - Unified interface for all AI coding agents
//!
//! This module provides seamless integration with:
//! - Claude Code
//! - Cursor
//! - Codex
//! - Gemini CLI
//! - OpenCode
//! - GitHub Copilot
//! - Windsurf
//! - Cline
//! - Aider
//! - Continue

use crate::error::{NeuraCodeError, Result};
use crate::types::*;
use crate::NeuraCodeConfig;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{info, instrument};

/// Multi-Agent Support
#[allow(dead_code)]
pub struct MultiAgentSupport {
    /// Configuration
    config: NeuraCodeConfig,

    /// Installed agents
    installed_agents: Vec<AgentType>,

    /// Agent configurations
    agent_configs: HashMap<AgentType, AgentConfig>,
}

/// Agent configuration
#[derive(Debug, Clone)]
pub struct AgentConfig {
    pub agent_type: AgentType,
    pub config_path: PathBuf,
    pub mcp_server_name: String,
    pub instructions_path: Option<PathBuf>,
    pub skills_path: Option<PathBuf>,
}

impl MultiAgentSupport {
    /// Create a new MultiAgentSupport
    pub fn new(config: &NeuraCodeConfig) -> Self {
        let mut agent_configs = HashMap::new();

        // Initialize configurations for all supported agents
        Self::init_agent_configs(&mut agent_configs);

        Self {
            config: config.clone(),
            installed_agents: Vec::new(),
            agent_configs,
        }
    }

    /// Initialize agent configurations
    fn init_agent_configs(configs: &mut HashMap<AgentType, AgentConfig>) {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));

        // Claude Code
        configs.insert(
            AgentType::ClaudeCode,
            AgentConfig {
                agent_type: AgentType::ClaudeCode,
                config_path: home.join(".claude").join("settings.json"),
                mcp_server_name: "neuracode".to_string(),
                instructions_path: Some(home.join(".claude").join("CLAUDE.md")),
                skills_path: Some(home.join(".claude").join("skills")),
            },
        );

        // Cursor
        configs.insert(
            AgentType::Cursor,
            AgentConfig {
                agent_type: AgentType::Cursor,
                config_path: home.join(".cursor").join("mcp.json"),
                mcp_server_name: "neuracode".to_string(),
                instructions_path: Some(home.join(".cursor").join("rules")),
                skills_path: None,
            },
        );

        // Codex
        configs.insert(
            AgentType::Codex,
            AgentConfig {
                agent_type: AgentType::Codex,
                config_path: home.join(".codex").join("config.toml"),
                mcp_server_name: "neuracode".to_string(),
                instructions_path: Some(home.join(".codex").join("AGENTS.md")),
                skills_path: None,
            },
        );

        // Gemini CLI
        configs.insert(
            AgentType::GeminiCLI,
            AgentConfig {
                agent_type: AgentType::GeminiCLI,
                config_path: home.join(".gemini").join("settings.json"),
                mcp_server_name: "neuracode".to_string(),
                instructions_path: Some(home.join(".gemini").join("GEMINI.md")),
                skills_path: None,
            },
        );

        // OpenCode
        configs.insert(
            AgentType::OpenCode,
            AgentConfig {
                agent_type: AgentType::OpenCode,
                config_path: home.join(".opencode").join("config.json"),
                mcp_server_name: "neuracode".to_string(),
                instructions_path: Some(home.join(".opencode").join("AGENTS.md")),
                skills_path: None,
            },
        );

        // GitHub Copilot
        configs.insert(
            AgentType::Copilot,
            AgentConfig {
                agent_type: AgentType::Copilot,
                config_path: home.join(".copilot").join("config.json"),
                mcp_server_name: "neuracode".to_string(),
                instructions_path: None,
                skills_path: None,
            },
        );

        // Windsurf
        configs.insert(
            AgentType::Windsurf,
            AgentConfig {
                agent_type: AgentType::Windsurf,
                config_path: home.join(".windsurf").join("config.json"),
                mcp_server_name: "neuracode".to_string(),
                instructions_path: Some(home.join(".windsurf").join("rules")),
                skills_path: None,
            },
        );

        // Cline
        configs.insert(
            AgentType::Cline,
            AgentConfig {
                agent_type: AgentType::Cline,
                config_path: home.join(".cline").join("config.json"),
                mcp_server_name: "neuracode".to_string(),
                instructions_path: Some(home.join(".cline").join("rules")),
                skills_path: None,
            },
        );

        // Aider
        configs.insert(
            AgentType::Aider,
            AgentConfig {
                agent_type: AgentType::Aider,
                config_path: home.join(".aider").join("config.yml"),
                mcp_server_name: "neuracode".to_string(),
                instructions_path: Some(home.join(".aider").join("CONVENTIONS.md")),
                skills_path: None,
            },
        );

        // Continue
        configs.insert(
            AgentType::Continue,
            AgentConfig {
                agent_type: AgentType::Continue,
                config_path: home.join(".continue").join("config.json"),
                mcp_server_name: "neuracode".to_string(),
                instructions_path: Some(home.join(".continue").join("config").join("rules")),
                skills_path: None,
            },
        );
    }

    /// Detect installed agents
    pub fn detect_installed_agents(&self) -> Vec<AgentType> {
        let mut installed = Vec::new();

        for (agent_type, config) in &self.agent_configs {
            if config.config_path.exists() {
                installed.push(*agent_type);
            }
        }

        installed
    }

    /// Install for all supported agents
    #[instrument(skip(self))]
    pub async fn install_all(&self) -> Result<Vec<AgentInstallResult>> {
        info!("Installing NeuraCode for all supported agents");

        let mut results = Vec::new();

        for agent_type in AgentType::all() {
            let result = self.install_for_agent(agent_type).await;
            results.push(result);
        }

        Ok(results)
    }

    /// Install for a specific agent
    #[instrument(skip(self))]
    pub async fn install_for_agent(&self, agent_type: AgentType) -> AgentInstallResult {
        info!("Installing for agent: {}", agent_type.name());

        let config = match self.agent_configs.get(&agent_type) {
            Some(c) => c,
            None => {
                return AgentInstallResult {
                    agent: agent_type,
                    success: false,
                    message: "Agent not supported".to_string(),
                };
            }
        };

        // Check if agent is installed
        if !config.config_path.exists() {
            return AgentInstallResult {
                agent: agent_type,
                success: false,
                message: format!(
                    "{} not found at {}",
                    agent_type.name(),
                    config.config_path.display()
                ),
            };
        }

        // Install MCP server configuration
        match self.install_mcp_server(config).await {
            Ok(_) => {
                // Install instructions
                if let Some(ref instructions_path) = config.instructions_path {
                    let _ = self
                        .install_instructions(instructions_path, agent_type)
                        .await;
                }

                // Install skills
                if let Some(ref skills_path) = config.skills_path {
                    let _ = self.install_skills(skills_path).await;
                }

                AgentInstallResult {
                    agent: agent_type,
                    success: true,
                    message: format!("Successfully installed for {}", agent_type.name()),
                }
            }
            Err(e) => AgentInstallResult {
                agent: agent_type,
                success: false,
                message: format!("Installation failed: {}", e),
            },
        }
    }

    /// Install MCP server configuration
    async fn install_mcp_server(&self, config: &AgentConfig) -> Result<()> {
        info!("Installing MCP server for: {}", config.agent_type.name());

        // Generate MCP server configuration
        let mcp_config = self.generate_mcp_config(config);

        // Write configuration
        let config_dir = config
            .config_path
            .parent()
            .ok_or_else(|| NeuraCodeError::Config("Invalid config path".to_string()))?;

        std::fs::create_dir_all(config_dir)?;

        // Merge with existing configuration
        let existing_config = if config.config_path.exists() {
            std::fs::read_to_string(&config.config_path)?
        } else {
            "{}".to_string()
        };

        let merged_config = self.merge_config(&existing_config, &mcp_config)?;
        std::fs::write(&config.config_path, merged_config)?;

        Ok(())
    }

    /// Generate MCP server configuration
    fn generate_mcp_config(&self, config: &AgentConfig) -> String {
        format!(
            r#"{{
  "mcpServers": {{
    "{}": {{
      "command": "neuracode-mcp",
      "args": ["--stdio"]
    }}
  }}
}}"#,
            config.mcp_server_name
        )
    }

    /// Merge configurations
    fn merge_config(&self, existing: &str, new: &str) -> Result<String> {
        // Simple JSON merge (in production, use a proper JSON library)
        if existing == "{}" {
            Ok(new.to_string())
        } else {
            // For now, just return the new config
            // In production, this would properly merge JSON
            Ok(new.to_string())
        }
    }

    /// Install instructions
    async fn install_instructions(
        &self,
        instructions_path: &PathBuf,
        agent_type: AgentType,
    ) -> Result<()> {
        info!("Installing instructions for: {}", agent_type.name());

        let instructions = self.generate_instructions(agent_type);

        let instructions_dir = instructions_path
            .parent()
            .ok_or_else(|| NeuraCodeError::Config("Invalid instructions path".to_string()))?;

        std::fs::create_dir_all(instructions_dir)?;
        std::fs::write(instructions_path, instructions)?;

        Ok(())
    }

    /// Generate instructions for an agent
    fn generate_instructions(&self, _agent_type: AgentType) -> String {
        r#"# NeuraCode Integration

This project uses NeuraCode for enhanced code intelligence.

## Available Tools

- `neuracode_search` - Semantic search across the codebase
- `neuracode_predict` - Predict context for tasks
- `neuracode_impact` - Analyze impact of changes
- `neuracode_architecture` - Get architecture information

## Usage

NeuraCode is automatically activated when you:
- Search for code
- Make changes to code
- Ask questions about the codebase

## Configuration

NeuraCode configuration is stored in `.neuracode/config.toml`
"#
        .to_string()
    }

    /// Install skills
    async fn install_skills(&self, skills_path: &PathBuf) -> Result<()> {
        info!("Installing skills");

        std::fs::create_dir_all(skills_path)?;

        // Create neuracode skill
        let neuracode_skill = skills_path.join("neuracode");
        std::fs::create_dir_all(&neuracode_skill)?;

        let skill_content = self.generate_skill_content();
        std::fs::write(neuracode_skill.join("SKILL.md"), skill_content)?;

        Ok(())
    }

    /// Generate skill content
    fn generate_skill_content(&self) -> String {
        r#"# NeuraCode Skill

## Description

NeuraCode provides enhanced code intelligence for AI agents.

## Trigger

Use this skill when:
- Searching for code
- Analyzing code structure
- Understanding dependencies
- Planning refactoring

## Commands

- `/neuracode search <query>` - Search the codebase
- `/neuracode predict <task>` - Predict context for a task
- `/neuracode impact <file>` - Analyze impact of changes
- `/neuracode architecture` - Get architecture information

## Examples

```
/neuracode search authentication
/neuracode predict fix login bug
/neuracode impact src/auth.ts
/neuracode architecture
```
"#
        .to_string()
    }

    /// Uninstall from all agents
    pub async fn uninstall_all(&self) -> Result<Vec<AgentInstallResult>> {
        info!("Uninstalling NeuraCode from all agents");

        let mut results = Vec::new();

        for agent_type in AgentType::all() {
            let result = self.uninstall_from_agent(agent_type).await;
            results.push(result);
        }

        Ok(results)
    }

    /// Uninstall from a specific agent
    pub async fn uninstall_from_agent(&self, agent_type: AgentType) -> AgentInstallResult {
        info!("Uninstalling from agent: {}", agent_type.name());

        let config = match self.agent_configs.get(&agent_type) {
            Some(c) => c,
            None => {
                return AgentInstallResult {
                    agent: agent_type,
                    success: false,
                    message: "Agent not supported".to_string(),
                };
            }
        };

        // Remove MCP server configuration
        if config.config_path.exists() {
            let _ = std::fs::remove_file(&config.config_path);
        }

        // Remove instructions
        if let Some(ref instructions_path) = config.instructions_path {
            if instructions_path.exists() {
                let _ = std::fs::remove_file(instructions_path);
            }
        }

        // Remove skills
        if let Some(ref skills_path) = config.skills_path {
            let neuracode_skill = skills_path.join("neuracode");
            if neuracode_skill.exists() {
                let _ = std::fs::remove_dir_all(neuracode_skill);
            }
        }

        AgentInstallResult {
            agent: agent_type,
            success: true,
            message: format!("Successfully uninstalled from {}", agent_type.name()),
        }
    }

    /// Get agent configuration
    pub fn get_agent_config(&self, agent_type: AgentType) -> Option<&AgentConfig> {
        self.agent_configs.get(&agent_type)
    }

    /// Check if agent is supported
    pub fn is_agent_supported(&self, agent_type: AgentType) -> bool {
        self.agent_configs.contains_key(&agent_type)
    }
}
