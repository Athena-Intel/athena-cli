pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PublishIssueOutSeverity {
    Error,
    Warning,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PublishIssueOutSeverity {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Error => serializer.serialize_str("error"),
            Self::Warning => serializer.serialize_str("warning"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PublishIssueOutSeverity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "error" => Ok(Self::Error),
            "warning" => Ok(Self::Warning),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PublishIssueOutSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => write!(f, "error"),
            Self::Warning => write!(f, "warning"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
