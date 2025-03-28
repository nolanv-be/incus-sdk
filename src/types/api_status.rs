#[derive(Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
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
            _ => Err(crate::error::FieldError::Unknown.into()),
        }
    }
}
