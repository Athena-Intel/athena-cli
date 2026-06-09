pub use crate::prelude::*;
use super::*;

/// Which layer was queried: 'values' for userEnteredValue (what user typed), 'effective_values' for effectiveValue (computed result), 'formatting' for formattedValue (display string)
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QuerySheetRangeMetadataLayer {
    Values,
    EffectiveValues,
    Formatting,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QuerySheetRangeMetadataLayer {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Values => serializer.serialize_str("values"),
            Self::EffectiveValues => serializer.serialize_str("effective_values"),
            Self::Formatting => serializer.serialize_str("formatting"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QuerySheetRangeMetadataLayer {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "values" => Ok(Self::Values),
            "effective_values" => Ok(Self::EffectiveValues),
            "formatting" => Ok(Self::Formatting),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QuerySheetRangeMetadataLayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Values => write!(f, "values"),
            Self::EffectiveValues => write!(f, "effective_values"),
            Self::Formatting => write!(f, "formatting"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
