pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The anchor's kind; a spreadsheet range is the only one today.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SubscriptionAnchorInputType {
    #[serde(rename = "sheet_range")]
    SheetRange,
}
impl fmt::Display for SubscriptionAnchorInputType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SheetRange => "sheet_range",
        };
        write!(f, "{}", s)
    }
}
