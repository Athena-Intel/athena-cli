pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExploreAssetsInputOwnershipItem {
    Mine,
    Shared,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ExploreAssetsInputOwnershipItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Mine => serializer.serialize_str("mine"),
            Self::Shared => serializer.serialize_str("shared"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ExploreAssetsInputOwnershipItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "mine" => Ok(Self::Mine),
            "shared" => Ok(Self::Shared),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ExploreAssetsInputOwnershipItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mine => write!(f, "mine"),
            Self::Shared => write!(f, "shared"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
