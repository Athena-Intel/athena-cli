pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One automation of a list page: what a card or a picker shows.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AutomationListItemOut {
    #[serde(default)]
    pub asset_id: String,
    /// Whether the current version is a blueprint: a template that never fires on its own and that new automations are made from
    #[serde(default)]
    pub blueprint: bool,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// Version number of the current publish; null before the first
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version_number: Option<i64>,
    /// The current version's description; null before the first publish
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the published triggers may fire
    #[serde(default)]
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_project_asset_id: Option<String>,
    /// The current version's `inputs` JSON Schema: what a run may be given, and what an automation made from this blueprint may preset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<HashMap<String, serde_json::Value>>,
    /// Earliest scheduled fire across the live schedule rows
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub next_fire_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// The automation's own principal; every step runs as it
    #[serde(default)]
    pub principal_ref: String,
    /// When the current version was published
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub published_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub title: String,
    /// The distinct trigger kinds of the current version, sorted
    #[serde(default)]
    pub trigger_kinds: Vec<String>,
    /// One line per trigger of the current version
    #[serde(default)]
    pub trigger_summary: Vec<String>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl AutomationListItemOut {
    pub fn builder() -> AutomationListItemOutBuilder {
        <AutomationListItemOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationListItemOutBuilder {
    asset_id: Option<String>,
    blueprint: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
    created_by: Option<String>,
    current_version_number: Option<i64>,
    description: Option<String>,
    enabled: Option<bool>,
    fingerprint: Option<String>,
    home_project_asset_id: Option<String>,
    inputs: Option<HashMap<String, serde_json::Value>>,
    next_fire_at: Option<DateTime<FixedOffset>>,
    parent_folder_id: Option<String>,
    principal_ref: Option<String>,
    published_at: Option<DateTime<FixedOffset>>,
    title: Option<String>,
    trigger_kinds: Option<Vec<String>>,
    trigger_summary: Option<Vec<String>>,
    updated_at: Option<DateTime<FixedOffset>>,
    workspace_id: Option<String>,
}

impl AutomationListItemOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn blueprint(mut self, value: bool) -> Self {
        self.blueprint = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    pub fn current_version_number(mut self, value: i64) -> Self {
        self.current_version_number = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.fingerprint = Some(value.into());
        self
    }

    pub fn home_project_asset_id(mut self, value: impl Into<String>) -> Self {
        self.home_project_asset_id = Some(value.into());
        self
    }

    pub fn inputs(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.inputs = Some(value);
        self
    }

    pub fn next_fire_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.next_fire_at = Some(value);
        self
    }

    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
        self
    }

    pub fn principal_ref(mut self, value: impl Into<String>) -> Self {
        self.principal_ref = Some(value.into());
        self
    }

    pub fn published_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.published_at = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn trigger_kinds(mut self, value: Vec<String>) -> Self {
        self.trigger_kinds = Some(value);
        self
    }

    pub fn trigger_summary(mut self, value: Vec<String>) -> Self {
        self.trigger_summary = Some(value);
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

    /// Consumes the builder and constructs a [`AutomationListItemOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](AutomationListItemOutBuilder::asset_id)
    /// - [`blueprint`](AutomationListItemOutBuilder::blueprint)
    /// - [`created_at`](AutomationListItemOutBuilder::created_at)
    /// - [`enabled`](AutomationListItemOutBuilder::enabled)
    /// - [`principal_ref`](AutomationListItemOutBuilder::principal_ref)
    /// - [`title`](AutomationListItemOutBuilder::title)
    /// - [`trigger_kinds`](AutomationListItemOutBuilder::trigger_kinds)
    /// - [`trigger_summary`](AutomationListItemOutBuilder::trigger_summary)
    /// - [`updated_at`](AutomationListItemOutBuilder::updated_at)
    pub fn build(self) -> Result<AutomationListItemOut, BuildError> {
        Ok(AutomationListItemOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            blueprint: self.blueprint.ok_or_else(|| BuildError::missing_field("blueprint"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            created_by: self.created_by,
            current_version_number: self.current_version_number,
            description: self.description,
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
            fingerprint: self.fingerprint,
            home_project_asset_id: self.home_project_asset_id,
            inputs: self.inputs,
            next_fire_at: self.next_fire_at,
            parent_folder_id: self.parent_folder_id,
            principal_ref: self.principal_ref.ok_or_else(|| BuildError::missing_field("principal_ref"))?,
            published_at: self.published_at,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            trigger_kinds: self.trigger_kinds.ok_or_else(|| BuildError::missing_field("trigger_kinds"))?,
            trigger_summary: self.trigger_summary.ok_or_else(|| BuildError::missing_field("trigger_summary"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            workspace_id: self.workspace_id,
        })
    }
}
