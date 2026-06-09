pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum CellFormatTextRotation {
        Integer(i64),

        CellFormatTextRotationOne(CellFormatTextRotationOne),
}

impl CellFormatTextRotation {
    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    pub fn is_cell_format_text_rotation_one(&self) -> bool {
        matches!(self, Self::CellFormatTextRotationOne(_))
    }


    pub fn as_integer(&self) -> Option<&i64> {
        match self {
                    Self::Integer(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_integer(self) -> Option<i64> {
        match self {
                    Self::Integer(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_cell_format_text_rotation_one(&self) -> Option<&CellFormatTextRotationOne> {
        match self {
                    Self::CellFormatTextRotationOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_cell_format_text_rotation_one(self) -> Option<CellFormatTextRotationOne> {
        match self {
                    Self::CellFormatTextRotationOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for CellFormatTextRotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(value) => write!(f, "{}", value),
            Self::CellFormatTextRotationOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
