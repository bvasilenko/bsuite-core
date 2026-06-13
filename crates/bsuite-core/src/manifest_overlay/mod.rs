mod allowlist;
mod error;
mod fs_reader;
mod signature;
mod types;

pub use allowlist::ALLOWED_OVERRIDE_KEYS;
pub use error::OverlayValidationError;
pub use fs_reader::FileSystemManifestOverlayReader;
pub use types::{BinaryDefaults, ManifestOverlay, OVERLAY_SCHEMA_VERSION, OverrideMap};

use crate::BsuiteCoreError;

pub trait ManifestOverlayReader {
    fn read(&self) -> Result<ManifestOverlay, BsuiteCoreError>;
}
