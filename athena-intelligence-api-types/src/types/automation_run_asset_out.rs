pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One asset a run's sessions created, as the caller may view it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationRunAssetOut {
    #[serde(default)]
    pub asset_id: String,
    /// The asset's type (document, spreadsheet, presentation, ...)
    #[serde(default)]
    pub asset_type: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// The steps (step_id) whose session created the asset
    #[serde(default)]
    pub step_ids: Vec<String>,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl AutomationRunAssetOut {
    pub fn builder() -> AutomationRunAssetOutBuilder {
        <AutomationRunAssetOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationRunAssetOutBuilder {
    asset_id: Option<String>,
    asset_type: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    parent_folder_id: Option<String>,
    step_ids: Option<Vec<String>>,
    title: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    workspace_id: Option<String>,
}

impl AutomationRunAssetOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn asset_type(mut self, value: impl Into<String>) -> Self {
        self.asset_type = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
        self
    }

    pub fn step_ids(mut self, value: Vec<String>) -> Self {
        self.step_ids = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AutomationRunAssetOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](AutomationRunAssetOutBuilder::asset_id)
    /// - [`asset_type`](AutomationRunAssetOutBuilder::asset_type)
    /// - [`created_at`](AutomationRunAssetOutBuilder::created_at)
    /// - [`step_ids`](AutomationRunAssetOutBuilder::step_ids)
    /// - [`title`](AutomationRunAssetOutBuilder::title)
    /// - [`updated_at`](AutomationRunAssetOutBuilder::updated_at)
    pub fn build(self) -> Result<AutomationRunAssetOut, BuildError> {
        Ok(AutomationRunAssetOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            asset_type: self.asset_type.ok_or_else(|| BuildError::missing_field("asset_type"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            parent_folder_id: self.parent_folder_id,
            step_ids: self.step_ids.ok_or_else(|| BuildError::missing_field("step_ids"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            workspace_id: self.workspace_id,
        })
    }
}
