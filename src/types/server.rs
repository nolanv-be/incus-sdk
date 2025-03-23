use crate::{
    Error, error::FieldError, inner_to_map_str_str_method, inner_to_str_method,
    inner_to_struct_method, inner_to_vec_str_method, inner_to_vec_struct_method, types::*,
    with_method,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Server(serde_json::value::Map<String, serde_json::Value>);
impl TryFrom<&IncusResponse> for Server {
    type Error = crate::Error;

    fn try_from(response: &IncusResponse) -> Result<Self, Self::Error> {
        response
            .metadata()?
            .as_object()
            .ok_or_else(|| FieldError::Invalid.into())
            .map(|m| Server(m.clone()))
    }
}

impl Server {
    pub fn inner<'a>(&'a self) -> &'a serde_json::value::Map<String, serde_json::Value> {
        &self.0
    }
    pub fn inner_mut<'a>(
        &'a mut self,
    ) -> &'a mut serde_json::value::Map<String, serde_json::Value> {
        &mut self.0
    }

    inner_to_vec_str_method!(api_extensions, "api_extensions");
    with_method!(
        with_api_extensions,
        api_extensions,
        Vec<String>,
        "api_extensions"
    );

    inner_to_struct_method!(api_status, "api_status", ApiStatus);

    inner_to_str_method!(api_version, "api_version");

    inner_to_struct_method!(auth, "auth", Auth);

    inner_to_vec_struct_method!(auth_methods, "auth_methods", AuthMethod);

    inner_to_str_method!(auth_user_method, "auth_user_method");

    inner_to_str_method!(auth_user_name, "auth_user_name");

    inner_to_map_str_str_method!(config, "config");

    with_method!(with_config, config, HashMap<String, String>, "config");

    pub fn environment(&self) -> Result<ServerEnvironment, Error> {
        self.try_into()
    }
}
