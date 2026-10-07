pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CapabilitySubscribeRequestInWhenIdle {
    Wake,
    Digest,
    Hold,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CapabilitySubscribeRequestInWhenIdle {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Wake => serializer.serialize_str("wake"),
            Self::Digest => serializer.serialize_str("digest"),
            Self::Hold => serializer.serialize_str("hold"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CapabilitySubscribeRequestInWhenIdle {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "wake" => Ok(Self::Wake),
            "digest" => Ok(Self::Digest),
            "hold" => Ok(Self::Hold),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CapabilitySubscribeRequestInWhenIdle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wake => write!(f, "wake"),
            Self::Digest => write!(f, "digest"),
            Self::Hold => write!(f, "hold"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
