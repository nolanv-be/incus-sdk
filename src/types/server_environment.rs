use crate::types::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct ServerEnvironment(serde_json::Value);
impl From<&serde_json::Value> for ServerEnvironment {
    fn from(s: &serde_json::Value) -> Self {
        ServerEnvironment(s.clone())
    }
}

impl ServerEnvironment {
    pub fn inner(&self) -> serde_json::Value {
        self.0.clone()
    }

    pub fn addresses(&self) -> Option<Vec<String>> {
        self.inner()
            .get("addresses")?
            .as_array()?
            .into_iter()
            .map(|addr| addr.as_str().map(|s| s.into()))
            .collect::<Option<Vec<String>>>()
    }

    pub fn architectures(&self) -> Option<Vec<Architecture>> {
        self.inner()
            .get("architectures")?
            .as_array()?
            .into_iter()
            .filter_map(|arch| arch.as_str().map(|s| s.try_into().ok()))
            .collect::<Option<Vec<Architecture>>>()
    }

    pub fn certificate(&self) -> Option<String> {
        Some(self.inner().get("certificate")?.as_str()?.into())
    }

    pub fn certificate_fingerprint(&self) -> Option<String> {
        Some(
            self.inner()
                .get("certificate_fingerprint")?
                .as_str()?
                .into(),
        )
    }

    pub fn driver(&self) -> Option<Vec<Driver>> {
        self.inner()
            .get("driver")?
            .as_str()?
            .split('|')
            .map(|d| d.trim().try_into().ok())
            .collect::<Option<Vec<Driver>>>()
    }

    pub fn driver_version(&self) -> Option<String> {
        Some(self.inner().get("driver_version")?.as_str()?.into())
    }

    pub fn firewall(&self) -> Option<Firewall> {
        self.inner().get("firewall")?.as_str()?.try_into().ok()
    }

    pub fn kernel(&self) -> Option<String> {
        Some(self.inner().get("kernel")?.as_str()?.into())
    }

    pub fn kernel_architecture(&self) -> Option<Architecture> {
        self.inner()
            .get("kernel_architecture")?
            .as_str()?
            .try_into()
            .ok()
    }

    pub fn kernel_features(&self) -> Option<HashMap<String, String>> {
        serde_json::from_value(self.inner().get("kernel_features")?.to_owned()).ok()
    }

    pub fn kernel_version(&self) -> Option<String> {
        Some(self.inner().get("kernel_version")?.as_str()?.into())
    }

    pub fn lxc_features(&self) -> Option<HashMap<String, String>> {
        serde_json::from_value(self.inner().get("lxc_features")?.to_owned()).ok()
    }

    pub fn os_name(&self) -> Option<String> {
        Some(self.inner().get("os_name")?.as_str()?.into())
    }

    pub fn project(&self) -> Option<String> {
        Some(self.inner().get("project")?.as_str()?.into())
    }

    pub fn server(&self) -> Option<String> {
        Some(self.inner().get("server")?.as_str()?.into())
    }

    pub fn server_clustered(&self) -> Option<bool> {
        Some(self.inner().get("server_clustered")?.as_bool()?.into())
    }

    pub fn server_event_mode(&self) -> Option<ServerEventMode> {
        self.inner()
            .get("server_event_mode")?
            .as_str()?
            .try_into()
            .ok()
    }

    pub fn server_name(&self) -> Option<String> {
        Some(self.inner().get("server_name")?.as_str()?.into())
    }

    pub fn server_pid(&self) -> Option<u64> {
        Some(self.inner().get("server_pid")?.as_u64()?.into())
    }

    pub fn server_version(&self) -> Option<String> {
        Some(self.inner().get("server_version")?.as_str()?.into())
    }

    pub fn storage(&self) -> Option<Storage> {
        self.inner().get("storage")?.as_str()?.try_into().ok()
    }

    pub fn storage_supported_drivers(&self) -> Option<Vec<StorageSupported>> {
        Some(
            self.inner()
                .get("storage_supported_drivers")?
                .as_array()?
                .into_iter()
                .map(|storage| storage.into())
                .collect(),
        )
    }

    pub fn storage_version(&self) -> Option<String> {
        Some(self.inner().get("storage_version")?.as_str()?.into())
    }
}
