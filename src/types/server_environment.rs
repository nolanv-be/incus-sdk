use std::collections::HashMap;

use crate::types::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ServerEnvironment {
    addresses: Vec<String>,
    architectures: Vec<Architecture>,
    certificate: String,
    certificate_fingerprint: String,
    driver: Drivers,
    driver_version: String,
    firewall: Firewall,
    kernel: String,
    kernel_architecture: Architecture,
    kernel_features: HashMap<String, String>,
    kernel_version: String,
    lxc_features: HashMap<String, String>,
    os_name: String,
    os_version: String,
    project: String,
    server: String,
    server_clustered: bool,
    server_event_mode: ServerEventMode,
    server_name: String,
    server_pid: usize,
    server_version: String,
    storage: Storage,
    storage_supported_drivers: Vec<ServerStorageDriverInfo>,
    storage_version: String,
}
