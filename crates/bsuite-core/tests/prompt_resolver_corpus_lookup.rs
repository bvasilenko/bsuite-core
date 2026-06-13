mod common;

use bsuite_core::{
    BsuiteCoreError, CorpusFile, CorpusResolver, EvidenceMap, ManifestOverlay, OverlayMap,
    PromptResolver, RoutingKey,
};
use common::{corpus_entry, corpus_file, signed_resolver};

fn resolver_with_all_keys() -> CorpusResolver {
    let entries = RoutingKey::ALL
        .into_iter()
        .enumerate()
        .map(|(index, key)| {
            corpus_entry(
                key,
                format!("directive for {}", key.stable_name()),
                index as u32,
            )
        })
        .collect();

    signed_resolver(CorpusFile {
        schema_version: 1,
        signature: String::new(),
        canonical_key_id: "lookup-key".to_string(),
        entries,
    })
}

#[test]
fn each_routing_key_variant_hits_the_right_entry() {
    let resolver = resolver_with_all_keys();

    for key in RoutingKey::ALL {
        let directive = resolver
            .resolve(key, EvidenceMap::new(), None)
            .expect("every routing key has an entry");

        assert_eq!(
            directive.as_str(),
            format!("directive for {}", key.stable_name())
        );
    }
}

#[test]
fn missing_key_returns_expected_error_for_empty_and_partial_corpora() {
    for resolver in [
        signed_resolver(corpus_file(Vec::new())),
        signed_resolver(corpus_file(vec![corpus_entry(
            RoutingKey::BGround,
            "only bground",
            1,
        )])),
    ] {
        let error = resolver
            .resolve(RoutingKey::BAnchor, EvidenceMap::new(), None)
            .expect_err("missing key must be explicit");

        assert_eq!(
            error,
            BsuiteCoreError::CorpusKeyMissing(RoutingKey::BAnchor)
        );
        assert!(resolver.entries_for(RoutingKey::BAnchor).is_empty());
    }
}

#[test]
fn evidence_and_overlay_do_not_change_lookup_in_this_cycle() {
    let resolver = resolver_with_all_keys();
    let mut evidence = EvidenceMap::new();
    evidence.insert("ignored".to_string(), "unchanged-by-lookup".to_string());
    let overlay = ManifestOverlay::new(OverlayMap::empty());

    let directive = resolver
        .resolve(RoutingKey::BSmell, evidence, Some(overlay))
        .expect("host inputs pass through without changing corpus lookup");

    assert_eq!(directive.as_str(), "directive for bsmell");
}

#[test]
fn duplicate_routing_entries_are_preserved_and_resolve_stably() {
    let resolver = signed_resolver(corpus_file(vec![
        corpus_entry(RoutingKey::BWatch, "first directive", 1),
        corpus_entry(RoutingKey::BWatch, "second directive", 2),
    ]));

    assert_eq!(resolver.entry_count(), 2);
    assert_eq!(resolver.entries_for(RoutingKey::BWatch).len(), 2);
    assert_eq!(
        resolver
            .resolve(RoutingKey::BWatch, EvidenceMap::new(), None)
            .expect("duplicate entries resolve by stable corpus order")
            .as_str(),
        "first directive"
    );
}

#[test]
fn resolver_metadata_preserves_loaded_corpus_identity() {
    let resolver = signed_resolver(CorpusFile {
        schema_version: 1,
        signature: String::new(),
        canonical_key_id: "lookup-key".to_string(),
        entries: vec![corpus_entry(RoutingKey::BSpector, "directive", 1)],
    });

    assert_eq!(resolver.entry_count(), 1);
    assert_eq!(resolver.schema_version(), 1);
    assert_eq!(resolver.canonical_key_id(), "lookup-key");
}
