use serde::Serialize;

pub mod post {
    use super::*;

    #[derive(Serialize, Debug)]
    pub struct Certificate {
        pub certificate: String,
        pub description: String,
        pub name: String,
        pub projects: Vec<String>,
        pub restricted: bool,
        pub token: bool,
        pub trust_token: String,
        #[serde(rename = "type")]
        pub certificate_type: CertificateType,
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CertificateType {
    Client,
    Server,
    Metrics,
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
