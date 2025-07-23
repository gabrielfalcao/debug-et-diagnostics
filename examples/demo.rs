use debug_et_diagnostics::{from_debug, from_display, step, step_dbg};

use std::env::args;

fn main() {
    let args = get_argv();
    let args = if args.is_empty() {
        vec![format!("<missing argument>")]
    } else {
        args
    };
    for arg in args
        .clone()
        .into_iter()
        .enumerate()
        .filter(|(i, _)| *i > 0)
        .map(|(_, c)| String::from(c))
    {
        let fg = from_display(&arg) as usize;
        step!(fg = fg, format!("{arg}"));
    }
    let fg = from_debug(&args) as usize;
    step_dbg!(fg = fg, args);
}

fn get_argv() -> Vec<String> {
    std::env::args().into_iter().collect::<Vec<String>>()
}
