use crate::BsuiteCoreError;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UpdateChannel(String);

impl UpdateChannel {
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UpdateOutcome {
    AlreadyCurrent,
    Updated { version: String },
    Deferred { reason: String },
}

pub trait Updater {
    fn update(&self, channel: UpdateChannel) -> Result<UpdateOutcome, BsuiteCoreError>;
}
