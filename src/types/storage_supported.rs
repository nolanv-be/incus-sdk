use crate::{Error, error::FieldError, types::Storage, *};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct StorageSupported(serde_json::value::Map<String, serde_json::Value>);
impl From<&serde_json::value::Map<String, serde_json::Value>> for StorageSupported {
    fn from(s: &serde_json::value::Map<String, serde_json::Value>) -> Self {
        StorageSupported(s.clone())
    }
}

impl StorageSupported {
    inner_method!(inner, inner_mut);

    inner_str_to_struct_method!(name, with_name, "Name", Storage);

    inner_to_bool_method!(remote, with_remote, "Remote");

    inner_to_str_method!(version, with_version, "Version");
}
