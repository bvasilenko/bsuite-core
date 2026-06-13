use crate::BsuiteCoreError;
use base64::Engine;
use chrono::{DateTime, Utc};
use ed25519_dalek::{Signature, VerifyingKey};
use reqwest::blocking::Client;
use semver::Version;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Component, Path, PathBuf};
use std::time::Duration;

pub const MANIFEST_SCHEMA_VERSION: u32 = 1;
const MANIFEST_BODY_LIMIT_BYTES: u64 = 1024 * 1024;
const SIGNATURE_BODY_LIMIT_BYTES: u64 = 1024 * 8;
const ARCHIVE_BODY_LIMIT_BYTES: u64 = 1024 * 1024 * 100;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UpdateChannel(String);

impl UpdateChannel {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SignedManifest {
    pub schema_version: u32,
    pub binary_name: String,
    pub version: Version,
    pub release_at: DateTime<Utc>,
    pub platforms: HashMap<String, PlatformArtefact>,
    pub corpus_version: u32,
    pub obfuscation_tier: String,
    pub signing_key_id: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlatformArtefact {
    pub archive_url: String,
    pub sha256: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PlatformId {
    LinuxX86_64,
    LinuxAarch64,
    MacosX86_64,
    MacosAarch64,
    WindowsX86_64,
}

impl PlatformId {
    pub const fn current() -> Self {
        if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
            Self::LinuxX86_64
        } else if cfg!(all(target_os = "linux", target_arch = "aarch64")) {
            Self::LinuxAarch64
        } else if cfg!(all(target_os = "macos", target_arch = "x86_64")) {
            Self::MacosX86_64
        } else if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
            Self::MacosAarch64
        } else if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
            Self::WindowsX86_64
        } else {
            panic!("unsupported updater platform")
        }
    }

    pub const fn key(self) -> &'static str {
        match self {
            Self::LinuxX86_64 => "linux-x86_64",
            Self::LinuxAarch64 => "linux-aarch64",
            Self::MacosX86_64 => "macos-x86_64",
            Self::MacosAarch64 => "macos-aarch64",
            Self::WindowsX86_64 => "windows-x86_64",
        }
    }

    pub const fn executable_suffix(self) -> &'static str {
        match self {
            Self::WindowsX86_64 => ".exe",
            Self::LinuxX86_64 | Self::LinuxAarch64 | Self::MacosX86_64 | Self::MacosAarch64 => "",
        }
    }

    pub const fn from_target(os: &str, arch: &str) -> Option<Self> {
        match (os.as_bytes(), arch.as_bytes()) {
            (b"linux", b"x86_64") => Some(Self::LinuxX86_64),
            (b"linux", b"aarch64") => Some(Self::LinuxAarch64),
            (b"macos", b"x86_64") => Some(Self::MacosX86_64),
            (b"macos", b"aarch64") => Some(Self::MacosAarch64),
            (b"windows", b"x86_64") => Some(Self::WindowsX86_64),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TrustedKey {
    pub key_id: String,
    pub verifying_key: VerifyingKey,
    pub valid_from: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UpdateOutcome {
    UpToDate,
    UpgradeAvailable {
        manifest: SignedManifest,
        platform: PlatformId,
    },
}

pub trait Updater {
    fn check(
        &self,
        current: &Version,
        channel: &UpdateChannel,
    ) -> Result<UpdateOutcome, BsuiteCoreError>;
}

#[derive(Clone)]
pub struct SignedManifestUpdater {
    client: Client,
    trust_bundle: Vec<TrustedKey>,
    platform: PlatformId,
}

impl SignedManifestUpdater {
    pub fn new() -> Result<Self, BsuiteCoreError> {
        Self::from_trust_bundle_str(include_str!("../keys/trust-bundle-v1.toml"))
    }

    pub fn from_trust_bundle_str(toml: &str) -> Result<Self, BsuiteCoreError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .map_err(|error| BsuiteCoreError::Update(error.to_string()))?;

        Ok(Self {
            client,
            trust_bundle: parse_trust_bundle(toml)?,
            platform: PlatformId::current(),
        })
    }

    pub fn from_trust_bundle_str_for_platform(
        toml: &str,
        platform: PlatformId,
    ) -> Result<Self, BsuiteCoreError> {
        let mut updater = Self::from_trust_bundle_str(toml)?;
        updater.platform = platform;
        Ok(updater)
    }

    pub fn apply(
        &self,
        outcome: &UpdateOutcome,
        install_dir: &Path,
    ) -> Result<(), BsuiteCoreError> {
        let UpdateOutcome::UpgradeAvailable { manifest, platform } = outcome else {
            return Ok(());
        };

        let artefact = manifest
            .platforms
            .get(platform.key())
            .ok_or_else(|| BsuiteCoreError::ManifestPlatformMissing(platform.key().to_string()))?;
        let archive_bytes = self.fetch_body(
            &artefact.archive_url,
            ARCHIVE_BODY_LIMIT_BYTES,
            FetchFailureKind::Artefact,
        )?;
        verify_sha256(&archive_bytes, &artefact.sha256)?;

        let executable_name = executable_name(&manifest.binary_name, *platform);
        let staging_dir = install_dir.join(staging_dir_name(&manifest.binary_name));
        let backup_path = install_dir.join(format!("{executable_name}.old"));
        let final_path = install_dir.join(&executable_name);
        let new_path = staging_dir.join(&executable_name);

        remove_dir_if_exists(&staging_dir)?;
        fs::create_dir_all(&staging_dir)
            .map_err(|error| BsuiteCoreError::AtomicInstallFailed(error.to_string()))?;

        let install_result = (|| {
            extract_tar_archive(&archive_bytes, &staging_dir)?;
            require_regular_file(&new_path)?;
            sync_tree(&staging_dir)?;
            fs::create_dir_all(install_dir)
                .map_err(|error| BsuiteCoreError::AtomicInstallFailed(error.to_string()))?;

            if backup_path.exists() {
                fs::remove_file(&backup_path)
                    .map_err(|error| BsuiteCoreError::AtomicInstallFailed(error.to_string()))?;
            }
            if final_path.exists() {
                fs::rename(&final_path, &backup_path)
                    .map_err(|error| BsuiteCoreError::AtomicInstallFailed(error.to_string()))?;
            }
            if let Err(error) = fs::rename(&new_path, &final_path) {
                rollback_install(&final_path, &backup_path)?;
                return Err(BsuiteCoreError::AtomicInstallFailed(error.to_string()));
            }
            if backup_path.exists() {
                fs::remove_file(&backup_path)
                    .map_err(|error| BsuiteCoreError::AtomicInstallFailed(error.to_string()))?;
            }
            sync_directory(install_dir)?;
            Ok(())
        })();

        let cleanup_result = remove_dir_if_exists(&staging_dir);
        install_result?;
        cleanup_result
    }

    fn fetch_body(
        &self,
        url: &str,
        limit_bytes: u64,
        failure_kind: FetchFailureKind,
    ) -> Result<Vec<u8>, BsuiteCoreError> {
        let mut response = self
            .client
            .get(url)
            .send()
            .map_err(|error| failure_kind.error(error.to_string()))?;

        if !response.status().is_success() {
            return Err(failure_kind.error(format!("HTTP status {}", response.status())));
        }

        if response
            .content_length()
            .is_some_and(|length| length > limit_bytes)
        {
            return Err(BsuiteCoreError::ResponseBodyTooLarge {
                limit_bytes,
                found_bytes: response.content_length().unwrap_or(limit_bytes + 1),
            });
        }

        let mut limited = response.by_ref().take(limit_bytes + 1);
        let mut bytes = Vec::new();
        limited
            .read_to_end(&mut bytes)
            .map_err(|error| failure_kind.error(error.to_string()))?;
        if bytes.len() as u64 > limit_bytes {
            return Err(BsuiteCoreError::ResponseBodyTooLarge {
                limit_bytes,
                found_bytes: bytes.len() as u64,
            });
        }
        Ok(bytes)
    }
}

impl Updater for SignedManifestUpdater {
    fn check(
        &self,
        current: &Version,
        channel: &UpdateChannel,
    ) -> Result<UpdateOutcome, BsuiteCoreError> {
        let manifest_url = manifest_url(channel);
        let signature_url = signature_url(channel);
        let manifest_bytes = self.fetch_body(
            &manifest_url,
            MANIFEST_BODY_LIMIT_BYTES,
            FetchFailureKind::Manifest,
        )?;
        let signature_bytes = self.fetch_body(
            &signature_url,
            SIGNATURE_BODY_LIMIT_BYTES,
            FetchFailureKind::Signature,
        )?;
        let manifest = parse_manifest(&manifest_bytes)?;

        verify_manifest_schema(manifest.schema_version)?;
        verify_manifest_signature(&manifest, &signature_bytes, &self.trust_bundle)?;
        require_current_platform(&manifest, self.platform)?;

        if manifest.version > *current {
            Ok(UpdateOutcome::UpgradeAvailable {
                manifest,
                platform: self.platform,
            })
        } else {
            Ok(UpdateOutcome::UpToDate)
        }
    }
}

#[derive(Clone, Copy)]
enum FetchFailureKind {
    Manifest,
    Signature,
    Artefact,
}

impl FetchFailureKind {
    fn error(self, message: String) -> BsuiteCoreError {
        match self {
            Self::Manifest => BsuiteCoreError::ManifestFetchFailed(message),
            Self::Signature => BsuiteCoreError::SignatureFetchFailed(message),
            Self::Artefact => BsuiteCoreError::ArtifactFetchFailed(message),
        }
    }
}

#[derive(Deserialize)]
struct TrustBundleFile {
    keys: Vec<TrustedKeyFile>,
}

#[derive(Deserialize)]
struct TrustedKeyFile {
    key_id: String,
    verifying_key_base64: String,
    valid_from: DateTime<Utc>,
    valid_until: DateTime<Utc>,
    revoked_at: Option<DateTime<Utc>>,
}

fn parse_trust_bundle(toml: &str) -> Result<Vec<TrustedKey>, BsuiteCoreError> {
    let file: TrustBundleFile =
        toml::from_str(toml).map_err(|error| BsuiteCoreError::Update(error.to_string()))?;

    file.keys
        .into_iter()
        .map(|key| {
            let key_bytes = base64::engine::general_purpose::STANDARD
                .decode(key.verifying_key_base64)
                .map_err(|error| BsuiteCoreError::Update(error.to_string()))?;
            let key_bytes: [u8; 32] = key_bytes
                .try_into()
                .map_err(|_| BsuiteCoreError::Update("trusted key must be 32 bytes".to_string()))?;
            let verifying_key = VerifyingKey::from_bytes(&key_bytes)
                .map_err(|error| BsuiteCoreError::Update(error.to_string()))?;
            Ok(TrustedKey {
                key_id: key.key_id,
                verifying_key,
                valid_from: key.valid_from,
                valid_until: key.valid_until,
                revoked_at: key.revoked_at,
            })
        })
        .collect()
}

fn manifest_url(channel: &UpdateChannel) -> String {
    format!("{}/manifest.json", channel.as_str().trim_end_matches('/'))
}

fn signature_url(channel: &UpdateChannel) -> String {
    format!(
        "{}/manifest.json.sig",
        channel.as_str().trim_end_matches('/')
    )
}

fn parse_manifest(bytes: &[u8]) -> Result<SignedManifest, BsuiteCoreError> {
    serde_json::from_slice(bytes)
        .map_err(|error| BsuiteCoreError::ManifestFetchFailed(error.to_string()))
}

fn verify_manifest_schema(found: u32) -> Result<(), BsuiteCoreError> {
    if found == MANIFEST_SCHEMA_VERSION {
        Ok(())
    } else {
        Err(BsuiteCoreError::ManifestSchemaMismatch {
            expected: MANIFEST_SCHEMA_VERSION,
            found,
        })
    }
}

fn verify_manifest_signature(
    manifest: &SignedManifest,
    signature_bytes: &[u8],
    trust_bundle: &[TrustedKey],
) -> Result<(), BsuiteCoreError> {
    let trusted_key = trust_bundle
        .iter()
        .find(|key| key.key_id == manifest.signing_key_id)
        .ok_or_else(|| {
            BsuiteCoreError::ManifestUnknownSigningKey(manifest.signing_key_id.clone())
        })?;
    verify_key_time_bounds(trusted_key, Utc::now())?;

    let signature = parse_signature(signature_bytes)?;
    let canonical_manifest = serde_json_canonicalizer::to_vec(manifest)
        .map_err(|error| BsuiteCoreError::ManifestFetchFailed(error.to_string()))?;

    trusted_key
        .verifying_key
        .verify_strict(&canonical_manifest, &signature)
        .map_err(|_| BsuiteCoreError::ManifestSignatureInvalid)
}

fn verify_key_time_bounds(key: &TrustedKey, now: DateTime<Utc>) -> Result<(), BsuiteCoreError> {
    if let Some(revoked_at) = key.revoked_at
        && revoked_at <= now
    {
        return Err(BsuiteCoreError::ManifestSigningKeyRevoked(
            key.key_id.clone(),
        ));
    }
    if key.valid_from > now {
        return Err(BsuiteCoreError::ManifestSigningKeyNotYetValid(
            key.key_id.clone(),
        ));
    }
    if key.valid_until <= now {
        return Err(BsuiteCoreError::ManifestSigningKeyExpired(
            key.key_id.clone(),
        ));
    }
    Ok(())
}

fn parse_signature(signature_bytes: &[u8]) -> Result<Signature, BsuiteCoreError> {
    let value = std::str::from_utf8(signature_bytes)
        .map_err(|_| BsuiteCoreError::ManifestSignatureInvalid)?
        .trim();
    let encoded = value
        .strip_prefix("ed25519:")
        .ok_or(BsuiteCoreError::ManifestSignatureInvalid)?;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|_| BsuiteCoreError::ManifestSignatureInvalid)?;
    Signature::from_slice(&bytes).map_err(|_| BsuiteCoreError::ManifestSignatureInvalid)
}

fn require_current_platform(
    manifest: &SignedManifest,
    platform: PlatformId,
) -> Result<(), BsuiteCoreError> {
    if manifest.platforms.contains_key(platform.key()) {
        Ok(())
    } else {
        Err(BsuiteCoreError::ManifestPlatformMissing(
            platform.key().to_string(),
        ))
    }
}

fn verify_sha256(bytes: &[u8], expected: &str) -> Result<(), BsuiteCoreError> {
    let found = format!("{:x}", Sha256::digest(bytes));
    if found.eq_ignore_ascii_case(expected) {
        Ok(())
    } else {
        Err(BsuiteCoreError::ArtifactSha256Mismatch {
            expected: expected.to_string(),
            found,
        })
    }
}

fn executable_name(binary_name: &str, platform: PlatformId) -> String {
    format!("{}{}", binary_name, platform.executable_suffix())
}

fn staging_dir_name(binary_name: &str) -> String {
    format!(".bsuite-staging-{binary_name}-{}", std::process::id())
}

fn extract_tar_archive(bytes: &[u8], staging_dir: &Path) -> Result<(), BsuiteCoreError> {
    let cursor = std::io::Cursor::new(bytes);
    let mut archive = tar::Archive::new(cursor);
    let entries = archive
        .entries()
        .map_err(|error| BsuiteCoreError::AtomicInstallFailed(error.to_string()))?;

    for entry in entries {
        let mut entry =
            entry.map_err(|error| BsuiteCoreError::AtomicInstallFailed(error.to_string()))?;
        if entry.header().entry_type().is_symlink() || entry.header().entry_type().is_hard_link() {
            return Err(BsuiteCoreError::AtomicInstallFailed(
                "archive links are not allowed".to_string(),
            ));
        }
        let entry_path = entry
            .path()
            .map_err(|error| BsuiteCoreError::AtomicInstallFailed(error.to_string()))?;
        let safe_path = safe_archive_path(&entry_path)?;
        let output_path = staging_dir.join(safe_path);
        entry
            .unpack(&output_path)
            .map_err(|error| BsuiteCoreError::AtomicInstallFailed(error.to_string()))?;
    }
    Ok(())
}

fn safe_archive_path(path: &Path) -> Result<PathBuf, BsuiteCoreError> {
    let mut safe = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(value) => safe.push(value),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(BsuiteCoreError::AtomicInstallFailed(
                    "archive entry escapes staging directory".to_string(),
                ));
            }
        }
    }
    if safe.as_os_str().is_empty() {
        return Err(BsuiteCoreError::AtomicInstallFailed(
            "archive entry path is empty".to_string(),
        ));
    }
    Ok(safe)
}

fn require_regular_file(path: &Path) -> Result<(), BsuiteCoreError> {
    let metadata = fs::metadata(path).map_err(|_| {
        BsuiteCoreError::AtomicInstallFailed("archive is missing expected executable".to_string())
    })?;
    if metadata.is_file() {
        Ok(())
    } else {
        Err(BsuiteCoreError::AtomicInstallFailed(
            "expected executable is not a regular file".to_string(),
        ))
    }
}

fn sync_tree(path: &Path) -> Result<(), BsuiteCoreError> {
    for entry in fs::read_dir(path)
        .map_err(|error| BsuiteCoreError::AtomicInstallFailed(error.to_string()))?
    {
        let entry =
            entry.map_err(|error| BsuiteCoreError::AtomicInstallFailed(error.to_string()))?;
        let path = entry.path();
        let metadata = entry
            .metadata()
            .map_err(|error| BsuiteCoreError::AtomicInstallFailed(error.to_string()))?;
        if metadata.is_dir() {
            sync_tree(&path)?;
        } else if metadata.is_file() {
            File::open(&path)
                .and_then(|file| file.sync_all())
                .map_err(|error| BsuiteCoreError::AtomicInstallFailed(error.to_string()))?;
        }
    }
    sync_directory(path)
}

#[cfg(unix)]
fn sync_directory(path: &Path) -> Result<(), BsuiteCoreError> {
    File::open(path)
        .and_then(|file| file.sync_all())
        .map_err(|error| BsuiteCoreError::AtomicInstallFailed(error.to_string()))
}

#[cfg(not(unix))]
fn sync_directory(_path: &Path) -> Result<(), BsuiteCoreError> {
    Ok(())
}

fn rollback_install(final_path: &Path, backup_path: &Path) -> Result<(), BsuiteCoreError> {
    if final_path.exists() {
        fs::remove_file(final_path)
            .map_err(|error| BsuiteCoreError::AtomicInstallFailed(error.to_string()))?;
    }
    if backup_path.exists() {
        fs::rename(backup_path, final_path)
            .map_err(|error| BsuiteCoreError::InstallRollbackFailed(error.to_string()))?;
    }
    Ok(())
}

fn remove_dir_if_exists(path: &Path) -> Result<(), BsuiteCoreError> {
    if path.exists() {
        fs::remove_dir_all(path)
            .map_err(|error| BsuiteCoreError::AtomicInstallFailed(error.to_string()))?;
    }
    Ok(())
}
