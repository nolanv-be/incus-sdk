use crate::{macros::*, types::*};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ServerEnvironment(serde_json::value::Map<String, serde_json::Value>);
impl From<&serde_json::value::Map<String, serde_json::Value>> for ServerEnvironment {
    fn from(value: &serde_json::value::Map<String, serde_json::Value>) -> Self {
        ServerEnvironment(value.clone())
    }
}

impl ServerEnvironment {
    get_set_inner_map!(inner, inner_mut);

    get_set_vec_str!(addresses, with_addresses, "addresses");

    get_set_array_str_to_vec_struct!(
        architectures,
        with_architectures,
        "architectures",
        Architecture
    );

    get_set_string!(certificate, with_certificate, "certificate");

    get_set_string!(
        certificate_fingerprint,
        with_certificate_fingerprint,
        "certificate_fingerprint"
    );

    get_set_array_str_to_vec_struct!(driver, with_driver, "driver", Driver);

    get_set_string!(driver_version, with_driver_version, "driver_version");

    get_set_struct_from_string!(firewall, with_firewall, "firewall", Firewall);

    get_set_string!(kernel, with_kernel, "kernel");

    get_set_struct_from_string!(
        kernel_architecture,
        with_kernel_architecture,
        "kernel_architecture",
        Architecture
    );

    get_set_map_string_string!(kernel_features, with_kernel_features, "kernel_features");

    get_set_string!(kernel_version, with_kernel_version, "kernel_version");

    get_set_map_string_string!(lxc_features, with_lxc_features, "lxc_features");

    get_set_string!(os_name, with_os_name, "os_name");

    get_set_string!(project, with_project, "project");

    get_set_string!(server, with_server, "server");

    get_set_bool!(server_clustered, with_server_clustered, "server_clustered");

    get_set_struct_from_string!(
        server_event_mode,
        with_server_event_mode,
        "server_event_mode",
        ServerEventMode
    );

    get_set_string!(server_name, with_server_name, "server_name");

    get_set_u64!(server_pid, with_server_pid, "server_pid");

    get_set_string!(server_version, with_server_version, "server_version");

    get_set_struct_from_string!(storage, with_storage, "storage", Storage);

    get_set_vec_struct!(
        storage_supported_drivers,
        with_storage_supported_drivers,
        "storage_supported_drivers",
        StorageSupported
    );

    get_set_string!(storage_version, with_storage_version, "storage_version");
}
