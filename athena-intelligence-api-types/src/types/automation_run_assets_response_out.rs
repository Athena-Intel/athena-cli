pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The assets a run created that the caller may view, oldest first.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationRunAssetsResponseOut {
    #[serde(default)]
    pub automation_asset_id: String,
    #[serde(default)]
    pub items: Vec<AutomationRunAssetOut>,
    #[serde(default)]
    pub run_id: String,
    /// True when the run created more assets than one read returns and the newest were left out
    #[serde(default)]
    pub truncated: bool,
}

impl AutomationRunAssetsResponseOut {
    pub fn builder() -> AutomationRunAssetsResponseOutBuilder {
        <AutomationRunAssetsResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationRunAssetsResponseOutBuilder {
    automation_asset_id: Option<String>,
    items: Option<Vec<AutomationRunAssetOut>>,
    run_id: Option<String>,
    truncated: Option<bool>,
}

impl AutomationRunAssetsResponseOutBuilder {
    pub fn automation_asset_id(mut self, value: impl Into<String>) -> Self {
        self.automation_asset_id = Some(value.into());
        self
    }

    pub fn items(mut self, value: Vec<AutomationRunAssetOut>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn truncated(mut self, value: bool) -> Self {
        self.truncated = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationRunAssetsResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`automation_asset_id`](AutomationRunAssetsResponseOutBuilder::automation_asset_id)
    /// - [`items`](AutomationRunAssetsResponseOutBuilder::items)
    /// - [`run_id`](AutomationRunAssetsResponseOutBuilder::run_id)
    /// - [`truncated`](AutomationRunAssetsResponseOutBuilder::truncated)
    pub fn build(self) -> Result<AutomationRunAssetsResponseOut, BuildError> {
        Ok(AutomationRunAssetsResponseOut {
            automation_asset_id: self.automation_asset_id.ok_or_else(|| BuildError::missing_field("automation_asset_id"))?,
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            truncated: self.truncated.ok_or_else(|| BuildError::missing_field("truncated"))?,
        })
    }
}
