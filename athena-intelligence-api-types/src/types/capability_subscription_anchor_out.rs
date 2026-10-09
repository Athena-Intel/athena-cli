pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Where in the subject a subscription listens (a reference-standard anchor).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CapabilitySubscriptionAnchorOut {
    /// The A1 range, e.g. B2:B11.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    /// The sheet's name, when the anchor names one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet: Option<String>,
    /// The sheet's engine id, when the anchor names one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<i64>,
    /// The anchor type: sheet_range.
    #[serde(default)]
    pub r#type: String,
}

impl CapabilitySubscriptionAnchorOut {
    pub fn builder() -> CapabilitySubscriptionAnchorOutBuilder {
        <CapabilitySubscriptionAnchorOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilitySubscriptionAnchorOutBuilder {
    range: Option<String>,
    sheet: Option<String>,
    sheet_id: Option<i64>,
    r#type: Option<String>,
}

impl CapabilitySubscriptionAnchorOutBuilder {
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

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CapabilitySubscriptionAnchorOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](CapabilitySubscriptionAnchorOutBuilder::r#type)
    pub fn build(self) -> Result<CapabilitySubscriptionAnchorOut, BuildError> {
        Ok(CapabilitySubscriptionAnchorOut {
            range: self.range,
            sheet: self.sheet,
            sheet_id: self.sheet_id,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
