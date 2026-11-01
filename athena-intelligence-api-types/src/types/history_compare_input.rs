pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HistoryCompareInput {
    /// The asset (asset_…).
    #[serde(default)]
    pub asset_id: String,
    /// Also return a text redline in Word's markup view: all (insertions {+…+} and deletions {-…-} inline), simple (changed lines marked), none (the revised text), original (the original text).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markup: Option<HistoryCompareInputMarkup>,
    /// The original state to compare from: current, published, a saved version id or name, a time (2026-10-28T14:00Z), or another asset id (asset_…, optionally @<version>).
    #[serde(default)]
    pub original: String,
    /// The revised state to compare to (default current): current, published, a saved version id or name, a time (2026-10-28T14:00Z), or another asset id (asset_…, optionally @<version>).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revised: Option<String>,
    /// Where: a cell or range (Sheet1!B6, 'Q3 Plan'!B2:F40), a sheet (sheet:Sheet1), a slide (slide 7), or text to look for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#where: Option<String>,
}

impl HistoryCompareInput {
    pub fn builder() -> HistoryCompareInputBuilder {
        <HistoryCompareInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HistoryCompareInputBuilder {
    asset_id: Option<String>,
    markup: Option<HistoryCompareInputMarkup>,
    original: Option<String>,
    revised: Option<String>,
    r#where: Option<String>,
}

impl HistoryCompareInputBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn markup(mut self, value: HistoryCompareInputMarkup) -> Self {
        self.markup = Some(value);
        self
    }

    pub fn original(mut self, value: impl Into<String>) -> Self {
        self.original = Some(value.into());
        self
    }

    pub fn revised(mut self, value: impl Into<String>) -> Self {
        self.revised = Some(value.into());
        self
    }

    pub fn r#where(mut self, value: impl Into<String>) -> Self {
        self.r#where = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`HistoryCompareInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](HistoryCompareInputBuilder::asset_id)
    /// - [`original`](HistoryCompareInputBuilder::original)
    pub fn build(self) -> Result<HistoryCompareInput, BuildError> {
        Ok(HistoryCompareInput {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            markup: self.markup,
            original: self.original.ok_or_else(|| BuildError::missing_field("original"))?,
            revised: self.revised,
            r#where: self.r#where,
        })
    }
}

