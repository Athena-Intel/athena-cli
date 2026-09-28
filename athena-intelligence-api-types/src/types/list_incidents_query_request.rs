pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_incidents
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListIncidentsQueryRequest {
    /// The project whose incidents to list
    #[serde(default)]
    pub project_id: String,
    /// Only this spec environment; both when omitted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<ListIncidentsSystemRequestEnvironment>,
    /// Only these states (repeat the parameter or separate with commas): open, acknowledged, mitigating, resolved, closed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Vec<String>>,
    /// Page size (1 to 200)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Incidents to skip; the previous page's next_offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl ListIncidentsQueryRequest {
    pub fn builder() -> ListIncidentsQueryRequestBuilder {
        <ListIncidentsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListIncidentsQueryRequestBuilder {
    project_id: Option<String>,
    environment: Option<ListIncidentsSystemRequestEnvironment>,
    state: Option<Vec<String>>,
    limit: Option<i64>,
    offset: Option<i64>,
}

impl ListIncidentsQueryRequestBuilder {
    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn environment(mut self, value: ListIncidentsSystemRequestEnvironment) -> Self {
        self.environment = Some(value);
        self
    }

    pub fn state(mut self, value: Vec<String>) -> Self {
        self.state = Some(value);
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

    /// Consumes the builder and constructs a [`ListIncidentsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`project_id`](ListIncidentsQueryRequestBuilder::project_id)
    pub fn build(self) -> Result<ListIncidentsQueryRequest, BuildError> {
        Ok(ListIncidentsQueryRequest {
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            environment: self.environment,
            state: self.state,
            limit: self.limit,
            offset: self.offset,
        })
    }
}

