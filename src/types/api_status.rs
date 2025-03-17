#[derive(Debug, Eq, PartialEq)]
pub enum ApiStatus {
    Devel,
    Stable,
    Deprecated,
}
impl TryFrom<&str> for ApiStatus {
    type Error = ();

    fn try_from(api: &str) -> Result<Self, Self::Error> {
        match api {
            "devel" => Ok(ApiStatus::Devel),
            "stable" => Ok(ApiStatus::Stable),
            "deprecated" => Ok(ApiStatus::Deprecated),
            _ => Err(()),
        }
    }
}
