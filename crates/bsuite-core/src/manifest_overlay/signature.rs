use base64::Engine;
use ed25519_dalek::{Signature, VerifyingKey};

use crate::manifest_overlay::error::OverlayValidationError;

pub fn verify_overlay_signature(
    overlay_bytes: &[u8],
    sig_b64: &str,
    pubkey_raw: &[u8],
) -> Result<(), OverlayValidationError> {
    let verifying_key = parse_pubkey(pubkey_raw)?;
    let signature = decode_signature(sig_b64)?;
    let canonical = toml_to_jcs(overlay_bytes)?;
    verifying_key
        .verify_strict(&canonical, &signature)
        .map_err(|_| OverlayValidationError::SignatureInvalid)
}

fn parse_pubkey(raw: &[u8]) -> Result<VerifyingKey, OverlayValidationError> {
    let bytes: &[u8; 32] = raw
        .try_into()
        .map_err(|_| OverlayValidationError::SignatureInvalid)?;
    VerifyingKey::from_bytes(bytes).map_err(|_| OverlayValidationError::SignatureInvalid)
}

fn decode_signature(b64: &str) -> Result<Signature, OverlayValidationError> {
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(b64.trim())
        .map_err(|_| OverlayValidationError::SignatureInvalid)?;
    Signature::from_slice(&bytes).map_err(|_| OverlayValidationError::SignatureInvalid)
}

fn toml_to_jcs(overlay_bytes: &[u8]) -> Result<Vec<u8>, OverlayValidationError> {
    let toml_str = std::str::from_utf8(overlay_bytes)
        .map_err(|e| OverlayValidationError::TomlParseFailed(e.to_string()))?;
    let toml_value: toml::Value = toml::from_str(toml_str)
        .map_err(|e| OverlayValidationError::TomlParseFailed(e.to_string()))?;
    let json_value: serde_json::Value = serde_json::to_value(toml_value)
        .map_err(|e| OverlayValidationError::TomlParseFailed(e.to_string()))?;
    serde_json_canonicalizer::to_vec(&json_value)
        .map_err(|e| OverlayValidationError::TomlParseFailed(e.to_string()))
}
