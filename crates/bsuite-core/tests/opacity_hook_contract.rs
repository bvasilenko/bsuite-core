use bsuite_core::{BsuiteCoreError, OpacityHookPublisher, RoutingKey, TierEvidence};

struct PendingOpacityHookPublisher;

impl OpacityHookPublisher for PendingOpacityHookPublisher {
    fn publish(&self, _evidence: TierEvidence) -> Result<(), BsuiteCoreError> {
        unimplemented!("not yet implemented")
    }
}

#[test]
#[should_panic(expected = "not yet implemented")]
fn placeholder_visibility_publisher_is_explicitly_pending() {
    let publisher = PendingOpacityHookPublisher;
    let evidence = TierEvidence::new(RoutingKey::bratch(), "release", "available");

    let _ = publisher.publish(evidence);
}

#[test]
fn tier_evidence_preserves_fields() {
    let evidence = TierEvidence::new(RoutingKey::bspector(), "release", "available");

    assert_eq!(evidence.routing_key, RoutingKey::BSpector);
    assert_eq!(evidence.tier, "release");
    assert_eq!(evidence.evidence, "available");
}
