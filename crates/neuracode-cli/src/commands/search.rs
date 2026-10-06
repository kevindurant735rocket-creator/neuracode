//! Search command

use anyhow::Result;
use colored::Colorize;
use neuracode_core::{NeuraCode, NeuraCodeConfig};

pub async fn execute(query: String, limit: usize, format: String) -> Result<()> {
    // Create configuration
    let config = NeuraCodeConfig::default();

    // Create NeuraCode instance
    let neuracode = NeuraCode::new(config).await?;

    // Search
    let results = neuracode.search(&query).await?;

    if format == "json" {
        println!("{}", serde_json::to_string_pretty(&results)?);
    } else {
        if results.is_empty() {
            println!("No results found for: {}", query);
            return Ok(());
        }

        println!("Found {} results for: {}\n", results.len(), query.bold());

        for (i, result) in results.iter().take(limit).enumerate() {
            println!(
                "{}. {} ({})",
                i + 1,
                result.node.name.cyan(),
                result.node.kind.name().yellow()
            );
            println!("   File: {}", result.node.file_path.display());
            println!("   Score: {:.2}", result.score);

            if let Some(ref sig) = result.node.signature {
                println!("   Signature: {}", sig.dimmed());
            }

            println!();
        }
    }

    Ok(())
}
