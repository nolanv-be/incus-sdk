use crate::{macros::*, types::*};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct Server(JsonWrapperMap);
impl TryFrom<&serde_json::Value> for Server {
    type Error = crate::Error;

    fn try_from(json: &serde_json::Value) -> Result<Self, Self::Error> {
        Ok(Server(json.clone().try_into()?))
    }
}

impl Server {
    get_set_strs!(api_extensions, with_api_extensions, "api_extensions");

    get_set_struct_from_str!(api_status, with_api_status, "api_status", ApiStatus);

    get_set_str!(api_version, with_api_version, "api_version");

    get_set_struct_from_str!(auth, with_auth, "auth", Auth);

    get_set_structs_from_strs!(auth_methods, with_auth_methods, "auth_methods", AuthMethod);

    get_set_str!(auth_user_method, with_auth_user_method, "auth_user_method");

    get_set_str!(auth_user_name, with_auth_user_name, "auth_user_name");

    get_set_map_string_string!(config, with_config, "config");

    get_set_struct_from_json_value!(
        environment,
        with_environment,
        "environment",
        ServerEnvironment
    );
}
