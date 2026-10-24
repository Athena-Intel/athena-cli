use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct DatabasesClient {
    pub http_client: HttpClient,
}

impl DatabasesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Check if a database is running, suspended, or starting up. Poll this endpoint to determine when a serverless database is ready.
    ///
    /// **Status Values:**
    /// - `running` - Database is active and accepting connections
    /// - `suspended` - Database is suspended (scale-to-zero). It wakes on `POST /databases/{asset_id}/wake` or on the first SQL request. When `reason` is `ComputeUnschedulable`, no node could host its compute size: waking it only restarts the same wait
    /// - `starting` - Database is waking up, or its compute is being recreated
    /// - `creating` - Database is being created; it has no compute yet
    /// - `failed` - Database failed to start, or its creation failed (`message` says which)
    /// - `unknown` - Status could not be determined (the control plane did not answer); retry
    ///
    /// A database suspends (scales to zero) after `suspend_timeout_seconds` without activity; `GET`/`PATCH /databases/{asset_id}/settings` read and set that window, and 0 keeps it running. Reading the compute status or the settings never wakes a suspended database. Two things do: `POST /databases/{asset_id}/wake`, which answers at once while the compute starts, or any request that runs SQL on it, such as `POST /databases/{asset_id}/sql`, which waits for the compute up to 120 seconds; if the compute is still starting then, the answer is 503 with `Retry-After` and the statement did not run. A cold start usually takes 30 seconds to 2 minutes.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_status(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DatabaseStatusResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/databases/{}/compute-status", asset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Get a list of all tables in the database with optional row counts.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_tables(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DatabaseTablesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/databases/{}/data", asset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Query rows from a table in the database. Supports filtering, ordering, and pagination using PostgREST-style query parameters.
    ///
    /// **Filter Syntax:**
    /// - `?column=eq.value` - Equal
    /// - `?column=neq.value` - Not equal
    /// - `?column=gt.value` - Greater than
    /// - `?column=gte.value` - Greater than or equal
    /// - `?column=lt.value` - Less than
    /// - `?column=lte.value` - Less than or equal
    /// - `?column=like.*pattern*` - LIKE (case-sensitive)
    /// - `?column=ilike.*pattern*` - ILIKE (case-insensitive)
    /// - `?column=in.(a,b,c)` - IN list
    /// - `?column=is.null` - IS NULL
    ///
    /// # Arguments
    ///
    /// * `select` - Columns to return (comma-separated, e.g., 'id,name,email')
    /// * `order` - Order by clause (e.g., 'created_at.desc', 'name.asc')
    /// * `limit` - Maximum number of rows to return
    /// * `offset` - Number of rows to skip
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn select(
        &self,
        asset_id: &str,
        table_name: &str,
        request: &SelectQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DatabaseDataResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/databases/{}/data/{}", asset_id, table_name),
                None,
                QueryBuilder::new()
                    .serialize("select", request.select.clone())
                    .serialize("order", request.order.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Insert one or more rows into a table.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn insert(
        &self,
        asset_id: &str,
        table_name: &str,
        request: &InsertDataRequest,
        options: Option<RequestOptions>,
    ) -> Result<DatabaseMutationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/databases/{}/data/{}", asset_id, table_name),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete rows matching the filter conditions. Filter conditions are passed as query parameters using PostgREST syntax.
    ///
    /// **Filter Syntax:**
    /// - `?column=eq.value` - Equal
    /// - `?column=neq.value` - Not equal
    /// - `?column=gt.value` - Greater than
    /// - `?column=gte.value` - Greater than or equal
    /// - `?column=lt.value` - Less than
    /// - `?column=lte.value` - Less than or equal
    /// - `?column=like.*pattern*` - LIKE (case-sensitive)
    /// - `?column=ilike.*pattern*` - ILIKE (case-insensitive)
    /// - `?column=in.(a,b,c)` - IN list
    /// - `?column=is.null` - IS NULL
    ///
    /// **Safety:** Filters are required by default to prevent accidental bulk deletes. To delete all rows intentionally, pass `?force=true`.
    ///
    /// # Arguments
    ///
    /// * `force` - Set to true to delete all rows (required when no filters provided)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete(
        &self,
        asset_id: &str,
        table_name: &str,
        request: &DeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<DatabaseMutationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("api/v0/databases/{}/data/{}", asset_id, table_name),
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .bool("force", request.force.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Update rows matching the filter conditions. Filter conditions are passed as query parameters using PostgREST syntax.
    ///
    /// **Filter Syntax:**
    /// - `?column=eq.value` - Equal
    /// - `?column=neq.value` - Not equal
    /// - `?column=gt.value` - Greater than
    /// - `?column=gte.value` - Greater than or equal
    /// - `?column=lt.value` - Less than
    /// - `?column=lte.value` - Less than or equal
    /// - `?column=like.*pattern*` - LIKE (case-sensitive)
    /// - `?column=ilike.*pattern*` - ILIKE (case-insensitive)
    /// - `?column=in.(a,b,c)` - IN list
    /// - `?column=is.null` - IS NULL
    ///
    /// **Safety:** Filters are required by default to prevent accidental bulk updates. To update all rows intentionally, pass `?force=true`.
    ///
    /// # Arguments
    ///
    /// * `force` - Set to true to update all rows (required when no filters provided)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        asset_id: &str,
        table_name: &str,
        request: &UpdateDataRequest,
        options: Option<RequestOptions>,
    ) -> Result<DatabaseMutationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("api/v0/databases/{}/data/{}", asset_id, table_name),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .bool("force", request.force.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get the schema for a specific table, including column names, types, nullability, and default values. Useful for agent tooling and dynamic form generation.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_table_schema(
        &self,
        asset_id: &str,
        table_name: &str,
        options: Option<RequestOptions>,
    ) -> Result<DatabaseTableSchemaResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/databases/{}/schema/{}", asset_id, table_name),
                None,
                None,
                options,
            )
            .await
    }

    /// Read the database's auto-suspend window, the same setting as auto-suspend in the database's Settings in Athena. Reading never wakes the database.
    ///
    /// A database suspends (scales to zero) after `suspend_timeout_seconds` without activity; `GET`/`PATCH /databases/{asset_id}/settings` read and set that window, and 0 keeps it running. Reading the compute status or the settings never wakes a suspended database. Two things do: `POST /databases/{asset_id}/wake`, which answers at once while the compute starts, or any request that runs SQL on it, such as `POST /databases/{asset_id}/sql`, which waits for the compute up to 120 seconds; if the compute is still starting then, the answer is 503 with `Retry-After` and the statement did not run. A cold start usually takes 30 seconds to 2 minutes.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_settings(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DatabaseComputeSettingsResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v0/databases/{}/settings", asset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Set the database's auto-suspend window: 0 keeps the database running (it never suspends, and keeps using compute capacity), otherwise 60 to 86400 seconds. The same setting, values and edit-access rule as auto-suspend in the database's Settings in Athena; the change applies at once and does not restart or wake the database.
    ///
    /// A database suspends (scales to zero) after `suspend_timeout_seconds` without activity; `GET`/`PATCH /databases/{asset_id}/settings` read and set that window, and 0 keeps it running. Reading the compute status or the settings never wakes a suspended database. Two things do: `POST /databases/{asset_id}/wake`, which answers at once while the compute starts, or any request that runs SQL on it, such as `POST /databases/{asset_id}/sql`, which waits for the compute up to 120 seconds; if the compute is still starting then, the answer is 503 with `Retry-After` and the statement did not run. A cold start usually takes 30 seconds to 2 minutes.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_settings(
        &self,
        asset_id: &str,
        request: &DatabaseComputeSettingsUpdateRequestIn,
        options: Option<RequestOptions>,
    ) -> Result<DatabaseComputeSettingsResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("api/v0/databases/{}/settings", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Execute a SQL statement against the database. SELECT queries return columns and rows. Non-SELECT statements (CREATE, INSERT, UPDATE, DELETE, ALTER, DROP, etc.) return execution statuses.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn execute_sql(
        &self,
        asset_id: &str,
        request: &DatabaseSqlRequest,
        options: Option<RequestOptions>,
    ) -> Result<DatabaseSqlResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/databases/{}/sql", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Start a suspended database's compute and answer at once, without waiting for it to run. Poll `GET /databases/{asset_id}/compute-status` until it reports `running`. Waking a running database resets its idle clock and answers 200 with `running`, so it is safe to call before every batch of work. The same action as Resume Database in Athena; it needs edit access to the database.
    ///
    /// A database suspends (scales to zero) after `suspend_timeout_seconds` without activity; `GET`/`PATCH /databases/{asset_id}/settings` read and set that window, and 0 keeps it running. Reading the compute status or the settings never wakes a suspended database. Two things do: `POST /databases/{asset_id}/wake`, which answers at once while the compute starts, or any request that runs SQL on it, such as `POST /databases/{asset_id}/sql`, which waits for the compute up to 120 seconds; if the compute is still starting then, the answer is 503 with `Retry-After` and the statement did not run. A cold start usually takes 30 seconds to 2 minutes.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn wake(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DatabaseWakeResponseOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v0/databases/{}/wake", asset_id),
                None,
                None,
                options,
            )
            .await
    }
}
