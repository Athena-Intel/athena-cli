pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response for listing tools.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListToolsResponseOut {
    #[serde(default)]
    pub tools: Vec<ToolDefinitionOut>,
    #[serde(default)]
    pub total: i64,
}

impl ListToolsResponseOut {
    pub fn builder() -> ListToolsResponseOutBuilder {
        <ListToolsResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListToolsResponseOutBuilder {
    tools: Option<Vec<ToolDefinitionOut>>,
    total: Option<i64>,
}

impl ListToolsResponseOutBuilder {
    pub fn tools(mut self, value: Vec<ToolDefinitionOut>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListToolsResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tools`](ListToolsResponseOutBuilder::tools)
    /// - [`total`](ListToolsResponseOutBuilder::total)
    pub fn build(self) -> Result<ListToolsResponseOut, BuildError> {
        Ok(ListToolsResponseOut {
            tools: self.tools.ok_or_else(|| BuildError::missing_field("tools"))?,
            total: self.total.ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
