use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct BundlesClient {
    pub http_client: HttpClient,
}

impl BundlesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Resolve a bundle member's name, support-desk/support_agent, to this workspace's own asset: its id, type and link, and with track the revision that track serves. The caller needs view access to the bundle as a member of its workspace, and to the member. Every refusal is 404, the same answer a name that doesn't exist gets.
    ///
    /// # Arguments
    ///
    /// * `bundle_key` - The bundle's permanent key.
    /// * `member_key` - The member's permanent key.
    /// * `workspace_id` - The workspace to resolve in. Optional for a caller with one workspace, and for a credential bound to one (a page's, a computer's); a caller in several names it. The caller must be a member.
    /// * `track` - A track's key (production, pilot…): answer with the revision it serves. The caller must be on the track.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_member(
        &self,
        bundle_key: &str,
        member_key: &str,
        request: &GetMemberQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<BundleMemberOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/bundles/{}/members/{}", bundle_key, member_key),
                None,
                QueryBuilder::new()
                    .serialize("workspace_id", request.workspace_id.clone())
                    .serialize("track", request.track.clone())
                    .build(),
                options,
            )
            .await
    }
}
