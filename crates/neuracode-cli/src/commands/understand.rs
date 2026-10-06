//! Understand command

use anyhow::Result;
use neuracode_core::{NeuraCode, NeuraCodeConfig};
use std::path::Path;

pub async fn execute(path: String, format: String) -> Result<()> {
    let image_path = Path::new(&path);

    if !image_path.exists() {
        anyhow::bail!("Image not found: {}", path);
    }

    // Create configuration
    let config = NeuraCodeConfig::default();

    // Create NeuraCode instance
    let neuracode = NeuraCode::new(config).await?;

    // Understand image
    let understanding = neuracode.understand_image(image_path).await?;

    if format == "json" {
        println!("{}", serde_json::to_string_pretty(&understanding)?);
    } else {
        println!("Image Type: {:?}", understanding.image_type);
        println!("Confidence: {:.2}", understanding.confidence);
        println!();

        if !understanding.content.is_empty() {
            println!("Content:");
            println!("{}", understanding.content);
            println!();
        }

        if let Some(ref diagram) = understanding.extracted_diagram {
            println!("Extracted Diagram:");
            println!("  Nodes: {}", diagram.nodes.len());
            println!("  Edges: {}", diagram.edges.len());
            println!("  Labels: {}", diagram.labels.len());
        }
    }

    Ok(())
}
