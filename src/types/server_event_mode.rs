#[derive(Debug, Eq, PartialEq, serde::Serialize)]
pub enum ServerEventMode {
    FullMesh,
    HubServer,
    HubClient,
}
impl TryFrom<&str> for ServerEventMode {
    type Error = crate::Error;

    fn try_from(mode: &str) -> Result<Self, Self::Error> {
        match mode {
            "full-mesh" => Ok(ServerEventMode::FullMesh),
            "hub-server" => Ok(ServerEventMode::HubServer),
            "hub-client" => Ok(ServerEventMode::HubClient),
            _ => Err(crate::error::FieldError::Unknown.into()),
        }
    }
}
