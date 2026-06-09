pub use crate::prelude::*;
use super::*;

/// The agent's response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StructuredDataExtractorResponse {
    /// The extracted structured data for each chunk.  A list where each element is guaranteed to match `json_schema`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_by_chunk_data: Option<Vec<ChunkResult>>,
    /// If reduce is True, the reduced structured data, otherwise null.  Guaranteed to match `json_schema`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduced_data: Option<HashMap<String, serde_json::Value>>,
}

impl StructuredDataExtractorResponse {
    pub fn builder() -> StructuredDataExtractorResponseBuilder {
        <StructuredDataExtractorResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StructuredDataExtractorResponseBuilder {
    chunk_by_chunk_data: Option<Vec<ChunkResult>>,
    reduced_data: Option<HashMap<String, serde_json::Value>>,
}

impl StructuredDataExtractorResponseBuilder {
    pub fn chunk_by_chunk_data(mut self, value: Vec<ChunkResult>) -> Self {
        self.chunk_by_chunk_data = Some(value);
        self
    }

    pub fn reduced_data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.reduced_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StructuredDataExtractorResponse`].
    pub fn build(self) -> Result<StructuredDataExtractorResponse, BuildError> {
        Ok(StructuredDataExtractorResponse {
            chunk_by_chunk_data: self.chunk_by_chunk_data,
            reduced_data: self.reduced_data,
        })
    }
}
