use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Drivers(String);

impl Drivers {
    pub fn drivers(&self) -> Vec<String> {
        self.0.split('|').map(|d| d.trim().into()).collect()
    }
}
#[derive(Deserialize, Debug)]
pub enum DriverType {
    Lxc,
    Qemu,
}

impl Into<String> for DriverType {
    fn into(self) -> String {
        match self {
            DriverType::Lxc => "lxc".into(),
            DriverType::Qemu => "qemu".into(),
        }
    }
}
