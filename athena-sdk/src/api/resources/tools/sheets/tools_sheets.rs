use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SheetsClient {
    pub http_client: HttpClient,
}

impl SheetsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Update a single cell in an Athena spreadsheet.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_cell(
        &self,
        request: &UpdateSheetCellRequest,
        options: Option<RequestOptions>,
    ) -> Result<SheetOperationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/sheets/cell/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete cells from an Athena spreadsheet.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_cells(
        &self,
        request: &DeleteCellsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SheetOperationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/sheets/cells/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete columns from an Athena spreadsheet.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_column(
        &self,
        request: &DeleteColumnRequest,
        options: Option<RequestOptions>,
    ) -> Result<SheetOperationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/sheets/column/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Insert a column in an Athena spreadsheet.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn insert_column(
        &self,
        request: &InsertColumnRequest,
        options: Option<RequestOptions>,
    ) -> Result<SheetOperationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/sheets/column/insert",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Clear formatting from cells in an Athena spreadsheet.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn clear_formatting(
        &self,
        request: &ClearFormattingRequest,
        options: Option<RequestOptions>,
    ) -> Result<SheetOperationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/sheets/formatting/clear",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Clear a range of cells in an Athena spreadsheet.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn clear_range(
        &self,
        request: &ClearSheetRangeRequest,
        options: Option<RequestOptions>,
    ) -> Result<SheetOperationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/sheets/range/clear",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Apply formatting to a range of cells in an Athena spreadsheet.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn format_range(
        &self,
        request: &FormatSheetRangeRequest,
        options: Option<RequestOptions>,
    ) -> Result<SheetOperationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/sheets/range/format",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Update a range of cells in an Athena spreadsheet.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_range(
        &self,
        request: &UpdateSheetRangeRequest,
        options: Option<RequestOptions>,
    ) -> Result<SheetOperationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/sheets/range/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Insert a row in an Athena spreadsheet.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn insert_row(
        &self,
        request: &InsertRowRequest,
        options: Option<RequestOptions>,
    ) -> Result<SheetOperationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/sheets/row/insert",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Duplicate an existing sheet in an Athena spreadsheet.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn duplicate_sheet(
        &self,
        request: &DuplicateSheetRequest,
        options: Option<RequestOptions>,
    ) -> Result<SheetOperationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/sheets/sheet/duplicate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create a new tab in an Athena spreadsheet.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_tab(
        &self,
        request: &CreateNewSheetTabRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateNewSheetTabResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/sheets/tab/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a column from a table within an Athena spreadsheet.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_table_column(
        &self,
        request: &DeleteTableColumnRequest,
        options: Option<RequestOptions>,
    ) -> Result<SheetOperationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/sheets/table/column/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Insert a column in a table within an Athena spreadsheet.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn insert_table_column(
        &self,
        request: &InsertTableColumnRequest,
        options: Option<RequestOptions>,
    ) -> Result<SheetOperationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/sheets/table/column/insert",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create a table in an Athena spreadsheet.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_table(
        &self,
        request: &CreateTableRequest,
        options: Option<RequestOptions>,
    ) -> Result<SheetOperationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/sheets/table/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve table data from an Athena spreadsheet.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_table(
        &self,
        request: &GetTableRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetTableResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/sheets/table/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Insert rows into a table in an Athena spreadsheet.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn insert_table_row(
        &self,
        request: &InsertTableRowRequest,
        options: Option<RequestOptions>,
    ) -> Result<SheetOperationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/sheets/table/insert-row",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Update an existing table in an Athena spreadsheet.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_table(
        &self,
        request: &UpdateTableRequest,
        options: Option<RequestOptions>,
    ) -> Result<SheetOperationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v0/tools/sheets/table/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
