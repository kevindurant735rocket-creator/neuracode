//! Export command - Export code graph data

use anyhow::Result;
use colored::Colorize;
use neuracode_core::{NeuraCode, NeuraCodeConfig};
use std::path::Path;

pub async fn execute(output: String, format: String) -> Result<()> {
    let config = NeuraCodeConfig::default();
    let neuracode = NeuraCode::new(config).await?;
    
    let output_path = Path::new(&output);
    
    // Ensure parent directory exists
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    match format.as_str() {
        "json" => {
            // Export as JSON
            let hotspots = neuracode.code_brain.identify_hotspots().await;
            let architecture = neuracode.code_brain.detect_architecture().await;
            
            let export_data = serde_json::json!({
                "version": env!("CARGO_PKG_VERSION"),
                "exported_at": chrono::Utc::now().to_rfc3339(),
                "architecture": architecture,
                "hotspots": hotspots,
            });
            
            std::fs::write(output_path, serde_json::to_string_pretty(&export_data)?)?;
        }
        "dot" | "graphviz" => {
            // Export as Graphviz DOT format
            let dot_content = generate_dot_format(&neuracode).await?;
            std::fs::write(output_path, dot_content)?;
        }
        "mermaid" => {
            // Export as Mermaid format
            let mermaid_content = generate_mermaid_format(&neuracode).await?;
            std::fs::write(output_path, mermaid_content)?;
        }
        _ => {
            anyhow::bail!("Unsupported format: {}. Use 'json', 'dot', or 'mermaid'", format);
        }
    }
    
    println!("{} Exported to: {}", "✓".green(), output_path.display());
    
    Ok(())
}

async fn generate_dot_format(neuracode: &NeuraCode) -> Result<String> {
    let mut dot = String::new();
    dot.push_str("digraph CodeGraph {\n");
    dot.push_str("  rankdir=TB;\n");
    dot.push_str("  node [shape=box, style=filled];\n\n");
    
    // Add nodes
    let hotspots = neuracode.code_brain.identify_hotspots().await;
    for hotspot in &hotspots {
        let color = if hotspot.score > 10.0 {
            "red"
        } else if hotspot.score > 5.0 {
            "orange"
        } else {
            "lightblue"
        };
        
        dot.push_str(&format!(
            "  \"{}\" [fillcolor={}];\n",
            hotspot.node.name, color
        ));
    }
    
    dot.push_str("}\n");
    Ok(dot)
}

async fn generate_mermaid_format(neuracode: &NeuraCode) -> Result<String> {
    let mut mermaid = String::new();
    mermaid.push_str("graph TD\n");
    
    let hotspots = neuracode.code_brain.identify_hotspots().await;
    for hotspot in &hotspots {
        let style = if hotspot.score > 10.0 {
            ":::hot"
        } else {
            ":::normal"
        };
        
        mermaid.push_str(&format!(
            "  {}[\"{}\"]{}\n",
            hotspot.node.name.replace("-", "_").replace(" ", "_"),
            hotspot.node.name,
            style
        ));
    }
    
    mermaid.push_str("\n  classDef hot fill:#f96,stroke:#333,stroke-width:2px;\n");
    mermaid.push_str("  classDef normal fill:#9cf,stroke:#333,stroke-width:1px;\n");
    
    Ok(mermaid)
}
