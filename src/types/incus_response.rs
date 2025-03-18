use crate::{Error, error::FieldError, inner_to_struct_method};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IncusResponse(serde_json::Value);
impl From<serde_json::Value> for IncusResponse {
    fn from(s: serde_json::Value) -> Self {
        IncusResponse(s)
    }
}

impl IncusResponse {
    pub fn inner(&self) -> serde_json::Value {
        self.0.clone()
    }

    pub fn status(&self) -> Result<IncusResponseStatus, Error> {
        self.inner()
            .get("status_code")
            .ok_or_else(|| FieldError::Missing)?
            .as_u64()
            .ok_or_else(|| FieldError::Invalid)?
            .try_into()
    }

    inner_to_struct_method!(response_type, "type", IncusResponseType);

    pub fn data<DATA: From<serde_json::Value>>(&self) -> Result<DATA, Error> {
        self.inner()
            .get("metadata")
            .ok_or_else(|| FieldError::Missing.into())
            .map(|e| e.clone().into())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum IncusResponseType {
    Sync,
    Async,
    ErrorIncus,
}
impl TryFrom<&str> for IncusResponseType {
    type Error = crate::Error;

    fn try_from(response_type: &str) -> Result<Self, Self::Error> {
        match response_type {
            "sync" => Ok(IncusResponseType::Sync),
            "async" => Ok(IncusResponseType::Async),
            "error" => Ok(IncusResponseType::ErrorIncus),
            _ => Err(crate::error::FieldError::Unknown.into()),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum IncusResponseStatus {
    OperationCreated = 100,
    Started = 101,
    Stopped = 102,
    Running = 103,
    Cancelling = 104,
    Pending = 105,
    Starting = 106,
    Stopping = 107,
    Aborting = 108,
    Freezing = 109,
    Frozen = 110,
    Thawed = 111,
    ErrorIncus = 112,
    Ready = 113,

    Success = 200,

    Failure = 400,
    Cancelled = 401,
}
impl TryFrom<u64> for IncusResponseStatus {
    type Error = crate::Error;

    fn try_from(code: u64) -> Result<Self, Self::Error> {
        match code {
            100 => Ok(IncusResponseStatus::OperationCreated),
            101 => Ok(IncusResponseStatus::Started),
            102 => Ok(IncusResponseStatus::Stopped),
            103 => Ok(IncusResponseStatus::Running),
            104 => Ok(IncusResponseStatus::Cancelling),
            105 => Ok(IncusResponseStatus::Pending),
            106 => Ok(IncusResponseStatus::Starting),
            107 => Ok(IncusResponseStatus::Stopping),
            108 => Ok(IncusResponseStatus::Aborting),
            109 => Ok(IncusResponseStatus::Freezing),
            110 => Ok(IncusResponseStatus::Frozen),
            111 => Ok(IncusResponseStatus::Thawed),
            112 => Ok(IncusResponseStatus::ErrorIncus),
            113 => Ok(IncusResponseStatus::Ready),
            200 => Ok(IncusResponseStatus::Success),
            400 => Ok(IncusResponseStatus::Failure),
            401 => Ok(IncusResponseStatus::Cancelled),
            _ => Err(crate::error::FieldError::Unknown.into()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct IncusResponseError(serde_json::Value);
impl From<serde_json::Value> for IncusResponseError {
    fn from(s: serde_json::Value) -> Self {
        IncusResponseError(s)
    }
}

impl IncusResponseError {
    pub fn inner(&self) -> serde_json::Value {
        self.0.clone()
    }

    pub fn status(&self) -> Result<IncusResponseErrorKind, Error> {
        self.inner()
            .get("error_code")
            .ok_or_else(|| FieldError::Missing)?
            .as_u64()
            .ok_or_else(|| FieldError::Invalid)?
            .try_into()
    }

    inner_to_struct_method!(response_type, "type", IncusResponseType);
}

#[derive(Debug, Eq, PartialEq)]
pub enum IncusResponseErrorKind {
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    Conflict = 409,
    PreconditionFailed = 412,
    InternalServer = 500,
}
impl TryFrom<u64> for IncusResponseErrorKind {
    type Error = crate::Error;

    fn try_from(code: u64) -> Result<Self, Self::Error> {
        match code {
            400 => Ok(IncusResponseErrorKind::BadRequest),
            401 => Ok(IncusResponseErrorKind::Unauthorized),
            403 => Ok(IncusResponseErrorKind::Forbidden),
            404 => Ok(IncusResponseErrorKind::NotFound),
            409 => Ok(IncusResponseErrorKind::Conflict),
            412 => Ok(IncusResponseErrorKind::PreconditionFailed),
            500 => Ok(IncusResponseErrorKind::InternalServer),
            _ => Err(FieldError::Unknown.into()),
        }
    }
}
