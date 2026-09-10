use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct PresentationClient {
    pub http_client: HttpClient,
}

impl PresentationClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create a new PowerPoint deck in PPTX Studio. Returns the asset ID for the new presentation. Use this to start a new presentation before adding content. Pass template_asset_id to seed the deck from any ready PPTX Studio deck or uploaded .pptx/.potx file; if the template cannot be applied, no deck is created and the error says why.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        request: &CreatePresentationPptxStudioInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/presentation/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Capture a screenshot of a specific slide from a PPTX Studio deck. Returns the screenshot as an image that can be viewed inline. Use this to inspect the visual appearance of slides during presentation editing.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn render(
        &self,
        request: &CaptureSlideScreenshotInput,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/presentation/render",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
