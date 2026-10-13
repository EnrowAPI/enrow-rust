use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnrowError {
    /// 401: missing or invalid API key, or a feature the account can't use.
    #[error("Authentication failed: {message}")]
    Authentication { message: String },

    /// 402: not enough credits for the request.
    #[error("Insufficient balance: {message}")]
    InsufficientBalance { message: String },

    /// 429: too many requests.
    #[error("Rate limit exceeded: {message}")]
    RateLimit { message: String },

    /// Any other error status. The API sends no error code: `error` is the HTTP reason phrase.
    #[error("API error ({status}): {message}")]
    Api { status: u16, error: String, message: String },

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, EnrowError>;

impl EnrowError {
    /// Error of a response with an error status. The body is `{"message": ...}`, or
    /// `{"reason": ..., "success": false}` for a 402 on a single endpoint.
    pub(crate) fn from_response(status: u16, body: &serde_json::Value) -> Self {
        let message = body["message"]
            .as_str()
            .or_else(|| body["reason"].as_str())
            .unwrap_or("Unknown error")
            .to_string();
        let error = reqwest::StatusCode::from_u16(status)
            .ok()
            .and_then(|code| code.canonical_reason())
            .unwrap_or("Unknown Error")
            .to_string();
        Self::from_status(status, error, message)
    }

    pub(crate) fn from_status(status: u16, error: String, message: String) -> Self {
        match status {
            401 => EnrowError::Authentication { message },
            402 => EnrowError::InsufficientBalance { message },
            429 => EnrowError::RateLimit { message },
            _ => EnrowError::Api { status, error, message },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn insufficient_credits_on_a_single_endpoint() {
        let body = json!({ "reason": "Insufficient credits", "success": false });

        match EnrowError::from_response(402, &body) {
            EnrowError::InsufficientBalance { message } => {
                assert_eq!(message, "Insufficient credits")
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn insufficient_credits_on_a_bulk_endpoint() {
        let body = json!({ "message": "insufficient_credits" });

        match EnrowError::from_response(402, &body) {
            EnrowError::InsufficientBalance { message } => {
                assert_eq!(message, "insufficient_credits")
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn a_422_is_not_read_as_insufficient_credits() {
        let body = json!({ "message": "Insufficient balance" });

        match EnrowError::from_response(422, &body) {
            EnrowError::Api { status, .. } => assert_eq!(status, 422),
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn rate_limit_and_authentication() {
        let rate_limited =
            EnrowError::from_response(429, &json!({ "message": "Too Many Requests" }));
        assert!(matches!(rate_limited, EnrowError::RateLimit { .. }));

        let unauthorized =
            EnrowError::from_response(401, &json!({ "message": "This apikey is not valid" }));
        assert!(matches!(unauthorized, EnrowError::Authentication { .. }));
    }

    #[test]
    fn other_errors_carry_the_status_and_the_message() {
        match EnrowError::from_response(400, &json!({ "message": "missing fullname" })) {
            EnrowError::Api { status, error, message } => {
                assert_eq!(status, 400);
                assert_eq!(error, "Bad Request");
                assert_eq!(message, "missing fullname");
            }
            other => panic!("unexpected error: {other:?}"),
        }

        match EnrowError::from_response(500, &serde_json::Value::Null) {
            EnrowError::Api { error, message, .. } => {
                assert_eq!(error, "Internal Server Error");
                assert_eq!(message, "Unknown error");
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
