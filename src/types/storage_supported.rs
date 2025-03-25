use crate::{macros::*, types::Storage};

#[derive(Debug, serde::Serialize)]
pub struct StorageSupported(serde_json::value::Map<String, serde_json::Value>);
impl From<&serde_json::value::Map<String, serde_json::Value>> for StorageSupported {
    fn from(s: &serde_json::value::Map<String, serde_json::Value>) -> Self {
        StorageSupported(s.clone())
    }
}

impl StorageSupported {
    get_set_inner_map!(inner, inner_mut);

    get_set_struct_from_string!(name, with_name, "Name", Storage);

    get_set_bool!(remote, with_remote, "Remote");

    get_set_string!(version, with_version, "Version");
}
