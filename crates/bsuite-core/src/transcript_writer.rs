use crate::{BsuiteCoreError, RoutingKey};
use std::time::SystemTime;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TranscriptRecord {
    pub routing_key: RoutingKey,
    pub message: String,
    pub recorded_at: SystemTime,
}

impl TranscriptRecord {
    pub fn new(
        routing_key: RoutingKey,
        message: impl Into<String>,
        recorded_at: SystemTime,
    ) -> Self {
        Self {
            routing_key,
            message: message.into(),
            recorded_at,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TranscriptHandle(String);

impl TranscriptHandle {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

pub trait TranscriptAppender {
    fn append(&self, record: TranscriptRecord) -> Result<TranscriptHandle, BsuiteCoreError>;
}
