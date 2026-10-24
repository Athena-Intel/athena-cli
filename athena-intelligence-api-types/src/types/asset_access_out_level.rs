pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The access the user has, as Athena's permission checks decide it: 'view' can open the asset, 'edit' can also change it and manage its sharing, 'owner' created it and can edit it, 'none' cannot open it. When a check refuses what a listed source grants, the level is lower than that source, and the level is what holds.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssetAccessOutLevel {
    Owner,
    Edit,
    View,
    None,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AssetAccessOutLevel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Owner => serializer.serialize_str("owner"),
            Self::Edit => serializer.serialize_str("edit"),
            Self::View => serializer.serialize_str("view"),
            Self::None => serializer.serialize_str("none"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AssetAccessOutLevel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "owner" => Ok(Self::Owner),
            "edit" => Ok(Self::Edit),
            "view" => Ok(Self::View),
            "none" => Ok(Self::None),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AssetAccessOutLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Owner => write!(f, "owner"),
            Self::Edit => write!(f, "edit"),
            Self::View => write!(f, "view"),
            Self::None => write!(f, "none"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
