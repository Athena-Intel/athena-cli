pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One tab of a spreadsheet file, as rows of values.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FileBackedSheetResponseOut {
    /// The tab's name; for a CSV, TSV or Parquet file, the file's name without its extension.
    #[serde(default)]
    pub name: String,
    /// The tab's rows from its first row down, each a list of its cell values: numbers, strings, booleans, ISO dates, or null for an empty cell. A CSV or TSV field that reads as a number is one.
    #[serde(default)]
    pub rows: Vec<Vec<serde_json::Value>>,
}

impl FileBackedSheetResponseOut {
    pub fn builder() -> FileBackedSheetResponseOutBuilder {
        <FileBackedSheetResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FileBackedSheetResponseOutBuilder {
    name: Option<String>,
    rows: Option<Vec<Vec<serde_json::Value>>>,
}

impl FileBackedSheetResponseOutBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<Vec<serde_json::Value>>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FileBackedSheetResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](FileBackedSheetResponseOutBuilder::name)
    /// - [`rows`](FileBackedSheetResponseOutBuilder::rows)
    pub fn build(self) -> Result<FileBackedSheetResponseOut, BuildError> {
        Ok(FileBackedSheetResponseOut {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
