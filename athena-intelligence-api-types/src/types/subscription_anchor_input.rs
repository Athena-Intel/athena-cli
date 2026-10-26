pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A place inside the subject the subscription listens to.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscriptionAnchorInput {
    /// An A1 range on that sheet, e.g. B2:B11.
    #[serde(default)]
    pub range: String,
    /// The sheet's name, when you have no id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet: Option<String>,
    /// The sheet's id, from read_asset's workbook.metadata.sheets; it survives a rename. Omit both sheet_id and sheet for any sheet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<i64>,
    /// The anchor's kind; a spreadsheet range is the only one today.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<SubscriptionAnchorInputType>,
}

impl SubscriptionAnchorInput {
    pub fn builder() -> SubscriptionAnchorInputBuilder {
        <SubscriptionAnchorInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionAnchorInputBuilder {
    range: Option<String>,
    sheet: Option<String>,
    sheet_id: Option<i64>,
    r#type: Option<SubscriptionAnchorInputType>,
}

impl SubscriptionAnchorInputBuilder {
    pub fn range(mut self, value: impl Into<String>) -> Self {
        self.range = Some(value.into());
        self
    }

    pub fn sheet(mut self, value: impl Into<String>) -> Self {
        self.sheet = Some(value.into());
        self
    }

    pub fn sheet_id(mut self, value: i64) -> Self {
        self.sheet_id = Some(value);
        self
    }

    pub fn r#type(mut self, value: SubscriptionAnchorInputType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionAnchorInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`range`](SubscriptionAnchorInputBuilder::range)
    pub fn build(self) -> Result<SubscriptionAnchorInput, BuildError> {
        Ok(SubscriptionAnchorInput {
            range: self.range.ok_or_else(|| BuildError::missing_field("range"))?,
            sheet: self.sheet,
            sheet_id: self.sheet_id,
            r#type: self.r#type,
        })
    }
}
