mod api_status;
mod architecture;
mod auth;
mod auth_method;
// mod certificate;
mod driver;
mod firewall;
// mod image;
// mod image_source;
mod image_type;
mod incus_response;
mod incus_version;
mod server;
mod server_environment;
mod server_event_mode;
mod storage;
mod storage_supported;

pub use api_status::ApiStatus;
pub use architecture::Architecture;
pub use auth::Auth;
pub use auth_method::AuthMethod;
use std::collections::HashMap;
// pub use certificate::*;
pub use driver::Driver;
pub use firewall::Firewall;
// pub use image::*;
// pub use image_source::*;
use crate::{Error, error::FieldError};
pub use image_type::ImageType;
pub use incus_response::*;
pub use incus_version::IncusVersion;
pub use server::Server;
pub use server_environment::ServerEnvironment;
pub use server_event_mode::ServerEventMode;
pub use storage::Storage;
pub use storage_supported::StorageSupported;

pub fn get_bool(
    map: &serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> Result<bool, Error> {
    map.get(key)
        .ok_or(FieldError::Missing)?
        .as_bool()
        .ok_or(FieldError::Invalid.into())
}

pub fn get_u64(map: &serde_json::Map<String, serde_json::Value>, key: &str) -> Result<u64, Error> {
    map.get(key)
        .ok_or(FieldError::Missing)?
        .as_u64()
        .ok_or(FieldError::Invalid.into())
}

pub fn get_string(
    map: &serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> Result<String, Error> {
    map.get(key)
        .ok_or(FieldError::Missing)?
        .as_str()
        .ok_or(FieldError::Invalid.into())
        .map(|s| s.into())
}

pub fn get_vec_string(
    map: &serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> Result<Vec<String>, Error> {
    map.get(key)
        .ok_or(FieldError::Missing)?
        .as_array()
        .ok_or(FieldError::Invalid)?
        .into_iter()
        .map(|item| {
            item.as_str()
                .ok_or(FieldError::Invalid.into())
                .map(|s| s.to_string())
        })
        .collect()
}

pub fn get_map_string_string(
    map: &serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> Result<HashMap<String, String>, Error> {
    serde_json::from_value(map.get(key).ok_or(FieldError::Missing)?.clone())
        .map_err(|_| FieldError::Invalid.into())
}

pub fn get_struct_from_string<OUT: for<'a> TryFrom<&'a str, Error = Error>>(
    map: &serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> Result<OUT, Error> {
    map.get(key)
        .ok_or(FieldError::Missing)?
        .as_str()
        .ok_or(FieldError::Invalid)?
        .try_into()
}

pub fn get_struct<OUT: for<'a> From<&'a serde_json::Map<String, serde_json::Value>>>(
    map: &serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> Result<OUT, Error> {
    map.get(key)
        .ok_or(FieldError::Missing)?
        .as_object()
        .ok_or(FieldError::Invalid.into())
        .map(|s| s.into())
}

pub fn get_vec_struct_from_array_string<OUT: for<'a> TryFrom<&'a str, Error = Error>>(
    map: &serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> Result<Vec<OUT>, Error> {
    map.get(key)
        .ok_or(FieldError::Missing)?
        .as_array()
        .ok_or(FieldError::Invalid)?
        .into_iter()
        .map(|item| {
            item.as_str()
                .ok_or(FieldError::Invalid)
                .map(|s| s.try_into())
        })
        .flatten()
        .collect::<Result<Vec<OUT>, Error>>()
}

pub fn get_vec_struct<OUT: for<'a> From<&'a serde_json::Map<String, serde_json::Value>>>(
    map: &serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> Result<Vec<OUT>, Error> {
    map.get(key)
        .ok_or(FieldError::Missing)?
        .as_array()
        .ok_or(FieldError::Invalid)?
        .into_iter()
        .map(|item| {
            item.as_object()
                .ok_or(FieldError::Invalid.into())
                .map(|s| s.into())
        })
        .collect::<Result<Vec<OUT>, Error>>()
}

pub fn get_unprefixed_string(value: &serde_json::Value, prefix: &str) -> Result<String, Error> {
    value
        .as_str()
        .ok_or(FieldError::Invalid)?
        .strip_prefix(prefix)
        .ok_or(crate::error::FieldError::Invalid.into())
        .map(|version| version.into())
}

pub fn insert_in_map<V: serde::Serialize>(
    map: &mut serde_json::Map<String, serde_json::Value>,
    key: &str,
    value: V,
) -> Result<(), Error> {
    map.insert(
        key.into(),
        serde_json::to_value(value).map_err(|_| FieldError::Invalid)?,
    );
    Ok(())
}
