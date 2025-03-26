use crate::{macros::*, types::*};

#[derive(serde::Deserialize, Debug)]
pub struct IncusVersionSupported(pub JsonWrapperValue);
impl From<&serde_json::Value> for IncusVersionSupported {
    fn from(json: &serde_json::Value) -> Self {
        IncusVersionSupported(json.into())
    }
}

impl IncusVersionSupported {
    get_unprefixed_strs!(versions, "/");
}
