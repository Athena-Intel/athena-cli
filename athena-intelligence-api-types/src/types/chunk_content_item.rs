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
            #[serde(flatten)]
            data: TextContent,
        },

        #[serde(rename = "image_url")]
        #[non_exhaustive]
        ImageUrl {
            #[serde(flatten)]
            data: ImageUrlContent,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl ChunkContentItem {
    pub fn text(data: TextContent) -> Self {
        Self::Text { data }
    }

    pub fn image_url(data: ImageUrlContent) -> Self {
        Self::ImageUrl { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
