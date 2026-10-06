//! Config command

use anyhow::Result;
use colored::Colorize;
use neuracode_core::NeuraCodeConfig;

pub async fn execute(full: bool) -> Result<()> {
    let config = NeuraCodeConfig::default();
    
    if full {
        println!("{}", "NeuraCode Configuration".bold());
        println!();
        println!("Database Path: {}", config.db_path.display());
        println!("Cache Size: {} MB", config.cache_size_mb);
        println!("Enable Prediction: {}", config.enable_prediction);
        println!("Enable Learning: {}", config.enable_learning);
        println!("Enable Multi-Modal: {}", config.enable_multimodal);
        println!("Max File Size: {} bytes", config.max_file_size);
        println!();
        println!("Supported Languages:");
        for lang in &config.languages {
            println!("  - {}", lang.name());
        }
        println!();
        println!("Ignore Patterns:");
        for pattern in &config.ignore_patterns {
            println!("  - {}", pattern);
        }
    } else {
        println!("NeuraCode Configuration");
        println!();
        println!("Use --full to see all settings");
    }
    
    Ok(())
}
