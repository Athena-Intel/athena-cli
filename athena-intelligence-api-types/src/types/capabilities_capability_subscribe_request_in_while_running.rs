pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CapabilitySubscribeRequestInWhileRunning {
    Queue,
    Notify,
    Prioritize,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CapabilitySubscribeRequestInWhileRunning {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Queue => serializer.serialize_str("queue"),
            Self::Notify => serializer.serialize_str("notify"),
            Self::Prioritize => serializer.serialize_str("prioritize"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CapabilitySubscribeRequestInWhileRunning {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "queue" => Ok(Self::Queue),
            "notify" => Ok(Self::Notify),
            "prioritize" => Ok(Self::Prioritize),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CapabilitySubscribeRequestInWhileRunning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Queue => write!(f, "queue"),
            Self::Notify => write!(f, "notify"),
            Self::Prioritize => write!(f, "prioritize"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
