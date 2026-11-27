pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_items
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListItemsQueryRequest {
    /// The id of a library or a folder of the source to open. Omit for the top level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    /// The next_page_token of the page before, for the next one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ListItemsQueryRequest {
    pub fn builder() -> ListItemsQueryRequestBuilder {
        <ListItemsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListItemsQueryRequestBuilder {
    folder_id: Option<String>,
    page_token: Option<String>,
}

impl ListItemsQueryRequestBuilder {
    pub fn folder_id(mut self, value: impl Into<String>) -> Self {
        self.folder_id = Some(value.into());
        self
    }

    pub fn page_token(mut self, value: impl Into<String>) -> Self {
        self.page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListItemsQueryRequest`].
    pub fn build(self) -> Result<ListItemsQueryRequest, BuildError> {
        Ok(ListItemsQueryRequest {
            folder_id: self.folder_id,
            page_token: self.page_token,
        })
    }
}

