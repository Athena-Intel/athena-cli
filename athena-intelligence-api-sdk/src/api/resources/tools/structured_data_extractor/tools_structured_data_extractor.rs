use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct StructuredDataExtractorClient {
    pub http_client: HttpClient,
}

impl StructuredDataExtractorClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Extract structured data.
    ///
    /// tl;dr:
    /// - pass a valid JSON schema in `json_schema`
    /// - pass the page chunks as a list of `Chunk` objects, by default: `{"type": "text", "content": "..."}`
    /// - leave all other fields as default
    ///
    /// Detailed configuration (only relevant for complex use cases):
    ///
    /// The structured data extractor's architecture follows the map-reduce pattern,
    /// where the asset is divided into chunks, the schema is extracted from each chunk,
    /// and the chunks are then reduced to a single structured data object.
    ///
    /// In some applications, you may not want to:
    ///
    /// - map (if your input asset is small enough)
    /// - reduce (if your output object is large enough that it will overflow the output length;
    /// if you're extracting a long list of entities; if youre )
    /// to extract all instances of the schema).
    ///
    /// You can configure these behaviors with the `map` and `reduce` fields.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn invoke(
        &self,
        request: &StructuredDataExtractorRequest,
        options: Option<RequestOptions>,
    ) -> Result<StructuredDataExtractorResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/structured-data-extractor/invoke",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
