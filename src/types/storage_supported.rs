use crate::{Error, error::FieldError, types::Storage};

#[derive(Debug)]
pub struct StorageSupported(serde_json::Value);
impl From<serde_json::Value> for StorageSupported {
    fn from(s: serde_json::Value) -> Self {
        StorageSupported(s)
    }
}

impl StorageSupported {
    pub fn inner(&self) -> serde_json::Value {
        self.0.clone()
    }

    pub fn name(&self) -> Result<Storage, Error> {
        self.inner()
            .get("Name")
            .ok_or_else(|| FieldError::Missing)?
            .as_str()
            .ok_or_else(|| FieldError::Invalid)?
            .try_into()
    }

    pub fn remote(&self) -> Result<bool, Error> {
        self.inner()
            .get("Remote")
            .ok_or_else(|| FieldError::Missing)?
            .as_bool()
            .ok_or_else(|| FieldError::Invalid.into())
            .into()
    }

    pub fn api_version(&self) -> Result<String, Error> {
        self.inner()
            .get("Version")
            .ok_or_else(|| FieldError::Missing)?
            .as_str()
            .ok_or_else(|| FieldError::Invalid.into())
            .map(|s| s.into())
    }
}
