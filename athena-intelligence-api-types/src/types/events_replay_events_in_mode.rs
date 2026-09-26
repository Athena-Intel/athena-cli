pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// `shadow` (the default) evaluates and writes nothing; `live` re-sends each would-fire event as a copy that fires only this automation's rules, starting its runs (needs `source: published` and EDIT on the automation)
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReplayEventsInMode {
    Shadow,
    Live,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReplayEventsInMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Shadow => serializer.serialize_str("shadow"),
            Self::Live => serializer.serialize_str("live"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReplayEventsInMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "shadow" => Ok(Self::Shadow),
            "live" => Ok(Self::Live),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReplayEventsInMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Shadow => write!(f, "shadow"),
            Self::Live => write!(f, "live"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
