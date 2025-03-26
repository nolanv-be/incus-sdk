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
    get_set_json!(addresses, with_addresses, "addresses", Vec<&str>);

    get_set_json!(
        architectures,
        with_architectures,
        "architectures",
        Vec<&str>,
        Architecture
    );

    get_set_json!(certificate, with_certificate, "certificate", &str);

    get_set_json!(
        certificate_fingerprint,
        with_certificate_fingerprint,
        "certificate_fingerprint",
        &str
    );

    get_set_json!(driver, with_driver, "driver", Vec<&str>, Driver);

    get_set_json!(driver_version, with_driver_version, "driver_version", &str);

    get_set_json!(firewall, with_firewall, "firewall", Vec<&str>, Firewall);

    get_set_json!(kernel, with_kernel, "kernel", &str);

    get_set_json!(
        kernel_architecture,
        with_kernel_architecture,
        "kernel_architecture",
        Vec<&str>,
        Architecture
    );

    get_set_json!(kernel_features, with_kernel_features, "kernel_features", std::collections::HashMap<String, String>);

    get_set_json!(kernel_version, with_kernel_version, "kernel_version", &str);

    get_set_json!(lxc_features, with_lxc_features, "lxc_features", std::collections::HashMap<String, String>);

    get_set_json!(os_name, with_os_name, "os_name", &str);

    get_set_json!(project, with_project, "project", &str);

    get_set_json!(server, with_server, "server", &str);

    get_set_json!(
        server_clustered,
        with_server_clustered,
        "server_clustered",
        bool
    );

    get_set_json!(
        server_event_mode,
        with_server_event_mode,
        "server_event_mode",
        &str,
        ServerEventMode
    );

    get_set_json!(server_name, with_server_name, "server_name", &str);

    get_set_json!(server_pid, with_server_pid, "server_pid", u64);

    get_set_json!(server_version, with_server_version, "server_version", &str);

    get_set_json!(storage, with_storage, "storage", &str, Storage);

    get_set_json!(
        storage_supported_drivers,
        with_storage_supported_drivers,
        "storage_supported_drivers",
        &Vec<serde_json::Value>,
        StorageSupported
    );

    get_set_json!(
        storage_version,
        with_storage_version,
        "storage_version",
        &str
    );
}
