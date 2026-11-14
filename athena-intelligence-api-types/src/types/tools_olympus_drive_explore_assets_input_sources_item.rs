pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExploreAssetsInputSourcesItem {
    Upload,
    Email,
    Sharepoint,
    GoogleDrive,
    Athena,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ExploreAssetsInputSourcesItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Upload => serializer.serialize_str("upload"),
            Self::Email => serializer.serialize_str("email"),
            Self::Sharepoint => serializer.serialize_str("sharepoint"),
            Self::GoogleDrive => serializer.serialize_str("google_drive"),
            Self::Athena => serializer.serialize_str("athena"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ExploreAssetsInputSourcesItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "upload" => Ok(Self::Upload),
            "email" => Ok(Self::Email),
            "sharepoint" => Ok(Self::Sharepoint),
            "google_drive" => Ok(Self::GoogleDrive),
            "athena" => Ok(Self::Athena),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ExploreAssetsInputSourcesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Upload => write!(f, "upload"),
            Self::Email => write!(f, "email"),
            Self::Sharepoint => write!(f, "sharepoint"),
            Self::GoogleDrive => write!(f, "google_drive"),
            Self::Athena => write!(f, "athena"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
