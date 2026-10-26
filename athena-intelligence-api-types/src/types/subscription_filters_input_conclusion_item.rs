pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SubscriptionFiltersInputConclusionItem {
    Success,
    Failure,
    Neutral,
    Cancelled,
    Skipped,
    TimedOut,
    ActionRequired,
    Stale,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SubscriptionFiltersInputConclusionItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Success => serializer.serialize_str("success"),
            Self::Failure => serializer.serialize_str("failure"),
            Self::Neutral => serializer.serialize_str("neutral"),
            Self::Cancelled => serializer.serialize_str("cancelled"),
            Self::Skipped => serializer.serialize_str("skipped"),
            Self::TimedOut => serializer.serialize_str("timed_out"),
            Self::ActionRequired => serializer.serialize_str("action_required"),
            Self::Stale => serializer.serialize_str("stale"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SubscriptionFiltersInputConclusionItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "success" => Ok(Self::Success),
            "failure" => Ok(Self::Failure),
            "neutral" => Ok(Self::Neutral),
            "cancelled" => Ok(Self::Cancelled),
            "skipped" => Ok(Self::Skipped),
            "timed_out" => Ok(Self::TimedOut),
            "action_required" => Ok(Self::ActionRequired),
            "stale" => Ok(Self::Stale),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SubscriptionFiltersInputConclusionItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Success => write!(f, "success"),
            Self::Failure => write!(f, "failure"),
            Self::Neutral => write!(f, "neutral"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Skipped => write!(f, "skipped"),
            Self::TimedOut => write!(f, "timed_out"),
            Self::ActionRequired => write!(f, "action_required"),
            Self::Stale => write!(f, "stale"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
