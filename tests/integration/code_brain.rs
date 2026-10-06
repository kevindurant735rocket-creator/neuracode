//! Integration tests for CodeBrain

use neuracode_core::{CodeBrain, NeuraCodeConfig};
use std::path::PathBuf;
use tempfile::TempDir;

#[tokio::test]
async fn test_code_brain_creation() {
    let config = NeuraCodeConfig::default();
    let code_brain = CodeBrain::new(&config).await;
    assert!(code_brain.is_ok());
}

#[tokio::test]
async fn test_index_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    let config = NeuraCodeConfig::default();
    let code_brain = CodeBrain::new(&config).await.unwrap();
    
    let report = code_brain.index(temp_dir.path()).await;
    assert!(report.is_ok());
    
    let report = report.unwrap();
    assert_eq!(report.files_indexed, 0);
}

#[tokio::test]
async fn test_index_rust_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.rs");
    
    std::fs::write(&file_path, r#"
fn main() {
    println!("Hello, world!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

struct Point {
    x: i32,
    y: i32,
}
"#).unwrap();
    
    let config = NeuraCodeConfig::default();
    let code_brain = CodeBrain::new(&config).await.unwrap();
    
    let report = code_brain.index(temp_dir.path()).await.unwrap();
    assert!(report.files_indexed > 0);
    assert!(report.nodes_created > 0);
}

#[tokio::test]
async fn test_search() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("auth.rs");
    
    std::fs::write(&file_path, r#"
fn authenticate_user(username: &str, password: &str) -> bool {
    // Authentication logic
    true
}

fn login(username: &str, password: &str) -> Result<String, String> {
    if authenticate_user(username, password) {
        Ok("token".to_string())
    } else {
        Err("Invalid credentials".to_string())
    }
}
"#).unwrap();
    
    let config = NeuraCodeConfig::default();
    let code_brain = CodeBrain::new(&config).await.unwrap();
    code_brain.index(temp_dir.path()).await.unwrap();
    
    let results = code_brain.semantic_search("authentication").await.unwrap();
    assert!(!results.is_empty());
}
