use crate::{Error, error::FieldError, types::*};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Server(serde_json::Value);
impl From<serde_json::Value> for Server {
    fn from(s: serde_json::Value) -> Self {
        Server(s)
    }
}

impl Server {
    pub fn inner(&self) -> serde_json::Value {
        self.0.clone()
    }

    pub fn api_extensions(&self) -> Result<Vec<String>, Error> {
        self.inner()
            .get("api_extensions")
            .ok_or_else(|| FieldError::Missing)?
            .as_array()
            .ok_or_else(|| FieldError::Invalid)?
            .into_iter()
            .map(|api| {
                api.as_str()
                    .ok_or_else(|| FieldError::Invalid.into())
                    .map(|s| s.to_string())
            })
            .collect()
    }

    pub fn api_status(&self) -> Result<ApiStatus, Error> {
        self.inner()
            .get("api_status")
            .ok_or_else(|| FieldError::Missing)?
            .as_str()
            .ok_or_else(|| FieldError::Invalid)?
            .try_into()
    }

    pub fn api_version(&self) -> Result<String, Error> {
        self.inner()
            .get("api_version")
            .ok_or_else(|| FieldError::Missing)?
            .as_str()
            .ok_or_else(|| FieldError::Invalid.into())
            .map(|s| s.into())
    }

    pub fn auth(&self) -> Result<Auth, Error> {
        self.inner()
            .get("auth")
            .ok_or_else(|| FieldError::Missing)?
            .as_str()
            .ok_or_else(|| FieldError::Invalid)?
            .try_into()
    }

    pub fn auth_methods(&self) -> Result<Vec<AuthMethod>, Error> {
        self.inner()
            .get("auth_methods")
            .ok_or_else(|| FieldError::Missing)?
            .as_array()
            .ok_or_else(|| FieldError::Invalid)?
            .into_iter()
            .map(|method| {
                method
                    .as_str()
                    .ok_or_else(|| FieldError::Invalid)
                    .map(|s| s.try_into())
            })
            .flatten()
            .collect::<Result<Vec<AuthMethod>, Error>>()
    }

    pub fn auth_user_method(&self) -> Result<String, Error> {
        self.inner()
            .get("auth_user_method")
            .ok_or_else(|| FieldError::Missing)?
            .as_str()
            .ok_or_else(|| FieldError::Invalid.into())
            .map(|s| s.into())
    }

    pub fn auth_user_name(&self) -> Result<String, Error> {
        self.inner()
            .get("auth_user_name")
            .ok_or_else(|| FieldError::Missing)?
            .as_str()
            .ok_or_else(|| FieldError::Invalid.into())
            .map(|s| s.into())
    }

    pub fn config(&self) -> Result<HashMap<String, String>, Error> {
        serde_json::from_value(
            self.inner()
                .get("config")
                .ok_or_else(|| FieldError::Missing)?
                .clone(),
        )
        .map_err(|_| FieldError::Invalid.into())
    }

    pub fn environment(&self) -> Result<ServerEnvironment, Error> {
        self.inner()
            .get("environment")
            .ok_or_else(|| FieldError::Missing.into())
            .map(|e| e.clone().into())
    }
}
