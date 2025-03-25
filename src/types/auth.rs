#[derive(Debug, Eq, PartialEq, serde::Serialize)]
pub enum Auth {
    Trusted,
    Untrusted,
}
impl TryFrom<&str> for Auth {
    type Error = crate::Error;

    fn try_from(auth: &str) -> Result<Self, Self::Error> {
        match auth {
            "trusted" => Ok(Auth::Trusted),
            "untrusted" => Ok(Auth::Untrusted),
            _ => Err(crate::error::FieldError::Unknown.into()),
        }
    }
}
