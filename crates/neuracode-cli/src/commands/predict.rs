//! Predict command

use anyhow::Result;
use colored::Colorize;
use neuracode_core::{NeuraCode, NeuraCodeConfig};

pub async fn execute(task: String, format: String) -> Result<()> {
    // Create configuration
    let config = NeuraCodeConfig::default();

    // Create NeuraCode instance
    let neuracode = NeuraCode::new(config).await?;

    // Predict context
    let context = neuracode.predict_context(&task).await?;

    if format == "json" {
        println!("{}", serde_json::to_string_pretty(&context)?);
    } else {
        println!("Task Type: {}", context.task_type.name().cyan());
        println!();

        if !context.relevant_files.is_empty() {
            println!("Relevant Files:");
            for file in &context.relevant_files {
                println!("  - {}", file.display());
            }
            println!();
        }

        if !context.test_files.is_empty() {
            println!("Test Files:");
            for file in &context.test_files {
                println!("  - {}", file.display());
            }
            println!();
        }

        if !context.recent_commits.is_empty() {
            println!("Recent Commits:");
            for commit in &context.recent_commits {
                println!("  - {}: {}", &commit.hash[..8], commit.message);
            }
            println!();
        }

        if let Some(ref arch) = context.architecture {
            println!("Architecture: {}", arch.pattern.yellow());
            println!();
        }

        if !context.dependencies.is_empty() {
            println!("Dependencies:");
            for dep in &context.dependencies {
                println!("  - {} ({})", dep.name, dep.version);
            }
            println!();
        }
    }

    Ok(())
}
