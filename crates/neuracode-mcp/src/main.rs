//! NeuraCode MCP Server
//! 
//! This server provides MCP (Model Context Protocol) tools for AI agents.

mod server;
mod tools;

use anyhow::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("neuracode=info")
        .init();
    
    info!("Starting NeuraCode MCP Server");
    
    // Start the server
    server::run().await?;
    
    Ok(())
}
