use crate::{Error, error::FieldError};
use std::collections::HashMap;

pub type JsonWrapperMap = JsonWrapper<serde_json::value::Map<String, serde_json::Value>>;
pub type JsonWrapperValue = JsonWrapper<serde_json::Value>;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct JsonWrapper<IN>(IN);
impl From<&serde_json::Value> for JsonWrapperValue {
    fn from(json: &serde_json::Value) -> Self {
        JsonWrapper(json.clone())
    }
}
impl TryFrom<serde_json::Value> for JsonWrapperMap {
    type Error = crate::Error;

    fn try_from(json: serde_json::Value) -> Result<Self, Self::Error> {
        json.as_object()
            .ok_or(crate::error::FieldError::Invalid.into())
            .map(|m| JsonWrapper(m.clone()))
    }
}
impl JsonWrapperMap {
    pub fn get_bool(&self, key: &str) -> Result<bool, Error> {
        self.0
            .get(key)
            .ok_or(FieldError::Missing)?
            .as_bool()
            .ok_or(FieldError::Invalid.into())
    }

    pub fn get_u64(&self, key: &str) -> Result<u64, Error> {
        self.0
            .get(key)
            .ok_or(FieldError::Missing)?
            .as_u64()
            .ok_or(FieldError::Invalid.into())
    }

    pub fn get_str(&self, key: &str) -> Result<&str, Error> {
        self.0
            .get(key)
            .ok_or(FieldError::Missing)?
            .as_str()
            .ok_or(FieldError::Invalid.into())
    }

    pub fn get_strs(&self, key: &str) -> Result<Vec<&str>, Error> {
        self.0
            .get(key)
            .ok_or(FieldError::Missing)?
            .as_array()
            .ok_or(FieldError::Invalid)?
            .into_iter()
            .map(|item| item.as_str().ok_or(FieldError::Invalid.into()))
            .collect()
    }

    pub fn get_map_string_string(&self, key: &str) -> Result<HashMap<String, String>, Error> {
        serde_json::from_value(self.0.get(key).ok_or(FieldError::Missing)?.clone())
            .map_err(|_| FieldError::Invalid.into())
    }

    pub fn get_json_value(&self, key: &str) -> Result<&serde_json::Value, Error> {
        self.0.get(key).ok_or(FieldError::Missing.into())
    }

    pub fn get_json_values(&self, key: &str) -> Result<&Vec<serde_json::Value>, Error> {
        self.0
            .get(key)
            .ok_or(FieldError::Missing)?
            .as_array()
            .ok_or(FieldError::Invalid.into())
    }

    pub fn insert_in_map<V: serde::Serialize>(&mut self, key: &str, value: V) -> Result<(), Error> {
        self.0.insert(
            key.into(),
            serde_json::to_value(value).map_err(|_| FieldError::Invalid)?,
        );
        Ok(())
    }
}

impl JsonWrapperValue {
    pub fn as_strs(&self) -> Result<Vec<&str>, Error> {
        self.0
            .as_array()
            .ok_or(FieldError::Invalid)?
            .into_iter()
            .map(|item| item.as_str().ok_or(FieldError::Invalid.into()))
            .collect()
    }
}
