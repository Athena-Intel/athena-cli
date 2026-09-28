pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AutomationRunOutOutcome {
    Success,
    Skipped,
    Failure,
    Canceled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AutomationRunOutOutcome {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Success => serializer.serialize_str("success"),
            Self::Skipped => serializer.serialize_str("skipped"),
            Self::Failure => serializer.serialize_str("failure"),
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AutomationRunOutOutcome {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "success" => Ok(Self::Success),
            "skipped" => Ok(Self::Skipped),
            "failure" => Ok(Self::Failure),
            "canceled" => Ok(Self::Canceled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AutomationRunOutOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Success => write!(f, "success"),
            Self::Skipped => write!(f, "skipped"),
            Self::Failure => write!(f, "failure"),
            Self::Canceled => write!(f, "canceled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
