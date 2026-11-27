pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One kind of content an Office file's import could not carry.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicImportLossOut {
    /// The importer's stable codes behind it, with their counts (e.g. {'alternate_content_a14_dropped': 2})
    #[serde(default)]
    pub codes: HashMap<String, i64>,
    /// How many there are, or null for a kind whose occurrences are not counted as one number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// What a download of the file does with it: 'kept', 'kept_unless_edited', 'edits_not_saved' or 'usually_kept'; null when unknown
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<String>,
    /// That, as a sentence (e.g. 'Downloads keep them.'); empty when unknown
    #[serde(default)]
    pub download_note: String,
    /// The kind of loss, a stable id (e.g. 'equations_blank')
    #[serde(default)]
    pub kind: String,
    /// What is affected (e.g. '2 equations are shown as blank')
    #[serde(default)]
    pub summary: String,
}

impl PublicImportLossOut {
    pub fn builder() -> PublicImportLossOutBuilder {
        <PublicImportLossOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicImportLossOutBuilder {
    codes: Option<HashMap<String, i64>>,
    count: Option<i64>,
    download: Option<String>,
    download_note: Option<String>,
    kind: Option<String>,
    summary: Option<String>,
}

impl PublicImportLossOutBuilder {
    pub fn codes(mut self, value: HashMap<String, i64>) -> Self {
        self.codes = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn download(mut self, value: impl Into<String>) -> Self {
        self.download = Some(value.into());
        self
    }

    pub fn download_note(mut self, value: impl Into<String>) -> Self {
        self.download_note = Some(value.into());
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn summary(mut self, value: impl Into<String>) -> Self {
        self.summary = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PublicImportLossOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`codes`](PublicImportLossOutBuilder::codes)
    /// - [`download_note`](PublicImportLossOutBuilder::download_note)
    /// - [`kind`](PublicImportLossOutBuilder::kind)
    /// - [`summary`](PublicImportLossOutBuilder::summary)
    pub fn build(self) -> Result<PublicImportLossOut, BuildError> {
        Ok(PublicImportLossOut {
            codes: self.codes.ok_or_else(|| BuildError::missing_field("codes"))?,
            count: self.count,
            download: self.download,
            download_note: self.download_note.ok_or_else(|| BuildError::missing_field("download_note"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            summary: self.summary.ok_or_else(|| BuildError::missing_field("summary"))?,
        })
    }
}
