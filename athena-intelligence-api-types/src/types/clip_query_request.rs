pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for clip
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ClipQueryRequest {
    /// Clip start, in seconds
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub start: f64,
    /// Clip end, in seconds
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub end: f64,
    /// 'video' (MP4) or 'audio' (M4A); defaults to video for meetings and video assets, audio for voice memos and audio assets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<ClipMeetingsRequestMedia>,
}

impl ClipQueryRequest {
    pub fn builder() -> ClipQueryRequestBuilder {
        <ClipQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClipQueryRequestBuilder {
    start: Option<f64>,
    end: Option<f64>,
    media: Option<ClipMeetingsRequestMedia>,
}

impl ClipQueryRequestBuilder {
    pub fn start(mut self, value: f64) -> Self {
        self.start = Some(value);
        self
    }

    pub fn end(mut self, value: f64) -> Self {
        self.end = Some(value);
        self
    }

    pub fn media(mut self, value: ClipMeetingsRequestMedia) -> Self {
        self.media = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClipQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start`](ClipQueryRequestBuilder::start)
    /// - [`end`](ClipQueryRequestBuilder::end)
    pub fn build(self) -> Result<ClipQueryRequest, BuildError> {
        Ok(ClipQueryRequest {
            start: self.start.ok_or_else(|| BuildError::missing_field("start"))?,
            end: self.end.ok_or_else(|| BuildError::missing_field("end"))?,
            media: self.media,
        })
    }
}

