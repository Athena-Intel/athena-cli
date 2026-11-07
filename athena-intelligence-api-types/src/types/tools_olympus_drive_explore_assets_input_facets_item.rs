pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExploreAssetsInputFacetsItem {
    Type,
    CreatedBy,
    Place,
    Tag,
    ModifiedMonth,
    Starred,
    CreatedMonth,
    Size,
    Ownership,
    Source,
    HasSummary,
    Access,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ExploreAssetsInputFacetsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Type => serializer.serialize_str("type"),
            Self::CreatedBy => serializer.serialize_str("created_by"),
            Self::Place => serializer.serialize_str("place"),
            Self::Tag => serializer.serialize_str("tag"),
            Self::ModifiedMonth => serializer.serialize_str("modified_month"),
            Self::Starred => serializer.serialize_str("starred"),
            Self::CreatedMonth => serializer.serialize_str("created_month"),
            Self::Size => serializer.serialize_str("size"),
            Self::Ownership => serializer.serialize_str("ownership"),
            Self::Source => serializer.serialize_str("source"),
            Self::HasSummary => serializer.serialize_str("has_summary"),
            Self::Access => serializer.serialize_str("access"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ExploreAssetsInputFacetsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "type" => Ok(Self::Type),
            "created_by" => Ok(Self::CreatedBy),
            "place" => Ok(Self::Place),
            "tag" => Ok(Self::Tag),
            "modified_month" => Ok(Self::ModifiedMonth),
            "starred" => Ok(Self::Starred),
            "created_month" => Ok(Self::CreatedMonth),
            "size" => Ok(Self::Size),
            "ownership" => Ok(Self::Ownership),
            "source" => Ok(Self::Source),
            "has_summary" => Ok(Self::HasSummary),
            "access" => Ok(Self::Access),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ExploreAssetsInputFacetsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Type => write!(f, "type"),
            Self::CreatedBy => write!(f, "created_by"),
            Self::Place => write!(f, "place"),
            Self::Tag => write!(f, "tag"),
            Self::ModifiedMonth => write!(f, "modified_month"),
            Self::Starred => write!(f, "starred"),
            Self::CreatedMonth => write!(f, "created_month"),
            Self::Size => write!(f, "size"),
            Self::Ownership => write!(f, "ownership"),
            Self::Source => write!(f, "source"),
            Self::HasSummary => write!(f, "has_summary"),
            Self::Access => write!(f, "access"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
