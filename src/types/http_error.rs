use serde::Deserialize;

/// Error returned by incus API
#[derive(Deserialize, Debug)]
pub struct HttpError {
    /// Error message
    /// example: "not authorized"
    error: String,
    /// Error code
    /// example 403
    error_code: i64,
    /// Error type
    /// example: "error"
    r#type: String,
}
