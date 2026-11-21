pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationsListQueryRequest {
    /// Workspace to list; the caller must be a member of it
    #[serde(default)]
    pub workspace_id: String,
    /// true: only blueprints; false: only automations that are not blueprints; omit for both
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint: Option<bool>,
    /// Maximum number of automations per page (1-200)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of automations to skip for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl AutomationsListQueryRequest {
    pub fn builder() -> AutomationsListQueryRequestBuilder {
        <AutomationsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationsListQueryRequestBuilder {
    workspace_id: Option<String>,
    blueprint: Option<bool>,
    limit: Option<i64>,
    offset: Option<i64>,
}

impl AutomationsListQueryRequestBuilder {
    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    pub fn blueprint(mut self, value: bool) -> Self {
        self.blueprint = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workspace_id`](AutomationsListQueryRequestBuilder::workspace_id)
    pub fn build(self) -> Result<AutomationsListQueryRequest, BuildError> {
        Ok(AutomationsListQueryRequest {
            workspace_id: self.workspace_id.ok_or_else(|| BuildError::missing_field("workspace_id"))?,
            blueprint: self.blueprint,
            limit: self.limit,
            offset: self.offset,
        })
    }
}

