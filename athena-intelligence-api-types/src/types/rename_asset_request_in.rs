pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RenameAssetRequestIn {
    /// Rename only while the asset still carries this title. When another client renamed it in the meantime the request fails with 409 and the stored title is kept.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_title: Option<String>,
    /// New display title for the asset
    #[serde(default)]
    pub title: String,
}

impl RenameAssetRequestIn {
    pub fn builder() -> RenameAssetRequestInBuilder {
        <RenameAssetRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RenameAssetRequestInBuilder {
    expected_title: Option<String>,
    title: Option<String>,
}

impl RenameAssetRequestInBuilder {
    pub fn expected_title(mut self, value: impl Into<String>) -> Self {
        self.expected_title = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RenameAssetRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](RenameAssetRequestInBuilder::title)
    pub fn build(self) -> Result<RenameAssetRequestIn, BuildError> {
        Ok(RenameAssetRequestIn {
            expected_title: self.expected_title,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}

