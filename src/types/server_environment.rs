use crate::{error::FieldError, types::*, *};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerEnvironment(serde_json::value::Map<String, serde_json::Value>);
impl From<&serde_json::value::Map<String, serde_json::Value>> for ServerEnvironment {
    fn from(value: &serde_json::value::Map<String, serde_json::Value>) -> Self {
        ServerEnvironment(value.clone())
    }
}

impl ServerEnvironment {
    inner_method!(inner, inner_mut);

    inner_to_vec_str_method!(addresses, with_addresses, "addresses");

    inner_array_str_to_vec_struct_method!(
        architectures,
        with_architectures,
        "architectures",
        Architecture
    );

    inner_to_str_method!(certificate, with_certificate, "certificate");

    inner_to_str_method!(
        certificate_fingerprint,
        with_certificate_fingerprint,
        "certificate_fingerprint"
    );

    inner_array_str_to_vec_struct_method!(driver, with_driver, "driver", Driver);

    inner_to_str_method!(driver_version, with_driver_version, "driver_version");

    inner_str_to_struct_method!(firewall, with_firewall, "firewall", Firewall);

    inner_to_str_method!(kernel, with_kernel, "kernel");

    inner_str_to_struct_method!(
        kernel_architecture,
        with_kernel_architecture,
        "kernel_architecture",
        Architecture
    );

    inner_to_map_str_str_method!(kernel_features, with_kernel_features, "kernel_features");

    inner_to_str_method!(kernel_version, with_kernel_version, "kernel_version");

    inner_to_map_str_str_method!(lxc_features, with_lxc_features, "lxc_features");

    inner_to_str_method!(os_name, with_os_name, "os_name");

    inner_to_str_method!(project, with_project, "project");

    inner_to_str_method!(server, with_server, "server");

    inner_to_bool_method!(server_clustered, with_server_clustered, "server_clustered");

    inner_str_to_struct_method!(
        server_event_mode,
        with_server_event_mode,
        "server_event_mode",
        ServerEventMode
    );

    inner_to_str_method!(server_name, with_server_name, "server_name");

    inner_to_u64_method!(server_pid, with_server_pid, "server_pid");

    inner_to_str_method!(server_version, with_server_version, "server_version");

    inner_str_to_struct_method!(storage, with_storage, "storage", Storage);

    inner_array_object_to_vec_struct_method!(
        storage_supported_drivers,
        with_storage_supported_drivers,
        "storage_supported_drivers",
        StorageSupported
    );

    inner_to_str_method!(storage_version, with_storage_version, "storage_version");
}
