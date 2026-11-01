pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StudioTokenRequestInAudience {
    #[serde(rename = "pptx-studio-source")]
    PptxStudioSource,
}
impl fmt::Display for StudioTokenRequestInAudience {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PptxStudioSource => "pptx-studio-source",
        };
        write!(f, "{}", s)
    }
}
