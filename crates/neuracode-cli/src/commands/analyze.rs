//! Analyze command - Deep code analysis

use anyhow::Result;
use colored::Colorize;
use neuracode_core::{NeuraCode, NeuraCodeConfig};
use std::collections::HashMap;

pub async fn execute(path: String, format: String) -> Result<()> {
    let config = NeuraCodeConfig::default();
    let neuracode = NeuraCode::new(config).await?;
    
    // Get hotspots
    let hotspots = neuracode.code_brain.identify_hotspots().await;
    
    // Get architecture
    let architecture = neuracode.code_brain.detect_architecture().await;
    
    if format == "json" {
        let output = serde_json::json!({
            "hotspots": hotspots,
            "architecture": architecture,
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("{}", "Code Analysis Report".bold().underline());
        println!();
        
        // Architecture
        if let Some(arch) = architecture {
            println!("{}", "Architecture:".bold());
            println!("  Pattern: {}", arch.pattern.cyan());
            println!();
        }
        
        // Hotspots
        if !hotspots.is_empty() {
            println!("{}", "Hotspots:".bold());
            for (i, hotspot) in hotspots.iter().take(10).enumerate() {
                println!("  {}. {} ({})", 
                    i + 1,
                    hotspot.node.name.yellow(),
                    hotspot.node.file_path.display()
                );
                println!("     Score: {:.2}", hotspot.score);
                for reason in &hotspot.reasons {
                    println!("     - {}", reason.dimmed());
                }
            }
            println!();
        }
        
        // Recommendations
        println!("{}", "Recommendations:".bold());
        if hotspots.len() > 5 {
            println!("  • Consider refactoring high-complexity areas");
        }
        if hotspots.iter().any(|h| h.score > 10.0) {
            println!("  • Review frequently modified code for stability");
        }
        println!("  • Add tests for critical paths");
    }
    
    Ok(())
}
