use serde::Deserialize;

use crate::{Error, error::FieldError};

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
    pub fn fingerprints(&self) -> Result<Vec<String>, Error> {
        self.inner()
            .as_array()
            .ok_or_else(|| FieldError::Invalid)?
            .iter()
            .map(|fingerprint| {
                fingerprint
                    .as_str()
                    .ok_or_else(|| FieldError::Invalid)
                    .map(|s| {
                        s.split("/")
                            .last()
                            .ok_or_else(|| FieldError::Invalid.into())
                            .map(|f| f.into())
                    })
            })
            .flatten()
            .collect::<Result<Vec<String>, Error>>()
    }
}
