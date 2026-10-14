//! Serve command — launches the real MCP stdio server (use `neuracode-mcp` binary directly for embedding)

use anyhow::Result;

pub async fn execute(_config_path: Option<String>, _port: u16) -> Result<()> {
    // The MCP server uses stdio transport; `neuracode-mcp` is the canonical binary.
    // `neuracode serve` execs into it so `--port` configs and docker `serve` don't silently no-op.
    println!("Starting NeuraCode MCP server (stdio)...");
    println!("Tip: for agent embedding use the `neuracode-mcp` binary directly.");
    println!();

    // Re-exec current-exe sibling or PATH lookup
    let status = tokio::process::Command::new("neuracode-mcp")
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .await;
    match status {
        Ok(s) if s.success() => Ok(()),
        Ok(s) => anyhow::bail!("neuracode-mcp exited with status: {}", s),
        Err(e) => anyhow::bail!(
            "failed to launch `neuracode-mcp` (is it installed? `cargo install --path crates/neuracode-mcp`): {}",
            e
        ),
    }
}
