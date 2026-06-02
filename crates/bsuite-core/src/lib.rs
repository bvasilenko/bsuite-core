pub mod adapter_host_bind;
pub mod error;
pub mod exit_code;
pub mod manifest_overlay;
pub mod opacity_hook;
pub mod prompt_resolver;
pub mod routing_key;
pub mod transcript_writer;
pub mod upgrade_carrier;

pub use adapter_host_bind::{AdapterBinding, AdapterHostBinder, HostContext};
pub use error::BsuiteCoreError;
pub use exit_code::{ExitCode, ExitCodeEmitter};
pub use manifest_overlay::{
    ManifestOverlay, ManifestOverlayReader, OverlayMap, OverlayValidationError,
};
pub use opacity_hook::{OpacityHookPublisher, TierEvidence};
pub use prompt_resolver::{DirectiveString, EvidenceMap, PromptResolver};
pub use routing_key::RoutingKey;
pub use transcript_writer::{TranscriptAppender, TranscriptHandle, TranscriptRecord};
pub use upgrade_carrier::{UpdateChannel, UpdateOutcome, Updater};
