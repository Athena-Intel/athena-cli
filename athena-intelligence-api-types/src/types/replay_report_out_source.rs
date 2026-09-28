pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReplayReportOutSource {
    Draft,
    Published,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReplayReportOutSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Draft => serializer.serialize_str("draft"),
            Self::Published => serializer.serialize_str("published"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReplayReportOutSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "draft" => Ok(Self::Draft),
            "published" => Ok(Self::Published),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReplayReportOutSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Published => write!(f, "published"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
