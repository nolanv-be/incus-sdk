use crate::error::FieldError;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum ImageType {
    #[serde(rename = "container")]
    Container,
    #[serde(rename = "virtual-machine")]
    VirtualMachine,
}
impl TryFrom<&str> for ImageType {
    type Error = crate::Error;

    fn try_from(cert: &str) -> Result<Self, Self::Error> {
        match cert {
            "container" => Ok(ImageType::Container),
            "virtual-machine" => Ok(ImageType::VirtualMachine),
            _ => Err(FieldError::Unknown.into()),
        }
    }
}
impl Into<String> for ImageType {
    fn into(self) -> String {
        match self {
            ImageType::Container => "client".into(),
            ImageType::VirtualMachine => "virtual-machine".into(),
        }
    }
}
