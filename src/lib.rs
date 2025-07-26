#![allow(unused)]
#![feature(intra_doc_pointers)]
#![doc(issue_tracker_base_url = "https://github.com/gabrielfalcao/debug-et-diagnostics/issues/")]
//! set of macros and tools to colorfully debug and diagnose non-trivial code

pub mod color;

#[doc(inline)]
pub use color::{
    ansi, ansi_clear, auto, auto_bright, auto_dark, back, bg, bgfg, bright, bright_rgb_band, byte,
    byte_bin, byte_hex, couple, cube_ansi_256, dark, dark_rgb_band, fg, fore, format_slice_debug,
    format_slice_display, format_slice_hex, from_byte, from_bytes, from_debug, from_display,
    get_ansi_rgb, invert_ansi, invert_bw, invert_rgb, is_bright_rgb_band, is_dark_rgb_band,
    merge_rgb, non_zero_be_bytes, pad, pad_columns, reset, rgb_from_byte, rgb_from_bytes,
    rgb_from_display, rgb_to_byte, term_cols, wrap, STD_COLORS,
};

mod macros;
