use crate::{Error, error::FieldError, types::*, *};
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
    inner_method!(inner, inner_mut);

    inner_to_vec_str_method!(api_extensions, with_api_extensions, "api_extensions");

    inner_str_to_struct_method!(api_status, with_api_status, "api_status", ApiStatus);

    inner_to_str_method!(api_version, with_api_version, "api_version");

    inner_str_to_struct_method!(auth, with_auth, "auth", Auth);

    inner_array_str_to_vec_struct_method!(
        auth_methods,
        with_auth_methods,
        "auth_methods",
        AuthMethod
    );

    inner_to_str_method!(auth_user_method, with_auth_user_method, "auth_user_method");

    inner_to_str_method!(auth_user_name, with_auth_user_name, "auth_user_name");

    inner_to_map_str_str_method!(config, with_config, "config");

    inner_object_to_struct_method!(
        environment,
        with_environment,
        "environment",
        ServerEnvironment
    );
}
