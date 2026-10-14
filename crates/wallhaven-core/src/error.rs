use thiserror::Error;

/// Error categories shared by providers, services, and storage.
#[derive(Debug, Error)]
pub enum CoreError {
    #[error("network error: {0}")]
    Network(String),
    #[error("provider error: {0}")]
    Provider(String),
    #[error("unauthorized or API key required")]
    Unauthorized,
    #[error("rate limited; try again later")]
    RateLimited,
    #[error("storage error: {0}")]
    Storage(String),
    #[error("filesystem error: {0}")]
    Filesystem(String),
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("operation cancelled")]
    Cancelled,
    #[error("internal error: {0}")]
    Other(String),
}
