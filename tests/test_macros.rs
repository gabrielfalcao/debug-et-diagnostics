use debug_et_diagnostics::tag;

#[test]
fn test_tag_open() {
    assert_eq!(tag!(@open, "text"), "\u{1b}[1;48;5;0m\u{1b}[1;38;5;254m<\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;254mtext\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;254m>\u{1b}[0m");
    assert_eq!(tag!(@open, "text", 220), "\u{1b}[1;48;5;0m\u{1b}[1;38;5;225m<\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;225mtext\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;225m>\u{1b}[0m");
    assert_eq!(tag!(@open, "text", @color=auto), "\u{1b}[1;48;5;0m\u{1b}[1;38;5;254m<\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;195mtext\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;254m>\u{1b}[0m");
    assert_eq!(tag!(@open, "text", @color=fore), "\u{1b}[1;48;5;0m\u{1b}[1;38;5;254m<\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;254mtext\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;254m>\u{1b}[0m");
    assert_eq!(tag!(@open, "text", @color=220), "\u{1b}[1;48;5;0m\u{1b}[1;38;5;254m<\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;225mtext\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;254m>\u{1b}[0m");
    assert_eq!(tag!(@open, "text", 220, @color=auto), "\u{1b}[1;48;5;0m\u{1b}[1;38;5;225m<\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;195mtext\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;225m>\u{1b}[0m");
    assert_eq!(tag!(@open, "text", 220, @color=fore), "\u{1b}[1;48;5;0m\u{1b}[1;38;5;225m<\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;225mtext\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;225m>\u{1b}[0m");
    assert_eq!(tag!(@open, "text", 220, @color=220), "\u{1b}[1;48;5;0m\u{1b}[1;38;5;225m<\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;225mtext\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;225m>\u{1b}[0m");
}
#[test]
fn test_tag_close() {
    assert_eq!(tag!(@close, "text"), "\u{1b}[1;48;5;0m\u{1b}[1;38;5;254m</\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;254mtext\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;254m>\u{1b}[0m");
    assert_eq!(tag!(@close, "text", 220), "\u{1b}[1;48;5;0m\u{1b}[1;38;5;225m</\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;225mtext\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;225m>\u{1b}[0m");
    assert_eq!(tag!(@close, "text", @color=auto), "\u{1b}[1;48;5;0m\u{1b}[1;38;5;254m</\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;195mtext\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;254m>\u{1b}[0m");
    assert_eq!(tag!(@close, "text", @color=fore), "\u{1b}[1;48;5;0m\u{1b}[1;38;5;254m</\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;254mtext\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;254m>\u{1b}[0m");
    assert_eq!(tag!(@close, "text", @color=220), "\u{1b}[1;48;5;0m\u{1b}[1;38;5;254m</\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;225mtext\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;254m>\u{1b}[0m");
    assert_eq!(tag!(@close, "text", 220, @color=auto), "\u{1b}[1;48;5;0m\u{1b}[1;38;5;225m</\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;195mtext\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;225m>\u{1b}[0m");
    assert_eq!(tag!(@close, "text", 220, @color=fore), "\u{1b}[1;48;5;0m\u{1b}[1;38;5;225m</\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;225mtext\u{1b}[0m\u{1b}[1;48;5;0m\u{1b}[1;38;5;225m>\u{1b}[0m");
    // assert_eq!(tag!(@close, "text", 220, @color=220), "");
}
// #[test]
// fn test_tag_wrap() {
//     assert_eq!(tag!(@wrap, "tag", "text"), "");
//     // assert_eq!(tag!(@wrap, "tag", "text", 220), "");
//     // assert_eq!(tag!(@wrap, "tag", "text", @color=auto), "");
//     // assert_eq!(tag!(@wrap, "tag", "text", @color=fore), "");
//     // assert_eq!(tag!(@wrap, "tag", "text", @color=220), "");
//     // assert_eq!(tag!(@wrap, "tag", "text", 220, @color=auto), "");
//     // assert_eq!(tag!(@wrap, "tag", "text", 220, @color=fore), "");
//     // assert_eq!(tag!(@wrap, "tag", "text", 220, @color=220), "");
// }

// // #[test]
// // fn test_tag_wrap() {
// //     assert_eq!(tag!(@wrap, "tag", "text"), "");
// //     assert_eq!(tag!(@wrap, "tag", "text", 220), "");
// //     assert_eq!(tag!(@wrap, "tag", "text", 220, @color=fore), "");
// //     assert_eq!(tag!(@wrap, "tag", "text", 220, @color=auto), "");
// //     assert_eq!(tag!(@wrap, "tag", "text", 220, @color=220), "");
// // }
