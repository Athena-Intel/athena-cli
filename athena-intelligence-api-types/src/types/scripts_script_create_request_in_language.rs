pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What interprets the source: python (the default) or bash. Decides the default entrypoint (main.py or main.sh) and the starter source
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScriptCreateRequestInLanguage {
    Python,
    Bash,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ScriptCreateRequestInLanguage {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Python => serializer.serialize_str("python"),
            Self::Bash => serializer.serialize_str("bash"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ScriptCreateRequestInLanguage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "python" => Ok(Self::Python),
            "bash" => Ok(Self::Bash),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ScriptCreateRequestInLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Python => write!(f, "python"),
            Self::Bash => write!(f, "bash"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
