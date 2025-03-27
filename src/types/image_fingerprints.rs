use crate::{macros::*, types::*};

#[derive(serde::Deserialize, Debug)]
pub struct ImageFingerprints(pub JsonWrapperValue);
impl From<&serde_json::Value> for ImageFingerprints {
    fn from(json: &serde_json::Value) -> Self {
        ImageFingerprints(json.into())
    }
}

impl ImageFingerprints {
    get_unprefixed_strs!(fingerprints, "/1.0/images/");
}
