//! Init command

use anyhow::Result;
use std::path::Path;

pub async fn execute(path: String) -> Result<()> {
    let project_path = Path::new(&path);
    
    println!("Initializing NeuraCode in: {}", project_path.display());
    
    // Create .neuracode directory
    let neuracode_dir = project_path.join(".neuracode");
    std::fs::create_dir_all(&neuracode_dir)?;
    
    // Create config file
    let config_path = neuracode_dir.join("config.toml");
    if !config_path.exists() {
        let config = r#"# NeuraCode Configuration

# Cache settings
cache_size_mb = 512

# Feature flags
enable_prediction = true
enable_learning = true
enable_multimodal = true

# Supported languages
languages = ["rust", "javascript", "typescript", "python", "go", "java", "c", "cpp"]

# Ignore patterns
ignore_patterns = [
    "node_modules",
    ".git",
    "target",
    "dist",
    "build",
    "__pycache__",
    ".venv",
]

# Maximum file size to index (in bytes)
max_file_size = 1048576
"#;
        std::fs::write(&config_path, config)?;
        println!("Created configuration: {}", config_path.display());
    }
    
    // Create cache directory
    let cache_dir = neuracode_dir.join("cache");
    std::fs::create_dir_all(&cache_dir)?;
    
    println!("✓ NeuraCode initialized successfully!");
    println!("  Run 'neuracode index' to index your codebase");
    
    Ok(())
}
