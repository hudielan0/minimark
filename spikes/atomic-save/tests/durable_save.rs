use minimark_atomic_save_spike::{SaveError, fingerprint, save_if_unchanged};
use std::fs;

#[test]
fn saves_new_text_only_when_disk_version_matches() {
    let directory = tempfile::tempdir().expect("temporary directory");
    let document = directory.path().join("note.md");
    fs::write(&document, "original\n").expect("seed document");
    let opened_version = fingerprint(&document).expect("fingerprint opened document");

    save_if_unchanged(&document, &opened_version, "edited 中文\n")
        .expect("matching version should save");

    assert_eq!(
        fs::read_to_string(&document).expect("read saved document"),
        "edited 中文\n"
    );
}

#[test]
fn refuses_to_overwrite_a_newer_disk_version() {
    let directory = tempfile::tempdir().expect("temporary directory");
    let document = directory.path().join("note.md");
    fs::write(&document, "original\n").expect("seed document");
    let opened_version = fingerprint(&document).expect("fingerprint opened document");
    fs::write(&document, "changed elsewhere\n").expect("external edit");

    let result = save_if_unchanged(&document, &opened_version, "my draft\n");

    assert!(matches!(result, Err(SaveError::ExternalChange)));
    assert_eq!(
        fs::read_to_string(&document).expect("read preserved document"),
        "changed elsewhere\n"
    );
}
