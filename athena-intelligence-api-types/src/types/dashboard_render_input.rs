pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DashboardRenderInput {
    /// Dashboard asset id (asset_…). Every figure tile the caller can view is rendered; query panels are counted, not rendered.
    #[serde(default)]
    pub asset_id: String,
}

impl DashboardRenderInput {
    pub fn builder() -> DashboardRenderInputBuilder {
        <DashboardRenderInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DashboardRenderInputBuilder {
    asset_id: Option<String>,
}

impl DashboardRenderInputBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DashboardRenderInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](DashboardRenderInputBuilder::asset_id)
    pub fn build(self) -> Result<DashboardRenderInput, BuildError> {
        Ok(DashboardRenderInput {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
        })
    }
}

