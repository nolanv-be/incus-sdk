use crate::{macros::*, types::*};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct Server(pub JsonWrapperMap);
impl TryFrom<&serde_json::Value> for Server {
    type Error = crate::Error;

    fn try_from(json: &serde_json::Value) -> Result<Self, Self::Error> {
        Ok(Server(json.clone().try_into()?))
    }
}

impl Server {
    get_set_json!(
        api_extensions,
        with_api_extensions,
        "api_extensions",
        Vec<&str>
    );

    get_set_json!(api_status, with_api_status, "api_status", &str, ApiStatus);

    get_set_json!(api_version, with_api_version, "api_version", &str);

    get_set_json!(auth, with_auth, "auth", &str, Auth);

    get_set_json!(
        auth_methods,
        with_auth_methods,
        "auth_methods",
        Vec<&str>,
        AuthMethod
    );

    get_set_json!(
        auth_user_method,
        with_auth_user_method,
        "auth_user_method",
        &str
    );

    get_set_json!(auth_user_name, with_auth_user_name, "auth_user_name", &str);

    get_set_json!(config, with_config, "config", std::collections::HashMap<String, String>);

    get_set_json!(
        environment,
        with_environment,
        "environment",
        &serde_json::Value,
        ServerEnvironment
    );
}
