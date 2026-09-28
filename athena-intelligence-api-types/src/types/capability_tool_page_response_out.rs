pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// R3: one page of an agent's tools.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CapabilityToolPageResponseOut {
    /// The agent reference.
    #[serde(default)]
    pub agent: String,
    /// The filters applied.
    #[serde(default)]
    pub filters: HashMap<String, Option<String>>,
    /// The tools on this page.
    #[serde(default)]
    pub items: Vec<HashMap<String, serde_json::Value>>,
    /// The next page's cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Tools matching the filters.
    #[serde(default)]
    pub total: i64,
}

impl CapabilityToolPageResponseOut {
    pub fn builder() -> CapabilityToolPageResponseOutBuilder {
        <CapabilityToolPageResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilityToolPageResponseOutBuilder {
    agent: Option<String>,
    filters: Option<HashMap<String, Option<String>>>,
    items: Option<Vec<HashMap<String, serde_json::Value>>>,
    next_cursor: Option<String>,
    total: Option<i64>,
}

impl CapabilityToolPageResponseOutBuilder {
    pub fn agent(mut self, value: impl Into<String>) -> Self {
        self.agent = Some(value.into());
        self
    }

    pub fn filters(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CapabilityToolPageResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent`](CapabilityToolPageResponseOutBuilder::agent)
    /// - [`filters`](CapabilityToolPageResponseOutBuilder::filters)
    /// - [`items`](CapabilityToolPageResponseOutBuilder::items)
    /// - [`total`](CapabilityToolPageResponseOutBuilder::total)
    pub fn build(self) -> Result<CapabilityToolPageResponseOut, BuildError> {
        Ok(CapabilityToolPageResponseOut {
            agent: self.agent.ok_or_else(|| BuildError::missing_field("agent"))?,
            filters: self.filters.ok_or_else(|| BuildError::missing_field("filters"))?,
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            next_cursor: self.next_cursor,
            total: self.total.ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
