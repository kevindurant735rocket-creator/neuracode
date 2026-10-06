//! Watch command - Watch for file changes and auto-index

use anyhow::Result;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use neuracode_core::{NeuraCode, NeuraCodeConfig};
use std::path::Path;
use std::time::Duration;

pub async fn execute(path: String) -> Result<()> {
    let project_path = Path::new(&path);

    if !project_path.exists() {
        anyhow::bail!("Path does not exist: {}", path);
    }

    println!("{}", "👁️  NeuraCode Watch Mode".bold());
    println!("Watching: {}", project_path.display());
    println!("Press Ctrl+C to stop");
    println!();

    let config = NeuraCodeConfig::default();
    let neuracode = NeuraCode::new(config).await?;

    // Initial index
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.blue} {msg}")
            .unwrap(),
    );
    pb.set_message("Initial indexing...");

    let report = neuracode.index_codebase(project_path).await?;
    pb.finish_with_message(format!(
        "Indexed {} files, {} nodes",
        report.files_indexed, report.nodes_created
    ));

    println!();
    println!("{}", "Watching for changes...".green());

    // In production, this would use notify crate for file watching
    // For now, just poll for changes
    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;

        // Check for changes (simplified)
        pb.set_message("Checking for changes...");

        // Re-index if needed
        // In production, this would be incremental
        let _ = neuracode.index_codebase(project_path).await;

        pb.set_message("Watching...");
    }
}
