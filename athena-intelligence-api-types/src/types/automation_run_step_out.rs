pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One step attempt of a run.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AutomationRunStepOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[serde(default)]
    pub attempt: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cost_usd: Option<f64>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub ended_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_thread_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub started_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub step_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<i64>,
}

impl AutomationRunStepOut {
    pub fn builder() -> AutomationRunStepOutBuilder {
        <AutomationRunStepOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationRunStepOutBuilder {
    approval_id: Option<String>,
    attempt: Option<i64>,
    cost_usd: Option<f64>,
    created_at: Option<DateTime<FixedOffset>>,
    ended_at: Option<DateTime<FixedOffset>>,
    error: Option<HashMap<String, serde_json::Value>>,
    input: Option<HashMap<String, serde_json::Value>>,
    kind: Option<String>,
    output: Option<HashMap<String, serde_json::Value>>,
    output_ref: Option<String>,
    session_thread_id: Option<String>,
    started_at: Option<DateTime<FixedOffset>>,
    status: Option<String>,
    step_id: Option<String>,
    tokens: Option<i64>,
}

impl AutomationRunStepOutBuilder {
    pub fn approval_id(mut self, value: impl Into<String>) -> Self {
        self.approval_id = Some(value.into());
        self
    }

    pub fn attempt(mut self, value: i64) -> Self {
        self.attempt = Some(value);
        self
    }

    pub fn cost_usd(mut self, value: f64) -> Self {
        self.cost_usd = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn ended_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.ended_at = Some(value);
        self
    }

    pub fn error(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.error = Some(value);
        self
    }

    pub fn input(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.input = Some(value);
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn output(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.output = Some(value);
        self
    }

    pub fn output_ref(mut self, value: impl Into<String>) -> Self {
        self.output_ref = Some(value.into());
        self
    }

    pub fn session_thread_id(mut self, value: impl Into<String>) -> Self {
        self.session_thread_id = Some(value.into());
        self
    }

    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn step_id(mut self, value: impl Into<String>) -> Self {
        self.step_id = Some(value.into());
        self
    }

    pub fn tokens(mut self, value: i64) -> Self {
        self.tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationRunStepOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attempt`](AutomationRunStepOutBuilder::attempt)
    /// - [`created_at`](AutomationRunStepOutBuilder::created_at)
    /// - [`kind`](AutomationRunStepOutBuilder::kind)
    /// - [`status`](AutomationRunStepOutBuilder::status)
    /// - [`step_id`](AutomationRunStepOutBuilder::step_id)
    pub fn build(self) -> Result<AutomationRunStepOut, BuildError> {
        Ok(AutomationRunStepOut {
            approval_id: self.approval_id,
            attempt: self.attempt.ok_or_else(|| BuildError::missing_field("attempt"))?,
            cost_usd: self.cost_usd,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            ended_at: self.ended_at,
            error: self.error,
            input: self.input,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            output: self.output,
            output_ref: self.output_ref,
            session_thread_id: self.session_thread_id,
            started_at: self.started_at,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            step_id: self.step_id.ok_or_else(|| BuildError::missing_field("step_id"))?,
            tokens: self.tokens,
        })
    }
}
