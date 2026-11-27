pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AutomationTriggersResumeRequestInReasonsItem {
    Operator,
    Error,
    BudgetExceeded,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AutomationTriggersResumeRequestInReasonsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Operator => serializer.serialize_str("operator"),
            Self::Error => serializer.serialize_str("error"),
            Self::BudgetExceeded => serializer.serialize_str("budget_exceeded"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AutomationTriggersResumeRequestInReasonsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "operator" => Ok(Self::Operator),
            "error" => Ok(Self::Error),
            "budget_exceeded" => Ok(Self::BudgetExceeded),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AutomationTriggersResumeRequestInReasonsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Operator => write!(f, "operator"),
            Self::Error => write!(f, "error"),
            Self::BudgetExceeded => write!(f, "budget_exceeded"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
