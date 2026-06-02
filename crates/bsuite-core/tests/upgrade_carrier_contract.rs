use bsuite_core::{BsuiteCoreError, UpdateChannel, UpdateOutcome, Updater};

struct PendingUpdater;

impl Updater for PendingUpdater {
    fn update(&self, _channel: UpdateChannel) -> Result<UpdateOutcome, BsuiteCoreError> {
        unimplemented!("not yet implemented")
    }
}

#[test]
#[should_panic(expected = "not yet implemented")]
fn placeholder_updater_is_explicitly_pending() {
    let updater = PendingUpdater;

    let _ = updater.update(UpdateChannel::new("stable"));
}

#[test]
fn update_channel_preserves_inner_value() {
    let channel = UpdateChannel::new("stable");

    assert_eq!(channel.as_str(), "stable");
    assert_eq!(channel.into_inner(), "stable");
}

#[test]
fn update_outcome_exposes_expected_shapes() {
    assert_eq!(UpdateOutcome::AlreadyCurrent, UpdateOutcome::AlreadyCurrent);
    assert_eq!(
        UpdateOutcome::Updated {
            version: String::from("0.1.1")
        },
        UpdateOutcome::Updated {
            version: String::from("0.1.1")
        }
    );
    assert_eq!(
        UpdateOutcome::Deferred {
            reason: String::from("operator choice")
        },
        UpdateOutcome::Deferred {
            reason: String::from("operator choice")
        }
    );
}
