pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NumberFormatType {
    General,
    Number,
    Currency,
    Accounting,
    Date,
    Time,
    DateTime,
    Percent,
    Fraction,
    Scientific,
    Text,
    Special,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for NumberFormatType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::General => serializer.serialize_str("GENERAL"),
            Self::Number => serializer.serialize_str("NUMBER"),
            Self::Currency => serializer.serialize_str("CURRENCY"),
            Self::Accounting => serializer.serialize_str("ACCOUNTING"),
            Self::Date => serializer.serialize_str("DATE"),
            Self::Time => serializer.serialize_str("TIME"),
            Self::DateTime => serializer.serialize_str("DATE_TIME"),
            Self::Percent => serializer.serialize_str("PERCENT"),
            Self::Fraction => serializer.serialize_str("FRACTION"),
            Self::Scientific => serializer.serialize_str("SCIENTIFIC"),
            Self::Text => serializer.serialize_str("TEXT"),
            Self::Special => serializer.serialize_str("SPECIAL"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for NumberFormatType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "GENERAL" => Ok(Self::General),
            "NUMBER" => Ok(Self::Number),
            "CURRENCY" => Ok(Self::Currency),
            "ACCOUNTING" => Ok(Self::Accounting),
            "DATE" => Ok(Self::Date),
            "TIME" => Ok(Self::Time),
            "DATE_TIME" => Ok(Self::DateTime),
            "PERCENT" => Ok(Self::Percent),
            "FRACTION" => Ok(Self::Fraction),
            "SCIENTIFIC" => Ok(Self::Scientific),
            "TEXT" => Ok(Self::Text),
            "SPECIAL" => Ok(Self::Special),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for NumberFormatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::General => write!(f, "GENERAL"),
            Self::Number => write!(f, "NUMBER"),
            Self::Currency => write!(f, "CURRENCY"),
            Self::Accounting => write!(f, "ACCOUNTING"),
            Self::Date => write!(f, "DATE"),
            Self::Time => write!(f, "TIME"),
            Self::DateTime => write!(f, "DATE_TIME"),
            Self::Percent => write!(f, "PERCENT"),
            Self::Fraction => write!(f, "FRACTION"),
            Self::Scientific => write!(f, "SCIENTIFIC"),
            Self::Text => write!(f, "TEXT"),
            Self::Special => write!(f, "SPECIAL"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
