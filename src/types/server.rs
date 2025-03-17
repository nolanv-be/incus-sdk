use crate::types::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Server(serde_json::Value);
impl From<&serde_json::Value> for Server {
    fn from(s: &serde_json::Value) -> Self {
        Server(s.clone())
    }
}

impl Server {
    pub fn inner(&self) -> serde_json::Value {
        self.0.clone()
    }

    pub fn api_extensions(&self) -> Option<Vec<String>> {
        self.inner()
            .get("api_extensions")?
            .as_array()?
            .into_iter()
            .map(|api| api.as_str().map(|s| s.to_string()))
            .collect::<Option<Vec<String>>>()
    }

    pub fn api_status(&self) -> Option<ApiStatus> {
        self.inner().get("api_status")?.as_str()?.try_into().ok()
    }

    pub fn api_version(&self) -> Option<String> {
        self.inner().get("api_version")?.as_str()?.try_into().ok()
    }

    pub fn auth(&self) -> Option<Auth> {
        self.inner().get("auth")?.as_str()?.try_into().ok()
    }

    pub fn auth_methods(&self) -> Option<Vec<AuthMethod>> {
        self.inner()
            .get("auth_methods")?
            .as_array()?
            .into_iter()
            .filter_map(|api| api.as_str().map(|s| s.try_into().ok()))
            .collect::<Option<Vec<AuthMethod>>>()
    }

    pub fn auth_user_method(&self) -> Option<String> {
        Some(self.inner().get("auth_user_method")?.as_str()?.into())
    }

    pub fn auth_user_name(&self) -> Option<String> {
        Some(self.inner().get("auth_user_name")?.as_str()?.into())
    }

    pub fn config(&self) -> Option<HashMap<String, String>> {
        serde_json::from_value(self.inner().get("config")?.to_owned()).ok()
    }

    pub fn environment(&self) -> Option<ServerEnvironment> {
        self.inner().get("environment").map(|e| e.into())
    }
}
