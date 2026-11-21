pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What the import of a Word document, spreadsheet or presentation could not carry.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicImportReportOut {
    /// The kinds of loss, most significant first
    #[serde(default)]
    pub items: Vec<PublicImportLossOut>,
}

impl PublicImportReportOut {
    pub fn builder() -> PublicImportReportOutBuilder {
        <PublicImportReportOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicImportReportOutBuilder {
    items: Option<Vec<PublicImportLossOut>>,
}

impl PublicImportReportOutBuilder {
    pub fn items(mut self, value: Vec<PublicImportLossOut>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PublicImportReportOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](PublicImportReportOutBuilder::items)
    pub fn build(self) -> Result<PublicImportReportOut, BuildError> {
        Ok(PublicImportReportOut {
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}
