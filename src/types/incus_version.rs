use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct IncusVersion(String);
impl From<&str> for IncusVersion {
    fn from(v: &str) -> Self {
        IncusVersion(v.into())
    }
}
impl IncusVersion {
    pub fn inner(&self) -> String {
        self.0.clone()
    }
    pub fn version(&self) -> Option<String> {
        self.0.split_once("/").map(|(_, version)| version.into())
    }
}
