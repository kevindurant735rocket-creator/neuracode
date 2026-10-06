//! Version command

use anyhow::Result;
use colored::Colorize;

pub async fn execute() -> Result<()> {
    println!("{}", "NeuraCode".bold());
    println!("Version: {}", env!("CARGO_PKG_VERSION").cyan());
    println!("License: MIT");
    println!();
    println!("{}", "The Next-Generation AI Agent Cognitive Enhancement System".dimmed());
    println!();
    println!("GitHub: https://github.com/neuracode/neuracode");
    println!("Docs: https://neuracode.github.io");
    
    Ok(())
}
