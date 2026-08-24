pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListContentsInput {
    /// ID of the asset (Folder, Collection, or Project) to list contents for.
    #[serde(default)]
    pub asset_id: String,
    /// Whether to include additional details for each asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_asset_details: Option<bool>,
    /// Whether to include system files in the output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_system_files: Option<bool>,
    /// Page number for paginated results (1-indexed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl ListContentsInput {
    pub fn builder() -> ListContentsInputBuilder {
        <ListContentsInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListContentsInputBuilder {
    asset_id: Option<String>,
    include_asset_details: Option<bool>,
    include_system_files: Option<bool>,
    page: Option<i64>,
}

impl ListContentsInputBuilder {
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

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListContentsInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](ListContentsInputBuilder::asset_id)
    pub fn build(self) -> Result<ListContentsInput, BuildError> {
        Ok(ListContentsInput {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            include_asset_details: self.include_asset_details,
            include_system_files: self.include_system_files,
            page: self.page,
        })
    }
}

