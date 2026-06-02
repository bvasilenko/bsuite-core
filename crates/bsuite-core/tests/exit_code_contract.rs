mod common;

use bsuite_core::ExitCode;
use common::{assert_projection_contains, assert_stable_mappings, assert_unique_projection};
use proptest::prelude::*;

fn any_exit_code() -> impl Strategy<Value = ExitCode> {
    prop_oneof![
        Just(ExitCode::Success),
        Just(ExitCode::Finding),
        Just(ExitCode::InternalError),
        Just(ExitCode::Usage),
    ]
}

#[test]
fn exit_codes_reserve_expected_values() {
    assert_stable_mappings(
        ExitCode::ALL.map(|code| (code, code.as_i32())),
        [
            (ExitCode::Success, 0),
            (ExitCode::Finding, 1),
            (ExitCode::InternalError, 2),
            (ExitCode::Usage, 64),
        ],
    );
}

proptest! {
    #[test]
    fn exit_code_values_are_unique_and_reserved(code in any_exit_code()) {
        assert_unique_projection(ExitCode::ALL, ExitCode::as_i32);
        assert_projection_contains(ExitCode::ALL, ExitCode::as_i32, code.as_i32());
        assert_stable_mappings(
            ExitCode::ALL.map(|item| (item, item.as_i32())),
            [
                (ExitCode::Success, 0),
                (ExitCode::Finding, 1),
                (ExitCode::InternalError, 2),
                (ExitCode::Usage, 64),
            ],
        );
    }
}
