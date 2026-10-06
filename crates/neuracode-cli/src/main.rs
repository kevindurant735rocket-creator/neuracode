//! NeuraCode CLI - Command-line interface for NeuraCode

mod commands;

use clap::{Parser, Subcommand};
use colored::Colorize;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

/// NeuraCode - The Next-Generation AI Agent Cognitive Enhancement System
#[derive(Parser)]
#[command(name = "neuracode")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Configuration file path
    #[arg(short, long, global = true)]
    config: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize NeuraCode in the current project
    Init {
        /// Project path (defaults to current directory)
        #[arg(default_value = ".")]
        path: String,
    },

    /// Index a codebase
    Index {
        /// Project path (defaults to current directory)
        #[arg(default_value = ".")]
        path: String,

        /// Force re-indexing
        #[arg(short, long)]
        force: bool,
    },

    /// Search the codebase
    Search {
        /// Search query
        query: String,

        /// Maximum number of results
        #[arg(short, long, default_value = "10")]
        limit: usize,

        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Predict context for a task
    Predict {
        /// Task description
        task: String,

        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Analyze impact of changes
    Impact {
        /// File or function to analyze
        target: String,

        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Show architecture information
    Architecture {
        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Identify hotspots
    Hotspots {
        /// Maximum number of hotspots
        #[arg(short, long, default_value = "10")]
        limit: usize,

        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Understand an image
    Understand {
        /// Image path
        path: String,

        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Install NeuraCode for AI agents
    Install {
        /// Install for specific agent
        #[arg(short, long)]
        agent: Option<String>,

        /// Install for all supported agents
        #[arg(short, long)]
        all: bool,
    },

    /// Uninstall NeuraCode from AI agents
    Uninstall {
        /// Uninstall from specific agent
        #[arg(short, long)]
        agent: Option<String>,

        /// Uninstall from all agents
        #[arg(short, long)]
        all: bool,
    },

    /// Show statistics
    Stats,

    /// Start MCP server
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },

    /// Show configuration
    Config {
        /// Show full configuration
        #[arg(short, long)]
        full: bool,
    },

    /// Deep code analysis
    Analyze {
        /// Project path (defaults to current directory)
        #[arg(default_value = ".")]
        path: String,

        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Export code graph data
    Export {
        /// Output file path
        output: String,

        /// Export format (json, dot, mermaid)
        #[arg(short, long, default_value = "json")]
        format: String,
    },

    /// Watch for file changes and auto-index
    Watch {
        /// Project path (defaults to current directory)
        #[arg(default_value = ".")]
        path: String,
    },

    /// Compare two codebases
    Compare {
        /// First path
        path1: String,

        /// Second path
        path2: String,

        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Get AI-powered suggestions
    Suggest {
        /// Task or context description
        context: String,

        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Show version information
    Version,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let level = if cli.verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };
    let subscriber = FmtSubscriber::builder().with_max_level(level).finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("NeuraCode CLI v{}", env!("CARGO_PKG_VERSION"));

    match cli.command {
        Commands::Init { path } => {
            commands::init::execute(path).await?;
        }
        Commands::Index { path, force } => {
            commands::index::execute(path, force).await?;
        }
        Commands::Search {
            query,
            limit,
            format,
        } => {
            commands::search::execute(query, limit, format).await?;
        }
        Commands::Predict { task, format } => {
            commands::predict::execute(task, format).await?;
        }
        Commands::Impact { target, format } => {
            commands::impact::execute(target, format).await?;
        }
        Commands::Architecture { format } => {
            commands::architecture::execute(format).await?;
        }
        Commands::Hotspots { limit, format } => {
            commands::hotspots::execute(limit, format).await?;
        }
        Commands::Understand { path, format } => {
            commands::understand::execute(path, format).await?;
        }
        Commands::Install { agent, all } => {
            commands::install::execute(agent, all).await?;
        }
        Commands::Uninstall { agent, all } => {
            commands::uninstall::execute(agent, all).await?;
        }
        Commands::Stats => {
            commands::stats::execute().await?;
        }
        Commands::Serve { port } => {
            commands::serve::execute(port).await?;
        }
        Commands::Config { full } => {
            commands::config::execute(full).await?;
        }
        Commands::Analyze { path, format } => {
            commands::analyze::execute(path, format).await?;
        }
        Commands::Export { output, format } => {
            commands::export::execute(output, format).await?;
        }
        Commands::Watch { path } => {
            commands::watch::execute(path).await?;
        }
        Commands::Compare {
            path1,
            path2,
            format,
        } => {
            commands::compare::execute(path1, path2, format).await?;
        }
        Commands::Suggest { context, format } => {
            commands::suggest::execute(context, format).await?;
        }
        Commands::Version => {
            commands::version::execute().await?;
        }
    }

    Ok(())
}

/// Print a success message
pub fn print_success(msg: &str) {
    println!("{} {}", "✓".green(), msg);
}

/// Print an error message
pub fn print_error(msg: &str) {
    eprintln!("{} {}", "✗".red(), msg);
}

/// Print a warning message
pub fn print_warning(msg: &str) {
    println!("{} {}", "⚠".yellow(), msg);
}

/// Print an info message
pub fn print_info(msg: &str) {
    println!("{} {}", "ℹ".blue(), msg);
}
