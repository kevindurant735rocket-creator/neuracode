//! Integration tests for PredictEngine

use neuracode_core::{PredictEngine, NeuraCodeConfig};

#[tokio::test]
async fn test_predict_engine_creation() {
    let config = NeuraCodeConfig::default();
    let predict_engine = PredictEngine::new(&config);
    // Just verify it creates successfully
}

#[tokio::test]
async fn test_classify_bug_fix() {
    let config = NeuraCodeConfig::default();
    let predict_engine = PredictEngine::new(&config);
    
    let task_type = predict_engine.classify_task("fix login bug");
    assert_eq!(task_type, neuracode_core::types::TaskType::BugFix);
}

#[tokio::test]
async fn test_classify_refactor() {
    let config = NeuraCodeConfig::default();
    let predict_engine = PredictEngine::new(&config);
    
    let task_type = predict_engine.classify_task("refactor user module");
    assert_eq!(task_type, neuracode_core::types::TaskType::Refactor);
}

#[tokio::test]
async fn test_classify_new_feature() {
    let config = NeuraCodeConfig::default();
    let predict_engine = PredictEngine::new(&config);
    
    let task_type = predict_engine.classify_task("add new feature");
    assert_eq!(task_type, neuracode_core::types::TaskType::NewFeature);
}

#[tokio::test]
async fn test_predict_context() {
    let config = NeuraCodeConfig::default();
    let predict_engine = PredictEngine::new(&config);
    
    let context = predict_engine.predict("fix bug").await;
    assert!(context.is_ok());
}
