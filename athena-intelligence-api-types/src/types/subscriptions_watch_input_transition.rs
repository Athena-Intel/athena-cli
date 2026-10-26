pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WatchInputTransition {
    EveryMatch,
    OnBecomeTrue,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WatchInputTransition {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::EveryMatch => serializer.serialize_str("every_match"),
            Self::OnBecomeTrue => serializer.serialize_str("on_become_true"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WatchInputTransition {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "every_match" => Ok(Self::EveryMatch),
            "on_become_true" => Ok(Self::OnBecomeTrue),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WatchInputTransition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EveryMatch => write!(f, "every_match"),
            Self::OnBecomeTrue => write!(f, "on_become_true"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
