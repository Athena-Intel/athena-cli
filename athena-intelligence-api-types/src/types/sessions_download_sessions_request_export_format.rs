pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Which representation to download: 'trace' (full trace with all tool calls), 'messages' (user/agent turns only), 'markdown' (readable transcript), or 'stats' (aggregate metrics)
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DownloadSessionsRequestExportFormat {
    Trace,
    Messages,
    Markdown,
    Stats,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DownloadSessionsRequestExportFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Trace => serializer.serialize_str("trace"),
            Self::Messages => serializer.serialize_str("messages"),
            Self::Markdown => serializer.serialize_str("markdown"),
            Self::Stats => serializer.serialize_str("stats"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DownloadSessionsRequestExportFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "trace" => Ok(Self::Trace),
            "messages" => Ok(Self::Messages),
            "markdown" => Ok(Self::Markdown),
            "stats" => Ok(Self::Stats),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DownloadSessionsRequestExportFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Trace => write!(f, "trace"),
            Self::Messages => write!(f, "messages"),
            Self::Markdown => write!(f, "markdown"),
            Self::Stats => write!(f, "stats"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
