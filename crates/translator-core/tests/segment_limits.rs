use translator_core::{
    validate_segments, ErrorCode, TranslatableSegment, MAX_SEGMENTS, MAX_SEGMENT_BYTES,
};

#[test]
fn accepts_segment_at_exact_byte_limit() {
    let text = "a".repeat(MAX_SEGMENT_BYTES);
    let segment = TranslatableSegment::new(0, text).expect("exact limit should pass");

    validate_segments(&[segment]).expect("single segment at limit should pass");
}

#[test]
fn rejects_segment_above_byte_limit() {
    let text = "a".repeat(MAX_SEGMENT_BYTES + 1);
    let err = TranslatableSegment::new(0, text).expect_err("oversized segment should fail");

    assert_eq!(err.code, ErrorCode::FileTooLarge);
}

#[test]
fn rejects_too_many_segments() {
    let mut segments = Vec::new();
    for id in 0..=MAX_SEGMENTS {
        segments.push(TranslatableSegment::new(id, "segment").expect("small segment"));
    }

    let err = validate_segments(&segments).expect_err("too many segments should fail");

    assert_eq!(err.code, ErrorCode::FileTooLarge);
}
