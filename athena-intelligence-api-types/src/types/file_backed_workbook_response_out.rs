pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An uploaded or linked spreadsheet file's contents, read-only.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FileBackedWorkbookResponseOut {
    /// Always true: a file's contents are read here, never written.
    #[serde(default)]
    pub read_only: bool,
    /// Every tab of the file, in its order.
    #[serde(default)]
    pub sheets: Vec<FileBackedSheetResponseOut>,
}

impl FileBackedWorkbookResponseOut {
    pub fn builder() -> FileBackedWorkbookResponseOutBuilder {
        <FileBackedWorkbookResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FileBackedWorkbookResponseOutBuilder {
    read_only: Option<bool>,
    sheets: Option<Vec<FileBackedSheetResponseOut>>,
}

impl FileBackedWorkbookResponseOutBuilder {
    pub fn read_only(mut self, value: bool) -> Self {
        self.read_only = Some(value);
        self
    }

    pub fn sheets(mut self, value: Vec<FileBackedSheetResponseOut>) -> Self {
        self.sheets = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FileBackedWorkbookResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`read_only`](FileBackedWorkbookResponseOutBuilder::read_only)
    /// - [`sheets`](FileBackedWorkbookResponseOutBuilder::sheets)
    pub fn build(self) -> Result<FileBackedWorkbookResponseOut, BuildError> {
        Ok(FileBackedWorkbookResponseOut {
            read_only: self.read_only.ok_or_else(|| BuildError::missing_field("read_only"))?,
            sheets: self.sheets.ok_or_else(|| BuildError::missing_field("sheets"))?,
        })
    }
}
