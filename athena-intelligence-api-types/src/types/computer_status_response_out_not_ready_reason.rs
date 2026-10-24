pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ComputerStatusResponseOutNotReadyReason {
    NotStarted,
    Starting,
    LauncherOutdated,
    Unobservable,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ComputerStatusResponseOutNotReadyReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NotStarted => serializer.serialize_str("not_started"),
            Self::Starting => serializer.serialize_str("starting"),
            Self::LauncherOutdated => serializer.serialize_str("launcher_outdated"),
            Self::Unobservable => serializer.serialize_str("unobservable"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ComputerStatusResponseOutNotReadyReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "not_started" => Ok(Self::NotStarted),
            "starting" => Ok(Self::Starting),
            "launcher_outdated" => Ok(Self::LauncherOutdated),
            "unobservable" => Ok(Self::Unobservable),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ComputerStatusResponseOutNotReadyReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotStarted => write!(f, "not_started"),
            Self::Starting => write!(f, "starting"),
            Self::LauncherOutdated => write!(f, "launcher_outdated"),
            Self::Unobservable => write!(f, "unobservable"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
