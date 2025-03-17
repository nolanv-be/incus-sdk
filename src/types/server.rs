use serde::Deserialize;

use super::ApiStatus;

#[derive(Deserialize, Debug)]
pub struct Server(serde_json::Value);
impl From<&serde_json::Value> for Server {
    fn from(s: &serde_json::Value) -> Self {
        Server(s.clone())
    }
}

impl Server {
    pub fn inner(&self) -> serde_json::Value {
        self.0.clone()
    }
    pub fn api_extensions(&self) -> Option<Vec<String>> {
        self.0
            .get("api_extensions")?
            .as_array()?
            .into_iter()
            .map(|api| api.as_str().map(|s| s.to_string()))
            .collect::<Option<Vec<String>>>()
    }
    pub fn api_status(&self) -> Option<ApiStatus> {
        self.0.get("api_status")?.as_str()?.try_into().ok()
    }
}
