//! Lookup latency stays bounded so command-line binaries can emit directives without
//! making resolver overhead visible to the calling workflow. The non-binding target for
//! warm in-process lookup is p99 <= 10 microseconds on a 50-entry corpus.

use crate::{
    BsuiteCoreError, CorpusEntry, CorpusFile, ManifestOverlay, RoutingKey,
    corpus::parse_signed_corpus,
};
use ed25519_dalek::VerifyingKey;
use std::collections::{BTreeMap, HashMap};

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

#[derive(Debug, Clone)]
pub struct CorpusResolver {
    entries: HashMap<RoutingKey, Vec<CorpusEntry>>,
    schema_version: u32,
    canonical_key_id: String,
    entry_count: usize,
}

impl CorpusResolver {
    pub fn from_toml_signed(
        toml_str: &str,
        pubkey: &VerifyingKey,
    ) -> Result<Self, BsuiteCoreError> {
        let corpus = parse_signed_corpus(toml_str, pubkey)?;
        Ok(Self::from_verified_corpus_file(corpus))
    }

    fn from_verified_corpus_file(corpus: CorpusFile) -> Self {
        let entry_count = corpus.entries.len();
        let mut entries: HashMap<RoutingKey, Vec<CorpusEntry>> = HashMap::new();

        for entry in corpus.entries {
            entries.entry(entry.routing_key).or_default().push(entry);
        }

        Self {
            entries,
            schema_version: corpus.schema_version,
            canonical_key_id: corpus.canonical_key_id,
            entry_count,
        }
    }

    pub fn entry_count(&self) -> usize {
        self.entry_count
    }

    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub fn canonical_key_id(&self) -> &str {
        &self.canonical_key_id
    }

    pub fn entries_for(&self, key: RoutingKey) -> &[CorpusEntry] {
        self.entries.get(&key).map(Vec::as_slice).unwrap_or(&[])
    }
}

impl PromptResolver for CorpusResolver {
    fn resolve(
        &self,
        key: RoutingKey,
        _evidence: EvidenceMap,
        _overlay: Option<ManifestOverlay>,
    ) -> Result<DirectiveString, BsuiteCoreError> {
        let entry = self
            .entries
            .get(&key)
            .and_then(|entries| entries.first())
            .ok_or(BsuiteCoreError::CorpusKeyMissing(key))?;

        Ok(DirectiveString::new(entry.directive.clone()))
    }
}
