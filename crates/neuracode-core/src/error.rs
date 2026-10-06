//! Error types for NeuraCode

use thiserror::Error;

/// NeuraCode error type
#[derive(Error, Debug)]
pub enum NeuraCodeError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Index error: {0}")]
    Index(String),

    #[error("Search error: {0}")]
    Search(String),

    #[error("Prediction error: {0}")]
    Prediction(String),

    #[error("Learning error: {0}")]
    Learning(String),

    #[error("Multi-modal error: {0}")]
    MultiModal(String),

    #[error("Agent error: {0}")]
    Agent(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Ignore error: {0}")]
    Ignore(String),
}

impl From<serde_json::Error> for NeuraCodeError {
    fn from(err: serde_json::Error) -> Self {
        NeuraCodeError::Serialization(err.to_string())
    }
}

impl From<ignore::Error> for NeuraCodeError {
    fn from(err: ignore::Error) -> Self {
        NeuraCodeError::Ignore(err.to_string())
    }
}

impl From<uuid::Error> for NeuraCodeError {
    fn from(err: uuid::Error) -> Self {
        NeuraCodeError::Internal(err.to_string())
    }
}

/// Result type alias
pub type Result<T> = std::result::Result<T, NeuraCodeError>;
