pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// list: your subscriptions (this conversation's and your own elsewhere). show: one of them. events: what one heard, without ending your turn (by default what is queued, oldest first). cancel, pause, resume: as named; cancel one once the task it waited for is done. renew: give one a new lifetime (expires). snooze: move a schedule's next fire (at or after).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ManageSubscriptionsInputOp {
    List,
    Show,
    Events,
    Cancel,
    Pause,
    Resume,
    Renew,
    Snooze,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ManageSubscriptionsInputOp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::List => serializer.serialize_str("list"),
            Self::Show => serializer.serialize_str("show"),
            Self::Events => serializer.serialize_str("events"),
            Self::Cancel => serializer.serialize_str("cancel"),
            Self::Pause => serializer.serialize_str("pause"),
            Self::Resume => serializer.serialize_str("resume"),
            Self::Renew => serializer.serialize_str("renew"),
            Self::Snooze => serializer.serialize_str("snooze"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ManageSubscriptionsInputOp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "list" => Ok(Self::List),
            "show" => Ok(Self::Show),
            "events" => Ok(Self::Events),
            "cancel" => Ok(Self::Cancel),
            "pause" => Ok(Self::Pause),
            "resume" => Ok(Self::Resume),
            "renew" => Ok(Self::Renew),
            "snooze" => Ok(Self::Snooze),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ManageSubscriptionsInputOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::List => write!(f, "list"),
            Self::Show => write!(f, "show"),
            Self::Events => write!(f, "events"),
            Self::Cancel => write!(f, "cancel"),
            Self::Pause => write!(f, "pause"),
            Self::Resume => write!(f, "resume"),
            Self::Renew => write!(f, "renew"),
            Self::Snooze => write!(f, "snooze"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
