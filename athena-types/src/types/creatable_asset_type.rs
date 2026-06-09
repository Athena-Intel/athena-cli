pub use crate::prelude::*;
use super::*;

/// Enum for asset types that can be created via the API.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreatableAssetType {
    Spreadsheet,
    Document,
    Folder,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreatableAssetType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Spreadsheet => serializer.serialize_str("spreadsheet"),
            Self::Document => serializer.serialize_str("document"),
            Self::Folder => serializer.serialize_str("folder"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreatableAssetType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "spreadsheet" => Ok(Self::Spreadsheet),
            "document" => Ok(Self::Document),
            "folder" => Ok(Self::Folder),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreatableAssetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Spreadsheet => write!(f, "spreadsheet"),
            Self::Document => write!(f, "document"),
            Self::Folder => write!(f, "folder"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
