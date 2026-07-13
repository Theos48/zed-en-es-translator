use lsp_types::{Position, Range};
use translator_lsp::selection::{position_to_offset, range_to_offsets};

#[test]
fn converts_ascii_and_multibyte_utf16_positions() {
    let text = "aé中\nnext";

    assert_eq!(position_to_offset(text, Position::new(0, 0)), Ok(0));
    assert_eq!(position_to_offset(text, Position::new(0, 1)), Ok(1));
    assert_eq!(position_to_offset(text, Position::new(0, 2)), Ok(3));
    assert_eq!(position_to_offset(text, Position::new(0, 3)), Ok(6));
    assert_eq!(position_to_offset(text, Position::new(1, 2)), Ok(9));
}

#[test]
fn converts_surrogate_pairs_and_rejects_split_surrogates() {
    let text = "A😀B";

    assert_eq!(position_to_offset(text, Position::new(0, 1)), Ok(1));
    assert!(position_to_offset(text, Position::new(0, 2)).is_err());
    assert_eq!(position_to_offset(text, Position::new(0, 3)), Ok(5));
}

#[test]
fn treats_crlf_as_a_line_terminator() {
    let text = "a\r\nb";

    assert_eq!(position_to_offset(text, Position::new(0, 1)), Ok(1));
    assert_eq!(position_to_offset(text, Position::new(1, 0)), Ok(3));
    assert!(position_to_offset(text, Position::new(0, 2)).is_err());
}

#[test]
fn rejects_missing_lines_and_out_of_range_characters() {
    let text = "one\ntwo";

    assert!(position_to_offset(text, Position::new(2, 0)).is_err());
    assert!(position_to_offset(text, Position::new(0, 4)).is_err());
}

#[test]
fn converts_checked_ranges_and_rejects_reverse_ranges() {
    let text = "A😀B";
    let valid = Range::new(Position::new(0, 1), Position::new(0, 3));
    let reversed = Range::new(Position::new(0, 3), Position::new(0, 1));

    assert_eq!(range_to_offsets(text, valid), Ok(1..5));
    assert!(range_to_offsets(text, reversed).is_err());
}
