pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ChunkContentItem {
        #[serde(rename = "text")]
        #[non_exhaustive]
        Text {
            #[serde(default)]
            text: String,
        },

        #[serde(rename = "image_url")]
        #[non_exhaustive]
        ImageUrl {
            #[serde(default)]
            image_url: HashMap<String, String>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl ChunkContentItem {
    pub fn text(text: String) -> Self {
        Self::Text { text }
    }

    pub fn image_url(image_url: HashMap<String, String>) -> Self {
        Self::ImageUrl { image_url }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
