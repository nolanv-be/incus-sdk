use crate::{
    Error, error::FieldError, inner_to_str_method, inner_to_struct_method, inner_to_vec_str_method,
    inner_to_vec_struct_method, types::*,
};
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

    inner_to_vec_str_method!(api_extensions, "api_extensions");

    inner_to_struct_method!(api_status, "api_status", ApiStatus);

    inner_to_str_method!(api_version, "api_version");

    inner_to_struct_method!(auth, "auth", Auth);

    inner_to_vec_struct_method!(auth_methods, "auth_methods", AuthMethod);

    inner_to_str_method!(auth_user_method, "auth_user_method");

    inner_to_str_method!(auth_user_name, "auth_user_name");

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
