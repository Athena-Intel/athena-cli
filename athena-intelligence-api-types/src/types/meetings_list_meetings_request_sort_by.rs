pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Field to sort by
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListMeetingsRequestSortBy {
    CreatedAt,
    UpdatedAt,
    Title,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListMeetingsRequestSortBy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::UpdatedAt => serializer.serialize_str("updated_at"),
            Self::Title => serializer.serialize_str("title"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListMeetingsRequestSortBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "created_at" => Ok(Self::CreatedAt),
            "updated_at" => Ok(Self::UpdatedAt),
            "title" => Ok(Self::Title),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListMeetingsRequestSortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "created_at"),
            Self::UpdatedAt => write!(f, "updated_at"),
            Self::Title => write!(f, "title"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
