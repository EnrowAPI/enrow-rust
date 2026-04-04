use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnrowError {
    #[error("Authentication failed: {message}")]
    Authentication { message: String },

    #[error("Insufficient balance: {message}")]
    InsufficientBalance { message: String },

    #[error("Rate limit exceeded: {message}")]
    RateLimit { message: String },

    #[error("API error ({status}): {message}")]
    Api { status: u16, error: String, message: String },

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, EnrowError>;

impl EnrowError {
    pub(crate) fn from_status(status: u16, error: String, message: String) -> Self {
        match status {
            401 => EnrowError::Authentication { message },
            402 => EnrowError::InsufficientBalance { message },
            429 => EnrowError::RateLimit { message },
            _ => EnrowError::Api { status, error, message },
        }
    }
}
