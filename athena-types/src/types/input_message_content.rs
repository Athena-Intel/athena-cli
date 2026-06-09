pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum InputMessageContent {
        String(String),

        InputMessageContentOneItemList(Vec<InputMessageContentOneItem>),
}

impl InputMessageContent {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_input_message_content_one_item_list(&self) -> bool {
        matches!(self, Self::InputMessageContentOneItemList(_))
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

    pub fn as_input_message_content_one_item_list(&self) -> Option<&Vec<InputMessageContentOneItem>> {
        match self {
                    Self::InputMessageContentOneItemList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_input_message_content_one_item_list(self) -> Option<Vec<InputMessageContentOneItem>> {
        match self {
                    Self::InputMessageContentOneItemList(value) => Some(value),
                    _ => None,
                }
    }
}
