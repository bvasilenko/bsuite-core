use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct OverlayMap(BTreeMap<String, String>);

impl OverlayMap {
    pub fn new(entries: BTreeMap<String, String>) -> Result<Self, OverlayValidationError> {
        if entries.keys().any(|key| key.trim().is_empty()) {
            return Err(OverlayValidationError::EmptyKey);
        }

        Ok(Self(entries))
    }

    pub fn empty() -> Self {
        Self(BTreeMap::new())
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(String::as_str)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.0
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }

    pub fn into_inner(self) -> BTreeMap<String, String> {
        self.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ManifestOverlay {
    pub entries: OverlayMap,
}

impl ManifestOverlay {
    pub fn new(entries: OverlayMap) -> Self {
        Self { entries }
    }
}

#[derive(Debug, Error, Clone, Eq, PartialEq)]
pub enum OverlayValidationError {
    #[error("overlay key must not be empty")]
    EmptyKey,
    #[error("overlay signature is missing")]
    MissingSignature,
    #[error("overlay signature is invalid")]
    InvalidSignature,
    #[error("overlay payload is malformed: {0}")]
    MalformedPayload(String),
}

pub trait ManifestOverlayReader {
    fn read_overlay(&self) -> Result<Option<ManifestOverlay>, OverlayValidationError>;
}
