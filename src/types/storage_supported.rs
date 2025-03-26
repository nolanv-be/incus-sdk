use crate::{macros::*, types::*};

#[derive(Debug, serde::Serialize)]
pub struct StorageSupported(JsonWrapperMap);
impl TryFrom<&serde_json::Value> for StorageSupported {
    type Error = crate::Error;

    fn try_from(json: &serde_json::Value) -> Result<Self, Self::Error> {
        Ok(StorageSupported(json.clone().try_into()?))
    }
}

impl StorageSupported {
    get_set_struct_from_str!(name, with_name, "Name", Storage);

    get_set_bool!(remote, with_remote, "Remote");

    get_set_str!(version, with_version, "Version");
}
