use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct MeetingsClient {
    pub http_client: HttpClient,
}

impl MeetingsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve a paginated list of meetings with optional keyword search (across title, AI summary, and cached transcript text), participant email filtering, attendee domain filtering, date range filtering, and sorting.
    ///
    /// # Arguments
    ///
    /// * `query` - Keyword to search across meeting title, AI summary, and cached transcript text
    /// * `participant_emails` - Participant email(s) to filter by. Repeat the parameter or pass a comma-separated list.
    /// * `participant_match` - Whether a meeting must include any or all of the given participant emails
    /// * `participant_domains` - Attendee email domain(s) to filter by (e.g. 'acme.com'). Repeat the parameter or pass a comma-separated list.
    /// * `domain_match` - Whether a meeting must include attendees from any or all of the given domains
    /// * `created_after` - Only include meetings created at or after this ISO 8601 timestamp
    /// * `created_before` - Only include meetings created at or before this ISO 8601 timestamp
    /// * `sort_by` - Field to sort by
    /// * `sort_direction` - Sort direction
    /// * `limit` - Maximum number of meetings to return per page (1-500)
    /// * `offset` - Number of meetings to skip for pagination
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &MeetingsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedMeetingsOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v0/meetings",
                None,
                QueryBuilder::new()
                    .structured_query("query", request.query.clone())
                    .serialize("participant_emails", request.participant_emails.clone())
                    .serialize("participant_match", request.participant_match.clone())
                    .serialize("participant_domains", request.participant_domains.clone())
                    .serialize("domain_match", request.domain_match.clone())
                    .serialize("created_after", request.created_after.clone())
                    .serialize("created_before", request.created_before.clone())
                    .serialize("sort_by", request.sort_by.clone())
                    .serialize("sort_direction", request.sort_direction.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve a single meeting by its asset ID, including status, AI summary, participants, and the asset IDs of its downloadable artifacts (recording, transcripts, chat).
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the meeting asset to retrieve
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<MeetingOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/meetings/{}", asset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Download a meeting artifact. By default streams a ZIP archive containing metadata.json plus every available artifact (video recording, raw transcript, formatted transcript, chat). Pass the artifact parameter to download a single artifact instead.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the meeting asset to download
    /// * `artifact` - Which artifact to download: 'zip' (full export), 'recording', 'transcript', 'formatted_transcript', or 'chat'
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn download(
        &self,
        asset_id: &str,
        request: &MeetingsDownloadQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/meetings/{}/download", asset_id),
                None,
                QueryBuilder::new()
                    .serialize("artifact", request.artifact.clone())
                    .build(),
                options,
            )
            .await
    }
}
