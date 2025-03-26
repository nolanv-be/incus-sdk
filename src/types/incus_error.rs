use crate::{Error, types::*};

#[derive(Debug, serde::Deserialize)]
pub struct IncusError(pub JsonWrapperMap);
impl TryFrom<serde_json::Value> for IncusError {
    type Error = crate::Error;

    fn try_from(json: serde_json::Value) -> Result<Self, Self::Error> {
        Ok(IncusError(json.try_into()?))
    }
}

impl IncusError {
    pub fn status(&self) -> Result<IncusErrorKind, Error> {
        self.0.get_u64("error_code")?.try_into()
    }

    pub fn response_type(&self) -> Result<IncusResponseType, Error> {
        self.0.get_str("type")?.try_into()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum IncusErrorKind {
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    Conflict = 409,
    PreconditionFailed = 412,
    InternalServer = 500,
}
impl TryFrom<u64> for IncusErrorKind {
    type Error = crate::Error;

    fn try_from(code: u64) -> Result<Self, Self::Error> {
        match code {
            400 => Ok(IncusErrorKind::BadRequest),
            401 => Ok(IncusErrorKind::Unauthorized),
            403 => Ok(IncusErrorKind::Forbidden),
            404 => Ok(IncusErrorKind::NotFound),
            409 => Ok(IncusErrorKind::Conflict),
            412 => Ok(IncusErrorKind::PreconditionFailed),
            500 => Ok(IncusErrorKind::InternalServer),
            _ => Err(crate::error::FieldError::Unknown.into()),
        }
    }
}
