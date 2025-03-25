#[derive(serde::Deserialize, Debug, PartialEq, Eq)]
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
    pub fn version(&self) -> Result<String, crate::Error> {
        self.inner()
            .split("/")
            .nth(1)
            .ok_or_else(|| crate::error::FieldError::Invalid.into())
            .map(|version| version.into())
    }
}
