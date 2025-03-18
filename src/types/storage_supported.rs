use crate::{
    Error, error::FieldError, inner_to_str_method, inner_to_struct_method, types::Storage,
};

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

    inner_to_struct_method!(name, "Name", Storage);

    pub fn remote(&self) -> Result<bool, Error> {
        self.inner()
            .get("Remote")
            .ok_or_else(|| FieldError::Missing)?
            .as_bool()
            .ok_or_else(|| FieldError::Invalid.into())
            .into()
    }
    inner_to_str_method!(version, "Version");
}
