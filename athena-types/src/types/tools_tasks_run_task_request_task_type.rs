pub use crate::prelude::*;
use super::*;

/// Type: 'script' or 'flow'
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RunTaskRequestTaskType {
    Script,
    Flow,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RunTaskRequestTaskType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Script => serializer.serialize_str("script"),
            Self::Flow => serializer.serialize_str("flow"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RunTaskRequestTaskType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "script" => Ok(Self::Script),
            "flow" => Ok(Self::Flow),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RunTaskRequestTaskType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Script => write!(f, "script"),
            Self::Flow => write!(f, "flow"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
