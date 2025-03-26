use crate::{macros::*, types::*};

#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Certificate(pub JsonWrapperMap);
impl TryFrom<&serde_json::Value> for Certificate {
    type Error = crate::Error;

    fn try_from(json: &serde_json::Value) -> Result<Self, Self::Error> {
        Ok(Certificate(json.clone().try_into()?))
    }
}

impl Certificate {
    get_set_str!(certificate, with_certificate, "certificate");

    get_set_str!(description, with_description, "description");

    get_set_str!(fingerprint, with_fingerprint, "fingerprint");

    get_set_str!(name, with_name, "name");

    get_set_strs!(projects, with_projects, "projects");

    get_set_bool!(restricted, with_restricted, "restricted");

    get_set_struct_from_str!(
        certificate_type,
        with_certificate_type,
        "type",
        CertificateType
    );

    get_set_bool!(token, with_token, "token");

    get_set_str!(trust_token, with_trust_token, "trust_token");
}

#[derive(serde::Serialize, Debug)]
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
            _ => Err(crate::error::FieldError::Unknown.into()),
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
