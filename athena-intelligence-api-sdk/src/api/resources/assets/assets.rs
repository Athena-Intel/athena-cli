use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AssetsClient {
    pub http_client: HttpClient,
}

impl AssetsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve a paginated list of assets with optional filtering and sorting. Assets include documents, presentations, spreadsheets, images, videos, and other file types managed by Athena Intelligence. Pass parent_folder_id to list one folder's direct children (not its whole subtree), or filters.parent_folder_id = null to list assets in no folder.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of assets to return per page (1-500)
    /// * `offset` - Number of assets to skip for pagination
    /// * `filters` - JSON string of filter criteria. Supports: created_by_id, created_by_email, tags, created_after/before, updated_after/before, title_substring, is_archived, is_hidden, athena_metadata, media_type, athena_converted_type, athena_original_type, summary_ready, summary_status, workspace_id, parent_folder_id (a folder id lists its direct children; null lists assets in no folder)
    /// * `sort` - JSON string of sort criteria: [{"field": "updated_at", "direction": "desc"}]. Supported fields: created_by_id, created_by_email, created_at, updated_at, is_archived, is_hidden, summary_ready, summary_status
    /// * `workspace_id` - Workspace to list assets from. Caller must be a member.
    /// * `parent_folder_id` - Only list the direct children of this folder (not its whole subtree). Only assets you can access are returned. To list assets in no folder, pass filters.parent_folder_id = null instead.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &AssetsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedAssetsOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v0/assets",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .serialize("filters", request.filters.clone())
                    .serialize("sort", request.sort.clone())
                    .serialize("workspace_id", request.workspace_id.clone())
                    .serialize("parent_folder_id", request.parent_folder_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Convert an uploaded Excel (.xlsx) asset into an editable Athena sheet — the same conversion the Athena UI performs. By default a new sheet asset is created alongside the source Excel asset; pass run_async for large workbooks to get the sheet immediately and poll its athena_metadata.conversionStatus for completion. With in_place the uploaded file itself becomes the sheet and keeps its asset id (sheet_asset_id is excel_asset_id, and no second asset is created); poll that asset until its media_type is 'athena/spreadsheet', or until athena_metadata.sheet_upload_conversion.status is 'failed' or 'skipped'.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn convert_excel_to_sheet(
        &self,
        request: &ConvertExcelToSheetRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<ConvertExcelToSheetResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/assets/convert-excel-to-sheet",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create a new asset such as a spreadsheet, document, folder, database, computer, user interface, skill, collab agent, figure, Word document, presentation, or generic doc (admin-only) in your workspace with your current permissions. Computer assets return 202 after durable submission, which commits the asset and initialization delivery intent together. Runtime provisioning continues asynchronously. Inspect the returned asset ID for progress instead of repeating creation. Computers accept the same configuration as the Athena UI's creation dialog: template, provider, cpu, memory (GiB), disk (GiB) and env_vars. Sizes must be offered by the environment's computer resource policy, and only admins may pick a size other than the template default (403 otherwise). In capability enforce mode, computer creation requires computer.create and returns 403 when denied. Databases accept suspend_timeout_seconds, the auto-suspend window from the database's Settings in Athena: 0 keeps the database running (it never suspends, and keeps using capacity), otherwise 60 to 86400 seconds. A database is provisioned in the background: poll GET /api/v0/databases/{asset_id}/compute-status until it reports running or suspended. Figures take figure_config, a FigureSpec (kind, rendered and provenance, plus spec or chart), validated as the Athena UI's figures are. A word_document is a Word document holding Athena's default template, and a presentation is a blank widescreen PPTX Studio deck (the workspace's default template is not applied): Athena creates each with its own studio credential, as your write, and answers once the studio has seeded it (502, keeping nothing, when PPTX Studio could not seed a deck). Edit them through DOCX Studio or PPTX Studio with a token from POST /api/v0/assets/{asset_id}/studio-token. From an agent run (a verified X-Athena-Session-Credential), their first write also records the run, and a presentation takes the create_presentation_pptx_studio tool policy. Without workspace_id the asset is created in the workspace of parent_folder_id when one is given, otherwise in your current workspace; you must be a member of that workspace.
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
        request: &CreateAssetRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<CreateAssetResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/assets/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create a Word document from a .docx file, or a presentation from a .pptx or .potx file, holding that file's content: the asset Athena makes when you create one from a file in the app. Athena imports the file through DOCX Studio or PPTX Studio with its own studio credential, as your write, and answers once the asset is ready (a large deck can take a few minutes). Nothing is left behind when the import fails. The file's name decides the type; the title defaults to the name without its extension. Gates are those of POST /api/v0/assets/create: membership in the workspace (workspace_id, else parent_folder_id's, else your current one) and EDIT on parent_folder_id. With source_asset_id (an asset you can read, for example the document the file was exported from) and no parent_folder_id, the new asset goes into the source's folder when you may create there, otherwise at your root. Unlike POST /api/v0/tools/file/save, which stores the file itself and turns an Office upload into its editable type in the background, this makes the editable asset in the request and keeps no file asset. A Word file may be up to 18.75 MiB, a PowerPoint file up to 100 MiB. From an agent run (a verified X-Athena-Session-Credential), a Word document's first write also records the run, and a presentation takes the create_presentation_pptx_studio tool policy.
    ///
    /// # Arguments
    ///
    /// * `title` - Title for the new asset; defaults to the file's name
    /// * `parent_folder_id` - ID of the folder to create the asset in
    /// * `workspace_id` - ID of the workspace to create the asset in; defaults to the workspace of parent_folder_id, else your current workspace
    /// * `source_asset_id` - An asset you can read that the file came from; without parent_folder_id the new asset goes next to it when you may create there
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_from_file(
        &self,
        request: &CreateFromFileRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateAssetResponseOut, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "api/v0/assets/create-from-file",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .serialize("title", request.title.clone())
                    .serialize("parent_folder_id", request.parent_folder_id.clone())
                    .serialize("workspace_id", request.workspace_id.clone())
                    .serialize("source_asset_id", request.source_asset_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new project with custom metadata. Projects can be typed (e.g., 'candidate', 'user', 'company') and include flexible custom metadata for storing additional information.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_project(
        &self,
        request: &CreateProjectRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<CreateProjectResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/assets/create_project",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Duplicate an asset using the same duplication service used by the Athena UI. Optionally target a workspace and/or destination folder.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn duplicate(
        &self,
        request: &DuplicateAssetRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<DuplicateAssetResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/assets/duplicate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve a single asset by its ID. Returns comprehensive metadata including creation info, tags, timestamps, media type, and AI-generated summary.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the asset to retrieve
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PublicAssetOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/assets/{}", asset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update an asset's display title. This supports folders and all other asset types the caller can edit, and applies the same rename side effects as the Athena application. Only the title changes: to change a skill's or collab agent's content, use PUT /api/v0/assets/{asset_id}/skill-config or /api/v0/assets/{asset_id}/agent-config.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the asset to rename
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn rename(
        &self,
        asset_id: &str,
        request: &RenameAssetRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<RenameAssetResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("api/v0/assets/{}", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read the access that you, or another user, have on an asset, and every grant that gives it. The level is what Athena's permission checks decide for the user in the asset's own workspace, whichever workspace is active for your key: 'view' can open the asset, 'edit' can also change it and manage its sharing, 'owner' created it and can edit it, and 'none' cannot open it. Each source names one grant: 'owner', 'direct' (shared with the user), 'group' (shared with a workspace group the user belongs to), 'workspace' (shared with the whole workspace), 'drive' (a role on the drive) or 'other' (access the share list does not hold, for example an environment admin). A folder's sharing reaches each asset inside it as that asset's own share, so a 'workspace' source on a sub-folder shows that a share of its parent folder reached it. Without user_id the route reads your own access; you need view access to the asset and membership in its workspace. To read another user's access, pass their user_id; you need edit access to the asset, as for sharing it. A user who is not a member of the asset's workspace, or who does not exist, reads as 'none' with is_member false. A suspended member reads as 'none' with no sources, and is_member still reports their membership. When a permission check cannot be answered (for example on an asset in a drive while the authorization service is unavailable), the route answers 503 and never a level. The route never changes a permission.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the asset
    /// * `user_id` - The user whose access to read. Leave it out to read your own. Reading another user's access needs edit access to the asset.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_access(
        &self,
        asset_id: &str,
        request: &GetAccessQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AssetAccessOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/assets/{}/access", asset_id),
                None,
                QueryBuilder::new()
                    .serialize("user_id", request.user_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Admin only. List the edit history of a collaborative asset, newest first: who edited it, when, and under which agent/session attribution. Works for every collaborative asset type. Each item's from_clock/to_clock identify the edit for the companion delta endpoint, which reports what actually changed.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum items to return.
    /// * `to_clock` - Return only items at or before this clock. Pass the previous response's next_page_to_clock to page backwards through history.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_activity(
        &self,
        asset_id: &str,
        request: &ListActivityQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AssetActivityResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/assets/{}/activity", asset_id),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .serialize("to_clock", request.to_clock.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Admin only. Report what changed between two Keryx clocks — for spreadsheets, the per-cell before/after values and row/column inserts and deletes (axis_changes); for documents, the inserted and deleted text; for presentations, the affected slides. Take the clocks from the activity endpoint. Computed by the same differ the in-app Activity pane renders, so the payload matches what a user sees. Always inspect delta.coverage: caps and non-decodable bulk regions are reported there rather than silently omitted.
    ///
    /// # Arguments
    ///
    /// * `from` - Start clock, from an activity item's from_clock.
    /// * `to` - End clock, from the same activity item's to_clock.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_activity_delta(
        &self,
        asset_id: &str,
        request: &GetActivityDeltaQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AssetActivityDeltaResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/assets/{}/activity/delta", asset_id),
                None,
                QueryBuilder::new()
                    .int("from", request.from.clone())
                    .int("to", request.to.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Admin only. Batch form of the activity-delta endpoint: diff several clock ranges in one request. Prefer this when walking a whole log — one call computes every range in a single pass over the document instead of one request each (up to 25 per call). Results come back in request order, and a range that could not be read carries its own `error` instead of failing the batch. Same payload and `coverage` semantics as the single-range endpoint.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_activity_deltas(
        &self,
        asset_id: &str,
        request: &AssetActivityDeltaBatchRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<AssetActivityDeltaResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/assets/{}/activity/deltas", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Write a collab agent's configuration (system prompt, model, tools, sources, behavior and channel settings) to its draft. Only the fields you send change; an omitted field keeps its current value, and a list or text field you send replaces the old one whole. The write goes to the live draft, the same draft the Athena editor shows, under the asset's existing id. Once the asset has been published, runs keep using the published version until you publish the draft with POST /api/v0/assets/{asset_id}/publish; one never published runs from its draft. Requires edit access to the asset; an archived asset is refused with 409. Unknown fields are refused with 422. A model must be available in the agent's workspace (422 otherwise). Tool or toolkit IDs the registry does not know are written and reported under warnings. A source (collab_source:<asset_id>) or Tool Asset (tool:<asset_id>) the agent does not already hold must be one the caller can open: one they cannot open, or an archived one, is refused with 403 naming it, and nothing is written. A new name also becomes the asset title. The agent's email address and its outbound approval recipients are not set here.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - ID of the collab agent to update
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_agent_config(
        &self,
        asset_id: &str,
        request: &UpdateAgentConfigRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<UpdateAssetConfigResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("api/v0/assets/{}/agent-config", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Archive an asset by its ID. The asset will be hidden from active listings (e.g. GET /assets with default filters) but can still be retrieved directly by ID. For folders, all children are also archived recursively. For meetings, associated sub-assets (recordings, transcripts) are archived as well. Only the creator of the asset can archive it.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the asset to archive
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn archive(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ArchiveAssetResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/assets/{}/archive", asset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Admin only. Mint a short-lived Keryx capability token for a Generic Doc asset, enabling live collaborative reads (and, with edit permission, writes) over WebSocket and REST. Only generic_doc assets are eligible — Athena-managed asset types are never reachable through this endpoint. The requested access is a ceiling clamped by the caller's permission on the asset.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_collab_token(
        &self,
        asset_id: &str,
        request: &CollabTokenRequest,
        options: Option<RequestOptions>,
    ) -> Result<CollabTokenResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/assets/{}/collab-token", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Turn an uploaded HTML file into a user interface (an app) under its own id — the conversion the Athena app's Convert to user interface performs. The asset keeps its id, folder, links and sharing (its title drops the .html extension), and no second asset is created. An uploaded HTML file stays a file until a request like this one asks: nothing converts on its own. The switch is one-way; the original upload stays downloadable from the asset's Versions pane. Requires edit access to the file. Idempotent: an asset that already is a user interface is answered as it is, with converted false. A refused conversion answers with a detail object: code CONVERSION_REFUSED, the reason, and a message saying why.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - ID of the uploaded HTML file to turn into a user interface
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn convert_to_user_interface(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ConvertToUserInterfaceResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/assets/{}/convert-to-user-interface", asset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Download an asset's file exactly as Athena stores or serves it — no type coercion, no pagination. Native collaborative assets are converted from live content to their canonical Office format: Athena documents download as .docx, spreadsheets as .xlsx (round-trip faithful — string identifiers, leading zeros, and number formats are preserved), PPTX Studio presentations and Word documents export their live studio content as .pptx/.docx. Uploaded files stream their original bytes. The response sets Content-Disposition with a filename derived from the asset title and media type. With `live_sync=true`, a spreadsheet's .xlsx, a PPTX Studio presentation's .pptx or a Word document's .docx also carries the Athena for Microsoft 365 add-in's link record, so opening it in Excel, PowerPoint or Word with the add-in installed starts syncing it with the asset; other asset types ignore the flag.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the asset to download
    /// * `live_sync` - Office live sync: when true and the asset is an Athena spreadsheet, a PPTX Studio presentation or a Word document, the downloaded .xlsx / .pptx / .docx carries the Athena for Microsoft 365 add-in's link record and opens already syncing with this asset. Ignored for every other asset type.
    /// * `addin_id` - GUID of the installed add-in manifest the live-sync record should reference (defaults to this deployment's Athena add-in). Only read together with `live_sync`; use it to target a preview-channel sideload.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
    pub async fn download(
        &self,
        asset_id: &str,
        request: &AssetsDownloadQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::GET,
                &format!("api/v0/assets/{}/download", asset_id),
                None,
                QueryBuilder::new()
                    .bool("live_sync", request.live_sync.clone())
                    .serialize("addin_id", request.addin_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Move an asset into a folder or to the workspace root. The asset ID determines the workspace used for authorization; parent_folder_id must belong to the same workspace. A move into a drive (target_drive_id, or a parent_folder_id inside a drive) carries the asset's whole folder subtree, so it is refused unless the caller can edit every item in it.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the asset to move
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn move_(
        &self,
        asset_id: &str,
        request: &MoveAssetRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<MoveAssetResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/assets/{}/move", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Publish a skill or collab agent so it is live for everyone who uses it. Omit version_id to snapshot and publish the current draft, or pass an existing version to promote it. Requires edit access to the asset.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - ID of the skill or collab agent to publish
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn publish(
        &self,
        asset_id: &str,
        request: &PublishAssetRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<PublishAssetResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/assets/{}/publish", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Share an asset with specific users by email. Only users who have edit access to the asset can share it. You can share with individual users (granting 'view' or 'edit' permission). Sharing with a user who does not have an account will result in an error for that recipient, but other recipients will still be processed.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the asset to share
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn share(
        &self,
        asset_id: &str,
        request: &ShareAssetRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<ShareAssetResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/assets/{}/share", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Write a skill's SKILL.md content (name, description, markdown body and the optional frontmatter) to its draft. Only the fields you send change; an omitted field keeps its current value, and a list or text field you send replaces the old one whole. The write goes to the live draft, the same draft the Athena editor shows, under the asset's existing id. Once the asset has been published, runs keep using the published version until you publish the draft with POST /api/v0/assets/{asset_id}/publish; one never published runs from its draft. Requires edit access to the asset; an archived asset is refused with 409. Unknown fields are refused with 422. The frontmatter is validated as it will read after the write; a skill that would not be spec-compliant is refused with 422 and nothing is written. A new name also becomes the asset title.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - ID of the skill to update
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_skill_config(
        &self,
        asset_id: &str,
        request: &UpdateSkillConfigRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<UpdateAssetConfigResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("api/v0/assets/{}/skill-config", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Mint a short-lived asset token for the service that edits an asset: a spreadsheet's, a Word document's or a deck's, named in ``audience``. The token names the caller, this asset and one access level, lasts 15 minutes and is good only on that service's routes for this asset, so code (an SDK in a computer, for example) never sends the service a long-lived credential. The caller must be able to view the asset and be a member of its workspace; the token is read-write only when the caller can edit the asset and asked for edit. A computer's or session's credential mints tokens for assets of its own workspace only. With audience 'pptx-studio-source' it mints the read-only token the deck editing service reads a spreadsheet, figure or semantic model with for a linked table, OLE object or chart; a recording's token is always that one. Any other asset (another type, a live provider file, or one outside every workspace) is refused with 422.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - The spreadsheet, Word document or deck to mint a token for
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_studio_token(
        &self,
        asset_id: &str,
        request: &Option<StudioTokenRequestIn>,
        options: Option<RequestOptions>,
    ) -> Result<StudioTokenResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/assets/{}/studio-token", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read a spreadsheet file's contents as rows, every tab: an uploaded .xlsx, .xlsm, .xls, .csv, .tsv or Parquet file, or a linked file in SharePoint, Google Drive, OneDrive or iManage. Excel and Parquet cells keep their types; a CSV or TSV field that reads as a number becomes one. A sheet is read whole up to a safety ceiling, and a file over 40 MB is refused (413). A native Athena spreadsheet answers 409: its contents are the collaborative sheet, which the spreadsheet SDK opens directly. Nothing is written.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - The spreadsheet file asset to read
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_workbook(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<FileBackedWorkbookResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/assets/{}/workbook", asset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update the workspace-level access on an asset. Needs edit access to the asset. Set 'view' or 'edit' to grant workspace-wide access, which also needs permission to share with the workspace. Set 'none' (or null) to remove the workspace-wide share, as DELETE does; removing a folder's share also removes it from the assets inside the folder. People and groups the asset is shared with directly keep their access.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the asset
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_workspace_access(
        &self,
        asset_id: &str,
        request: &UpdateWorkspaceAccessRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceAccessResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("api/v0/assets/{}/workspace-access", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Remove the workspace-wide share from an asset, so members of the workspace no longer get access through it. Needs edit access to the asset. Removing a folder's share also removes it from the assets inside the folder. People and groups the asset is shared with directly keep their access. Calling it on an asset with no workspace share, or on one in the trash, succeeds with the same answer.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - Unique identifier of the asset
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn remove_workspace_access(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceAccessResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("api/v0/assets/{}/workspace-access", asset_id),
                None,
                None,
                options,
            )
            .await
    }
}
