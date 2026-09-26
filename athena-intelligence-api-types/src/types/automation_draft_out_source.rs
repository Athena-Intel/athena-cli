pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AutomationDraftOutSource {
    SnapshotLatest,
    Live,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AutomationDraftOutSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SnapshotLatest => serializer.serialize_str("snapshot:latest"),
            Self::Live => serializer.serialize_str("live"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AutomationDraftOutSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "snapshot:latest" => Ok(Self::SnapshotLatest),
            "live" => Ok(Self::Live),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AutomationDraftOutSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SnapshotLatest => write!(f, "snapshot:latest"),
            Self::Live => write!(f, "live"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
