//! Hotspots command

use anyhow::Result;
use colored::Colorize;
use neuracode_core::{NeuraCode, NeuraCodeConfig};

pub async fn execute(limit: usize, format: String) -> Result<()> {
    // Create configuration
    let config = NeuraCodeConfig::default();

    // Create NeuraCode instance
    let neuracode = NeuraCode::new(config).await?;

    // Get hotspots
    let hotspots = neuracode.code_brain.identify_hotspots().await;

    if format == "json" {
        println!("{}", serde_json::to_string_pretty(&hotspots)?);
    } else {
        if hotspots.is_empty() {
            println!("No hotspots found.");
            return Ok(());
        }

        println!("Codebase Hotspots:\n");

        for (i, hotspot) in hotspots.iter().take(limit).enumerate() {
            println!(
                "{}. {} ({})",
                i + 1,
                hotspot.node.name.cyan(),
                hotspot.node.file_path.display()
            );
            println!("   Score: {:.2}", hotspot.score);
            println!("   Reasons:");
            for reason in &hotspot.reasons {
                println!("     - {}", reason);
            }
            println!();
        }
    }

    Ok(())
}
