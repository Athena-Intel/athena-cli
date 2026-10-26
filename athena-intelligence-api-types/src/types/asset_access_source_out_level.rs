pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The level this grant gives
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssetAccessSourceOutLevel {
    Owner,
    Edit,
    View,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AssetAccessSourceOutLevel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Owner => serializer.serialize_str("owner"),
            Self::Edit => serializer.serialize_str("edit"),
            Self::View => serializer.serialize_str("view"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AssetAccessSourceOutLevel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "owner" => Ok(Self::Owner),
            "edit" => Ok(Self::Edit),
            "view" => Ok(Self::View),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AssetAccessSourceOutLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Owner => write!(f, "owner"),
            Self::Edit => write!(f, "edit"),
            Self::View => write!(f, "view"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
