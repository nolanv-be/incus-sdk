use crate::types::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Server {
    pub api_extensions: Vec<ApiExtension>,
    pub api_status: ApiStatus,
    pub api_version: String,
    pub auth: Auth,
    pub auth_methods: Vec<AuthMethods>,
    pub auth_user_method: String,
    pub auth_user_name: String,
    pub config: HashMap<String, String>,
    pub environment: ServerEnvironment,
    pub public: bool,
}
