use crate::macros::*;

#[derive(serde::Deserialize, Debug, PartialEq, Eq)]
pub struct IncusVersion(serde_json::Value);
impl TryFrom<&crate::types::IncusResponse> for Vec<IncusVersion> {
    type Error = crate::Error;

    fn try_from(response: &crate::types::IncusResponse) -> Result<Self, Self::Error> {
        response
            .metadata()?
            .as_array()
            .ok_or(crate::error::FieldError::Invalid.into())
            .map(|arr| arr.iter().map(|item| IncusVersion(item.clone())).collect())
    }
}
impl IncusVersion {
    get_set_inner_value!(inner, inner_mut);

    get_unprefixed_string!(version, "/");
}
