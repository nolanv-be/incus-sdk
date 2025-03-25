use crate::{macros::*, types::*};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct Server(serde_json::value::Map<String, serde_json::Value>);
impl TryFrom<&IncusResponse> for Server {
    type Error = crate::Error;

    fn try_from(response: &IncusResponse) -> Result<Self, Self::Error> {
        response
            .metadata()?
            .as_object()
            .ok_or(crate::error::FieldError::Invalid.into())
            .map(|m| Server(m.clone()))
    }
}

impl Server {
    get_set_inner_map!(inner, inner_mut);

    get_set_vec_str!(api_extensions, with_api_extensions, "api_extensions");

    get_set_struct_from_string!(api_status, with_api_status, "api_status", ApiStatus);

    get_set_string!(api_version, with_api_version, "api_version");

    get_set_struct_from_string!(auth, with_auth, "auth", Auth);

    get_set_array_str_to_vec_struct!(auth_methods, with_auth_methods, "auth_methods", AuthMethod);

    get_set_string!(auth_user_method, with_auth_user_method, "auth_user_method");

    get_set_string!(auth_user_name, with_auth_user_name, "auth_user_name");

    get_set_map_string_string!(config, with_config, "config");

    get_set_struct!(
        environment,
        with_environment,
        "environment",
        ServerEnvironment
    );
}
