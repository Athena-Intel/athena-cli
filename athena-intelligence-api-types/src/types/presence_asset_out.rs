pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PresenceAssetOut {
    /// Athena asset type, e.g. document, spreadsheet, project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<String>,
    /// Asset guid.
    #[serde(default)]
    pub id: String,
    /// Display title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl PresenceAssetOut {
    pub fn builder() -> PresenceAssetOutBuilder {
        <PresenceAssetOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PresenceAssetOutBuilder {
    asset_type: Option<String>,
    id: Option<String>,
    title: Option<String>,
}

impl PresenceAssetOutBuilder {
    pub fn asset_type(mut self, value: impl Into<String>) -> Self {
        self.asset_type = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PresenceAssetOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PresenceAssetOutBuilder::id)
    pub fn build(self) -> Result<PresenceAssetOut, BuildError> {
        Ok(PresenceAssetOut {
            asset_type: self.asset_type,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self.title,
        })
    }
}
