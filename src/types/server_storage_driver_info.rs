use crate::types::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ServerStorageDriverInfo {
    name: Storage,
    remote: bool,
    version: String,
}
