use bsuite_core::{BsuiteCoreError, ExitCode, ExitCodeEmitter};

struct PendingExitCodeEmitter;

impl ExitCodeEmitter for PendingExitCodeEmitter {
    fn emit(&self, _code: ExitCode) -> Result<(), BsuiteCoreError> {
        unimplemented!("not yet implemented")
    }
}

#[test]
#[should_panic(expected = "not yet implemented")]
fn placeholder_exit_code_emitter_is_explicitly_pending() {
    let emitter = PendingExitCodeEmitter;

    let _ = emitter.emit(ExitCode::Success);
}
