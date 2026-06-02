use bsuite_core::{
    BsuiteCoreError, DirectiveString, EvidenceMap, ManifestOverlay, PromptResolver, RoutingKey,
};

struct PendingPromptResolver;

impl PromptResolver for PendingPromptResolver {
    fn resolve(
        &self,
        _key: RoutingKey,
        _evidence: EvidenceMap,
        _overlay: Option<ManifestOverlay>,
    ) -> Result<DirectiveString, BsuiteCoreError> {
        unimplemented!("not yet implemented")
    }
}

#[test]
#[should_panic(expected = "not yet implemented")]
fn placeholder_prompt_resolver_is_explicitly_pending() {
    let resolver = PendingPromptResolver;

    let _ = resolver.resolve(RoutingKey::bground(), EvidenceMap::new(), None);
}

#[test]
fn directive_string_preserves_inner_value() {
    let directive = DirectiveString::new("proceed");

    assert_eq!(directive.as_str(), "proceed");
    assert_eq!(directive.into_inner(), "proceed");
}
