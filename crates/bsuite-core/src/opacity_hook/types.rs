use serde::{Deserialize, Serialize};

pub(super) const SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TierProbes {
    pub control_flow_flattening_density: f64,
    pub instruction_substitution_coverage: f64,
    pub bogus_control_flow_blocks: u32,
    pub basic_block_splitting_ratio: f64,
    pub anti_debug_heuristic_score: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TierEvidence {
    pub schema_version: u32,
    pub tier_id: String,
    pub build_sha: String,
    pub signing_key_id: String,
    pub probes: TierProbes,
}

impl TierEvidence {
    pub fn new(
        tier_id: impl Into<String>,
        build_sha: impl Into<String>,
        signing_key_id: impl Into<String>,
        probes: TierProbes,
    ) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            tier_id: tier_id.into(),
            build_sha: build_sha.into(),
            signing_key_id: signing_key_id.into(),
            probes,
        }
    }
}
