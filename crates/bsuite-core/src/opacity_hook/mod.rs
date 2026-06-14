mod types;

pub mod section;

#[cfg(feature = "verify")]
mod verify;

pub use section::{OPACITY_SECTION_ELF, OPACITY_SECTION_MACHO, OPACITY_SECTION_PE};
pub use types::{TierEvidence, TierProbes};

#[cfg(feature = "verify")]
pub use verify::{validate_tier_evidence_toml, verify_tier_evidence};

use crate::BsuiteCoreError;

pub trait OpacityHookPublisher {
    fn publish(&self, evidence: TierEvidence) -> Result<(), BsuiteCoreError>;
}

#[macro_export]
macro_rules! tier_evidence_marker {
    ($evidence_toml:literal) => {
        #[cfg(target_os = "macos")]
        #[unsafe(link_section = "__DATA,__BSUITE_OPACITY")]
        #[used]
        static BSUITE_TIER_EVIDENCE: [u8; ::core::include_bytes!($evidence_toml).len()] =
            *::core::include_bytes!($evidence_toml);

        #[cfg(target_os = "windows")]
        #[unsafe(link_section = ".bsopac")]
        #[used]
        static BSUITE_TIER_EVIDENCE: [u8; ::core::include_bytes!($evidence_toml).len()] =
            *::core::include_bytes!($evidence_toml);

        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        #[unsafe(link_section = "__BSUITE_OPACITY")]
        #[used]
        static BSUITE_TIER_EVIDENCE: [u8; ::core::include_bytes!($evidence_toml).len()] =
            *::core::include_bytes!($evidence_toml);
    };
}
