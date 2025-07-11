#![allow(unused)]
#![feature(intra_doc_pointers)]
#![doc(issue_tracker_base_url = "https://github.com/gabrielfalcao/debug-et-diagnostics/issues/")]
//! set of macros and tools to colorfully debug and diagnose non-trivial code

pub mod color;

#[doc(inline)]
pub use color::{
    ansi, ansi_clear, auto, back, bg, bgfg, bright, byte, byte_bin, byte_hex, couple, fg, fore,
    from_byte, from_bytes, from_display, invert_bw, pad, pad_columns, reset, rgb_from_bytes,
    rgb_from_display, term_cols, wrap,
};

mod macros;
