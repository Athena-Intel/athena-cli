pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BorderStyle {
    Dotted,
    Dashed,
    Solid,
    SolidMedium,
    SolidThick,
    Double,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BorderStyle {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Dotted => serializer.serialize_str("dotted"),
            Self::Dashed => serializer.serialize_str("dashed"),
            Self::Solid => serializer.serialize_str("solid"),
            Self::SolidMedium => serializer.serialize_str("solid_medium"),
            Self::SolidThick => serializer.serialize_str("solid_thick"),
            Self::Double => serializer.serialize_str("double"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BorderStyle {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "dotted" => Ok(Self::Dotted),
            "dashed" => Ok(Self::Dashed),
            "solid" => Ok(Self::Solid),
            "solid_medium" => Ok(Self::SolidMedium),
            "solid_thick" => Ok(Self::SolidThick),
            "double" => Ok(Self::Double),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BorderStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dotted => write!(f, "dotted"),
            Self::Dashed => write!(f, "dashed"),
            Self::Solid => write!(f, "solid"),
            Self::SolidMedium => write!(f, "solid_medium"),
            Self::SolidThick => write!(f, "solid_thick"),
            Self::Double => write!(f, "double"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
