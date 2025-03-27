#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ImageProtocol {
    Incus,
    Direct,
    Simplestreams,
    Oci,
}
impl TryFrom<&str> for ImageProtocol {
    type Error = crate::Error;

    fn try_from(cert: &str) -> Result<Self, Self::Error> {
        match cert {
            "incus" => Ok(ImageProtocol::Incus),
            "direct" => Ok(ImageProtocol::Direct),
            "simplestreams" => Ok(ImageProtocol::Simplestreams),
            "oci" => Ok(ImageProtocol::Oci),
            _ => Err(crate::error::FieldError::Unknown.into()),
        }
    }
}
impl Into<String> for ImageProtocol {
    fn into(self) -> String {
        match self {
            ImageProtocol::Incus => "incus".into(),
            ImageProtocol::Direct => "direct".into(),
            ImageProtocol::Simplestreams => "simplestreams".into(),
            ImageProtocol::Oci => "oci".into(),
        }
    }
}
