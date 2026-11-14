pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Order of the rows; last changed first by default. relevance puts the best title match for the words first.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExploreAssetsInputSort {
    UpdatedDesc,
    UpdatedAsc,
    CreatedDesc,
    CreatedAsc,
    TitleAsc,
    TitleDesc,
    TypeAsc,
    TypeDesc,
    CreatedByAsc,
    CreatedByDesc,
    SizeAsc,
    SizeDesc,
    Relevance,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ExploreAssetsInputSort {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::UpdatedDesc => serializer.serialize_str("updated_desc"),
            Self::UpdatedAsc => serializer.serialize_str("updated_asc"),
            Self::CreatedDesc => serializer.serialize_str("created_desc"),
            Self::CreatedAsc => serializer.serialize_str("created_asc"),
            Self::TitleAsc => serializer.serialize_str("title_asc"),
            Self::TitleDesc => serializer.serialize_str("title_desc"),
            Self::TypeAsc => serializer.serialize_str("type_asc"),
            Self::TypeDesc => serializer.serialize_str("type_desc"),
            Self::CreatedByAsc => serializer.serialize_str("created_by_asc"),
            Self::CreatedByDesc => serializer.serialize_str("created_by_desc"),
            Self::SizeAsc => serializer.serialize_str("size_asc"),
            Self::SizeDesc => serializer.serialize_str("size_desc"),
            Self::Relevance => serializer.serialize_str("relevance"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ExploreAssetsInputSort {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "updated_desc" => Ok(Self::UpdatedDesc),
            "updated_asc" => Ok(Self::UpdatedAsc),
            "created_desc" => Ok(Self::CreatedDesc),
            "created_asc" => Ok(Self::CreatedAsc),
            "title_asc" => Ok(Self::TitleAsc),
            "title_desc" => Ok(Self::TitleDesc),
            "type_asc" => Ok(Self::TypeAsc),
            "type_desc" => Ok(Self::TypeDesc),
            "created_by_asc" => Ok(Self::CreatedByAsc),
            "created_by_desc" => Ok(Self::CreatedByDesc),
            "size_asc" => Ok(Self::SizeAsc),
            "size_desc" => Ok(Self::SizeDesc),
            "relevance" => Ok(Self::Relevance),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ExploreAssetsInputSort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UpdatedDesc => write!(f, "updated_desc"),
            Self::UpdatedAsc => write!(f, "updated_asc"),
            Self::CreatedDesc => write!(f, "created_desc"),
            Self::CreatedAsc => write!(f, "created_asc"),
            Self::TitleAsc => write!(f, "title_asc"),
            Self::TitleDesc => write!(f, "title_desc"),
            Self::TypeAsc => write!(f, "type_asc"),
            Self::TypeDesc => write!(f, "type_desc"),
            Self::CreatedByAsc => write!(f, "created_by_asc"),
            Self::CreatedByDesc => write!(f, "created_by_desc"),
            Self::SizeAsc => write!(f, "size_asc"),
            Self::SizeDesc => write!(f, "size_desc"),
            Self::Relevance => write!(f, "relevance"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
