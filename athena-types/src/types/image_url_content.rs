pub use crate::prelude::*;
use super::*;

/// An image content item.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ImageUrlContent {
    #[serde(default)]
    pub image_url: HashMap<String, String>,
}

impl ImageUrlContent {
    pub fn builder() -> ImageUrlContentBuilder {
        <ImageUrlContentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImageUrlContentBuilder {
    image_url: Option<HashMap<String, String>>,
}

impl ImageUrlContentBuilder {
    pub fn image_url(mut self, value: HashMap<String, String>) -> Self {
        self.image_url = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ImageUrlContent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`image_url`](ImageUrlContentBuilder::image_url)
    pub fn build(self) -> Result<ImageUrlContent, BuildError> {
        Ok(ImageUrlContent {
            image_url: self.image_url.ok_or_else(|| BuildError::missing_field("image_url"))?,
        })
    }
}
