mod common;

use std::io::Cursor;

use translator_provider_manager::artifact::expand_zstandard;

#[test]
fn zstandard_expansion_requires_exact_installed_size_and_hash() {
    let artifacts = common::fixture_artifacts();
    let artifact = &artifacts[0];

    let expanded = expand_zstandard(
        Cursor::new(&artifact.compressed),
        artifact.installed.len(),
        &common::sha256(&artifact.installed),
    )
    .expect("verified expansion");

    assert_eq!(expanded, artifact.installed);
}

#[test]
fn expansion_rejects_corrupt_oversized_or_wrong_identity_output() {
    let artifacts = common::fixture_artifacts();
    let artifact = &artifacts[0];

    assert!(expand_zstandard(
        Cursor::new(b"not zstandard"),
        artifact.installed.len(),
        &common::sha256(&artifact.installed),
    )
    .is_err());
    assert!(expand_zstandard(
        Cursor::new(&artifact.compressed),
        artifact.installed.len() - 1,
        &common::sha256(&artifact.installed),
    )
    .is_err());
    assert!(expand_zstandard(
        Cursor::new(&artifact.compressed),
        artifact.installed.len(),
        &"f".repeat(64),
    )
    .is_err());
}
