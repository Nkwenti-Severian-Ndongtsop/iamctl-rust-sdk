use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Represents a stable address for a resource in the workspace.
/// Address format: type.name (e.g., realm.demo, client.demo/my-app)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResourceAddress {
    pub resource_type: String,
    pub name: String,
    pub namespace: Option<String>,
}

impl fmt::Display for ResourceAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref ns) = self.namespace {
            write!(f, "{}.{}/{}", self.resource_type, ns, self.name)
        } else {
            write!(f, "{}.{}", self.resource_type, self.name)
        }
    }
}

/// Represents the desired state of a resource as defined in configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Resource {
    pub address: ResourceAddress,
    pub spec: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

/// Represents a requested change to a resource.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChangeType {
    Create,
    Update,
    Delete,
    NoOp,
}

/// Represents a calculated diff between desired and current state.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Change {
    pub address: ResourceAddress,
    pub change_type: ChangeType,
    pub before: Option<Resource>,
    pub after: Option<Resource>,
    #[serde(default)]
    pub computed_fields: Vec<String>,
}
