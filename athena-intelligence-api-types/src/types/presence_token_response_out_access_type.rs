pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Always r: subscribe only. The app never publishes a slot.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PresenceTokenResponseOutAccessType {
    #[serde(rename = "r")]
    R,
}
impl fmt::Display for PresenceTokenResponseOutAccessType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::R => "r",
        };
        write!(f, "{}", s)
    }
}
