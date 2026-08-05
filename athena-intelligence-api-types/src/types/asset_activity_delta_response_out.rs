pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Deltas for the requested clock ranges.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AssetActivityDeltaResponseOut {
    #[serde(default)]
    pub asset_id: String,
    #[serde(default)]
    pub asset_type: String,
    #[serde(default)]
    pub deltas: Vec<ActivityDeltaOut>,
    /// Version of the delta payload shape. Detail fields are added additively within a version; a breaking change bumps it.
    #[serde(default)]
    pub schema_version: String,
}

impl AssetActivityDeltaResponseOut {
    pub fn builder() -> AssetActivityDeltaResponseOutBuilder {
        <AssetActivityDeltaResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetActivityDeltaResponseOutBuilder {
    asset_id: Option<String>,
    asset_type: Option<String>,
    deltas: Option<Vec<ActivityDeltaOut>>,
    schema_version: Option<String>,
}

impl AssetActivityDeltaResponseOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn asset_type(mut self, value: impl Into<String>) -> Self {
        self.asset_type = Some(value.into());
        self
    }

    pub fn deltas(mut self, value: Vec<ActivityDeltaOut>) -> Self {
        self.deltas = Some(value);
        self
    }

    pub fn schema_version(mut self, value: impl Into<String>) -> Self {
        self.schema_version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssetActivityDeltaResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](AssetActivityDeltaResponseOutBuilder::asset_id)
    /// - [`asset_type`](AssetActivityDeltaResponseOutBuilder::asset_type)
    /// - [`deltas`](AssetActivityDeltaResponseOutBuilder::deltas)
    /// - [`schema_version`](AssetActivityDeltaResponseOutBuilder::schema_version)
    pub fn build(self) -> Result<AssetActivityDeltaResponseOut, BuildError> {
        Ok(AssetActivityDeltaResponseOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            asset_type: self.asset_type.ok_or_else(|| BuildError::missing_field("asset_type"))?,
            deltas: self.deltas.ok_or_else(|| BuildError::missing_field("deltas"))?,
            schema_version: self.schema_version.ok_or_else(|| BuildError::missing_field("schema_version"))?,
        })
    }
}
