use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("LLM error: {0}")]
    Llm(String),

    #[error("Generation error: {0}")]
    Generation(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Analysis error: {0}")]
    Analysis(String),

    #[error("Strategy error: {0}")]
    Strategy(String),

    #[error("Export error: {0}")]
    Export(String),

    #[error("Task cancelled")]
    Cancelled,
}

pub type Result<T> = std::result::Result<T, Error>;
