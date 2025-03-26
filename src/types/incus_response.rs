use crate::{Error, types::*};

#[derive(Debug, serde::Deserialize)]
pub struct IncusResponse(pub JsonWrapperMap);
impl TryFrom<serde_json::Value> for IncusResponse {
    type Error = crate::Error;

    fn try_from(json: serde_json::Value) -> Result<Self, Self::Error> {
        Ok(IncusResponse(json.try_into()?))
    }
}

impl IncusResponse {
    pub fn status(&self) -> Result<IncusResponseStatus, Error> {
        self.0.get_u64("status_code")?.try_into()
    }

    pub fn response_type(&self) -> Result<IncusResponseType, Error> {
        self.0.get_str("type")?.try_into()
    }

    pub fn metadata(&self) -> Result<&serde_json::Value, Error> {
        self.0.get_json_value("metadata")
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
