use crate::manifest_overlay::error::OverlayValidationError;

pub const ALLOWED_OVERRIDE_KEYS: &[&str] = &[
    "transcript_retention_days",
    "transcript_dir",
    "corpus_dir",
    "update_check_interval_minutes",
    "stdout_byte_cap",
    "binary_timeout_ms",
];

pub fn validate_override_keys(toml_value: &toml::Value) -> Result<(), OverlayValidationError> {
    let Some(overrides) = toml_value.get("overrides") else {
        return Ok(());
    };
    let toml::Value::Table(table) = overrides else {
        return Err(OverlayValidationError::TomlParseFailed(
            "[overrides] must be a table".into(),
        ));
    };
    for key in table.keys() {
        if !ALLOWED_OVERRIDE_KEYS.contains(&key.as_str()) {
            return Err(OverlayValidationError::UnknownKey { key: key.clone() });
        }
    }
    Ok(())
}
