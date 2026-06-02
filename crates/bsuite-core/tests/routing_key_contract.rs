mod common;

use bsuite_core::RoutingKey;
use common::{assert_projection_contains, assert_stable_mappings, assert_unique_projection};
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

#[test]
fn constructors_return_expected_routing_keys() {
    assert_stable_mappings(
        [
            (RoutingKey::bground(), RoutingKey::BGround),
            (RoutingKey::banchor(), RoutingKey::BAnchor),
            (RoutingKey::bsmell(), RoutingKey::BSmell),
            (RoutingKey::bratch(), RoutingKey::BRatch),
            (RoutingKey::bwatch(), RoutingKey::BWatch),
            (RoutingKey::bspector(), RoutingKey::BSpector),
        ],
        [
            (RoutingKey::BGround, RoutingKey::BGround),
            (RoutingKey::BAnchor, RoutingKey::BAnchor),
            (RoutingKey::BSmell, RoutingKey::BSmell),
            (RoutingKey::BRatch, RoutingKey::BRatch),
            (RoutingKey::BWatch, RoutingKey::BWatch),
            (RoutingKey::BSpector, RoutingKey::BSpector),
        ],
    );
}

#[test]
fn all_routing_keys_have_stable_public_names() {
    assert_stable_mappings(
        RoutingKey::ALL.map(|key| (key, key.stable_name())),
        [
            (RoutingKey::BGround, "bground"),
            (RoutingKey::BAnchor, "banchor"),
            (RoutingKey::BSmell, "bsmell"),
            (RoutingKey::BRatch, "bratch"),
            (RoutingKey::BWatch, "bwatch"),
            (RoutingKey::BSpector, "bspector"),
        ],
    );
}

proptest! {
    #[test]
    fn routing_key_names_are_unique_and_complete(key in any_routing_key()) {
        assert_unique_projection(RoutingKey::ALL, RoutingKey::stable_name);
        assert_projection_contains(RoutingKey::ALL, RoutingKey::stable_name, key.stable_name());
    }
}
