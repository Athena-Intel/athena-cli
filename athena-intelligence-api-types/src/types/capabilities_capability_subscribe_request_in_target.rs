pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CapabilitySubscribeRequestInTarget {
    Thread,
    Inbox,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CapabilitySubscribeRequestInTarget {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Thread => serializer.serialize_str("thread"),
            Self::Inbox => serializer.serialize_str("inbox"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CapabilitySubscribeRequestInTarget {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "thread" => Ok(Self::Thread),
            "inbox" => Ok(Self::Inbox),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CapabilitySubscribeRequestInTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Thread => write!(f, "thread"),
            Self::Inbox => write!(f, "inbox"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
