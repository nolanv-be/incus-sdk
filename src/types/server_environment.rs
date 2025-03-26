use crate::{macros::*, types::*};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ServerEnvironment(pub JsonWrapperMap);
impl TryFrom<&serde_json::Value> for ServerEnvironment {
    type Error = crate::Error;

    fn try_from(json: &serde_json::Value) -> Result<Self, Self::Error> {
        Ok(ServerEnvironment(json.clone().try_into()?))
    }
}

impl ServerEnvironment {
    get_set_strs!(addresses, with_addresses, "addresses");

    get_set_structs_from_strs!(
        architectures,
        with_architectures,
        "architectures",
        Architecture
    );

    get_set_str!(certificate, with_certificate, "certificate");

    get_set_str!(
        certificate_fingerprint,
        with_certificate_fingerprint,
        "certificate_fingerprint"
    );

    get_set_structs_from_strs!(driver, with_driver, "driver", Driver);

    get_set_str!(driver_version, with_driver_version, "driver_version");

    get_set_structs_from_strs!(firewall, with_firewall, "firewall", Firewall);

    get_set_str!(kernel, with_kernel, "kernel");

    get_set_structs_from_strs!(
        kernel_architecture,
        with_kernel_architecture,
        "kernel_architecture",
        Architecture
    );

    get_set_map_string_string!(kernel_features, with_kernel_features, "kernel_features");

    get_set_str!(kernel_version, with_kernel_version, "kernel_version");

    get_set_map_string_string!(lxc_features, with_lxc_features, "lxc_features");

    get_set_str!(os_name, with_os_name, "os_name");

    get_set_str!(project, with_project, "project");

    get_set_str!(server, with_server, "server");

    get_set_bool!(server_clustered, with_server_clustered, "server_clustered");

    get_set_struct_from_str!(
        server_event_mode,
        with_server_event_mode,
        "server_event_mode",
        ServerEventMode
    );

    get_set_str!(server_name, with_server_name, "server_name");

    get_set_u64!(server_pid, with_server_pid, "server_pid");

    get_set_str!(server_version, with_server_version, "server_version");

    get_set_struct_from_str!(storage, with_storage, "storage", Storage);

    get_set_structs_from_json_values!(
        storage_supported_drivers,
        with_storage_supported_drivers,
        "storage_supported_drivers",
        StorageSupported
    );

    get_set_str!(storage_version, with_storage_version, "storage_version");
}
