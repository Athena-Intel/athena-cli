pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response for listing toolkits.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListToolkitsResponseOut {
    #[serde(default)]
    pub toolkits: Vec<ToolkitDefinitionOut>,
    #[serde(default)]
    pub total: i64,
}

impl ListToolkitsResponseOut {
    pub fn builder() -> ListToolkitsResponseOutBuilder {
        <ListToolkitsResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListToolkitsResponseOutBuilder {
    toolkits: Option<Vec<ToolkitDefinitionOut>>,
    total: Option<i64>,
}

impl ListToolkitsResponseOutBuilder {
    pub fn toolkits(mut self, value: Vec<ToolkitDefinitionOut>) -> Self {
        self.toolkits = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListToolkitsResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`toolkits`](ListToolkitsResponseOutBuilder::toolkits)
    /// - [`total`](ListToolkitsResponseOutBuilder::total)
    pub fn build(self) -> Result<ListToolkitsResponseOut, BuildError> {
        Ok(ListToolkitsResponseOut {
            toolkits: self.toolkits.ok_or_else(|| BuildError::missing_field("toolkits"))?,
            total: self.total.ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
