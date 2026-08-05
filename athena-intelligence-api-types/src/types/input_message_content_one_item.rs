pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum InputMessageContentOneItem {
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

        #[serde(rename = "tool_use")]
        #[non_exhaustive]
        ToolUse {
            #[serde(default)]
            id: String,
            #[serde(default)]
            input: HashMap<String, serde_json::Value>,
            #[serde(default)]
            name: String,
        },

        #[serde(rename = "tool_result")]
        #[non_exhaustive]
        ToolResult {
            content: ToolResultContentContent,
            #[serde(default)]
            is_error: bool,
            #[serde(default)]
            tool_use_id: String,
        },

        #[serde(rename = "thinking")]
        #[non_exhaustive]
        Thinking {
            #[serde(default)]
            signature: String,
            #[serde(default)]
            thinking: String,
        },

        #[serde(rename = "redacted_thinking")]
        #[non_exhaustive]
        RedactedThinking {
            #[serde(default)]
            data: String,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl InputMessageContentOneItem {
    pub fn text(data: TextContent) -> Self {
        Self::Text { data }
    }

    pub fn image_url(data: ImageUrlContent) -> Self {
        Self::ImageUrl { data }
    }

    pub fn tool_use(id: String, input: HashMap<String, serde_json::Value>, name: String) -> Self {
        Self::ToolUse { id, input, name }
    }

    pub fn tool_result(content: ToolResultContentContent, is_error: bool, tool_use_id: String) -> Self {
        Self::ToolResult { content, is_error, tool_use_id }
    }

    pub fn thinking(signature: String, thinking: String) -> Self {
        Self::Thinking { signature, thinking }
    }

    pub fn redacted_thinking(data: String) -> Self {
        Self::RedactedThinking { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
