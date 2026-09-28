pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// resume: continue the session with the note; notify: only tell the caller
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FollowUpCreateRequestInThen {
    Resume,
    Notify,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FollowUpCreateRequestInThen {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Resume => serializer.serialize_str("resume"),
            Self::Notify => serializer.serialize_str("notify"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FollowUpCreateRequestInThen {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "resume" => Ok(Self::Resume),
            "notify" => Ok(Self::Notify),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FollowUpCreateRequestInThen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Resume => write!(f, "resume"),
            Self::Notify => write!(f, "notify"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
