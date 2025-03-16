use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum ServerEventMode {
    #[serde(rename = "full-mesh")]
    FullMesh,
    #[serde(rename = "hub-server")]
    HubServer,
    #[serde(rename = "hub-client")]
    HubClient,
}
