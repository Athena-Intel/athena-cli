pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssetReadRequestIn {
    /// 1-10 asset identifiers to read. Each id may carry inline read options using citation-style query syntax, e.g. 'asset_xxx?anchor=page&page=3&format=image'. Versioned ('asset_xxx@version') and live ('asset_xxx_providerId') ids are supported.
    #[serde(default)]
    pub asset_ids: Vec<String>,
    /// Optional password for reading password-protected Office files. Applied to every asset in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl AssetReadRequestIn {
    pub fn builder() -> AssetReadRequestInBuilder {
        <AssetReadRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetReadRequestInBuilder {
    asset_ids: Option<Vec<String>>,
    password: Option<String>,
}

impl AssetReadRequestInBuilder {
    pub fn asset_ids(mut self, value: Vec<String>) -> Self {
        self.asset_ids = Some(value);
        self
    }

    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssetReadRequestIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_ids`](AssetReadRequestInBuilder::asset_ids)
    pub fn build(self) -> Result<AssetReadRequestIn, BuildError> {
        Ok(AssetReadRequestIn {
            asset_ids: self.asset_ids.ok_or_else(|| BuildError::missing_field("asset_ids"))?,
            password: self.password,
        })
    }
}

