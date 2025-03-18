use crate::types::*;
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

    pub fn status(&self) -> Option<IncusResponseStatus> {
        self.inner().get("status_code")?.as_u64()?.try_into().ok()
    }

    pub fn response_type(&self) -> Option<IncusResponseType> {
        self.inner().get("type")?.as_str()?.try_into().ok()
    }

    pub fn data<DATA: From<serde_json::Value>>(&self) -> Option<DATA> {
        self.inner().get("metadata").map(|e| e.clone().into())
    }
}
