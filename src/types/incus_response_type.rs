#[derive(Debug, Eq, PartialEq)]
pub enum IncusResponseType {
    Sync,
    Async,
    ErrorIncus,
}
impl TryFrom<&str> for IncusResponseType {
    type Error = ();

    fn try_from(response_type: &str) -> Result<Self, Self::Error> {
        match response_type {
            "sync" => Ok(IncusResponseType::Sync),
            "async" => Ok(IncusResponseType::Async),
            "error" => Ok(IncusResponseType::ErrorIncus),
            _ => Err(()),
        }
    }
}
