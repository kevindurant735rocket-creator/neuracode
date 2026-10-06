//! Serve command

use anyhow::Result;

pub async fn execute(port: u16) -> Result<()> {
    println!("Starting NeuraCode MCP server on port {}...", port);
    println!();
    println!("Press Ctrl+C to stop");
    println!();

    // In production, this would start the MCP server
    // For now, just print a message
    println!("MCP server is not yet implemented.");
    println!("Please use 'neuracode install' to configure your AI agent.");

    Ok(())
}
