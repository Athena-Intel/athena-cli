pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum SemanticModelQueryRequestInOrderItem {
        StringList(Vec<String>),

        SemanticModelOrderEntry(SemanticModelOrderEntry),
}

impl SemanticModelQueryRequestInOrderItem {
    pub fn is_string_list(&self) -> bool {
        matches!(self, Self::StringList(_))
    }

    pub fn is_semantic_model_order_entry(&self) -> bool {
        matches!(self, Self::SemanticModelOrderEntry(_))
    }


    pub fn as_string_list(&self) -> Option<&Vec<String>> {
        match self {
                    Self::StringList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string_list(self) -> Option<Vec<String>> {
        match self {
                    Self::StringList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_semantic_model_order_entry(&self) -> Option<&SemanticModelOrderEntry> {
        match self {
                    Self::SemanticModelOrderEntry(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_semantic_model_order_entry(self) -> Option<SemanticModelOrderEntry> {
        match self {
                    Self::SemanticModelOrderEntry(value) => Some(value),
                    _ => None,
                }
    }
}
