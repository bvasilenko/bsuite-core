mod common;

use bsuite_core::{BsuiteCoreError, CorpusEntry, CorpusFile, ProvenanceRecord, RoutingKey};
use common::{corpus_entry, corpus_file};
use proptest::prelude::*;

fn any_routing_key() -> impl Strategy<Value = RoutingKey> {
    prop_oneof![
        Just(RoutingKey::BGround),
        Just(RoutingKey::BAnchor),
        Just(RoutingKey::BSmell),
        Just(RoutingKey::BRatch),
        Just(RoutingKey::BWatch),
        Just(RoutingKey::BSpector),
    ]
}

fn safe_string() -> impl Strategy<Value = String> {
    "[a-zA-Z0-9 .,:;_/-]{1,80}"
}

fn any_entry() -> impl Strategy<Value = CorpusEntry> {
    (
        any_routing_key(),
        safe_string(),
        safe_string(),
        0_u32..10_000,
        safe_string(),
        0_f64..1_f64,
        0_f64..1_f64,
    )
        .prop_map(
            |(
                routing_key,
                directive,
                run_id,
                iteration,
                observation_source,
                pre_compliance,
                post_compliance,
            )| CorpusEntry {
                routing_key,
                directive,
                provenance: ProvenanceRecord {
                    run_id,
                    iteration,
                    observation_source,
                    pre_compliance,
                    post_compliance,
                },
            },
        )
}

proptest! {
    #[test]
    fn valid_corpus_toml_round_trips(entries in prop::collection::vec(any_entry(), 1..20)) {
        let mut corpus = corpus_file(entries);
        corpus.signature = "ed25519:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==".to_string();

        let encoded = toml::to_string(&corpus).expect("valid corpus encodes as TOML");
        let decoded: CorpusFile = toml::from_str(&encoded).expect("encoded corpus decodes from TOML");
        let reencoded = toml::to_string(&decoded).expect("decoded corpus re-encodes as TOML");
        let decoded_again: CorpusFile = toml::from_str(&reencoded).expect("re-encoded corpus decodes from TOML");

        assert_eq!(decoded_again, corpus);
    }
}

#[test]
fn routing_keys_serialize_to_stable_public_names() {
    for key in RoutingKey::ALL {
        let encoded =
            toml::to_string(&corpus_entry(key, "directive", 1)).expect("routing key encodes");

        assert!(encoded.contains(&format!("routing_key = \"{}\"", key.stable_name())));
    }
}

#[test]
fn malformed_toml_maps_to_deserialization_error_variant() {
    let error = toml::from_str::<CorpusFile>("schema_version = \"not-a-number\"")
        .map_err(|error| BsuiteCoreError::CorpusDeserializationFailed(error.to_string()))
        .expect_err("malformed corpus must fail");

    assert!(matches!(
        error,
        BsuiteCoreError::CorpusDeserializationFailed(_)
    ));
}

#[test]
fn unknown_routing_key_is_rejected_by_grammar() {
    let malformed = r#"
schema_version = 1
signature = "ed25519:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=="
canonical_key_id = "test-key"

[[entries]]
routing_key = "unknown"
directive = "Stop and supply evidence before proceeding."

[entries.provenance]
run_id = "fixture"
iteration = 1
observation_source = "unit-test"
pre_compliance = 0.1
post_compliance = 0.9
"#;

    let error = toml::from_str::<CorpusFile>(malformed)
        .map_err(|error| BsuiteCoreError::CorpusDeserializationFailed(error.to_string()))
        .expect_err("unknown routing key must fail");

    assert!(matches!(
        error,
        BsuiteCoreError::CorpusDeserializationFailed(_)
    ));
}

#[test]
fn missing_required_top_level_fields_are_rejected() {
    for toml in [
        r#"signature = "ed25519:AA=="
canonical_key_id = "test-key"
entries = []"#,
        r#"schema_version = 1
canonical_key_id = "test-key"
entries = []"#,
        r#"schema_version = 1
signature = "ed25519:AA=="
entries = []"#,
    ] {
        let error = toml::from_str::<CorpusFile>(toml)
            .map_err(|error| BsuiteCoreError::CorpusDeserializationFailed(error.to_string()))
            .expect_err("missing required field must fail");

        assert!(matches!(
            error,
            BsuiteCoreError::CorpusDeserializationFailed(_)
        ));
    }
}

#[test]
fn missing_required_entry_fields_are_rejected() {
    for toml in [
        r#"schema_version = 1
signature = "ed25519:AA=="
canonical_key_id = "test-key"

[[entries]]
directive = "directive"

[entries.provenance]
run_id = "fixture"
iteration = 1
observation_source = "unit-test"
pre_compliance = 0.1
post_compliance = 0.9"#,
        r#"schema_version = 1
signature = "ed25519:AA=="
canonical_key_id = "test-key"

[[entries]]
routing_key = "bground"

[entries.provenance]
run_id = "fixture"
iteration = 1
observation_source = "unit-test"
pre_compliance = 0.1
post_compliance = 0.9"#,
    ] {
        let error = toml::from_str::<CorpusFile>(toml)
            .map_err(|error| BsuiteCoreError::CorpusDeserializationFailed(error.to_string()))
            .expect_err("missing required entry field must fail");

        assert!(matches!(
            error,
            BsuiteCoreError::CorpusDeserializationFailed(_)
        ));
    }
}
