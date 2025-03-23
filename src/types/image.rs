use std::collections::HashMap;

use crate::types::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{Error, error::FieldError, inner_split_get_str_method};

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct ImageFingerprints(serde_json::Value);
impl From<serde_json::Value> for ImageFingerprints {
    fn from(f: serde_json::Value) -> Self {
        ImageFingerprints(f.into())
    }
}
impl ImageFingerprints {
    pub fn inner(&self) -> serde_json::Value {
        self.0.clone()
    }

    inner_split_get_str_method!(fingerprints, "/", 3);
}

#[derive(Serialize, Debug)]
pub struct Image {
    auto_update: Option<bool>,
    properties: Option<HashMap<String, String>>,
    public: Option<bool>,
    expires_at: Option<DateTime<Utc>>,
    profiles: Option<Vec<String>>,
    filename: Option<String>,
    source: Option<ImageSource>,
    compression_algorithm: Option<String>,
    aliases: Option<Vec<ImageAlias>>,
}

#[derive(Serialize, Debug)]
pub struct ImageAlias {
    description: Option<String>,
    name: Option<String>,
    target: Option<String>,
    r#type: Option<ImageType>,
}
