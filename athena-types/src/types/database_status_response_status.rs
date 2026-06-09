pub use crate::prelude::*;
use super::*;

/// Standardized database status
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DatabaseStatusResponseStatus {
    Running,
    Suspended,
    Starting,
    Failed,
    Unknown,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DatabaseStatusResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Running => serializer.serialize_str("running"),
            Self::Suspended => serializer.serialize_str("suspended"),
            Self::Starting => serializer.serialize_str("starting"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::Unknown => serializer.serialize_str("unknown"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DatabaseStatusResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "running" => Ok(Self::Running),
            "suspended" => Ok(Self::Suspended),
            "starting" => Ok(Self::Starting),
            "failed" => Ok(Self::Failed),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DatabaseStatusResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Running => write!(f, "running"),
            Self::Suspended => write!(f, "suspended"),
            Self::Starting => write!(f, "starting"),
            Self::Failed => write!(f, "failed"),
            Self::Unknown => write!(f, "unknown"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
