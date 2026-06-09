pub use crate::prelude::*;
use super::*;

/// The result of a chunk extraction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChunkResult {
    pub chunk_id: ChunkResultChunkId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_result: Option<HashMap<String, serde_json::Value>>,
}

impl ChunkResult {
    pub fn builder() -> ChunkResultBuilder {
        <ChunkResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChunkResultBuilder {
    chunk_id: Option<ChunkResultChunkId>,
    chunk_result: Option<HashMap<String, serde_json::Value>>,
}

impl ChunkResultBuilder {
    pub fn chunk_id(mut self, value: ChunkResultChunkId) -> Self {
        self.chunk_id = Some(value);
        self
    }

    pub fn chunk_result(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.chunk_result = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChunkResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chunk_id`](ChunkResultBuilder::chunk_id)
    pub fn build(self) -> Result<ChunkResult, BuildError> {
        Ok(ChunkResult {
            chunk_id: self.chunk_id.ok_or_else(|| BuildError::missing_field("chunk_id"))?,
            chunk_result: self.chunk_result,
        })
    }
}
