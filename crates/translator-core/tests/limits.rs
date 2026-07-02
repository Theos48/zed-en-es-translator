use translator_core::limits::{
    MAX_INPUT_BYTES, MAX_OUTPUT_BYTES, MAX_SEGMENTS, MAX_SEGMENT_BYTES, PROVIDER_TIMEOUT_MS,
};

#[test]
fn exposes_constitutional_limits() {
    assert_eq!(MAX_INPUT_BYTES, 20 * 1024);
    assert_eq!(MAX_SEGMENT_BYTES, 4 * 1024);
    assert_eq!(MAX_SEGMENTS, 256);
    assert_eq!(MAX_OUTPUT_BYTES, 40 * 1024);
    assert_eq!(PROVIDER_TIMEOUT_MS, 15_000);
}
