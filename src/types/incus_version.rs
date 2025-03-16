use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct IncusVersion(pub String);
