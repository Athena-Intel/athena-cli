pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Workspace disclaimer configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceDisclaimerOut {
    /// Text for the accept button
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_button_text: Option<String>,
    /// Whether the disclaimer is enabled
    #[serde(default)]
    pub enabled: bool,
    /// Markdown content of the disclaimer message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown_text: Option<String>,
    /// Title displayed at the top of the disclaimer modal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl WorkspaceDisclaimerOut {
    pub fn builder() -> WorkspaceDisclaimerOutBuilder {
        <WorkspaceDisclaimerOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceDisclaimerOutBuilder {
    accept_button_text: Option<String>,
    enabled: Option<bool>,
    markdown_text: Option<String>,
    title: Option<String>,
}

impl WorkspaceDisclaimerOutBuilder {
    pub fn accept_button_text(mut self, value: impl Into<String>) -> Self {
        self.accept_button_text = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn markdown_text(mut self, value: impl Into<String>) -> Self {
        self.markdown_text = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceDisclaimerOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enabled`](WorkspaceDisclaimerOutBuilder::enabled)
    pub fn build(self) -> Result<WorkspaceDisclaimerOut, BuildError> {
        Ok(WorkspaceDisclaimerOut {
            accept_button_text: self.accept_button_text,
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
            markdown_text: self.markdown_text,
            title: self.title,
        })
    }
}
