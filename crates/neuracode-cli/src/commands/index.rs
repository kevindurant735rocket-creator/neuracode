//! Index command

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use neuracode_core::{NeuraCode, NeuraCodeConfig};
use std::path::Path;
use std::time::Instant;

pub async fn execute(path: String, force: bool) -> Result<()> {
    let project_path = Path::new(&path);
    
    if !project_path.exists() {
        anyhow::bail!("Path does not exist: {}", path);
    }
    
    println!("Indexing codebase at: {}", project_path.display());
    
    // Create configuration
    let config = NeuraCodeConfig::default();
    
    // Create NeuraCode instance
    let neuracode = NeuraCode::new(config).await?;
    
    // Show progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.blue} {msg}")
            .unwrap(),
    );
    pb.set_message("Indexing files...");
    
    let start = Instant::now();
    
    // Index the codebase
    let report = neuracode.index_codebase(project_path).await?;
    
    pb.finish_and_clear();
    
    let duration = start.elapsed();
    
    println!("✓ Indexing complete!");
    println!("  Files indexed: {}", report.files_indexed);
    println!("  Nodes created: {}", report.nodes_created);
    println!("  Edges created: {}", report.edges_created);
    println!("  Duration: {}ms", report.duration_ms);
    println!("  Languages: {}", report.languages.iter()
        .map(|l| l.name())
        .collect::<Vec<_>>()
        .join(", "));
    
    Ok(())
}
