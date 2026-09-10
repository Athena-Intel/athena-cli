pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Result of `GET /tools/email/search`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmailSearchResponseOut {
    /// The connected account that was searched.
    #[serde(default)]
    pub catalog_id: String,
    /// Number of results returned.
    #[serde(default)]
    pub count: i64,
    /// How many of the results are unsent drafts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_count: Option<i64>,
    /// Gmail search operators that have no Outlook equivalent and were dropped from the query. Outlook accounts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored_operators: Option<Vec<String>>,
    /// Human-readable caveats about the results, when there are any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Account provider as Athena reports it: `gmail` or `outlook` for accounts connected through the Integrations page; `google` or `microsoft365` for directly-connected accounts (read-only for drafts).
    #[serde(default)]
    pub provider: String,
    /// The query as executed.
    #[serde(default)]
    pub query: String,
    /// Matching messages, newest first.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<EmailSearchResultOut>>,
}

impl EmailSearchResponseOut {
    pub fn builder() -> EmailSearchResponseOutBuilder {
        <EmailSearchResponseOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmailSearchResponseOutBuilder {
    catalog_id: Option<String>,
    count: Option<i64>,
    draft_count: Option<i64>,
    ignored_operators: Option<Vec<String>>,
    note: Option<String>,
    provider: Option<String>,
    query: Option<String>,
    results: Option<Vec<EmailSearchResultOut>>,
}

impl EmailSearchResponseOutBuilder {
    pub fn catalog_id(mut self, value: impl Into<String>) -> Self {
        self.catalog_id = Some(value.into());
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn draft_count(mut self, value: i64) -> Self {
        self.draft_count = Some(value);
        self
    }

    pub fn ignored_operators(mut self, value: Vec<String>) -> Self {
        self.ignored_operators = Some(value);
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn results(mut self, value: Vec<EmailSearchResultOut>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EmailSearchResponseOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`catalog_id`](EmailSearchResponseOutBuilder::catalog_id)
    /// - [`count`](EmailSearchResponseOutBuilder::count)
    /// - [`provider`](EmailSearchResponseOutBuilder::provider)
    /// - [`query`](EmailSearchResponseOutBuilder::query)
    pub fn build(self) -> Result<EmailSearchResponseOut, BuildError> {
        Ok(EmailSearchResponseOut {
            catalog_id: self.catalog_id.ok_or_else(|| BuildError::missing_field("catalog_id"))?,
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            draft_count: self.draft_count,
            ignored_operators: self.ignored_operators,
            note: self.note,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            query: self.query.ok_or_else(|| BuildError::missing_field("query"))?,
            results: self.results,
        })
    }
}
