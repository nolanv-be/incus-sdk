use crate::error::FieldError;

#[derive(Debug, Eq, PartialEq)]
pub enum Storage {
    Btrfs,
    Ceph,
    Cephfs,
    Cephobject,
    Dir,
    Lvm,
    Lvmcluster,
    Zfs,
}
impl TryFrom<&str> for Storage {
    type Error = crate::Error;

    fn try_from(storage: &str) -> Result<Self, Self::Error> {
        match storage {
            "btrfs" => Ok(Storage::Btrfs),
            "ceph" => Ok(Storage::Ceph),
            "cephfs" => Ok(Storage::Cephfs),
            "cephobject" => Ok(Storage::Cephobject),
            "dir" => Ok(Storage::Dir),
            "lvm" => Ok(Storage::Lvm),
            "lvmcluster" => Ok(Storage::Lvmcluster),
            "zfs" => Ok(Storage::Zfs),
            _ => Err(FieldError::Unknown.into()),
        }
    }
}
