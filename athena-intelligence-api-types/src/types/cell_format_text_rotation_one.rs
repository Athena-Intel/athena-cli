pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CellFormatTextRotationOne {
    #[serde(rename = "vertical")]
    Vertical,
}
impl fmt::Display for CellFormatTextRotationOne {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Vertical => "vertical",
        };
        write!(f, "{}", s)
    }
}
