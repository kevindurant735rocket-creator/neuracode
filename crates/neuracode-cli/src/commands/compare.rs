//! Compare command - Compare two codebases or versions

use anyhow::Result;
use colored::Colorize;
use neuracode_core::{NeuraCode, NeuraCodeConfig};

pub async fn execute(path1: String, path2: String, format: String) -> Result<()> {
    let config = NeuraCodeConfig::default();
    let neuracode = NeuraCode::new(config).await?;

    println!("{}", "Codebase Comparison".bold().underline());
    println!();
    println!("Path 1: {}", path1.cyan());
    println!("Path 2: {}", path2.cyan());
    println!();

    // Index both paths
    let report1 = neuracode
        .index_codebase(std::path::Path::new(&path1))
        .await?;
    let report2 = neuracode
        .index_codebase(std::path::Path::new(&path2))
        .await?;

    if format == "json" {
        let output = serde_json::json!({
            "path1": {
                "path": path1,
                "files": report1.files_indexed,
                "nodes": report1.nodes_created,
                "edges": report1.edges_created,
            },
            "path2": {
                "path": path2,
                "files": report2.files_indexed,
                "nodes": report2.nodes_created,
                "edges": report2.edges_created,
            },
            "differences": {
                "files": report2.files_indexed as i64 - report1.files_indexed as i64,
                "nodes": report2.nodes_created as i64 - report1.nodes_created as i64,
                "edges": report2.edges_created as i64 - report1.edges_created as i64,
            }
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("{}", "Statistics:".bold());
        println!(
            "{:<20} {:>10} {:>10} {:>10}",
            "Metric", "Path 1", "Path 2", "Diff"
        );
        println!("{}", "─".repeat(55));
        println!(
            "{:<20} {:>10} {:>10} {:>+10}",
            "Files",
            report1.files_indexed,
            report2.files_indexed,
            report2.files_indexed as i64 - report1.files_indexed as i64
        );
        println!(
            "{:<20} {:>10} {:>10} {:>+10}",
            "Nodes",
            report1.nodes_created,
            report2.nodes_created,
            report2.nodes_created as i64 - report1.nodes_created as i64
        );
        println!(
            "{:<20} {:>10} {:>10} {:>+10}",
            "Edges",
            report1.edges_created,
            report2.edges_created,
            report2.edges_created as i64 - report1.edges_created as i64
        );
        println!();

        // Summary
        let file_diff = report2.files_indexed as i64 - report1.files_indexed as i64;
        if file_diff > 0 {
            println!("Path 2 has {} more files", file_diff);
        } else if file_diff < 0 {
            println!("Path 2 has {} fewer files", -file_diff);
        } else {
            println!("Both paths have the same number of files");
        }
    }

    Ok(())
}
