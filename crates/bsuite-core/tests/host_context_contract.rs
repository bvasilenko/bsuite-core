mod common;

use bsuite_core::HostContext;
use common::{assert_projection_contains, assert_stable_mappings, assert_unique_projection};
use proptest::prelude::*;

fn any_host_context() -> impl Strategy<Value = HostContext> {
    prop_oneof![
        Just(HostContext::L2a),
        Just(HostContext::PayloadV3),
        Just(HostContext::StrapiV5),
        Just(HostContext::SanityV3),
        Just(HostContext::DirectusV10),
    ]
}

#[test]
fn host_contexts_have_stable_public_names() {
    assert_stable_mappings(
        HostContext::ALL.map(|host| (host, host.stable_name())),
        [
            (HostContext::L2a, "l2a"),
            (HostContext::PayloadV3, "payload-v3"),
            (HostContext::StrapiV5, "strapi-v5"),
            (HostContext::SanityV3, "sanity-v3"),
            (HostContext::DirectusV10, "directus-v10"),
        ],
    );
}

proptest! {
    #[test]
    fn host_context_names_are_unique_and_complete(host in any_host_context()) {
        assert_unique_projection(HostContext::ALL, HostContext::stable_name);
        assert_projection_contains(HostContext::ALL, HostContext::stable_name, host.stable_name());
    }
}
