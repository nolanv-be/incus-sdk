use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct InstanceName(String);
impl InstanceName {
    pub fn new(endpoint: &str) -> Self {
        InstanceName(endpoint.into())
    }
    pub fn version(&self) -> Option<String> {
        self.0.split('/').nth(1).map(|v| v.into())
    }
    pub fn name(&self) -> Option<String> {
        self.0.split('/').nth(3).map(|v| v.into())
    }
}
