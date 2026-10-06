//! Architecture command

use anyhow::Result;
use colored::Colorize;
use neuracode_core::{NeuraCode, NeuraCodeConfig};

pub async fn execute(format: String) -> Result<()> {
    // Create configuration
    let config = NeuraCodeConfig::default();
    
    // Create NeuraCode instance
    let neuracode = NeuraCode::new(config).await?;
    
    // Get architecture info
    let arch = neuracode.code_brain.detect_architecture().await;
    
    if format == "json" {
        println!("{}", serde_json::to_string_pretty(&arch)?);
    } else {
        match arch {
            Some(arch) => {
                println!("Architecture Pattern: {}", arch.pattern.cyan());
                println!();
                
                if !arch.layers.is_empty() {
                    println!("Layers:");
                    for layer in &arch.layers {
                        println!("  - {}: {}", layer.name.yellow(), layer.description);
                    }
                    println!();
                }
                
                if !arch.components.is_empty() {
                    println!("Components:");
                    for comp in &arch.components {
                        println!("  - {} ({})", comp.name, comp.kind);
                    }
                    println!();
                }
                
                if !arch.data_flow.is_empty() {
                    println!("Data Flow:");
                    for flow in &arch.data_flow {
                        println!("  - {} -> {} ({})", flow.from, flow.to, flow.data_type);
                    }
                }
            }
            None => {
                println!("No architecture information available.");
                println!("Run 'neuracode index' first to analyze your codebase.");
            }
        }
    }
    
    Ok(())
}
