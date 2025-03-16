use crate::types::*;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;

/// [Instance] represents an instance
#[derive(Deserialize, Debug)]
pub struct Instance {
    /// Architecture name
    /// example: Architecture::X86_64
    pub architecture: Architecture,
    /// Instance configuration
    /// example: HashMap::from(["security.nesting", "true"])
    pub config: HashMap<String, String>,
    /// Instance creation timestamp
    /// example: "2021-03-23T20:00:00-04:00"
    pub created_at: DateTime<Utc>,
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
    pub last_used_at: DateTime<Utc>,
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
    pub status: Status,
    /// StatusCode represents a valid operation and container status
    /// example: 200
    pub status_code: usize,
    /// The type of instance (container or virtual-machine)
    /// example: "container"
    pub r#type: InstanceType,
}
