pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The service the token is for, as the token's audience identifier: the editing service of spreadsheets, of Word documents or of decks, or 'pptx-studio-source', the deck editing service reading this asset for a deck (a recording, or a source asked for with 'audience'). Compare it with the service you call before sending the token.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StudioTokenResponseOutAudience {
    DocxStudio,
    PptxStudio,
    PptxStudioSource,
    XlsxStudio,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for StudioTokenResponseOutAudience {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::DocxStudio => serializer.serialize_str("docx-studio"),
            Self::PptxStudio => serializer.serialize_str("pptx-studio"),
            Self::PptxStudioSource => serializer.serialize_str("pptx-studio-source"),
            Self::XlsxStudio => serializer.serialize_str("xlsx-studio"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for StudioTokenResponseOutAudience {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "docx-studio" => Ok(Self::DocxStudio),
            "pptx-studio" => Ok(Self::PptxStudio),
            "pptx-studio-source" => Ok(Self::PptxStudioSource),
            "xlsx-studio" => Ok(Self::XlsxStudio),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for StudioTokenResponseOutAudience {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DocxStudio => write!(f, "docx-studio"),
            Self::PptxStudio => write!(f, "pptx-studio"),
            Self::PptxStudioSource => write!(f, "pptx-studio-source"),
            Self::XlsxStudio => write!(f, "xlsx-studio"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
