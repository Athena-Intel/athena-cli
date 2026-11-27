use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ConnectorsClient {
    pub http_client: HttpClient,
}

impl ConnectorsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List one page of what a connected account or a source holds, read with the caller's own sign-in. Start from a connected account's catalog (the `catalogId` the React SDK's `useAthenaConnectors` gives for each account): its items are its sources (a OneDrive, SharePoint sites). List a source's `catalog_asset_id` for its libraries, folders and files, and pass `folder_id` to open a library or a folder. The caller needs a connected account of their own for the source: being able to see a colleague's catalog is answered 403 `connect_your_own`, and nothing is asked of the provider. Listings carry names and ids, never a token or a signed download link.
    ///
    /// # Arguments
    ///
    /// * `catalog_asset_id` - The catalog of a connected account, or of one of its sources.
    /// * `folder_id` - The id of a library or a folder of the source to open. Omit for the top level.
    /// * `page_token` - The next_page_token of the page before, for the next one.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_items(
        &self,
        catalog_asset_id: &str,
        request: &ListItemsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConnectorItemsResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/connectors/{}/items", catalog_asset_id),
                None,
                QueryBuilder::new()
                    .serialize("folder_id", request.folder_id.clone())
                    .serialize("page_token", request.page_token.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Bring one file of a source in as a new Athena asset, filed in `folder_asset_id`. The file is downloaded inside Athena with the caller's own sign-in and is at most 100 MiB. The caller needs a connected account of their own for the source, and must be able to edit the folder. The new asset takes the sharing of where it is filed: whoever can open the Drive or the place the folder is in can open it. A SharePoint file is imported as the same protected copy Athena's own import makes: it cannot be shared person by person, moved or renamed afterwards, and a share made on the folder itself for one person is not copied onto it. OneDrive files cannot be imported yet (`import_not_supported`). Returns the new asset's id; no token or download link is returned.
    ///
    /// # Arguments
    ///
    /// * `catalog_asset_id` - The catalog of the source the file is in.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn import_item(
        &self,
        catalog_asset_id: &str,
        request: &ImportConnectorItemRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<ImportConnectorItemResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/connectors/{}/items/import", catalog_asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
