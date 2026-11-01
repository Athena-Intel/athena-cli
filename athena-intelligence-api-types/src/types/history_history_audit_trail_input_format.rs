pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Rows as JSON, or as CSV text.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HistoryAuditTrailInputFormat {
    Json,
    Csv,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for HistoryAuditTrailInputFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Json => serializer.serialize_str("json"),
            Self::Csv => serializer.serialize_str("csv"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for HistoryAuditTrailInputFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "json" => Ok(Self::Json),
            "csv" => Ok(Self::Csv),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for HistoryAuditTrailInputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json => write!(f, "json"),
            Self::Csv => write!(f, "csv"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
