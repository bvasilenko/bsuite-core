use crate::{BsuiteCoreError, ManifestOverlay, RoutingKey};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DirectiveString(String);

impl DirectiveString {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

pub type EvidenceMap = BTreeMap<String, String>;

pub trait PromptResolver {
    fn resolve(
        &self,
        key: RoutingKey,
        evidence: EvidenceMap,
        overlay: Option<ManifestOverlay>,
    ) -> Result<DirectiveString, BsuiteCoreError>;
}
