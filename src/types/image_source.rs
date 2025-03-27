use crate::{macros::*, types::*};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct ImageSource(pub JsonWrapperMap);
impl TryFrom<&serde_json::Value> for ImageSource {
    type Error = crate::Error;

    fn try_from(json: &serde_json::Value) -> Result<Self, Self::Error> {
        Ok(ImageSource(json.clone().try_into()?))
    }
}

impl ImageSource {
    get_set_json!(alias, with_alias, "alias", &str);

    get_set_json!(
        certificate,
        with_certificate,
        "certificate",
        &serde_json::Value,
        Certificate
    );

    get_set_json!(image_type, with_image_type, "image_type", &str, ImageType);

    get_set_json!(protocol, with_protocol, "protocol", &str, ImageProtocol);

    get_set_json!(server, with_server, "server", &str);

    get_set_json!(mode, with_mode, "mode", &str, ImageTransferMode);

    get_set_json!(source_type, with_source_type, "type", &str, ImageSourceType);

    get_set_json!(url, with_url, "url", &str);

    get_set_json!(name, with_name, "name", &str);

    get_set_json!(fingerprint, with_fingerprint, "fingerprint", &str);

    get_set_json!(secret, with_secret, "secret", &str);

    get_set_json!(project, with_project, "project", &str);
}

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ImageSourceType {
    Instance,
    Snapshot,
    Image,
    Url,
}
impl TryFrom<&str> for ImageSourceType {
    type Error = crate::Error;

    fn try_from(cert: &str) -> Result<Self, Self::Error> {
        match cert {
            "instance" => Ok(ImageSourceType::Instance),
            "snapshot" => Ok(ImageSourceType::Snapshot),
            "image" => Ok(ImageSourceType::Image),
            "url" => Ok(ImageSourceType::Url),
            _ => Err(crate::error::FieldError::Unknown.into()),
        }
    }
}
impl Into<String> for ImageSourceType {
    fn into(self) -> String {
        match self {
            ImageSourceType::Instance => "instance".into(),
            ImageSourceType::Snapshot => "snapshot".into(),
            ImageSourceType::Image => "image".into(),
            ImageSourceType::Url => "url".into(),
        }
    }
}
