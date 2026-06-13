#![allow(dead_code)]

use std::collections::BTreeSet;
use std::fmt::Debug;

pub fn assert_stable_mappings<T, U, const N: usize>(actual: [(T, U); N], expected: [(T, U); N])
where
    T: Debug + Eq,
    U: Debug + Eq,
{
    assert_eq!(actual, expected);
}

pub fn assert_unique_projection<T, U, const N: usize>(items: [T; N], projection: impl Fn(T) -> U)
where
    T: Copy,
    U: Ord + Debug,
{
    let values = items.into_iter().map(projection).collect::<BTreeSet<_>>();

    assert_eq!(values.len(), N);
}

pub fn assert_projection_contains<T, U, const N: usize>(
    items: [T; N],
    projection: impl Fn(T) -> U,
    expected: U,
) where
    T: Copy,
    U: Ord + Debug,
{
    let values = items.into_iter().map(projection).collect::<BTreeSet<_>>();

    assert!(values.contains(&expected));
}

use base64::Engine;
use bsuite_core::{
    CorpusEntry, CorpusFile, CorpusResolver, ProvenanceRecord, RoutingKey,
    corpus::canonical_payload_bytes,
};
use ed25519_dalek::{Signer, SigningKey};

pub fn corpus_signing_key(seed: u8) -> SigningKey {
    SigningKey::from_bytes(&[seed; 32])
}

pub fn provenance(iteration: u32) -> ProvenanceRecord {
    ProvenanceRecord {
        run_id: format!("test-run-{iteration}"),
        iteration,
        observation_source: "contract-test".to_string(),
        pre_compliance: 0.25,
        post_compliance: 0.95,
    }
}

pub fn corpus_entry(
    routing_key: RoutingKey,
    directive: impl Into<String>,
    iteration: u32,
) -> CorpusEntry {
    CorpusEntry {
        routing_key,
        directive: directive.into(),
        provenance: provenance(iteration),
    }
}

pub fn corpus_file(entries: Vec<CorpusEntry>) -> CorpusFile {
    CorpusFile {
        schema_version: 1,
        signature: String::new(),
        canonical_key_id: "test-key".to_string(),
        entries,
    }
}

pub fn single_entry_corpus(routing_key: RoutingKey) -> CorpusFile {
    corpus_file(vec![corpus_entry(
        routing_key,
        "Stop and inspect the supplied evidence before acting.",
        1,
    )])
}

pub fn signed_toml(mut corpus: CorpusFile, signing_key: &SigningKey) -> String {
    let payload = canonical_payload_bytes(&corpus).expect("canonical payload is available");
    let signature = signing_key.sign(&payload);
    corpus.signature = format!(
        "ed25519:{}",
        base64::engine::general_purpose::STANDARD.encode(signature.to_bytes())
    );
    toml::to_string(&corpus).expect("signed corpus encodes as TOML")
}

pub fn signed_resolver(corpus: CorpusFile) -> CorpusResolver {
    let signing_key = corpus_signing_key(42);
    let toml = signed_toml(corpus, &signing_key);
    CorpusResolver::from_toml_signed(&toml, &(&signing_key).into())
        .expect("test corpus signature verifies")
}
