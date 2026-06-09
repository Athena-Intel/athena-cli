pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum GeneralAgentConfigEnabledToolsItem {
        GeneralAgentConfigEnabledToolsItemZero(GeneralAgentConfigEnabledToolsItemZero),

        String(String),
}

impl GeneralAgentConfigEnabledToolsItem {
    pub fn is_general_agent_config_enabled_tools_item_zero(&self) -> bool {
        matches!(self, Self::GeneralAgentConfigEnabledToolsItemZero(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }


    pub fn as_general_agent_config_enabled_tools_item_zero(&self) -> Option<&GeneralAgentConfigEnabledToolsItemZero> {
        match self {
                    Self::GeneralAgentConfigEnabledToolsItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_general_agent_config_enabled_tools_item_zero(self) -> Option<GeneralAgentConfigEnabledToolsItemZero> {
        match self {
                    Self::GeneralAgentConfigEnabledToolsItemZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for GeneralAgentConfigEnabledToolsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GeneralAgentConfigEnabledToolsItemZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::String(value) => write!(f, "{}", value),
        }
    }
}
