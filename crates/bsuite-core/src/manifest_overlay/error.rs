use thiserror::Error;

#[derive(Debug, Error, Clone, Eq, PartialEq)]
pub enum OverlayValidationError {
    #[error("overlay signature is missing")]
    SignatureMissing,
    #[error("overlay signature is invalid")]
    SignatureInvalid,
    #[error("overlay public key is missing")]
    PubkeyMissing,
    #[error("overlay schema version mismatch: expected {expected}, found {found}")]
    SchemaMismatch { expected: u32, found: u32 },
    #[error("overlay contains unknown key in [overrides]: {key}")]
    UnknownKey { key: String },
    #[error("overlay TOML is malformed: {0}")]
    TomlParseFailed(String),
}
