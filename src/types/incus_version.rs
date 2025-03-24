use serde::Deserialize;

use crate::{Error, error::FieldError};

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct IncusVersion(String);
impl From<&str> for IncusVersion {
    fn from(v: &str) -> Self {
        IncusVersion(v.into())
    }
}
impl IncusVersion {
    pub fn inner(&self) -> String {
        self.0.clone()
    }
    pub fn version(&self) -> Result<String, Error> {
        self.inner()
            .split("/")
            .nth(1)
            .ok_or_else(|| FieldError::Invalid.into())
            .map(|version| version.into())
    }
}
