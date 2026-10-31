pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateSkillConfigRequestIn {
    /// Space-separated list of pre-approved tools (experimental); an empty string clears it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_tools: Option<String>,
    /// The markdown instructions below the SKILL.md frontmatter. Replaces the whole body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_markdown: Option<String>,
    /// Environment requirements (max 500 chars); an empty string clears it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility: Option<String>,
    /// What the skill does and when to use it (max 1024 chars)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// License name or reference; an empty string clears it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    /// Additional string metadata; replaces the whole map
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Option<String>>>,
    /// Skill name slug (lowercase letters, numbers, single hyphens; max 64 chars). Also becomes the asset title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateSkillConfigRequestIn {
    pub fn builder() -> UpdateSkillConfigRequestInBuilder {
        <UpdateSkillConfigRequestInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateSkillConfigRequestInBuilder {
    allowed_tools: Option<String>,
    body_markdown: Option<String>,
    compatibility: Option<String>,
    description: Option<String>,
    license: Option<String>,
    metadata: Option<HashMap<String, Option<String>>>,
    name: Option<String>,
}

impl UpdateSkillConfigRequestInBuilder {
    pub fn allowed_tools(mut self, value: impl Into<String>) -> Self {
        self.allowed_tools = Some(value.into());
        self
    }

    pub fn body_markdown(mut self, value: impl Into<String>) -> Self {
        self.body_markdown = Some(value.into());
        self
    }

    pub fn compatibility(mut self, value: impl Into<String>) -> Self {
        self.compatibility = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn license(mut self, value: impl Into<String>) -> Self {
        self.license = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateSkillConfigRequestIn`].
    pub fn build(self) -> Result<UpdateSkillConfigRequestIn, BuildError> {
        Ok(UpdateSkillConfigRequestIn {
            allowed_tools: self.allowed_tools,
            body_markdown: self.body_markdown,
            compatibility: self.compatibility,
            description: self.description,
            license: self.license,
            metadata: self.metadata,
            name: self.name,
        })
    }
}

