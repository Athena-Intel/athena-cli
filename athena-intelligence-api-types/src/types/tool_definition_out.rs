pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A registry tool as the public API describes it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ToolDefinitionOut {
    /// Short description; this is the text the model sees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Where the tool runs: backend, frontend, or frontend with a backend fallback.
    #[serde(default)]
    pub execution_environment: String,
    /// Stable tool identifier.
    #[serde(default)]
    pub id: String,
    /// JSON Schema for the tool's arguments, when one is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<HashMap<String, serde_json::Value>>,
    /// Whether this tool can currently be invoked over HTTP by the calling user. False means a call would be refused; see refusal_reason.
    #[serde(default)]
    pub invocable: bool,
    /// Extended prose for documentation and CLI help.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_description: Option<String>,
    /// Human-readable tool name.
    #[serde(default)]
    pub name: String,
    /// Why the tool cannot be invoked over HTTP, when invocable is false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refusal_reason: Option<String>,
    /// Whether the tool requires human approval before it runs.
    #[serde(default)]
    pub requires_approval: bool,
    /// How the tool's side effects are undone on session rollback.
    #[serde(default)]
    pub reversal: String,
    /// Whether the tool has an outward effect that needs confirmation.
    #[serde(default)]
    pub sensitive: bool,
    /// Primary toolkit this tool belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkit_key: Option<String>,
}

impl ToolDefinitionOut {
    pub fn builder() -> ToolDefinitionOutBuilder {
        <ToolDefinitionOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolDefinitionOutBuilder {
    description: Option<String>,
    execution_environment: Option<String>,
    id: Option<String>,
    input_schema: Option<HashMap<String, serde_json::Value>>,
    invocable: Option<bool>,
    long_description: Option<String>,
    name: Option<String>,
    refusal_reason: Option<String>,
    requires_approval: Option<bool>,
    reversal: Option<String>,
    sensitive: Option<bool>,
    toolkit_key: Option<String>,
}

impl ToolDefinitionOutBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn execution_environment(mut self, value: impl Into<String>) -> Self {
        self.execution_environment = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn input_schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.input_schema = Some(value);
        self
    }

    pub fn invocable(mut self, value: bool) -> Self {
        self.invocable = Some(value);
        self
    }

    pub fn long_description(mut self, value: impl Into<String>) -> Self {
        self.long_description = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn refusal_reason(mut self, value: impl Into<String>) -> Self {
        self.refusal_reason = Some(value.into());
        self
    }

    pub fn requires_approval(mut self, value: bool) -> Self {
        self.requires_approval = Some(value);
        self
    }

    pub fn reversal(mut self, value: impl Into<String>) -> Self {
        self.reversal = Some(value.into());
        self
    }

    pub fn sensitive(mut self, value: bool) -> Self {
        self.sensitive = Some(value);
        self
    }

    pub fn toolkit_key(mut self, value: impl Into<String>) -> Self {
        self.toolkit_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ToolDefinitionOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`execution_environment`](ToolDefinitionOutBuilder::execution_environment)
    /// - [`id`](ToolDefinitionOutBuilder::id)
    /// - [`invocable`](ToolDefinitionOutBuilder::invocable)
    /// - [`name`](ToolDefinitionOutBuilder::name)
    /// - [`requires_approval`](ToolDefinitionOutBuilder::requires_approval)
    /// - [`reversal`](ToolDefinitionOutBuilder::reversal)
    /// - [`sensitive`](ToolDefinitionOutBuilder::sensitive)
    pub fn build(self) -> Result<ToolDefinitionOut, BuildError> {
        Ok(ToolDefinitionOut {
            description: self.description,
            execution_environment: self.execution_environment.ok_or_else(|| BuildError::missing_field("execution_environment"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            input_schema: self.input_schema,
            invocable: self.invocable.ok_or_else(|| BuildError::missing_field("invocable"))?,
            long_description: self.long_description,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            refusal_reason: self.refusal_reason,
            requires_approval: self.requires_approval.ok_or_else(|| BuildError::missing_field("requires_approval"))?,
            reversal: self.reversal.ok_or_else(|| BuildError::missing_field("reversal"))?,
            sensitive: self.sensitive.ok_or_else(|| BuildError::missing_field("sensitive"))?,
            toolkit_key: self.toolkit_key,
        })
    }
}
