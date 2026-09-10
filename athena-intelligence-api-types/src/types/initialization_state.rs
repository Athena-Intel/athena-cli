pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InitializationState {
    Queued,
    Running,
    RecoveryRequired,
    Failed,
    Cancelled,
    Succeeded,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for InitializationState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Queued => serializer.serialize_str("queued"),
            Self::Running => serializer.serialize_str("running"),
            Self::RecoveryRequired => serializer.serialize_str("recovery_required"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::Cancelled => serializer.serialize_str("cancelled"),
            Self::Succeeded => serializer.serialize_str("succeeded"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for InitializationState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "queued" => Ok(Self::Queued),
            "running" => Ok(Self::Running),
            "recovery_required" => Ok(Self::RecoveryRequired),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            "succeeded" => Ok(Self::Succeeded),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for InitializationState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Queued => write!(f, "queued"),
            Self::Running => write!(f, "running"),
            Self::RecoveryRequired => write!(f, "recovery_required"),
            Self::Failed => write!(f, "failed"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Succeeded => write!(f, "succeeded"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
