//! Uninstall command

use anyhow::Result;
use colored::Colorize;
use neuracode_core::{NeuraCode, NeuraCodeConfig, AgentType};

pub async fn execute(agent: Option<String>, all: bool) -> Result<()> {
    // Create configuration
    let config = NeuraCodeConfig::default();
    
    // Create NeuraCode instance
    let neuracode = NeuraCode::new(config).await?;
    
    if all {
        println!("Uninstalling NeuraCode from all agents...\n");
        
        let results = neuracode.multi_agent.uninstall_all().await?;
        
        let mut success_count = 0;
        let mut fail_count = 0;
        
        for result in &results {
            if result.success {
                println!("{} {}: {}", "✓".green(), result.agent.name(), result.message);
                success_count += 1;
            } else {
                println!("{} {}: {}", "✗".red(), result.agent.name(), result.message);
                fail_count += 1;
            }
        }
        
        println!();
        println!("Uninstall complete: {} succeeded, {} failed", success_count, fail_count);
    } else if let Some(agent_name) = agent {
        let agent_type = parse_agent_type(&agent_name)?;
        
        println!("Uninstalling NeuraCode from {}...\n", agent_type.name());
        
        let result = neuracode.multi_agent.uninstall_from_agent(agent_type).await;
        
        if result.success {
            println!("{} {}", "✓".green(), result.message);
        } else {
            println!("{} {}", "✗".red(), result.message);
        }
    } else {
        println!("Please specify --all or --agent <name>");
    }
    
    Ok(())
}

fn parse_agent_type(name: &str) -> Result<AgentType> {
    match name.to_lowercase().as_str() {
        "claude-code" | "claude" => Ok(AgentType::ClaudeCode),
        "cursor" => Ok(AgentType::Cursor),
        "codex" => Ok(AgentType::Codex),
        "gemini-cli" | "gemini" => Ok(AgentType::GeminiCLI),
        "opencode" | "open-code" => Ok(AgentType::OpenCode),
        "copilot" => Ok(AgentType::Copilot),
        "windsurf" => Ok(AgentType::Windsurf),
        "cline" => Ok(AgentType::Cline),
        "aider" => Ok(AgentType::Aider),
        "continue" => Ok(AgentType::Continue),
        _ => anyhow::bail!("Unknown agent: {}", name),
    }
}
