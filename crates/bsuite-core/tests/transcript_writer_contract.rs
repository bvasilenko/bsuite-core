use bsuite_core::{
    BsuiteCoreError, RoutingKey, TranscriptAppender, TranscriptHandle, TranscriptRecord,
};
use std::time::SystemTime;

struct PendingTranscriptAppender;

impl TranscriptAppender for PendingTranscriptAppender {
    fn append(&self, _record: TranscriptRecord) -> Result<TranscriptHandle, BsuiteCoreError> {
        unimplemented!("not yet implemented")
    }
}

#[test]
#[should_panic(expected = "not yet implemented")]
fn placeholder_transcript_appender_is_explicitly_pending() {
    let appender = PendingTranscriptAppender;
    let record = TranscriptRecord::new(RoutingKey::banchor(), "captured", SystemTime::UNIX_EPOCH);

    let _ = appender.append(record);
}

#[test]
fn transcript_record_preserves_fields() {
    let record = TranscriptRecord::new(RoutingKey::bwatch(), "captured", SystemTime::UNIX_EPOCH);

    assert_eq!(record.routing_key, RoutingKey::BWatch);
    assert_eq!(record.message, "captured");
    assert_eq!(record.recorded_at, SystemTime::UNIX_EPOCH);
}

#[test]
fn transcript_handle_preserves_inner_value() {
    let handle = TranscriptHandle::new("transcript-1");

    assert_eq!(handle.as_str(), "transcript-1");
    assert_eq!(handle.into_inner(), "transcript-1");
}
