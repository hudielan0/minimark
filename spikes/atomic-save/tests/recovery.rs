use minimark_atomic_save_spike::{RecoveryState, fingerprint, load_recovery, write_recovery};
use std::fs;

#[test]
fn offers_unfinished_text_after_a_restart() {
    let directory = tempfile::tempdir().expect("temporary directory");
    let document = directory.path().join("note.md");
    let recovery_directory = directory.path().join("recovery");
    fs::write(&document, "saved text\n").expect("seed document");
    let opened_version = fingerprint(&document).expect("fingerprint opened document");

    write_recovery(
        &recovery_directory,
        &document,
        &opened_version,
        "unfinished 中文 draft\n",
    )
    .expect("write recovery record");

    let offer = load_recovery(&recovery_directory, &document)
        .expect("read recovery record")
        .expect("newer draft should be offered");
    assert_eq!(offer.draft, "unfinished 中文 draft\n");
    assert_eq!(offer.disk_state, RecoveryState::DiskUnchanged);
    assert_eq!(
        fs::read_to_string(&document).expect("disk document remains available"),
        "saved text\n"
    );
}

#[test]
fn preserves_both_versions_when_disk_changed_after_the_draft() {
    let directory = tempfile::tempdir().expect("temporary directory");
    let document = directory.path().join("note.md");
    let recovery_directory = directory.path().join("recovery");
    fs::write(&document, "saved text\n").expect("seed document");
    let opened_version = fingerprint(&document).expect("fingerprint opened document");
    write_recovery(
        &recovery_directory,
        &document,
        &opened_version,
        "unfinished draft\n",
    )
    .expect("write recovery record");
    fs::write(&document, "changed elsewhere\n").expect("external edit");

    let offer = load_recovery(&recovery_directory, &document)
        .expect("read recovery record")
        .expect("conflicting draft should be offered");

    assert_eq!(offer.draft, "unfinished draft\n");
    assert_eq!(offer.disk_state, RecoveryState::DiskChanged);
    assert_eq!(
        fs::read_to_string(&document).expect("external version remains available"),
        "changed elsewhere\n"
    );
}
