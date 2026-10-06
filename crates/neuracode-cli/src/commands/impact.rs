//! Impact command

use anyhow::Result;
use colored::Colorize;
use neuracode_core::{NeuraCode, NeuraCodeConfig};

pub async fn execute(target: String, format: String) -> Result<()> {
    // Create configuration
    let config = NeuraCodeConfig::default();

    // Create NeuraCode instance
    let neuracode = NeuraCode::new(config).await?;

    // For now, we need to find the node by name
    // In production, this would be more sophisticated
    let results = neuracode.search(&target).await?;

    if results.is_empty() {
        println!("No matching node found for: {}", target);
        return Ok(());
    }

    let node_id = results[0].node.id;
    let report = neuracode.code_brain.impact_analysis(node_id).await?;

    if format == "json" {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!("Impact Analysis for: {}", report.target.name.cyan());
        println!("Risk Level: {}", format_risk_level(&report.risk_level));
        println!();

        if !report.direct_dependents.is_empty() {
            println!("Direct Dependents ({}):", report.direct_dependents.len());
            for dep in &report.direct_dependents {
                println!("  - {} ({})", dep.name, dep.file_path.display());
            }
            println!();
        }

        if !report.transitive_dependents.is_empty() {
            println!(
                "Transitive Dependents ({}):",
                report.transitive_dependents.len()
            );
            for dep in report.transitive_dependents.iter().take(10) {
                println!("  - {} ({})", dep.name, dep.file_path.display());
            }
            if report.transitive_dependents.len() > 10 {
                println!("  ... and {} more", report.transitive_dependents.len() - 10);
            }
            println!();
        }

        if !report.affected_tests.is_empty() {
            println!("Affected Tests ({}):", report.affected_tests.len());
            for test in &report.affected_tests {
                println!("  - {} ({})", test.name, test.file_path.display());
            }
            println!();
        }

        if !report.recommendations.is_empty() {
            println!("Recommendations:");
            for rec in &report.recommendations {
                println!("  • {}", rec.yellow());
            }
        }
    }

    Ok(())
}

fn format_risk_level(level: &neuracode_core::types::RiskLevel) -> String {
    match level {
        neuracode_core::types::RiskLevel::Low => "Low".green().to_string(),
        neuracode_core::types::RiskLevel::Medium => "Medium".yellow().to_string(),
        neuracode_core::types::RiskLevel::High => "High".red().to_string(),
        neuracode_core::types::RiskLevel::Critical => "Critical".red().bold().to_string(),
    }
}
