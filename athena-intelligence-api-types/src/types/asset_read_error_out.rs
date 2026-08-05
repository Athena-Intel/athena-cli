pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Teaching error explaining why a read failed and how to recover.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AssetReadErrorOut {
    /// One sentence describing the exact re-query syntax to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
    /// Machine-readable error category: not_found, out_of_range, invalid_anchor_params, unsupported_anchor, unavailable, transport
    #[serde(default)]
    pub kind: String,
    /// Human-readable description of the failure
    #[serde(default)]
    pub message: String,
    /// Whether retrying the identical request could succeed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retryable: Option<bool>,
    /// Concrete valid values to use instead (e.g. available sheet ids)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_options: Option<HashMap<String, serde_json::Value>>,
}

impl AssetReadErrorOut {
    pub fn builder() -> AssetReadErrorOutBuilder {
        <AssetReadErrorOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetReadErrorOutBuilder {
    hint: Option<String>,
    kind: Option<String>,
    message: Option<String>,
    retryable: Option<bool>,
    valid_options: Option<HashMap<String, serde_json::Value>>,
}

impl AssetReadErrorOutBuilder {
    pub fn hint(mut self, value: impl Into<String>) -> Self {
        self.hint = Some(value.into());
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn retryable(mut self, value: bool) -> Self {
        self.retryable = Some(value);
        self
    }

    pub fn valid_options(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.valid_options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssetReadErrorOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](AssetReadErrorOutBuilder::kind)
    /// - [`message`](AssetReadErrorOutBuilder::message)
    pub fn build(self) -> Result<AssetReadErrorOut, BuildError> {
        Ok(AssetReadErrorOut {
            hint: self.hint,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            retryable: self.retryable,
            valid_options: self.valid_options,
        })
    }
}
