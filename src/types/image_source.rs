use crate::{error::FieldError, types::*};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ImageSource {
    pub alias: Option<String>,
    pub certificate: Option<String>,
    pub protocol: Option<ImageSourceProtocol>,
    pub server: Option<String>,
    pub image_type: Option<ImageType>,
    pub mode: Option<TransferMode>,
    pub r#type: Option<ImageSourceType>,
    pub url: Option<String>,
    pub name: Option<String>,
    pub fingerprint: Option<String>,
    pub secret: Option<String>,
    pub project: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TransferMode {
    Push,
    Pull,
}
impl TryFrom<&str> for TransferMode {
    type Error = crate::Error;

    fn try_from(cert: &str) -> Result<Self, Self::Error> {
        match cert {
            "push" => Ok(TransferMode::Push),
            "pull" => Ok(TransferMode::Pull),
            _ => Err(FieldError::Unknown.into()),
        }
    }
}
impl Into<String> for TransferMode {
    fn into(self) -> String {
        match self {
            TransferMode::Push => "push".into(),
            TransferMode::Pull => "pull".into(),
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ImageSourceProtocol {
    Incus,
    Direct,
    Simplestreams,
    Oci,
}
impl TryFrom<&str> for ImageSourceProtocol {
    type Error = crate::Error;

    fn try_from(cert: &str) -> Result<Self, Self::Error> {
        match cert {
            "incus" => Ok(ImageSourceProtocol::Incus),
            "direct" => Ok(ImageSourceProtocol::Direct),
            "simplestreams" => Ok(ImageSourceProtocol::Simplestreams),
            "oci" => Ok(ImageSourceProtocol::Oci),
            _ => Err(FieldError::Unknown.into()),
        }
    }
}
impl Into<String> for ImageSourceProtocol {
    fn into(self) -> String {
        match self {
            ImageSourceProtocol::Incus => "incus".into(),
            ImageSourceProtocol::Direct => "direct".into(),
            ImageSourceProtocol::Simplestreams => "simplestreams".into(),
            ImageSourceProtocol::Oci => "oci".into(),
        }
    }
}

#[derive(Serialize, Debug)]
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
            _ => Err(FieldError::Unknown.into()),
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
