pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The access the token carries: r reads only (the service refuses its writes), rw reads and writes. The service still checks the caller's live permission on every request.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StudioTokenResponseOutAccessType {
    R,
    Rw,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for StudioTokenResponseOutAccessType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::R => serializer.serialize_str("r"),
            Self::Rw => serializer.serialize_str("rw"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for StudioTokenResponseOutAccessType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "r" => Ok(Self::R),
            "rw" => Ok(Self::Rw),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for StudioTokenResponseOutAccessType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::R => write!(f, "r"),
            Self::Rw => write!(f, "rw"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
