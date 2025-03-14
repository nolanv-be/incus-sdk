use serde::Deserialize;

/// Incus response containing Vector of strings
#[derive(Debug, Deserialize)]
pub struct IncusStrings {
    /// Status description
    /// example: "Success"
    pub status: String,
    /// Status code
    /// example: 200
    pub status_code: usize,
    /// Response type
    /// example: "sync"
    pub r#type: String,
    /// Vector of strings
    /// example: vec!["/1.0/instances/foo", "/1.0/instances/bar"]
    pub metadata: Vec<String>,
}
