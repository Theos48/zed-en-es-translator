use std::io::{self, Cursor, Read};

use translator_provider_manager::acquisition::AcquisitionPolicy;

#[test]
fn acquisition_requires_exact_https_size_and_hash() {
    let bytes = b"controlled compressed artifact";
    let digest = sha256(bytes);
    let policy =
        AcquisitionPolicy::new("https://example.invalid/artifact.zst", bytes.len(), &digest)
            .expect("exact policy");

    let acquired = policy
        .verify_reader(Cursor::new(bytes), Some(bytes.len()))
        .expect("verified acquisition");

    assert_eq!(acquired, bytes);
}

#[test]
fn acquisition_rejects_non_https_unknown_length_truncation_oversize_and_corruption() {
    let bytes = b"controlled compressed artifact";
    let digest = sha256(bytes);
    assert!(AcquisitionPolicy::new("http://example.invalid/a", bytes.len(), &digest).is_err());

    let policy =
        AcquisitionPolicy::new("https://example.invalid/a", bytes.len(), &digest).expect("policy");
    let transport = policy.transport_policy();
    assert!(transport.https_only());
    assert_eq!(transport.expected_status(), 200);
    assert_eq!(transport.max_redirects(), 0);
    assert!(!transport.inherited_proxy());
    assert_eq!(transport.retries(), 0);
    assert!(policy.verify_reader(Cursor::new(bytes), None).is_err());
    assert!(policy
        .verify_reader(Cursor::new(&bytes[..bytes.len() - 1]), Some(bytes.len()))
        .is_err());
    assert!(policy
        .verify_reader(
            Cursor::new([bytes.as_slice(), b"x"].concat()),
            Some(bytes.len() + 1)
        )
        .is_err());
    assert!(policy
        .verify_reader(
            Cursor::new(b"different bytes"),
            Some(b"different bytes".len())
        )
        .is_err());
    let error = policy
        .verify_reader(CancelledReader, Some(bytes.len()))
        .expect_err("cancelled read must fail without a partial result");
    assert_eq!(error.code(), "ACQUISITION_FAILED");
}

struct CancelledReader;

impl Read for CancelledReader {
    fn read(&mut self, _buffer: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::other("controlled cancellation"))
    }
}

fn sha256(content: &[u8]) -> String {
    use sha2::{Digest as _, Sha256};
    Sha256::digest(content)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
