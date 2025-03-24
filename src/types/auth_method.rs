use crate::error::FieldError;
use serde::Serialize;

#[derive(Debug, Eq, PartialEq, Serialize)]
pub enum AuthMethod {
    Tls,
    Oidc,
}
impl TryFrom<&str> for AuthMethod {
    type Error = crate::Error;

    fn try_from(method: &str) -> Result<Self, Self::Error> {
        match method {
            "tls" => Ok(AuthMethod::Tls),
            "oidc" => Ok(AuthMethod::Oidc),
            _ => Err(FieldError::Unknown.into()),
        }
    }
}
