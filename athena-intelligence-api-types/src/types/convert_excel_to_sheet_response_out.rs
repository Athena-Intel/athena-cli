pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for converting an Excel asset to an Athena sheet.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConvertExcelToSheetResponseOut {
    /// Type of the created sheet asset
    #[serde(default)]
    pub asset_type: String,
    /// Conversion status for background (run_async) conversions: 'converting' immediately after the request. Poll the asset's athena_metadata.conversionStatus for 'completed' | 'failed'. Null for synchronous conversions, which have already completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_status: Option<String>,
    /// ID of the source Excel asset
    #[serde(default)]
    pub excel_asset_id: String,
    /// ID of the sheet asset's parent folder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// ID of the newly created Athena sheet asset
    #[serde(default)]
    pub sheet_asset_id: String,
    /// Title of the created sheet asset
    #[serde(default)]
    pub title: String,
    /// ID of the workspace that owns the sheet asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl ConvertExcelToSheetResponseOut {
    pub fn builder() -> ConvertExcelToSheetResponseOutBuilder {
        <ConvertExcelToSheetResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConvertExcelToSheetResponseOutBuilder {
    asset_type: Option<String>,
    conversion_status: Option<String>,
    excel_asset_id: Option<String>,
    parent_folder_id: Option<String>,
    sheet_asset_id: Option<String>,
    title: Option<String>,
    workspace_id: Option<String>,
}

impl ConvertExcelToSheetResponseOutBuilder {
    pub fn asset_type(mut self, value: impl Into<String>) -> Self {
        self.asset_type = Some(value.into());
        self
    }

    pub fn conversion_status(mut self, value: impl Into<String>) -> Self {
        self.conversion_status = Some(value.into());
        self
    }

    pub fn excel_asset_id(mut self, value: impl Into<String>) -> Self {
        self.excel_asset_id = Some(value.into());
        self
    }

    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
        self
    }

    pub fn sheet_asset_id(mut self, value: impl Into<String>) -> Self {
        self.sheet_asset_id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConvertExcelToSheetResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_type`](ConvertExcelToSheetResponseOutBuilder::asset_type)
    /// - [`excel_asset_id`](ConvertExcelToSheetResponseOutBuilder::excel_asset_id)
    /// - [`sheet_asset_id`](ConvertExcelToSheetResponseOutBuilder::sheet_asset_id)
    /// - [`title`](ConvertExcelToSheetResponseOutBuilder::title)
    pub fn build(self) -> Result<ConvertExcelToSheetResponseOut, BuildError> {
        Ok(ConvertExcelToSheetResponseOut {
            asset_type: self.asset_type.ok_or_else(|| BuildError::missing_field("asset_type"))?,
            conversion_status: self.conversion_status,
            excel_asset_id: self.excel_asset_id.ok_or_else(|| BuildError::missing_field("excel_asset_id"))?,
            parent_folder_id: self.parent_folder_id,
            sheet_asset_id: self.sheet_asset_id.ok_or_else(|| BuildError::missing_field("sheet_asset_id"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            workspace_id: self.workspace_id,
        })
    }
}
