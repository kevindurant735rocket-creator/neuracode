//! End-to-end regression: index → search → impact → predict → hotspots → stats.
//! Guards the full agent-facing chain in one isolated temp project.

use neuracode_core::{NeuraCode, NeuraCodeConfig};
use tempfile::TempDir;

#[tokio::test]
async fn e2e_index_search_impact_predict_hotspots() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(
        tmp.path().join("auth.rs"),
        r#"
pub fn authenticate_user(username: &str, password: &str) -> bool {
    !username.is_empty() && !password.is_empty()
}

pub fn login(username: &str, password: &str) -> Result<String, String> {
    if authenticate_user(username, password) {
        Ok("token".to_string())
    } else {
        Err("Invalid credentials".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_login() {
        assert!(login("u", "p").is_ok());
    }
}
"#,
    )
    .unwrap();

    let mut cfg = NeuraCodeConfig::default();
    cfg.db_path = tmp.path().join("db");
    let nc = NeuraCode::new(cfg).await.unwrap();

    // 1. index
    let report = nc.index_codebase(tmp.path()).await.unwrap();
    assert!(report.files_indexed >= 1, "e2e: expected files indexed");
    assert!(report.nodes_created >= 2, "e2e: expected nodes created");

    // 2. search
    let hits = nc.search("authentication").await.unwrap();
    assert!(!hits.is_empty(), "e2e: search must hit");
    let first_id = hits[0].node.id;

    // 3. impact (must not error, risk level present)
    let impact = nc.code_brain.impact_analysis(first_id).await.unwrap();
    let _ = format!("{:?}", impact.risk_level);

    // 4. predict (bug-fix task classifies correctly)
    let ctx = nc.predict_context("fix login bug").await.unwrap();
    assert_eq!(
        ctx.task_type,
        neuracode_core::TaskType::BugFix,
        "e2e: 'fix login bug' must classify as BugFix"
    );

    // 5. hotspots + counts (stats path)
    let (nodes, edges) = nc.code_brain.counts();
    assert!(nodes >= 2, "e2e: counts must reflect indexed graph");
    let _ = (nodes, edges);
    let _hotspots = nc.code_brain.identify_hotspots().await;

    // 6. re-index stability: counts must not grow unboundedly
    let (n1, _) = nc.code_brain.counts();
    nc.index_codebase(tmp.path()).await.unwrap();
    let (n2, _) = nc.code_brain.counts();
    assert!(
        n2 <= n1 + 2,
        "e2e: re-index must not duplicate nodes ({} -> {})",
        n1,
        n2
    );
}
