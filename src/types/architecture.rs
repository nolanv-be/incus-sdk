use crate::error::FieldError;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
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
impl TryFrom<&str> for Architecture {
    type Error = crate::Error;

    fn try_from(arch: &str) -> Result<Self, Self::Error> {
        match arch {
            "i686" => Ok(Architecture::I686),
            "x86_64" => Ok(Architecture::X86_64),
            "armv6l" => Ok(Architecture::Armv6l),
            "armv7l" => Ok(Architecture::Armv7l),
            "armv8l" => Ok(Architecture::Armv8l),
            "aarch64" => Ok(Architecture::Aarch64),
            "ppc" => Ok(Architecture::Ppc),
            "ppc64" => Ok(Architecture::Ppc64),
            "ppc64le" => Ok(Architecture::Ppc64le),
            "s390x" => Ok(Architecture::S390x),
            "mips" => Ok(Architecture::Mips),
            "mips64" => Ok(Architecture::Mips64),
            "riscv32" => Ok(Architecture::Riscv32),
            "riscv64" => Ok(Architecture::Riscv64),
            "loongarch64" => Ok(Architecture::Loongarch64),
            _ => Err(FieldError::Unknown.into()),
        }
    }
}
