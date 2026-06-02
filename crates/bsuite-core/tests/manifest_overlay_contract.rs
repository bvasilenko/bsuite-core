use bsuite_core::{ManifestOverlay, ManifestOverlayReader, OverlayMap, OverlayValidationError};
use std::collections::BTreeMap;

struct PendingManifestOverlayReader;

impl ManifestOverlayReader for PendingManifestOverlayReader {
    fn read_overlay(&self) -> Result<Option<ManifestOverlay>, OverlayValidationError> {
        unimplemented!("not yet implemented")
    }
}

#[test]
#[should_panic(expected = "not yet implemented")]
fn placeholder_manifest_overlay_reader_is_explicitly_pending() {
    let reader = PendingManifestOverlayReader;

    let _ = reader.read_overlay();
}

#[test]
fn overlay_map_rejects_empty_keys() {
    let entries = BTreeMap::from([(String::from(""), String::from("value"))]);

    assert_eq!(
        OverlayMap::new(entries),
        Err(OverlayValidationError::EmptyKey)
    );
}

#[test]
fn overlay_map_preserves_entries_in_key_order() {
    let entries = BTreeMap::from([
        (String::from("beta"), String::from("2")),
        (String::from("alpha"), String::from("1")),
    ]);

    let overlay = OverlayMap::new(entries).expect("valid overlay entries");
    let pairs = overlay.iter().collect::<Vec<_>>();

    assert_eq!(overlay.get("alpha"), Some("1"));
    assert_eq!(pairs, vec![("alpha", "1"), ("beta", "2")]);
}

#[test]
fn empty_overlay_map_contains_no_entries() {
    let overlay = OverlayMap::empty();

    assert_eq!(
        overlay.iter().collect::<Vec<_>>(),
        Vec::<(&str, &str)>::new()
    );
    assert_eq!(overlay.get("missing"), None);
}

#[test]
fn overlay_map_into_inner_returns_owned_entries() {
    let entries = BTreeMap::from([(String::from("alpha"), String::from("1"))]);
    let overlay = OverlayMap::new(entries.clone()).expect("valid overlay entries");

    assert_eq!(overlay.into_inner(), entries);
}

#[test]
fn manifest_overlay_wraps_overlay_entries() {
    let entries = BTreeMap::from([(String::from("alpha"), String::from("1"))]);
    let overlay_map = OverlayMap::new(entries).expect("valid overlay entries");
    let manifest_overlay = ManifestOverlay::new(overlay_map);

    assert_eq!(manifest_overlay.entries.get("alpha"), Some("1"));
}
