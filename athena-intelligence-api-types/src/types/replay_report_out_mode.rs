pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReplayReportOutMode {
    Shadow,
    Live,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReplayReportOutMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Shadow => serializer.serialize_str("shadow"),
            Self::Live => serializer.serialize_str("live"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReplayReportOutMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "shadow" => Ok(Self::Shadow),
            "live" => Ok(Self::Live),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReplayReportOutMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Shadow => write!(f, "shadow"),
            Self::Live => write!(f, "live"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
