pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An automation: asset facts, mirror, draft, versions and trigger rows.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AutomationResponseOut {
    #[serde(default)]
    pub asset_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<AutomationCurrentVersionOut>,
    /// Version number of the current publish; null before the first
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version_number: Option<i64>,
    #[serde(default)]
    pub draft: AutomationDraftOut,
    /// Whether the published triggers may fire
    #[serde(default)]
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_project_asset_id: Option<String>,
    #[serde(default)]
    pub is_archived: bool,
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
    #[serde(default)]
    pub title: String,
    /// One line per materialised trigger, for cards and search
    #[serde(default)]
    pub trigger_summary: Vec<String>,
    #[serde(default)]
    pub triggers: Vec<AutomationTriggerRowOut>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// Version history, newest first (at most the latest 20)
    #[serde(default)]
    pub versions: Vec<AutomationVersionOut>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl AutomationResponseOut {
    pub fn builder() -> AutomationResponseOutBuilder {
        <AutomationResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationResponseOutBuilder {
    asset_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    created_by: Option<String>,
    current_version: Option<AutomationCurrentVersionOut>,
    current_version_number: Option<i64>,
    draft: Option<AutomationDraftOut>,
    enabled: Option<bool>,
    fingerprint: Option<String>,
    home_project_asset_id: Option<String>,
    is_archived: Option<bool>,
    next_fire_at: Option<DateTime<FixedOffset>>,
    parent_folder_id: Option<String>,
    principal_ref: Option<String>,
    title: Option<String>,
    trigger_summary: Option<Vec<String>>,
    triggers: Option<Vec<AutomationTriggerRowOut>>,
    updated_at: Option<DateTime<FixedOffset>>,
    versions: Option<Vec<AutomationVersionOut>>,
    workspace_id: Option<String>,
}

impl AutomationResponseOutBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
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

    pub fn current_version(mut self, value: AutomationCurrentVersionOut) -> Self {
        self.current_version = Some(value);
        self
    }

    pub fn current_version_number(mut self, value: i64) -> Self {
        self.current_version_number = Some(value);
        self
    }

    pub fn draft(mut self, value: AutomationDraftOut) -> Self {
        self.draft = Some(value);
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

    pub fn is_archived(mut self, value: bool) -> Self {
        self.is_archived = Some(value);
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

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn trigger_summary(mut self, value: Vec<String>) -> Self {
        self.trigger_summary = Some(value);
        self
    }

    pub fn triggers(mut self, value: Vec<AutomationTriggerRowOut>) -> Self {
        self.triggers = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn versions(mut self, value: Vec<AutomationVersionOut>) -> Self {
        self.versions = Some(value);
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AutomationResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](AutomationResponseOutBuilder::asset_id)
    /// - [`created_at`](AutomationResponseOutBuilder::created_at)
    /// - [`draft`](AutomationResponseOutBuilder::draft)
    /// - [`enabled`](AutomationResponseOutBuilder::enabled)
    /// - [`is_archived`](AutomationResponseOutBuilder::is_archived)
    /// - [`principal_ref`](AutomationResponseOutBuilder::principal_ref)
    /// - [`title`](AutomationResponseOutBuilder::title)
    /// - [`trigger_summary`](AutomationResponseOutBuilder::trigger_summary)
    /// - [`triggers`](AutomationResponseOutBuilder::triggers)
    /// - [`updated_at`](AutomationResponseOutBuilder::updated_at)
    /// - [`versions`](AutomationResponseOutBuilder::versions)
    pub fn build(self) -> Result<AutomationResponseOut, BuildError> {
        Ok(AutomationResponseOut {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            created_by: self.created_by,
            current_version: self.current_version,
            current_version_number: self.current_version_number,
            draft: self.draft.ok_or_else(|| BuildError::missing_field("draft"))?,
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
            fingerprint: self.fingerprint,
            home_project_asset_id: self.home_project_asset_id,
            is_archived: self.is_archived.ok_or_else(|| BuildError::missing_field("is_archived"))?,
            next_fire_at: self.next_fire_at,
            parent_folder_id: self.parent_folder_id,
            principal_ref: self.principal_ref.ok_or_else(|| BuildError::missing_field("principal_ref"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            trigger_summary: self.trigger_summary.ok_or_else(|| BuildError::missing_field("trigger_summary"))?,
            triggers: self.triggers.ok_or_else(|| BuildError::missing_field("triggers"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            versions: self.versions.ok_or_else(|| BuildError::missing_field("versions"))?,
            workspace_id: self.workspace_id,
        })
    }
}
