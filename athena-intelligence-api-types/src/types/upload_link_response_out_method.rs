pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The HTTP method the upload uses
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UploadLinkResponseOutMethod {
    #[serde(rename = "PUT")]
    Put,
}
impl fmt::Display for UploadLinkResponseOutMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Put => "PUT",
        };
        write!(f, "{}", s)
    }
}
