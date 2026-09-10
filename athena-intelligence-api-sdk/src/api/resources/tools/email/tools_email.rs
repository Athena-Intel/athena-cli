use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct EmailClient {
    pub http_client: HttpClient,
}

impl EmailClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Save a draft in the caller's connected Gmail or Outlook account.
    ///
    /// Nothing is sent. The draft appears in the account's Drafts folder, where it
    /// is reviewed, edited and sent from the mail client — that review step is why
    /// drafting is available here while sending is not. Set `reply_to_message_id`
    /// to thread the draft as a reply.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_draft(
        &self,
        request: &EmailDraftRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<EmailDraftResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/email/draft",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Search the caller's connected Gmail or Outlook mailbox.
    ///
    /// Results come from the connected account the caller can access in their
    /// current workspace (the default account unless `catalog_id` names another).
    /// Unsent drafts are included and flagged with `is_draft`.
    ///
    /// # Arguments
    ///
    /// * `query` - Search query. Gmail operators (`from:`, `to:`, `subject:`, `has:attachment`, `newer_than:7d`, `-term`, …) are accepted for both providers; operators with no Outlook equivalent are dropped and reported in `ignored_operators`. Use `in:drafts` to search only unsent drafts.
    /// * `catalog_id` - Connected email account to use, as the catalog asset id returned by the Athena UI or the assets API. Defaults to the caller's default email account. An id that is not one of the caller's own connected accounts in the current workspace is a 404.
    /// * `limit` - Maximum number of results (1-50).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn search(
        &self,
        request: &SearchQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<EmailSearchResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v0/tools/email/search",
                None,
                QueryBuilder::new()
                    .structured_query("query", request.query.clone())
                    .serialize("catalog_id", request.catalog_id.clone())
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }
}
