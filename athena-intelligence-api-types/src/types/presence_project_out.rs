pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PresenceProjectOut {
    /// Documents linked into the project that the caller may open.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<PresenceAssetOut>>,
    /// Project guid.
    #[serde(default)]
    pub id: String,
    /// Display title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl PresenceProjectOut {
    pub fn builder() -> PresenceProjectOutBuilder {
        <PresenceProjectOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PresenceProjectOutBuilder {
    assets: Option<Vec<PresenceAssetOut>>,
    id: Option<String>,
    title: Option<String>,
}

impl PresenceProjectOutBuilder {
    pub fn assets(mut self, value: Vec<PresenceAssetOut>) -> Self {
        self.assets = Some(value);
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

    /// Consumes the builder and constructs a [`PresenceProjectOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PresenceProjectOutBuilder::id)
    pub fn build(self) -> Result<PresenceProjectOut, BuildError> {
        Ok(PresenceProjectOut {
            assets: self.assets,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self.title,
        })
    }
}
