pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Granted access: r = read + live subscribe, rw = read-write.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CollabTokenResponseAccessType {
    R,
    Rw,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CollabTokenResponseAccessType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::R => serializer.serialize_str("r"),
            Self::Rw => serializer.serialize_str("rw"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CollabTokenResponseAccessType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "r" => Ok(Self::R),
            "rw" => Ok(Self::Rw),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CollabTokenResponseAccessType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::R => write!(f, "r"),
            Self::Rw => write!(f, "rw"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
