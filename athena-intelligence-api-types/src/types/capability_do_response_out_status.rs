pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// done or failed (the tool ran), blocked, needs_approval, capability_changed or not_runnable (nothing ran).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CapabilityDoResponseOutStatus {
    Done,
    Failed,
    Blocked,
    NeedsApproval,
    CapabilityChanged,
    NotRunnable,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CapabilityDoResponseOutStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Done => serializer.serialize_str("done"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::Blocked => serializer.serialize_str("blocked"),
            Self::NeedsApproval => serializer.serialize_str("needs_approval"),
            Self::CapabilityChanged => serializer.serialize_str("capability_changed"),
            Self::NotRunnable => serializer.serialize_str("not_runnable"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CapabilityDoResponseOutStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "done" => Ok(Self::Done),
            "failed" => Ok(Self::Failed),
            "blocked" => Ok(Self::Blocked),
            "needs_approval" => Ok(Self::NeedsApproval),
            "capability_changed" => Ok(Self::CapabilityChanged),
            "not_runnable" => Ok(Self::NotRunnable),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CapabilityDoResponseOutStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Done => write!(f, "done"),
            Self::Failed => write!(f, "failed"),
            Self::Blocked => write!(f, "blocked"),
            Self::NeedsApproval => write!(f, "needs_approval"),
            Self::CapabilityChanged => write!(f, "capability_changed"),
            Self::NotRunnable => write!(f, "not_runnable"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
