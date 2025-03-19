use crate::{
    Error, error::FieldError, inner_to_bool_method, inner_to_map_str_str_method,
    inner_to_str_method, inner_to_struct_method, inner_to_u64_method, inner_to_vec_str_method,
    inner_to_vec_struct_method, types::*,
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct ServerEnvironment(serde_json::Value);
impl From<serde_json::Value> for ServerEnvironment {
    fn from(s: serde_json::Value) -> Self {
        ServerEnvironment(s)
    }
}

impl ServerEnvironment {
    pub fn inner(&self) -> serde_json::Value {
        self.0.clone()
    }

    inner_to_vec_str_method!(addresses, "addresses");

    inner_to_vec_struct_method!(architectures, "architectures", Architecture);

    inner_to_str_method!(certificate, "certificate");

    inner_to_str_method!(certificate_fingerprint, "certificate_fingerprint");

    inner_to_vec_struct_method!(driver, "driver", Driver);

    inner_to_str_method!(driver_version, "driver_version");

    inner_to_struct_method!(firewall, "firewall", Firewall);

    inner_to_str_method!(kernel, "kernel");

    inner_to_struct_method!(kernel_architecture, "kernel_architecture", Architecture);

    inner_to_map_str_str_method!(kernel_features, "kernel_features");

    inner_to_str_method!(kernel_version, "kernel_version");

    inner_to_map_str_str_method!(lxc_features, "lxc_features");

    inner_to_str_method!(os_name, "os_name");

    inner_to_str_method!(project, "project");

    inner_to_str_method!(server, "server");

    inner_to_bool_method!(server_clustered, "server_clustered");

    inner_to_struct_method!(server_event_mode, "server_event_mode", ServerEventMode);

    inner_to_str_method!(server_name, "server_name");

    inner_to_u64_method!(server_pid, "server_pid");

    inner_to_str_method!(server_version, "server_version");

    inner_to_struct_method!(storage, "storage", Storage);

    pub fn storage_supported_drivers(&self) -> Result<Vec<StorageSupported>, Error> {
        Ok(self
            .inner()
            .get("storage_supported_drivers")
            .ok_or_else(|| FieldError::Missing)?
            .as_array()
            .ok_or_else(|| FieldError::Invalid)?
            .into_iter()
            .map(|d| d.clone().into())
            .collect())
    }

    inner_to_str_method!(storage_version, "storage_version");
}
