use crate::{Error, error::FieldError, types::*};
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

    pub fn addresses(&self) -> Result<Vec<String>, Error> {
        self.inner()
            .get("addresses")
            .ok_or_else(|| FieldError::Missing)?
            .as_array()
            .ok_or_else(|| FieldError::Invalid)?
            .into_iter()
            .map(|addr| {
                addr.as_str()
                    .ok_or_else(|| FieldError::Invalid.into())
                    .map(|s| s.to_string())
            })
            .collect()
    }

    pub fn architectures(&self) -> Result<Vec<Architecture>, Error> {
        self.inner()
            .get("architectures")
            .ok_or_else(|| FieldError::Missing)?
            .as_array()
            .ok_or_else(|| FieldError::Invalid)?
            .into_iter()
            .map(|method| {
                method
                    .as_str()
                    .ok_or_else(|| FieldError::Invalid)
                    .map(|s| s.try_into())
            })
            .flatten()
            .collect::<Result<Vec<Architecture>, Error>>()
    }

    pub fn certificate(&self) -> Result<String, Error> {
        self.inner()
            .get("certificate")
            .ok_or_else(|| FieldError::Missing)?
            .as_str()
            .ok_or_else(|| FieldError::Invalid.into())
            .map(|s| s.into())
    }

    pub fn certificate_fingerprint(&self) -> Result<String, Error> {
        self.inner()
            .get("certificate_fingerprint")
            .ok_or_else(|| FieldError::Missing)?
            .as_str()
            .ok_or_else(|| FieldError::Invalid.into())
            .map(|s| s.into())
    }

    pub fn driver(&self) -> Result<Vec<Driver>, Error> {
        self.inner()
            .get("driver")
            .ok_or_else(|| FieldError::Missing)?
            .as_array()
            .ok_or_else(|| FieldError::Invalid)?
            .into_iter()
            .map(|d| {
                d.as_str()
                    .ok_or_else(|| FieldError::Invalid)
                    .map(|s| s.try_into())
            })
            .flatten()
            .collect::<Result<Vec<Driver>, Error>>()
    }

    pub fn driver_version(&self) -> Result<String, Error> {
        self.inner()
            .get("driver_version")
            .ok_or_else(|| FieldError::Missing)?
            .as_str()
            .ok_or_else(|| FieldError::Invalid.into())
            .map(|s| s.into())
    }

    pub fn firewall(&self) -> Result<Firewall, Error> {
        self.inner()
            .get("firewall")
            .ok_or_else(|| FieldError::Missing)?
            .as_str()
            .ok_or_else(|| FieldError::Invalid)?
            .try_into()
    }

    pub fn kernel(&self) -> Result<String, Error> {
        self.inner()
            .get("kernel")
            .ok_or_else(|| FieldError::Missing)?
            .as_str()
            .ok_or_else(|| FieldError::Invalid.into())
            .map(|s| s.into())
    }

    pub fn kernel_architecture(&self) -> Result<Architecture, Error> {
        self.inner()
            .get("kernel_architecture")
            .ok_or_else(|| FieldError::Missing)?
            .as_str()
            .ok_or_else(|| FieldError::Invalid)?
            .try_into()
    }

    pub fn kernel_features(&self) -> Result<HashMap<String, String>, Error> {
        serde_json::from_value(
            self.inner()
                .get("kernel_features")
                .ok_or_else(|| FieldError::Missing)?
                .clone(),
        )
        .map_err(|_| FieldError::Invalid.into())
    }

    pub fn kernel_version(&self) -> Result<String, Error> {
        self.inner()
            .get("kernel_version")
            .ok_or_else(|| FieldError::Missing)?
            .as_str()
            .ok_or_else(|| FieldError::Invalid.into())
            .map(|s| s.into())
    }

    pub fn lxc_features(&self) -> Result<HashMap<String, String>, Error> {
        serde_json::from_value(
            self.inner()
                .get("lxc_features")
                .ok_or_else(|| FieldError::Missing)?
                .clone(),
        )
        .map_err(|_| FieldError::Invalid.into())
    }

    pub fn os_name(&self) -> Result<String, Error> {
        self.inner()
            .get("os_name")
            .ok_or_else(|| FieldError::Missing)?
            .as_str()
            .ok_or_else(|| FieldError::Invalid.into())
            .map(|s| s.into())
    }

    pub fn project(&self) -> Result<String, Error> {
        self.inner()
            .get("project")
            .ok_or_else(|| FieldError::Missing)?
            .as_str()
            .ok_or_else(|| FieldError::Invalid.into())
            .map(|s| s.into())
    }

    pub fn server(&self) -> Result<String, Error> {
        self.inner()
            .get("server")
            .ok_or_else(|| FieldError::Missing)?
            .as_str()
            .ok_or_else(|| FieldError::Invalid.into())
            .map(|s| s.into())
    }

    pub fn server_clustered(&self) -> Result<bool, Error> {
        self.inner()
            .get("server_clustered")
            .ok_or_else(|| FieldError::Missing)?
            .as_bool()
            .ok_or_else(|| FieldError::Invalid.into())
            .into()
    }

    pub fn server_event_mode(&self) -> Result<ServerEventMode, Error> {
        self.inner()
            .get("server_event_mode")
            .ok_or_else(|| FieldError::Missing)?
            .as_str()
            .ok_or_else(|| FieldError::Invalid)?
            .try_into()
    }

    pub fn server_name(&self) -> Result<String, Error> {
        self.inner()
            .get("server_name")
            .ok_or_else(|| FieldError::Missing)?
            .as_str()
            .ok_or_else(|| FieldError::Invalid.into())
            .map(|s| s.into())
    }

    pub fn server_pid(&self) -> Result<u64, Error> {
        self.inner()
            .get("server_pid")
            .ok_or_else(|| FieldError::Missing)?
            .as_u64()
            .ok_or_else(|| FieldError::Invalid.into())
            .map(|s| s.into())
    }

    pub fn server_version(&self) -> Result<String, Error> {
        self.inner()
            .get("server_version")
            .ok_or_else(|| FieldError::Missing)?
            .as_str()
            .ok_or_else(|| FieldError::Invalid.into())
            .map(|s| s.into())
    }

    pub fn storage(&self) -> Result<Storage, Error> {
        self.inner()
            .get("storage")
            .ok_or_else(|| FieldError::Missing)?
            .as_str()
            .ok_or_else(|| FieldError::Invalid)?
            .try_into()
    }

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

    pub fn storage_version(&self) -> Result<String, Error> {
        self.inner()
            .get("storage_version")
            .ok_or_else(|| FieldError::Missing)?
            .as_str()
            .ok_or_else(|| FieldError::Invalid.into())
            .map(|s| s.into())
    }
}
