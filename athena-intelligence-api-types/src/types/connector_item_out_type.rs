pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// source: a source of a connected account (a OneDrive, a SharePoint site). library: a document library. folder: a folder. file: a file.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectorItemOutType {
    Source,
    Library,
    Folder,
    File,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConnectorItemOutType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Source => serializer.serialize_str("source"),
            Self::Library => serializer.serialize_str("library"),
            Self::Folder => serializer.serialize_str("folder"),
            Self::File => serializer.serialize_str("file"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConnectorItemOutType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "source" => Ok(Self::Source),
            "library" => Ok(Self::Library),
            "folder" => Ok(Self::Folder),
            "file" => Ok(Self::File),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConnectorItemOutType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Source => write!(f, "source"),
            Self::Library => write!(f, "library"),
            Self::Folder => write!(f, "folder"),
            Self::File => write!(f, "file"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
