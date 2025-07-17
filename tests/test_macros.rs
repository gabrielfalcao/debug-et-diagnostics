use debug_et_diagnostics::tag;

#[test]
fn test_tag_open() {
    assert_eq!(tag!(@open, "text"), "");
    // assert_eq!(tag!(@open, "text", 220), "");
    // assert_eq!(tag!(@open, "text", @color=auto), "");
    // assert_eq!(tag!(@open, "text", @color=fore), "");
    // assert_eq!(tag!(@open, "text", @color=220), "");
    // assert_eq!(tag!(@open, "text", 220, @color=auto), "");
    // assert_eq!(tag!(@open, "text", 220, @color=fore), "");
    // assert_eq!(tag!(@open, "text", 220, @color=220), "");
}
#[test]
fn test_tag_close() {
    assert_eq!(tag!(@close, "text"), "");
    assert_eq!(tag!(@close, "text", 220), "");
    assert_eq!(tag!(@close, "text", @color=auto), "");
    assert_eq!(tag!(@close, "text", @color=fore), "");
    assert_eq!(tag!(@close, "text", @color=220), "");
    assert_eq!(tag!(@close, "text", 220, @color=auto), "");
    assert_eq!(tag!(@close, "text", 220, @color=fore), "");
    assert_eq!(tag!(@close, "text", 220, @color=220), "");
}
#[test]
fn test_tag_wrap() {
    assert_eq!(tag!(@wrap, "tag", "text"), "");
    // assert_eq!(tag!(@wrap, "tag", "text", 220), "");
    // assert_eq!(tag!(@wrap, "tag", "text", @color=auto), "");
    // assert_eq!(tag!(@wrap, "tag", "text", @color=fore), "");
    // assert_eq!(tag!(@wrap, "tag", "text", @color=220), "");
    // assert_eq!(tag!(@wrap, "tag", "text", 220, @color=auto), "");
    // assert_eq!(tag!(@wrap, "tag", "text", 220, @color=fore), "");
    // assert_eq!(tag!(@wrap, "tag", "text", 220, @color=220), "");
}

// // #[test]
// // fn test_tag_wrap() {
// //     assert_eq!(tag!(@wrap, "tag", "text"), "");
// //     assert_eq!(tag!(@wrap, "tag", "text", 220), "");
// //     assert_eq!(tag!(@wrap, "tag", "text", 220, @color=fore), "");
// //     assert_eq!(tag!(@wrap, "tag", "text", 220, @color=auto), "");
// //     assert_eq!(tag!(@wrap, "tag", "text", 220, @color=220), "");
// // }
