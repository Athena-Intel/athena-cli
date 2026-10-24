pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConvertExcelToSheetRequestIn {
    /// ID of the Excel (.xlsx) asset to convert into an Athena sheet
    #[serde(default)]
    pub excel_asset_id: String,
    /// Make the uploaded file itself the Athena sheet, keeping its asset id: sheet_asset_id is excel_asset_id and no second asset is created. Needs edit access to the upload, and takes an .xlsx/.xlsm/.xlsb workbook or a .csv/.tsv file. A password-protected workbook is refused (convert it into a new sheet with `password` instead), and the sheet is always on the rnc engine. Without run_async the request waits up to about 50 seconds for the conversion; conversion_status 'converting' means it is still running. Poll the asset until its media_type is 'athena/spreadsheet', or until athena_metadata.sheet_upload_conversion.status is 'failed' or 'skipped' (with its reason).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_place: Option<bool>,
    /// Optional password used to decrypt a password-protected workbook. Password-protected workbooks convert on either engine (they are decrypted server-side before an rnc import).
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
    in_place: Option<bool>,
    password: Option<String>,
    run_async: Option<bool>,
    spreadsheet_engine: Option<String>,
}

impl ConvertExcelToSheetRequestInBuilder {
    pub fn excel_asset_id(mut self, value: impl Into<String>) -> Self {
        self.excel_asset_id = Some(value.into());
        self
    }

    pub fn in_place(mut self, value: bool) -> Self {
        self.in_place = Some(value);
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
            in_place: self.in_place,
            password: self.password,
            run_async: self.run_async,
            spreadsheet_engine: self.spreadsheet_engine,
        })
    }
}

