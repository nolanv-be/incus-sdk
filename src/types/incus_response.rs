use serde::{Deserialize, de::DeserializeOwned};

/// Incus response containing metadata
#[derive(Debug, Deserialize)]
#[serde(bound = "T: DeserializeOwned")]
pub struct IncusResponse<T: DeserializeOwned> {
    /// Status description
    /// example: "Success"
    pub status: String,
    /// Status code
    /// example: 200
    pub status_code: usize,
    /// Response type
    /// example: "sync"
    pub r#type: String,
    /// Metadata deserializable
    /// example: vec!["/1.0/instances/foo", "/1.0/instances/bar"]
    pub metadata: T,
}
