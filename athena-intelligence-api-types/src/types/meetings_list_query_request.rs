pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MeetingsListQueryRequest {
    /// Keyword to search across meeting title, AI summary, and cached transcript text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Participant email(s) to filter by. Repeat the parameter or pass a comma-separated list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_emails: Option<Vec<String>>,
    /// Whether a meeting must include any or all of the given participant emails
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_match: Option<ListMeetingsRequestParticipantMatch>,
    /// Attendee email domain(s) to filter by (e.g. 'acme.com'). Repeat the parameter or pass a comma-separated list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_domains: Option<Vec<String>>,
    /// Whether a meeting must include attendees from any or all of the given domains
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_match: Option<ListMeetingsRequestDomainMatch>,
    /// Only include meetings created at or after this ISO 8601 timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<DateTime<FixedOffset>>,
    /// Only include meetings created at or before this ISO 8601 timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<DateTime<FixedOffset>>,
    /// Field to sort by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<ListMeetingsRequestSortBy>,
    /// Sort direction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<ListMeetingsRequestSortDirection>,
    /// Maximum number of meetings to return per page (1-500)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of meetings to skip for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl MeetingsListQueryRequest {
    pub fn builder() -> MeetingsListQueryRequestBuilder {
        <MeetingsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MeetingsListQueryRequestBuilder {
    query: Option<String>,
    participant_emails: Option<Vec<String>>,
    participant_match: Option<ListMeetingsRequestParticipantMatch>,
    participant_domains: Option<Vec<String>>,
    domain_match: Option<ListMeetingsRequestDomainMatch>,
    created_after: Option<DateTime<FixedOffset>>,
    created_before: Option<DateTime<FixedOffset>>,
    sort_by: Option<ListMeetingsRequestSortBy>,
    sort_direction: Option<ListMeetingsRequestSortDirection>,
    limit: Option<i64>,
    offset: Option<i64>,
}

impl MeetingsListQueryRequestBuilder {
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn participant_emails(mut self, value: Vec<String>) -> Self {
        self.participant_emails = Some(value);
        self
    }

    pub fn participant_match(mut self, value: ListMeetingsRequestParticipantMatch) -> Self {
        self.participant_match = Some(value);
        self
    }

    pub fn participant_domains(mut self, value: Vec<String>) -> Self {
        self.participant_domains = Some(value);
        self
    }

    pub fn domain_match(mut self, value: ListMeetingsRequestDomainMatch) -> Self {
        self.domain_match = Some(value);
        self
    }

    pub fn created_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_after = Some(value);
        self
    }

    pub fn created_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_before = Some(value);
        self
    }

    pub fn sort_by(mut self, value: ListMeetingsRequestSortBy) -> Self {
        self.sort_by = Some(value);
        self
    }

    pub fn sort_direction(mut self, value: ListMeetingsRequestSortDirection) -> Self {
        self.sort_direction = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MeetingsListQueryRequest`].
    pub fn build(self) -> Result<MeetingsListQueryRequest, BuildError> {
        Ok(MeetingsListQueryRequest {
            query: self.query,
            participant_emails: self.participant_emails,
            participant_match: self.participant_match,
            participant_domains: self.participant_domains,
            domain_match: self.domain_match,
            created_after: self.created_after,
            created_before: self.created_before,
            sort_by: self.sort_by,
            sort_direction: self.sort_direction,
            limit: self.limit,
            offset: self.offset,
        })
    }
}

