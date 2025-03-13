use std::collections::HashMap;

use serde::Deserialize;

/// [Instance] represents an instance
#[derive(Deserialize, Debug)]
pub struct Instance {
    /// Status description
    /// example: "Success"
    pub status: String,
    /// Status code
    /// example: 200
    pub status_code: usize,
    /// Response type
    /// example: "sync"
    pub r#type: String,
    /// Instance body data
    pub metadata: InstanceMetadata,
}

/// [InstanceMetadata] is the body data of [Instance]
#[derive(Deserialize, Debug)]
pub struct InstanceMetadata {
    /// Architecture name
    /// example: "x86_64"
    pub architecture: String,
    /// Instance configuration
    /// example: HashMap::from(["security.nesting", "true"])
    pub config: HashMap<String, String>,
    /// Instance creation timestamp
    /// example: "2021-03-23T20:00:00-04:00"
    pub created_at: String,
    /// Instance description
    /// "My test instance"
    pub description: String,
    /// Instance devices
    /// example HashMap::from([("root", HashMap::from([("path", "/"), ("pool", "default"), ("type", "disk")]))])
    pub devices: HashMap<String, HashMap<String, String>>,
    /// Whether the instance is ephemeral (deleted on shutdown)
    /// example: false
    pub ephemeral: bool,
    /// Expanded configuration (all profiles and local config merged)
    /// example: HashMap::from(["security.nesting", "true"])
    pub expanded_config: HashMap<String, String>,
    /// Expanded devices (all profiles and local devices merged)
    /// example: HashMap::from([("root", HashMap::from([("path", "/"), ("pool", "default"), ("type", "disk")]))])
    pub expanded_devices: HashMap<String, HashMap<String, String>>,
    /// Last start timestamp
    /// example: "2021-03-23T20:00:00-04:00"
    pub last_used_at: String,
    /// What cluster member this instance is located on
    /// example: server01
    pub location: String,
    /// Instance name
    /// example: "foo"
    pub name: String,
    /// List of profiles applied to the instance
    /// example: vec!["default"]
    pub profiles: Vec<String>,
    /// Instance project name
    /// example: "foo"
    pub project: String,
    /// If set, instance will be restored to the provided snapshot name
    /// example: "snap0"
    pub restore: Option<String>,
    /// Whether the instance currently has saved state on disk
    /// example: false
    pub stateful: bool,
    /// Instance status (see instance_state)
    /// example: "Running"
    pub status: String,
    /// StatusCode represents a valid operation and container status
    /// example: 200
    pub status_code: i64,
    /// The type of instance (container or virtual-machine)
    /// example: "container"
    pub r#type: String,
}
