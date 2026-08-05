pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Which artifact to download: 'zip' (full export), 'recording', 'transcript', 'formatted_transcript', or 'chat'
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DownloadMeetingsRequestArtifact {
    Zip,
    Recording,
    Transcript,
    FormattedTranscript,
    Chat,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DownloadMeetingsRequestArtifact {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Zip => serializer.serialize_str("zip"),
            Self::Recording => serializer.serialize_str("recording"),
            Self::Transcript => serializer.serialize_str("transcript"),
            Self::FormattedTranscript => serializer.serialize_str("formatted_transcript"),
            Self::Chat => serializer.serialize_str("chat"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DownloadMeetingsRequestArtifact {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "zip" => Ok(Self::Zip),
            "recording" => Ok(Self::Recording),
            "transcript" => Ok(Self::Transcript),
            "formatted_transcript" => Ok(Self::FormattedTranscript),
            "chat" => Ok(Self::Chat),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DownloadMeetingsRequestArtifact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zip => write!(f, "zip"),
            Self::Recording => write!(f, "recording"),
            Self::Transcript => write!(f, "transcript"),
            Self::FormattedTranscript => write!(f, "formatted_transcript"),
            Self::Chat => write!(f, "chat"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
