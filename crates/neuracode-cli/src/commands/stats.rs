//! Stats command

use anyhow::Result;
use colored::Colorize;
use neuracode_core::{NeuraCode, NeuraCodeConfig};

pub async fn execute() -> Result<()> {
    // Create configuration
    let config = NeuraCodeConfig::default();

    // Create NeuraCode instance
    let neuracode = NeuraCode::new(config).await?;

    // Get hotspots as a proxy for stats
    let hotspots = neuracode.code_brain.identify_hotspots().await;

    println!("{}", "NeuraCode Statistics".bold());
    println!();

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
