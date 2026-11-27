pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExploreAssetsInputSizeBucketsItem {
    None,
    Lt100Kb,
    OneHundredKb1Mb,
    OneMb10Mb,
    TenMb100Mb,
    Gte100Mb,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ExploreAssetsInputSizeBucketsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::None => serializer.serialize_str("none"),
            Self::Lt100Kb => serializer.serialize_str("lt_100kb"),
            Self::OneHundredKb1Mb => serializer.serialize_str("100kb_1mb"),
            Self::OneMb10Mb => serializer.serialize_str("1mb_10mb"),
            Self::TenMb100Mb => serializer.serialize_str("10mb_100mb"),
            Self::Gte100Mb => serializer.serialize_str("gte_100mb"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ExploreAssetsInputSizeBucketsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "none" => Ok(Self::None),
            "lt_100kb" => Ok(Self::Lt100Kb),
            "100kb_1mb" => Ok(Self::OneHundredKb1Mb),
            "1mb_10mb" => Ok(Self::OneMb10Mb),
            "10mb_100mb" => Ok(Self::TenMb100Mb),
            "gte_100mb" => Ok(Self::Gte100Mb),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ExploreAssetsInputSizeBucketsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Lt100Kb => write!(f, "lt_100kb"),
            Self::OneHundredKb1Mb => write!(f, "100kb_1mb"),
            Self::OneMb10Mb => write!(f, "1mb_10mb"),
            Self::TenMb100Mb => write!(f, "10mb_100mb"),
            Self::Gte100Mb => write!(f, "gte_100mb"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
