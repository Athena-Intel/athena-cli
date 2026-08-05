pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConvertExcelToSheetRequestIn {
    /// ID of the Excel (.xlsx) asset to convert into an Athena sheet
    #[serde(default)]
    pub excel_asset_id: String,
    /// Optional password used to decrypt a password-protected workbook. Password-protected workbooks always convert on the 'legacy' engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Return the sheet asset immediately and convert in the background. Large workbooks outlive the gateway's ~60s response window on the synchronous path — with run_async the caller polls athena_metadata.conversionStatus ('converting' | 'completed' | 'failed', with conversionError on failure) instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_async: Option<bool>,
    /// Engine for the converted sheet ('rnc' | 'legacy'). Defaults to the deployment's default engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spreadsheet_engine: Option<String>,
}

impl ConvertExcelToSheetRequestIn {
    pub fn builder() -> ConvertExcelToSheetRequestInBuilder {
        <ConvertExcelToSheetRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConvertExcelToSheetRequestInBuilder {
    excel_asset_id: Option<String>,
    password: Option<String>,
    run_async: Option<bool>,
    spreadsheet_engine: Option<String>,
}

impl ConvertExcelToSheetRequestInBuilder {
    pub fn excel_asset_id(mut self, value: impl Into<String>) -> Self {
        self.excel_asset_id = Some(value.into());
        self
    }

    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = Some(value.into());
        self
    }

    pub fn run_async(mut self, value: bool) -> Self {
        self.run_async = Some(value);
        self
    }

    pub fn spreadsheet_engine(mut self, value: impl Into<String>) -> Self {
        self.spreadsheet_engine = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConvertExcelToSheetRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`excel_asset_id`](ConvertExcelToSheetRequestInBuilder::excel_asset_id)
    pub fn build(self) -> Result<ConvertExcelToSheetRequestIn, BuildError> {
        Ok(ConvertExcelToSheetRequestIn {
            excel_asset_id: self.excel_asset_id.ok_or_else(|| BuildError::missing_field("excel_asset_id"))?,
            password: self.password,
            run_async: self.run_async,
            spreadsheet_engine: self.spreadsheet_engine,
        })
    }
}

