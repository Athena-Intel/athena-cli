pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A chunk of content to extract data from.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Chunk {
    #[serde(default)]
    pub chunk_id: String,
    #[serde(default)]
    pub content: Vec<ChunkContentItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Option<String>>>,
}

impl Chunk {
    pub fn builder() -> ChunkBuilder {
        <ChunkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChunkBuilder {
    chunk_id: Option<String>,
    content: Option<Vec<ChunkContentItem>>,
    metadata: Option<HashMap<String, Option<String>>>,
}

impl ChunkBuilder {
    pub fn chunk_id(mut self, value: impl Into<String>) -> Self {
        self.chunk_id = Some(value.into());
        self
    }

    pub fn content(mut self, value: Vec<ChunkContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Chunk`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chunk_id`](ChunkBuilder::chunk_id)
    /// - [`content`](ChunkBuilder::content)
    pub fn build(self) -> Result<Chunk, BuildError> {
        Ok(Chunk {
            chunk_id: self.chunk_id.ok_or_else(|| BuildError::missing_field("chunk_id"))?,
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            metadata: self.metadata,
        })
    }
}
