#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ImageTransferMode {
    Push,
    Pull,
}
impl TryFrom<&str> for ImageTransferMode {
    type Error = crate::Error;

    fn try_from(cert: &str) -> Result<Self, Self::Error> {
        match cert {
            "push" => Ok(ImageTransferMode::Push),
            "pull" => Ok(ImageTransferMode::Pull),
            _ => Err(crate::error::FieldError::Unknown.into()),
        }
    }
}
