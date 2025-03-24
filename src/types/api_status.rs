use crate::error::FieldError;
use serde::Serialize;

#[derive(Debug, Eq, PartialEq, Serialize)]
pub enum ApiStatus {
    Devel,
    Stable,
    Deprecated,
}
impl TryFrom<&str> for ApiStatus {
    type Error = crate::Error;

    fn try_from(api: &str) -> Result<Self, Self::Error> {
        match api {
            "devel" => Ok(ApiStatus::Devel),
            "stable" => Ok(ApiStatus::Stable),
            "deprecated" => Ok(ApiStatus::Deprecated),
            _ => Err(FieldError::Unknown.into()),
        }
    }
}
