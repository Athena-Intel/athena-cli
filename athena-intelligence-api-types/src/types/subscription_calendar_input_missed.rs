pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What a fire on a non-business day does.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SubscriptionCalendarInputMissed {
    Skip,
    NextBusinessDay,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SubscriptionCalendarInputMissed {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Skip => serializer.serialize_str("skip"),
            Self::NextBusinessDay => serializer.serialize_str("next_business_day"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SubscriptionCalendarInputMissed {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "skip" => Ok(Self::Skip),
            "next_business_day" => Ok(Self::NextBusinessDay),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SubscriptionCalendarInputMissed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Skip => write!(f, "skip"),
            Self::NextBusinessDay => write!(f, "next_business_day"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
