//! Stats command

use anyhow::Result;
use colored::Colorize;
use neuracode_core::{NeuraCode, NeuraCodeConfig};

pub async fn execute(_config_path: Option<String>) -> Result<()> {
    // Create configuration (auto-detects .neuracode/config.toml)
    let config = NeuraCodeConfig::load_or_default(None);

    // Create NeuraCode instance
    let neuracode = NeuraCode::new(config).await?;
    let (nodes, edges) = neuracode.code_brain.counts();

    // Get hotspots as supplementary signal
    let hotspots = neuracode.code_brain.identify_hotspots().await;

    println!("{}", "NeuraCode Statistics".bold());
    println!();
    println!("Nodes: {}", nodes);
    println!("Edges: {}", edges);
    println!("Codebase Hotspots: {}", hotspots.len());

    if !hotspots.is_empty() {
        println!();
        println!("Top Hotspots:");
        for (i, hotspot) in hotspots.iter().take(5).enumerate() {
            println!(
                "  {}. {} (score: {:.2})",
                i + 1,
                hotspot.node.name.cyan(),
                hotspot.score
            );
        }
    }

    Ok(())
}
