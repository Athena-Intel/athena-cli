pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// SKILL.md content for a skill created via the public API.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicSkillConfigIn {
    /// Space-separated list of pre-approved tools (experimental)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_tools: Option<String>,
    /// The markdown instructions below the SKILL.md frontmatter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_markdown: Option<String>,
    /// Environment requirements (max 500 chars)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility: Option<String>,
    /// What the skill does and when to use it (max 1024 chars)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// License name or reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    /// Additional string metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Option<String>>>,
    /// Skill name slug (lowercase letters, numbers, single hyphens; max 64 chars). Defaults to a slug of the asset title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PublicSkillConfigIn {
    pub fn builder() -> PublicSkillConfigInBuilder {
        <PublicSkillConfigInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicSkillConfigInBuilder {
    allowed_tools: Option<String>,
    body_markdown: Option<String>,
    compatibility: Option<String>,
    description: Option<String>,
    license: Option<String>,
    metadata: Option<HashMap<String, Option<String>>>,
    name: Option<String>,
}

impl PublicSkillConfigInBuilder {
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

    /// Consumes the builder and constructs a [`PublicSkillConfigIn`].
    pub fn build(self) -> Result<PublicSkillConfigIn, BuildError> {
        Ok(PublicSkillConfigIn {
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
