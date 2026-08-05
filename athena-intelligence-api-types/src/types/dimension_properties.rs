pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Properties for a row or column dimension in the spreadsheet.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DimensionProperties {
    /// True if this row/column is hidden due to a filter being applied. Do not modify directly - managed by filter operations
    #[serde(rename = "hiddenByFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_by_filter: Option<bool>,
    /// True if the user explicitly hid this row/column (e.g., right-click > Hide). Set to False to unhide
    #[serde(rename = "hiddenByUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_by_user: Option<bool>,
    /// True if the user manually resized this dimension (e.g., dragged column border to resize). Must be set to True whenever you set the 'size' field
    #[serde(rename = "resizedByUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resized_by_user: Option<bool>,
    /// Size in pixels. For columns, this is the width. For rows, this is the height. Default: 100px for columns, 21px for rows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

impl DimensionProperties {
    pub fn builder() -> DimensionPropertiesBuilder {
        <DimensionPropertiesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DimensionPropertiesBuilder {
    hidden_by_filter: Option<bool>,
    hidden_by_user: Option<bool>,
    resized_by_user: Option<bool>,
    size: Option<i64>,
}

impl DimensionPropertiesBuilder {
    pub fn hidden_by_filter(mut self, value: bool) -> Self {
        self.hidden_by_filter = Some(value);
        self
    }

    pub fn hidden_by_user(mut self, value: bool) -> Self {
        self.hidden_by_user = Some(value);
        self
    }

    pub fn resized_by_user(mut self, value: bool) -> Self {
        self.resized_by_user = Some(value);
        self
    }

    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DimensionProperties`].
    pub fn build(self) -> Result<DimensionProperties, BuildError> {
        Ok(DimensionProperties {
            hidden_by_filter: self.hidden_by_filter,
            hidden_by_user: self.hidden_by_user,
            resized_by_user: self.resized_by_user,
            size: self.size,
        })
    }
}
