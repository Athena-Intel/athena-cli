pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Workspace-level access to grant on an asset.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorkspaceShareAccess {
    View,
    Edit,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WorkspaceShareAccess {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::View => serializer.serialize_str("view"),
            Self::Edit => serializer.serialize_str("edit"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WorkspaceShareAccess {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "view" => Ok(Self::View),
            "edit" => Ok(Self::Edit),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WorkspaceShareAccess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::View => write!(f, "view"),
            Self::Edit => write!(f, "edit"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
