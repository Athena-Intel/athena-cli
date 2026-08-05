pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Approval requirement modes for tools.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ApprovalMode {
    NoApproval,
    RequiredWithOverride,
    RequiredNoOverride,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ApprovalMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NoApproval => serializer.serialize_str("no_approval"),
            Self::RequiredWithOverride => serializer.serialize_str("required_with_override"),
            Self::RequiredNoOverride => serializer.serialize_str("required_no_override"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ApprovalMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "no_approval" => Ok(Self::NoApproval),
            "required_with_override" => Ok(Self::RequiredWithOverride),
            "required_no_override" => Ok(Self::RequiredNoOverride),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ApprovalMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoApproval => write!(f, "no_approval"),
            Self::RequiredWithOverride => write!(f, "required_with_override"),
            Self::RequiredNoOverride => write!(f, "required_no_override"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
