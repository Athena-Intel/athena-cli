pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One place a person is, as the viewer may see it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PresenceRosterContextOut {
    /// The spot in a form the asset's own editor understands (a sheet and cell, a slide, a block). Opaque to anything else.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<HashMap<String, serde_json::Value>>,
    /// Athena asset type, e.g. document, spreadsheet, session, project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<String>,
    /// True when the viewer may not open where the person is, only a project that contains it: the entry is that project, and says no more about what the person has open or how many.
    #[serde(default)]
    pub coarse: bool,
    /// Whether this is the place the person is looking at now.
    #[serde(default)]
    pub focused: bool,
    /// The asset id, the chat's thread id, or the project id.
    #[serde(default)]
    pub guid: String,
    /// Where in it the person is, as its editor names the spot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_label: Option<String>,
    /// Ids of the projects containing it that the viewer may open.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<String>>,
    /// Display title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// asset: a document, sheet or other asset. session: a chat, named by its thread id. project: somewhere in a project (see coarse).
    pub r#type: PresenceRosterContextOutType,
}

impl PresenceRosterContextOut {
    pub fn builder() -> PresenceRosterContextOutBuilder {
        <PresenceRosterContextOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PresenceRosterContextOutBuilder {
    anchor: Option<HashMap<String, serde_json::Value>>,
    asset_type: Option<String>,
    coarse: Option<bool>,
    focused: Option<bool>,
    guid: Option<String>,
    location_label: Option<String>,
    projects: Option<Vec<String>>,
    title: Option<String>,
    r#type: Option<PresenceRosterContextOutType>,
}

impl PresenceRosterContextOutBuilder {
    pub fn anchor(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.anchor = Some(value);
        self
    }

    pub fn asset_type(mut self, value: impl Into<String>) -> Self {
        self.asset_type = Some(value.into());
        self
    }

    pub fn coarse(mut self, value: bool) -> Self {
        self.coarse = Some(value);
        self
    }

    pub fn focused(mut self, value: bool) -> Self {
        self.focused = Some(value);
        self
    }

    pub fn guid(mut self, value: impl Into<String>) -> Self {
        self.guid = Some(value.into());
        self
    }

    pub fn location_label(mut self, value: impl Into<String>) -> Self {
        self.location_label = Some(value.into());
        self
    }

    pub fn projects(mut self, value: Vec<String>) -> Self {
        self.projects = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PresenceRosterContextOutType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PresenceRosterContextOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`coarse`](PresenceRosterContextOutBuilder::coarse)
    /// - [`focused`](PresenceRosterContextOutBuilder::focused)
    /// - [`guid`](PresenceRosterContextOutBuilder::guid)
    /// - [`r#type`](PresenceRosterContextOutBuilder::r#type)
    pub fn build(self) -> Result<PresenceRosterContextOut, BuildError> {
        Ok(PresenceRosterContextOut {
            anchor: self.anchor,
            asset_type: self.asset_type,
            coarse: self.coarse.ok_or_else(|| BuildError::missing_field("coarse"))?,
            focused: self.focused.ok_or_else(|| BuildError::missing_field("focused"))?,
            guid: self.guid.ok_or_else(|| BuildError::missing_field("guid"))?,
            location_label: self.location_label,
            projects: self.projects,
            title: self.title,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
