use crate::macros::*;

#[derive(serde::Deserialize, Debug, PartialEq, Eq)]
pub struct IncusVersion(serde_json::Value);
impl From<&serde_json::Value> for IncusVersion {
    fn from(v: &serde_json::Value) -> Self {
        IncusVersion(v.clone())
    }
}
impl IncusVersion {
    get_set_inner_value!(inner, inner_mut);

    get_unprefixed_string!(version, "/");
}
