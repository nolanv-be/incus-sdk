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
    get_set_json!(certificate, with_certificate, "certificate", &str);

    get_set_json!(description, with_description, "description", &str);

    get_set_json!(fingerprint, with_fingerprint, "fingerprint", &str);

    get_set_json!(name, with_name, "name", &str);

    get_set_json!(projects, with_projects, "projects", Vec<&str>);

    get_set_json!(restricted, with_restricted, "restricted", bool);

    get_set_json!(
        certificate_type,
        with_certificate_type,
        "type",
        &str,
        CertificateType
    );

    get_set_json!(token, with_token, "token", bool);

    get_set_json!(trust_token, with_trust_token, "trust_token", &str);
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
