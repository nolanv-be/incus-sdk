use crate::{macros::*, types::*};

#[derive(serde::Deserialize, Debug)]
pub struct CertificateFingerprints(pub JsonWrapperValue);
impl From<&serde_json::Value> for CertificateFingerprints {
    fn from(json: &serde_json::Value) -> Self {
        CertificateFingerprints(json.into())
    }
}

impl CertificateFingerprints {
    get_unprefixed_strs!(fingerprints, "/1.0/certificates/");
}
