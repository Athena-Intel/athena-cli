pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StructuredDataExtractorRequest {
    /// The prompt to use for the data extraction over *each individual chunk*. It must be a list of messages.  The chunk content will be appended as a list of human messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_messages: Option<Vec<PromptMessage>>,
    /// The chunks from which to extract structured data.
    #[serde(default)]
    pub chunks: Vec<Chunk>,
    /// The JSON schema to use for validation (version draft 2020-12). See the docs [here](https://json-schema.org/learn/getting-started-step-by-step).
    #[serde(default)]
    pub json_schema: HashMap<String, serde_json::Value>,
    /// If `map`, whether to reduce the chunks to a single structured object (true) or return the full list (false).  Use True unless you want to preserve duplicates from each page or expect the object to overflow the output context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce: Option<bool>,
    /// The prompt to use for the reduce steps. It must be a list of messages. The two extraction attempts will be appended as a list of human messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_messages: Option<Vec<PromptMessage>>,
}

impl StructuredDataExtractorRequest {
    pub fn builder() -> StructuredDataExtractorRequestBuilder {
        <StructuredDataExtractorRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StructuredDataExtractorRequestBuilder {
    chunk_messages: Option<Vec<PromptMessage>>,
    chunks: Option<Vec<Chunk>>,
    json_schema: Option<HashMap<String, serde_json::Value>>,
    reduce: Option<bool>,
    reduce_messages: Option<Vec<PromptMessage>>,
}

impl StructuredDataExtractorRequestBuilder {
    pub fn chunk_messages(mut self, value: Vec<PromptMessage>) -> Self {
        self.chunk_messages = Some(value);
        self
    }

    pub fn chunks(mut self, value: Vec<Chunk>) -> Self {
        self.chunks = Some(value);
        self
    }

    pub fn json_schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.json_schema = Some(value);
        self
    }

    pub fn reduce(mut self, value: bool) -> Self {
        self.reduce = Some(value);
        self
    }

    pub fn reduce_messages(mut self, value: Vec<PromptMessage>) -> Self {
        self.reduce_messages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StructuredDataExtractorRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chunks`](StructuredDataExtractorRequestBuilder::chunks)
    /// - [`json_schema`](StructuredDataExtractorRequestBuilder::json_schema)
    pub fn build(self) -> Result<StructuredDataExtractorRequest, BuildError> {
        Ok(StructuredDataExtractorRequest {
            chunk_messages: self.chunk_messages,
            chunks: self.chunks.ok_or_else(|| BuildError::missing_field("chunks"))?,
            json_schema: self.json_schema.ok_or_else(|| BuildError::missing_field("json_schema"))?,
            reduce: self.reduce,
            reduce_messages: self.reduce_messages,
        })
    }
}

