pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// `running`: the compute was already running; its idle clock was reset. `starting`: the compute is starting; poll `GET /databases/{asset_id}/compute-status` until it reports `running`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DatabaseWakeResponseOutStatus {
    Running,
    Starting,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DatabaseWakeResponseOutStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Running => serializer.serialize_str("running"),
            Self::Starting => serializer.serialize_str("starting"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DatabaseWakeResponseOutStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "running" => Ok(Self::Running),
            "starting" => Ok(Self::Starting),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DatabaseWakeResponseOutStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Running => write!(f, "running"),
            Self::Starting => write!(f, "starting"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
