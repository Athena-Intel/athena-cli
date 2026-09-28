pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One half of the environment's generated reconciler pair.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeneratedAutomationOut {
    /// Null when the automation is not shared with the caller
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_asset_id: Option<String>,
    /// detector or healer
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub visible: bool,
}

impl GeneratedAutomationOut {
    pub fn builder() -> GeneratedAutomationOutBuilder {
        <GeneratedAutomationOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeneratedAutomationOutBuilder {
    automation_asset_id: Option<String>,
    role: Option<String>,
    visible: Option<bool>,
}

impl GeneratedAutomationOutBuilder {
    pub fn automation_asset_id(mut self, value: impl Into<String>) -> Self {
        self.automation_asset_id = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn visible(mut self, value: bool) -> Self {
        self.visible = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeneratedAutomationOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`role`](GeneratedAutomationOutBuilder::role)
    /// - [`visible`](GeneratedAutomationOutBuilder::visible)
    pub fn build(self) -> Result<GeneratedAutomationOut, BuildError> {
        Ok(GeneratedAutomationOut {
            automation_asset_id: self.automation_asset_id,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            visible: self.visible.ok_or_else(|| BuildError::missing_field("visible"))?,
        })
    }
}
