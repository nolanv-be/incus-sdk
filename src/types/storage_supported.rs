use crate::{macros::*, types::*};

#[derive(Debug, serde::Serialize)]
pub struct StorageSupported(pub JsonWrapperMap);
impl TryFrom<&serde_json::Value> for StorageSupported {
    type Error = crate::Error;

    fn try_from(json: &serde_json::Value) -> Result<Self, Self::Error> {
        Ok(StorageSupported(json.clone().try_into()?))
    }
}

impl StorageSupported {
    get_set_json!(name, with_name, "Name", &str, Storage);

    get_set_json!(remote, with_remote, "Remote", bool);

    get_set_json!(version, with_version, "Version", &str);
}
