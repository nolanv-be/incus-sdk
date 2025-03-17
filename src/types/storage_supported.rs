use crate::types::Storage;

#[derive(Debug)]
pub struct StorageSupported(serde_json::Value);
impl From<&serde_json::Value> for StorageSupported {
    fn from(s: &serde_json::Value) -> Self {
        StorageSupported(s.clone())
    }
}

impl StorageSupported {
    pub fn inner(&self) -> serde_json::Value {
        self.0.clone()
    }

    pub fn name(&self) -> Option<Storage> {
        self.inner().get("Name")?.as_str()?.try_into().ok()
    }

    pub fn remote(&self) -> Option<bool> {
        Some(self.inner().get("Remote")?.as_bool()?.into())
    }

    pub fn api_version(&self) -> Option<String> {
        self.inner().get("Version")?.as_str()?.try_into().ok()
    }
}
