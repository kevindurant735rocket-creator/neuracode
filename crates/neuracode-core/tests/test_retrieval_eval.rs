//! Retrieval evaluation set — pins recall cases from docs/retrieval.md.
//! Fails loudly if ranking regresses; run with:
//!   cargo test -p neuracode-core --test test_retrieval_eval

use neuracode_core::{NeuraCode, NeuraCodeConfig};
use tempfile::TempDir;

fn isolated_config(dir: &TempDir) -> NeuraCodeConfig {
    let mut cfg = NeuraCodeConfig::default();
    cfg.db_path = dir.path().join("db");
    cfg
}

fn write_auth_fixture(dir: &TempDir) {
    std::fs::write(
        dir.path().join("auth.rs"),
        r#"
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
"#,
    )
    .unwrap();
}

#[tokio::test]
async fn eval_authentication_finds_authenticate_family() {
    let tmp = TempDir::new().unwrap();
    write_auth_fixture(&tmp);
    let nc = NeuraCode::new(isolated_config(&tmp)).await.unwrap();
    nc.index_codebase(tmp.path()).await.unwrap();

    let results = nc.search("authentication").await.unwrap();
    assert!(
        !results.is_empty(),
        "eval: 'authentication' must return results (stem recall)"
    );
    let names: Vec<String> = results.iter().map(|r| r.node.name.clone()).collect();
    assert!(
        names
            .iter()
            .any(|n| n.contains("authenticate") || n.contains("login")),
        "eval: expected authenticate/login family, got {:?}",
        names
    );
}

#[tokio::test]
async fn eval_exact_name_has_single_top_hit() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(
        tmp.path().join("lib.rs"),
        "pub struct CodeBrain {\n    pub x: i32,\n}\n",
    )
    .unwrap();
    let nc = NeuraCode::new(isolated_config(&tmp)).await.unwrap();
    nc.index_codebase(tmp.path()).await.unwrap();

    let results = nc.search("CodeBrain").await.unwrap();
    let exact: Vec<_> = results
        .iter()
        .filter(|r| r.node.name == "CodeBrain")
        .collect();
    assert_eq!(
        exact.len(),
        1,
        "eval: exactly one CodeBrain node expected (no duplicates), got {}",
        exact.len()
    );
}
