use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
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
