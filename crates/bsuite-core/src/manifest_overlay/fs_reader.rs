use std::path::{Path, PathBuf};

use crate::BsuiteCoreError;
use crate::manifest_overlay::{
    ManifestOverlay, ManifestOverlayReader, allowlist::validate_override_keys,
    error::OverlayValidationError, signature::verify_overlay_signature,
    types::OVERLAY_SCHEMA_VERSION,
};

pub struct FileSystemManifestOverlayReader {
    overlay_path: PathBuf,
    sig_path: PathBuf,
    pubkey_path: PathBuf,
}

impl FileSystemManifestOverlayReader {
    pub fn new(binary_name: &str, install_dir: &Path) -> Self {
        let overlay_path = env_path_override("BSUITE_OVERLAY_PATH")
            .unwrap_or_else(|| install_dir.join(format!("{binary_name}.overlay.toml")));
        let pubkey_path = env_path_override("BSUITE_OVERLAY_PUBKEY_PATH")
            .unwrap_or_else(|| install_dir.join(format!("{binary_name}.overlay.pubkey")));
        let sig_path = overlay_path.with_extension("toml.sig");
        Self::from_paths(overlay_path, sig_path, pubkey_path)
    }

    pub fn from_paths(overlay: PathBuf, sig: PathBuf, pubkey: PathBuf) -> Self {
        Self {
            overlay_path: overlay,
            sig_path: sig,
            pubkey_path: pubkey,
        }
    }
}

impl ManifestOverlayReader for FileSystemManifestOverlayReader {
    fn read(&self) -> Result<ManifestOverlay, BsuiteCoreError> {
        if !self.overlay_path.exists() {
            return Ok(ManifestOverlay::empty());
        }
        if !self.sig_path.exists() {
            return Err(OverlayValidationError::SignatureMissing.into());
        }
        if !self.pubkey_path.exists() {
            return Err(OverlayValidationError::PubkeyMissing.into());
        }

        let overlay_bytes = read_file(&self.overlay_path)?;
        let sig_b64 = read_file_string(&self.sig_path)?;
        let pubkey_bytes = read_file(&self.pubkey_path)?;

        verify_overlay_signature(&overlay_bytes, &sig_b64, &pubkey_bytes)?;

        let toml_str = std::str::from_utf8(&overlay_bytes)
            .map_err(|e| OverlayValidationError::TomlParseFailed(e.to_string()))?;

        let toml_value: toml::Value = toml::from_str(toml_str)
            .map_err(|e| OverlayValidationError::TomlParseFailed(e.to_string()))?;

        validate_override_keys(&toml_value)?;

        let overlay: ManifestOverlay = toml::from_str(toml_str)
            .map_err(|e| OverlayValidationError::TomlParseFailed(e.to_string()))?;

        if overlay.schema_version != OVERLAY_SCHEMA_VERSION {
            return Err(OverlayValidationError::SchemaMismatch {
                expected: OVERLAY_SCHEMA_VERSION,
                found: overlay.schema_version,
            }
            .into());
        }

        Ok(overlay)
    }
}

fn env_path_override(var: &str) -> Option<PathBuf> {
    std::env::var_os(var).map(PathBuf::from)
}

fn read_file(path: &Path) -> Result<Vec<u8>, OverlayValidationError> {
    std::fs::read(path).map_err(|e| OverlayValidationError::TomlParseFailed(e.to_string()))
}

fn read_file_string(path: &Path) -> Result<String, OverlayValidationError> {
    std::fs::read_to_string(path)
        .map_err(|e| OverlayValidationError::TomlParseFailed(e.to_string()))
}
