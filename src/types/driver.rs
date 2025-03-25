#[derive(Debug, Eq, PartialEq, serde::Serialize)]
pub enum Driver {
    Lxc,
    Qemu,
}
impl TryFrom<&str> for Driver {
    type Error = crate::Error;

    fn try_from(driver: &str) -> Result<Self, Self::Error> {
        match driver {
            "lxc" => Ok(Driver::Lxc),
            "qemu" => Ok(Driver::Qemu),
            _ => Err(crate::error::FieldError::Unknown.into()),
        }
    }
}
