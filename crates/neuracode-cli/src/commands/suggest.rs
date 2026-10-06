//! Suggest command - Get AI-powered suggestions

use anyhow::Result;
use colored::Colorize;
use neuracode_core::{NeuraCode, NeuraCodeConfig};

pub async fn execute(context: String, format: String) -> Result<()> {
    let config = NeuraCodeConfig::default();
    let neuracode = NeuraCode::new(config).await?;

    // Predict context for the given task
    let context_pkg = neuracode.predict_context(&context).await?;

    if format == "json" {
        println!("{}", serde_json::to_string_pretty(&context_pkg)?);
    } else {
        println!("{}", "AI Suggestions".bold().underline());
        println!();
        println!("Task: {}", context.cyan());
        println!("Type: {}", context_pkg.task_type.name().yellow());
        println!();

        if !context_pkg.relevant_files.is_empty() {
            println!("{}", "Relevant Files:".bold());
            for file in &context_pkg.relevant_files {
                println!("  • {}", file.display());
            }
            println!();
        }

        if !context_pkg.test_files.is_empty() {
            println!("{}", "Test Files:".bold());
            for file in &context_pkg.test_files {
                println!("  • {}", file.display());
            }
            println!();
        }

        if let Some(ref arch) = context_pkg.architecture {
            println!("{}", "Architecture Context:".bold());
            println!("  Pattern: {}", arch.pattern);
            println!();
        }

        // Generate suggestions based on task type
        println!("{}", "Suggestions:".bold());
        match context_pkg.task_type {
            neuracode_core::types::TaskType::BugFix => {
                println!("  1. Identify the root cause first");
                println!("  2. Write a test that reproduces the bug");
                println!("  3. Check recent changes that might have caused it");
                println!("  4. Look at error logs and stack traces");
            }
            neuracode_core::types::TaskType::Refactor => {
                println!("  1. Ensure tests pass before refactoring");
                println!("  2. Make small, incremental changes");
                println!("  3. Preserve existing behavior");
                println!("  4. Update documentation");
            }
            neuracode_core::types::TaskType::NewFeature => {
                println!("  1. Design the API first");
                println!("  2. Write tests before implementation");
                println!("  3. Consider edge cases");
                println!("  4. Update documentation");
            }
            neuracode_core::types::TaskType::CodeReview => {
                println!("  1. Check for bugs and logic errors");
                println!("  2. Verify code style consistency");
                println!("  3. Look for security issues");
                println!("  4. Check performance implications");
            }
            neuracode_core::types::TaskType::PerformanceOptimization => {
                println!("  1. Profile before optimizing");
                println!("  2. Identify bottlenecks");
                println!("  3. Consider caching strategies");
                println!("  4. Measure after changes");
            }
            _ => {
                println!("  1. Break down the task into smaller steps");
                println!("  2. Write tests");
                println!("  3. Document your changes");
            }
        }
    }

    Ok(())
}
