pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// `applied`: every Computer usage session in the window was read. `partial`: the window holds more of them than one request reads; every one from `covered_since` on was read, and usage attributed before that day may still be under the Computer's owner. `unavailable`: the read failed and `per_user` is the breakdown without attribution.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorkspaceUsageGatewayAttributionOutStatus {
    Applied,
    Partial,
    Unavailable,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WorkspaceUsageGatewayAttributionOutStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Applied => serializer.serialize_str("applied"),
            Self::Partial => serializer.serialize_str("partial"),
            Self::Unavailable => serializer.serialize_str("unavailable"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WorkspaceUsageGatewayAttributionOutStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "applied" => Ok(Self::Applied),
            "partial" => Ok(Self::Partial),
            "unavailable" => Ok(Self::Unavailable),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WorkspaceUsageGatewayAttributionOutStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Applied => write!(f, "applied"),
            Self::Partial => write!(f, "partial"),
            Self::Unavailable => write!(f, "unavailable"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
