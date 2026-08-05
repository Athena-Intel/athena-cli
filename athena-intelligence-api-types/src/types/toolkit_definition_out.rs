pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A registry toolkit as the public API describes it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ToolkitDefinitionOut {
    /// Alternative identifiers that resolve to this toolkit.
    #[serde(default)]
    pub aliases: Vec<String>,
    /// Coarse domain grouping, when assigned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// What the toolkit is for.
    #[serde(default)]
    pub description: String,
    /// Human-readable toolkit name.
    #[serde(default)]
    pub display_name: String,
    /// Stable toolkit identifier.
    #[serde(default)]
    pub name: String,
    /// Tool ids belonging to this toolkit.
    #[serde(default)]
    pub tools: Vec<String>,
}

impl ToolkitDefinitionOut {
    pub fn builder() -> ToolkitDefinitionOutBuilder {
        <ToolkitDefinitionOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolkitDefinitionOutBuilder {
    aliases: Option<Vec<String>>,
    category: Option<String>,
    description: Option<String>,
    display_name: Option<String>,
    name: Option<String>,
    tools: Option<Vec<String>>,
}

impl ToolkitDefinitionOutBuilder {
    pub fn aliases(mut self, value: Vec<String>) -> Self {
        self.aliases = Some(value);
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn tools(mut self, value: Vec<String>) -> Self {
        self.tools = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolkitDefinitionOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`aliases`](ToolkitDefinitionOutBuilder::aliases)
    /// - [`description`](ToolkitDefinitionOutBuilder::description)
    /// - [`display_name`](ToolkitDefinitionOutBuilder::display_name)
    /// - [`name`](ToolkitDefinitionOutBuilder::name)
    /// - [`tools`](ToolkitDefinitionOutBuilder::tools)
    pub fn build(self) -> Result<ToolkitDefinitionOut, BuildError> {
        Ok(ToolkitDefinitionOut {
            aliases: self.aliases.ok_or_else(|| BuildError::missing_field("aliases"))?,
            category: self.category,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            display_name: self.display_name.ok_or_else(|| BuildError::missing_field("display_name"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            tools: self.tools.ok_or_else(|| BuildError::missing_field("tools"))?,
        })
    }
}
