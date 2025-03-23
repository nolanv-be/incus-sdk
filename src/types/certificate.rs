use crate::{
    Error, error::FieldError, inner_split_get_str_method, inner_to_bool_method,
    inner_to_str_method, inner_to_struct_method, inner_to_vec_str_method,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct CertificateFingerprints(serde_json::Value);
impl From<serde_json::Value> for CertificateFingerprints {
    fn from(f: serde_json::Value) -> Self {
        CertificateFingerprints(f.into())
    }
}
impl CertificateFingerprints {
    pub fn inner(&self) -> serde_json::Value {
        self.0.clone()
    }

    inner_split_get_str_method!(fingerprints, "/", 3);
}

#[derive(Serialize, Debug)]
pub struct Certificate {
    pub certificate: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub projects: Option<Vec<String>>,
    pub restricted: Option<bool>,
    pub token: Option<bool>,
    pub trust_token: Option<String>,
    #[serde(rename = "type")]
    pub certificate_type: Option<CertificateType>,
}

#[derive(Deserialize, Debug)]
pub struct CertificateReturned(serde_json::Value);
impl From<serde_json::Value> for CertificateReturned {
    fn from(s: serde_json::Value) -> Self {
        CertificateReturned(s)
    }
}

impl CertificateReturned {
    pub fn inner(&self) -> serde_json::Value {
        self.0.clone()
    }

    inner_to_str_method!(certificate, "certificate");

    inner_to_str_method!(description, "description");

    inner_to_str_method!(fingerprint, "fingerprint");

    inner_to_str_method!(name, "name");

    inner_to_vec_str_method!(projects, "projects");

    inner_to_bool_method!(restricted, "restricted");

    inner_to_struct_method!(certificate_type, "type", CertificateType);
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CertificateType {
    Client,
    Server,
    Metrics,
}
impl TryFrom<&str> for CertificateType {
    type Error = crate::Error;

    fn try_from(cert: &str) -> Result<Self, Self::Error> {
        match cert {
            "client" => Ok(CertificateType::Client),
            "server" => Ok(CertificateType::Server),
            "metrics" => Ok(CertificateType::Metrics),
            _ => Err(FieldError::Unknown.into()),
        }
    }
}
impl Into<String> for CertificateType {
    fn into(self) -> String {
        match self {
            CertificateType::Client => "client".into(),
            CertificateType::Server => "server".into(),
            CertificateType::Metrics => "metrics".into(),
        }
    }
}
