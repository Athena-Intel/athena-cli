pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An approval of any subject kind, with its decisions and deliveries.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ApprovalOut {
    #[serde(default)]
    pub allow_self_approval: bool,
    #[serde(default)]
    pub approver_refs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approver_selectors: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_asset_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<String>,
    #[serde(default)]
    pub channels: Vec<String>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub decided_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub decisions: Vec<ApprovalDecisionOut>,
    #[serde(default)]
    pub deliveries: Vec<ApprovalDeliveryOut>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable_schema: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub escalated_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalation: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_fingerprint: Option<String>,
    #[serde(default)]
    pub invalidate_on_input_change: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub invalidated_at: Option<DateTime<FixedOffset>>,
    /// any, all or n_of_m
    #[serde(default)]
    pub mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode_n: Option<i64>,
    #[serde(default)]
    pub options: Vec<ApprovalOptionOut>,
    /// approved, rejected, custom:<option id>, skipped or proceeded once settled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    /// What is being approved, inline; null when stored by reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_ref: Option<String>,
    #[serde(default)]
    pub reminded_count: i64,
    /// `{title, summary}`
    #[serde(default)]
    pub request: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub requester_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    #[serde(default)]
    pub self_refs: Vec<String>,
    #[serde(default)]
    pub separation_of_duties: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standing_grant: Option<HashMap<String, serde_json::Value>>,
    /// pending, escalated (active) or decided, expired, invalidated
    #[serde(default)]
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_id: Option<String>,
    #[serde(default)]
    pub subject: HashMap<String, serde_json::Value>,
    /// automation_step, session_interrupt, publish or proposal
    #[serde(default)]
    pub subject_kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub timeout_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_level: Option<String>,
}

impl ApprovalOut {
    pub fn builder() -> ApprovalOutBuilder {
        <ApprovalOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApprovalOutBuilder {
    allow_self_approval: Option<bool>,
    approver_refs: Option<Vec<String>>,
    approver_selectors: Option<Vec<serde_json::Value>>,
    automation_asset_id: Option<String>,
    batch_id: Option<String>,
    channels: Option<Vec<String>>,
    created_at: Option<DateTime<FixedOffset>>,
    decided_at: Option<DateTime<FixedOffset>>,
    decisions: Option<Vec<ApprovalDecisionOut>>,
    deliveries: Option<Vec<ApprovalDeliveryOut>>,
    editable_schema: Option<HashMap<String, serde_json::Value>>,
    escalated_at: Option<DateTime<FixedOffset>>,
    escalation: Option<HashMap<String, serde_json::Value>>,
    id: Option<String>,
    input_fingerprint: Option<String>,
    invalidate_on_input_change: Option<bool>,
    invalidated_at: Option<DateTime<FixedOffset>>,
    mode: Option<String>,
    mode_n: Option<i64>,
    options: Option<Vec<ApprovalOptionOut>>,
    outcome: Option<String>,
    payload: Option<HashMap<String, serde_json::Value>>,
    payload_ref: Option<String>,
    reminded_count: Option<i64>,
    request: Option<HashMap<String, serde_json::Value>>,
    requester_ref: Option<String>,
    run_id: Option<String>,
    scope_id: Option<String>,
    self_refs: Option<Vec<String>>,
    separation_of_duties: Option<String>,
    standing_grant: Option<HashMap<String, serde_json::Value>>,
    state: Option<String>,
    step_id: Option<String>,
    subject: Option<HashMap<String, serde_json::Value>>,
    subject_kind: Option<String>,
    thread_id: Option<String>,
    timeout_at: Option<DateTime<FixedOffset>>,
    tool_call_id: Option<String>,
    trust_level: Option<String>,
}

impl ApprovalOutBuilder {
    pub fn allow_self_approval(mut self, value: bool) -> Self {
        self.allow_self_approval = Some(value);
        self
    }

    pub fn approver_refs(mut self, value: Vec<String>) -> Self {
        self.approver_refs = Some(value);
        self
    }

    pub fn approver_selectors(mut self, value: Vec<serde_json::Value>) -> Self {
        self.approver_selectors = Some(value);
        self
    }

    pub fn automation_asset_id(mut self, value: impl Into<String>) -> Self {
        self.automation_asset_id = Some(value.into());
        self
    }

    pub fn batch_id(mut self, value: impl Into<String>) -> Self {
        self.batch_id = Some(value.into());
        self
    }

    pub fn channels(mut self, value: Vec<String>) -> Self {
        self.channels = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn decided_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.decided_at = Some(value);
        self
    }

    pub fn decisions(mut self, value: Vec<ApprovalDecisionOut>) -> Self {
        self.decisions = Some(value);
        self
    }

    pub fn deliveries(mut self, value: Vec<ApprovalDeliveryOut>) -> Self {
        self.deliveries = Some(value);
        self
    }

    pub fn editable_schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.editable_schema = Some(value);
        self
    }

    pub fn escalated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.escalated_at = Some(value);
        self
    }

    pub fn escalation(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.escalation = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn input_fingerprint(mut self, value: impl Into<String>) -> Self {
        self.input_fingerprint = Some(value.into());
        self
    }

    pub fn invalidate_on_input_change(mut self, value: bool) -> Self {
        self.invalidate_on_input_change = Some(value);
        self
    }

    pub fn invalidated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.invalidated_at = Some(value);
        self
    }

    pub fn mode(mut self, value: impl Into<String>) -> Self {
        self.mode = Some(value.into());
        self
    }

    pub fn mode_n(mut self, value: i64) -> Self {
        self.mode_n = Some(value);
        self
    }

    pub fn options(mut self, value: Vec<ApprovalOptionOut>) -> Self {
        self.options = Some(value);
        self
    }

    pub fn outcome(mut self, value: impl Into<String>) -> Self {
        self.outcome = Some(value.into());
        self
    }

    pub fn payload(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.payload = Some(value);
        self
    }

    pub fn payload_ref(mut self, value: impl Into<String>) -> Self {
        self.payload_ref = Some(value.into());
        self
    }

    pub fn reminded_count(mut self, value: i64) -> Self {
        self.reminded_count = Some(value);
        self
    }

    pub fn request(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.request = Some(value);
        self
    }

    pub fn requester_ref(mut self, value: impl Into<String>) -> Self {
        self.requester_ref = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn scope_id(mut self, value: impl Into<String>) -> Self {
        self.scope_id = Some(value.into());
        self
    }

    pub fn self_refs(mut self, value: Vec<String>) -> Self {
        self.self_refs = Some(value);
        self
    }

    pub fn separation_of_duties(mut self, value: impl Into<String>) -> Self {
        self.separation_of_duties = Some(value.into());
        self
    }

    pub fn standing_grant(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.standing_grant = Some(value);
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn step_id(mut self, value: impl Into<String>) -> Self {
        self.step_id = Some(value.into());
        self
    }

    pub fn subject(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.subject = Some(value);
        self
    }

    pub fn subject_kind(mut self, value: impl Into<String>) -> Self {
        self.subject_kind = Some(value.into());
        self
    }

    pub fn thread_id(mut self, value: impl Into<String>) -> Self {
        self.thread_id = Some(value.into());
        self
    }

    pub fn timeout_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timeout_at = Some(value);
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    pub fn trust_level(mut self, value: impl Into<String>) -> Self {
        self.trust_level = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApprovalOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`allow_self_approval`](ApprovalOutBuilder::allow_self_approval)
    /// - [`approver_refs`](ApprovalOutBuilder::approver_refs)
    /// - [`channels`](ApprovalOutBuilder::channels)
    /// - [`created_at`](ApprovalOutBuilder::created_at)
    /// - [`decisions`](ApprovalOutBuilder::decisions)
    /// - [`deliveries`](ApprovalOutBuilder::deliveries)
    /// - [`id`](ApprovalOutBuilder::id)
    /// - [`invalidate_on_input_change`](ApprovalOutBuilder::invalidate_on_input_change)
    /// - [`mode`](ApprovalOutBuilder::mode)
    /// - [`options`](ApprovalOutBuilder::options)
    /// - [`reminded_count`](ApprovalOutBuilder::reminded_count)
    /// - [`request`](ApprovalOutBuilder::request)
    /// - [`requester_ref`](ApprovalOutBuilder::requester_ref)
    /// - [`self_refs`](ApprovalOutBuilder::self_refs)
    /// - [`separation_of_duties`](ApprovalOutBuilder::separation_of_duties)
    /// - [`state`](ApprovalOutBuilder::state)
    /// - [`subject`](ApprovalOutBuilder::subject)
    /// - [`subject_kind`](ApprovalOutBuilder::subject_kind)
    pub fn build(self) -> Result<ApprovalOut, BuildError> {
        Ok(ApprovalOut {
            allow_self_approval: self.allow_self_approval.ok_or_else(|| BuildError::missing_field("allow_self_approval"))?,
            approver_refs: self.approver_refs.ok_or_else(|| BuildError::missing_field("approver_refs"))?,
            approver_selectors: self.approver_selectors,
            automation_asset_id: self.automation_asset_id,
            batch_id: self.batch_id,
            channels: self.channels.ok_or_else(|| BuildError::missing_field("channels"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            decided_at: self.decided_at,
            decisions: self.decisions.ok_or_else(|| BuildError::missing_field("decisions"))?,
            deliveries: self.deliveries.ok_or_else(|| BuildError::missing_field("deliveries"))?,
            editable_schema: self.editable_schema,
            escalated_at: self.escalated_at,
            escalation: self.escalation,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            input_fingerprint: self.input_fingerprint,
            invalidate_on_input_change: self.invalidate_on_input_change.ok_or_else(|| BuildError::missing_field("invalidate_on_input_change"))?,
            invalidated_at: self.invalidated_at,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
            mode_n: self.mode_n,
            options: self.options.ok_or_else(|| BuildError::missing_field("options"))?,
            outcome: self.outcome,
            payload: self.payload,
            payload_ref: self.payload_ref,
            reminded_count: self.reminded_count.ok_or_else(|| BuildError::missing_field("reminded_count"))?,
            request: self.request.ok_or_else(|| BuildError::missing_field("request"))?,
            requester_ref: self.requester_ref.ok_or_else(|| BuildError::missing_field("requester_ref"))?,
            run_id: self.run_id,
            scope_id: self.scope_id,
            self_refs: self.self_refs.ok_or_else(|| BuildError::missing_field("self_refs"))?,
            separation_of_duties: self.separation_of_duties.ok_or_else(|| BuildError::missing_field("separation_of_duties"))?,
            standing_grant: self.standing_grant,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            step_id: self.step_id,
            subject: self.subject.ok_or_else(|| BuildError::missing_field("subject"))?,
            subject_kind: self.subject_kind.ok_or_else(|| BuildError::missing_field("subject_kind"))?,
            thread_id: self.thread_id,
            timeout_at: self.timeout_at,
            tool_call_id: self.tool_call_id,
            trust_level: self.trust_level,
        })
    }
}
