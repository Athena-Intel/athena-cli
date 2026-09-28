pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Result of a cancel.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationRunCancelResponseOut {
    /// True when the run had already ended and nothing changed
    #[serde(default)]
    pub already_terminal: bool,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub run_id: String,
    /// The run's status after the call; 'canceled' when this call did it
    #[serde(default)]
    pub run_status: String,
}

impl AutomationRunCancelResponseOut {
    pub fn builder() -> AutomationRunCancelResponseOutBuilder {
        <AutomationRunCancelResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationRunCancelResponseOutBuilder {
    already_terminal: Option<bool>,
    message: Option<String>,
    run_id: Option<String>,
    run_status: Option<String>,
}

impl AutomationRunCancelResponseOutBuilder {
    pub fn already_terminal(mut self, value: bool) -> Self {
        self.already_terminal = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn run_status(mut self, value: impl Into<String>) -> Self {
        self.run_status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AutomationRunCancelResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`already_terminal`](AutomationRunCancelResponseOutBuilder::already_terminal)
    /// - [`message`](AutomationRunCancelResponseOutBuilder::message)
    /// - [`run_id`](AutomationRunCancelResponseOutBuilder::run_id)
    /// - [`run_status`](AutomationRunCancelResponseOutBuilder::run_status)
    pub fn build(self) -> Result<AutomationRunCancelResponseOut, BuildError> {
        Ok(AutomationRunCancelResponseOut {
            already_terminal: self.already_terminal.ok_or_else(|| BuildError::missing_field("already_terminal"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            run_status: self.run_status.ok_or_else(|| BuildError::missing_field("run_status"))?,
        })
    }
}
