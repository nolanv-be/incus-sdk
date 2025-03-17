#[derive(Debug, Eq, PartialEq)]
pub enum Auth {
    Trusted,
    Untrusted,
}
impl TryFrom<&str> for Auth {
    type Error = ();

    fn try_from(auth: &str) -> Result<Self, Self::Error> {
        match auth {
            "trusted" => Ok(Auth::Trusted),
            "untrusted" => Ok(Auth::Untrusted),
            _ => Err(()),
        }
    }
}
