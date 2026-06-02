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
}
