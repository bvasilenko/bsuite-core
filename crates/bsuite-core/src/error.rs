use crate::RoutingKey;
use crate::manifest_overlay::OverlayValidationError;
use thiserror::Error;

#[derive(Debug, Error, Clone, Eq, PartialEq)]
pub enum BsuiteCoreError {
    #[error("prompt resolution failed: {0}")]
    PromptResolution(String),
    #[error("update failed: {0}")]
    Update(String),
    #[error("transcript append failed: {0}")]
    Transcript(String),
    #[error("manifest overlay validation failed: {0}")]
    ManifestOverlay(#[from] OverlayValidationError),
    #[error("exit code emission failed: {0}")]
    ExitCode(String),
    #[error("visibility evidence publication failed: {0}")]
    VisibilityEvidence(String),
    #[error("adapter host binding failed: {0}")]
    AdapterHostBinding(String),
    #[error("corpus signature is invalid")]
    CorpusSignatureInvalid,
    #[error("corpus schema mismatch: expected {expected}, found {found}")]
    CorpusSchemaMismatch { expected: u32, found: u32 },
    #[error("corpus deserialization failed: {0}")]
    CorpusDeserializationFailed(String),
    #[error("corpus key is missing: {0}")]
    CorpusKeyMissing(RoutingKey),
}
