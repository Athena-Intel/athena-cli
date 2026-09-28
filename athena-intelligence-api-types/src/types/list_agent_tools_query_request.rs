pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_agent_tools
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAgentToolsQueryRequest {
    /// Whose broadcast: none (default), default, a managed agent id, collab_agent:<asset_id>, or toolkits:a,b to simulate an agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    /// Opaque page cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Tools per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// read, write, share or destructive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// Toolkit id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkit: Option<String>,
    /// Free-text filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Include tools tied to one asset type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_asset_scoped: Option<bool>,
}

impl ListAgentToolsQueryRequest {
    pub fn builder() -> ListAgentToolsQueryRequestBuilder {
        <ListAgentToolsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAgentToolsQueryRequestBuilder {
    agent: Option<String>,
    cursor: Option<String>,
    page_size: Option<i64>,
    effect: Option<String>,
    toolkit: Option<String>,
    query: Option<String>,
    include_asset_scoped: Option<bool>,
}

impl ListAgentToolsQueryRequestBuilder {
    pub fn agent(mut self, value: impl Into<String>) -> Self {
        self.agent = Some(value.into());
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn effect(mut self, value: impl Into<String>) -> Self {
        self.effect = Some(value.into());
        self
    }

    pub fn toolkit(mut self, value: impl Into<String>) -> Self {
        self.toolkit = Some(value.into());
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn include_asset_scoped(mut self, value: bool) -> Self {
        self.include_asset_scoped = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAgentToolsQueryRequest`].
    pub fn build(self) -> Result<ListAgentToolsQueryRequest, BuildError> {
        Ok(ListAgentToolsQueryRequest {
            agent: self.agent,
            cursor: self.cursor,
            page_size: self.page_size,
            effect: self.effect,
            toolkit: self.toolkit,
            query: self.query,
            include_asset_scoped: self.include_asset_scoped,
        })
    }
}

