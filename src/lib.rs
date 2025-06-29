#![allow(unused)]
#![feature(intra_doc_pointers)]
#![doc(issue_tracker_base_url = "https://github.com/gabrielfalcao/debug-et-diagnostics/issues/")]
//! set of macros and tools to colorfully debug and diagnose non-trivial code

pub mod color;

#[doc(inline)]
pub use color::{
    ansi, ansi_clear, auto, back, bg, bgfg, couple, fg, fore, from_bytes, from_string, invert_bw,
    pad_columns, reset, rgb_from_bytes, rgb_from_string, term_cols, wrap,
};


mod macros;
