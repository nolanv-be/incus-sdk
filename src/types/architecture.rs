use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Architecture {
    I686,
    X86_64,
    Armv6l,
    Armv7l,
    Armv8l,
    Aarch64,
    Ppc,
    Ppc64,
    Ppc64le,
    S390x,
    Mips,
    Mips64,
    Riscv32,
    Riscv64,
    Loongarch64,
}
