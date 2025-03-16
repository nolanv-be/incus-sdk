use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all="kebab-case")]
pub enum InstanceType {
    Container,
    VirtualMachine,
}
