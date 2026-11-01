pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HistoryCompareInputMarkup {
    All,
    Simple,
    None,
    Original,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for HistoryCompareInputMarkup {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::All => serializer.serialize_str("all"),
            Self::Simple => serializer.serialize_str("simple"),
            Self::None => serializer.serialize_str("none"),
            Self::Original => serializer.serialize_str("original"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for HistoryCompareInputMarkup {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "all" => Ok(Self::All),
            "simple" => Ok(Self::Simple),
            "none" => Ok(Self::None),
            "original" => Ok(Self::Original),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for HistoryCompareInputMarkup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Simple => write!(f, "simple"),
            Self::None => write!(f, "none"),
            Self::Original => write!(f, "original"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
