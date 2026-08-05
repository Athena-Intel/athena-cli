pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GeneralAgentConfigEnabledToolsItemZero {
    Search,
    TavilySearch,
    Browse,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GeneralAgentConfigEnabledToolsItemZero {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Search => serializer.serialize_str("search"),
            Self::TavilySearch => serializer.serialize_str("tavily_search"),
            Self::Browse => serializer.serialize_str("browse"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GeneralAgentConfigEnabledToolsItemZero {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "search" => Ok(Self::Search),
            "tavily_search" => Ok(Self::TavilySearch),
            "browse" => Ok(Self::Browse),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GeneralAgentConfigEnabledToolsItemZero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Search => write!(f, "search"),
            Self::TavilySearch => write!(f, "tavily_search"),
            Self::Browse => write!(f, "browse"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
