use crate::error::FieldError;

#[derive(Debug, Eq, PartialEq)]
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
            _ => Err(FieldError::Unknown.into()),
        }
    }
}
