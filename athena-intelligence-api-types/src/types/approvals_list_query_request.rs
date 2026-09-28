pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApprovalsListQueryRequest {
    /// Only these subject kinds (repeat the parameter or separate with commas): automation_step, session_interrupt, publish, proposal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_kind: Option<Vec<String>>,
    /// Only approvals in this state: pending, escalated, decided, expired or invalidated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Page size (1 to 200)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The previous page's next_before; omit for the first page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<DateTime<FixedOffset>>,
    /// The previous page's next_before_id, sent together with before
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_id: Option<String>,
}

impl ApprovalsListQueryRequest {
    pub fn builder() -> ApprovalsListQueryRequestBuilder {
        <ApprovalsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApprovalsListQueryRequestBuilder {
    subject_kind: Option<Vec<String>>,
    state: Option<String>,
    limit: Option<i64>,
    before: Option<DateTime<FixedOffset>>,
    before_id: Option<String>,
}

impl ApprovalsListQueryRequestBuilder {
    pub fn subject_kind(mut self, value: Vec<String>) -> Self {
        self.subject_kind = Some(value);
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.before = Some(value);
        self
    }

    pub fn before_id(mut self, value: impl Into<String>) -> Self {
        self.before_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApprovalsListQueryRequest`].
    pub fn build(self) -> Result<ApprovalsListQueryRequest, BuildError> {
        Ok(ApprovalsListQueryRequest {
            subject_kind: self.subject_kind,
            state: self.state,
            limit: self.limit,
            before: self.before,
            before_id: self.before_id,
        })
    }
}

