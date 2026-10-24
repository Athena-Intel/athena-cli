pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Enum for asset types that can be created via the API.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreatableAssetType {
    Spreadsheet,
    Document,
    Folder,
    Database,
    Computer,
    GenericDoc,
    UserInterface,
    Skill,
    CollabAgent,
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
            Self::Database => serializer.serialize_str("database"),
            Self::Computer => serializer.serialize_str("computer"),
            Self::GenericDoc => serializer.serialize_str("generic_doc"),
            Self::UserInterface => serializer.serialize_str("user_interface"),
            Self::Skill => serializer.serialize_str("skill"),
            Self::CollabAgent => serializer.serialize_str("collab_agent"),
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
            "database" => Ok(Self::Database),
            "computer" => Ok(Self::Computer),
            "generic_doc" => Ok(Self::GenericDoc),
            "user_interface" => Ok(Self::UserInterface),
            "skill" => Ok(Self::Skill),
            "collab_agent" => Ok(Self::CollabAgent),
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
            Self::Database => write!(f, "database"),
            Self::Computer => write!(f, "computer"),
            Self::GenericDoc => write!(f, "generic_doc"),
            Self::UserInterface => write!(f, "user_interface"),
            Self::Skill => write!(f, "skill"),
            Self::CollabAgent => write!(f, "collab_agent"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
