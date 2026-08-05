pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for _list_contents
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListContentsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_asset_details: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_system_files: Option<bool>,
}

impl ListContentsQueryRequest {
    pub fn builder() -> ListContentsQueryRequestBuilder {
        <ListContentsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListContentsQueryRequestBuilder {
    asset_id: Option<String>,
    include_asset_details: Option<bool>,
    include_system_files: Option<bool>,
}

impl ListContentsQueryRequestBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn include_asset_details(mut self, value: bool) -> Self {
        self.include_asset_details = Some(value);
        self
    }

    pub fn include_system_files(mut self, value: bool) -> Self {
        self.include_system_files = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListContentsQueryRequest`].
    pub fn build(self) -> Result<ListContentsQueryRequest, BuildError> {
        Ok(ListContentsQueryRequest {
            asset_id: self.asset_id,
            include_asset_details: self.include_asset_details,
            include_system_files: self.include_system_files,
        })
    }
}

