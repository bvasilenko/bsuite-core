use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const OVERLAY_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ManifestOverlay {
    pub schema_version: u32,
    #[serde(default)]
    pub overrides: OverrideMap,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct OverrideMap {
    pub transcript_retention_days: Option<u32>,
    pub transcript_dir: Option<PathBuf>,
    pub corpus_dir: Option<PathBuf>,
    pub update_check_interval_minutes: Option<u32>,
    pub stdout_byte_cap: Option<u64>,
    pub binary_timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BinaryDefaults {
    pub transcript_retention_days: u32,
    pub transcript_dir: PathBuf,
    pub corpus_dir: PathBuf,
    pub update_check_interval_minutes: u32,
    pub stdout_byte_cap: u64,
    pub binary_timeout_ms: u64,
}

impl ManifestOverlay {
    pub fn empty() -> Self {
        Self {
            schema_version: OVERLAY_SCHEMA_VERSION,
            overrides: OverrideMap::default(),
        }
    }

    pub fn merge_into_defaults(&self, defaults: &mut BinaryDefaults) {
        if let Some(v) = self.overrides.transcript_retention_days {
            defaults.transcript_retention_days = v;
        }
        if let Some(ref v) = self.overrides.transcript_dir {
            defaults.transcript_dir = v.clone();
        }
        if let Some(ref v) = self.overrides.corpus_dir {
            defaults.corpus_dir = v.clone();
        }
        if let Some(v) = self.overrides.update_check_interval_minutes {
            defaults.update_check_interval_minutes = v;
        }
        if let Some(v) = self.overrides.stdout_byte_cap {
            defaults.stdout_byte_cap = v;
        }
        if let Some(v) = self.overrides.binary_timeout_ms {
            defaults.binary_timeout_ms = v;
        }
    }
}
