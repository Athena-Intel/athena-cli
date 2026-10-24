pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ComputerStatusResponseOutExplicitStop {
    Stopping,
    Stopped,
    Uncertain,
    Cleared,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ComputerStatusResponseOutExplicitStop {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Stopping => serializer.serialize_str("stopping"),
            Self::Stopped => serializer.serialize_str("stopped"),
            Self::Uncertain => serializer.serialize_str("uncertain"),
            Self::Cleared => serializer.serialize_str("cleared"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ComputerStatusResponseOutExplicitStop {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "stopping" => Ok(Self::Stopping),
            "stopped" => Ok(Self::Stopped),
            "uncertain" => Ok(Self::Uncertain),
            "cleared" => Ok(Self::Cleared),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ComputerStatusResponseOutExplicitStop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stopping => write!(f, "stopping"),
            Self::Stopped => write!(f, "stopped"),
            Self::Uncertain => write!(f, "uncertain"),
            Self::Cleared => write!(f, "cleared"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
