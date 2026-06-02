use bsuite_core::{AdapterBinding, AdapterHostBinder, BsuiteCoreError, HostContext};

struct PendingAdapterHostBinder;

impl AdapterHostBinder for PendingAdapterHostBinder {
    fn bind(&self, _host: HostContext) -> Result<AdapterBinding, BsuiteCoreError> {
        unimplemented!("not yet implemented")
    }
}

#[test]
#[should_panic(expected = "not yet implemented")]
fn placeholder_adapter_host_binder_is_explicitly_pending() {
    let binder = PendingAdapterHostBinder;

    let _ = binder.bind(HostContext::PayloadV3);
}

#[test]
fn adapter_binding_preserves_fields() {
    let binding = AdapterBinding::new(HostContext::DirectusV10, "bdirectus");

    assert_eq!(binding.host, HostContext::DirectusV10);
    assert_eq!(binding.package_name, "bdirectus");
}
