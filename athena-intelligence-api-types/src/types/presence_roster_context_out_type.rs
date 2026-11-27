pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// asset: a document, sheet or other asset. session: a chat, named by its thread id. project: somewhere in a project (see coarse).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PresenceRosterContextOutType {
    Asset,
    Session,
    Project,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PresenceRosterContextOutType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Asset => serializer.serialize_str("asset"),
            Self::Session => serializer.serialize_str("session"),
            Self::Project => serializer.serialize_str("project"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PresenceRosterContextOutType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "asset" => Ok(Self::Asset),
            "session" => Ok(Self::Session),
            "project" => Ok(Self::Project),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PresenceRosterContextOutType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Asset => write!(f, "asset"),
            Self::Session => write!(f, "session"),
            Self::Project => write!(f, "project"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
