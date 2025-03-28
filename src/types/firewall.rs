#[derive(Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Firewall {
    Nftables,
    Xtables,
}
impl TryFrom<&str> for Firewall {
    type Error = crate::Error;

    fn try_from(firewall: &str) -> Result<Self, Self::Error> {
        match firewall {
            "nftables" => Ok(Firewall::Nftables),
            "xtables" => Ok(Firewall::Xtables),
            _ => Err(crate::error::FieldError::Unknown.into()),
        }
    }
}
