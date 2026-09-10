pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePresentationPptxStudioInput {
    /// Asset ID of the folder to create the deck in. Defaults to the user's root.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// Skip the workspace default template and create a blank deck. Only when a blank presentation is explicitly wanted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_template: Option<bool>,
    /// Asset ID to seed the deck from: any ready PPTX Studio deck the user can access, or an uploaded .pptx/.potx file. If it cannot be applied, no deck is created and the error says why.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_asset_id: Option<String>,
    /// Title for the new deck. Defaults to "Untitled Presentation".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl CreatePresentationPptxStudioInput {
    pub fn builder() -> CreatePresentationPptxStudioInputBuilder {
        <CreatePresentationPptxStudioInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePresentationPptxStudioInputBuilder {
    parent_folder_id: Option<String>,
    skip_template: Option<bool>,
    template_asset_id: Option<String>,
    title: Option<String>,
}

impl CreatePresentationPptxStudioInputBuilder {
    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
        self
    }

    pub fn skip_template(mut self, value: bool) -> Self {
        self.skip_template = Some(value);
        self
    }

    pub fn template_asset_id(mut self, value: impl Into<String>) -> Self {
        self.template_asset_id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePresentationPptxStudioInput`].
    pub fn build(self) -> Result<CreatePresentationPptxStudioInput, BuildError> {
        Ok(CreatePresentationPptxStudioInput {
            parent_folder_id: self.parent_folder_id,
            skip_template: self.skip_template,
            template_asset_id: self.template_asset_id,
            title: self.title,
        })
    }
}

