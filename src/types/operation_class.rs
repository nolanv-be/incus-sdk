#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OperationClass {
    Task,
    Token,
    Websocket,
}
impl TryFrom<&str> for OperationClass {
    type Error = crate::Error;

    fn try_from(op: &str) -> Result<Self, Self::Error> {
        match op {
            "task" => Ok(OperationClass::Task),
            "token" => Ok(OperationClass::Token),
            "websocket" => Ok(OperationClass::Websocket),
            _ => Err(crate::error::FieldError::Unknown.into()),
        }
    }
}
