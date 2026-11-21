pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// asset, ontology_object or external_url.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaskLinkInReferenced {
    Asset,
    OntologyObject,
    ExternalUrl,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TaskLinkInReferenced {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Asset => serializer.serialize_str("asset"),
            Self::OntologyObject => serializer.serialize_str("ontology_object"),
            Self::ExternalUrl => serializer.serialize_str("external_url"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TaskLinkInReferenced {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "asset" => Ok(Self::Asset),
            "ontology_object" => Ok(Self::OntologyObject),
            "external_url" => Ok(Self::ExternalUrl),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TaskLinkInReferenced {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Asset => write!(f, "asset"),
            Self::OntologyObject => write!(f, "ontology_object"),
            Self::ExternalUrl => write!(f, "external_url"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
