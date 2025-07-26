/// colofully prints the "location" of the macro call (function name, filename and line number) in the code
#[macro_export]
macro_rules! location {
    () => {{
        let location = format!(
            "{}{}{}:{}",
            $crate::color::auto($crate::function_name!()),
            $crate::color::ansi(" @ ", 220, 16),
            $crate::filename!(),
            $crate::color::auto(line!().to_string())
        );
        location
    }};
    (begin) => {
        $crate::tag!([
            $crate::color::auto(format!("in function")),
            $crate::location!()
        ]
        .join(" "))
    };
    (end) => {
        $crate::tag!([
            $crate::color::auto(format!("from function")),
            $crate::location!()
        ]
        .join(" "))
    };
    (unexpected) => {
        [
            $crate::color::fore(format!("<unexpected branch in function"), 160),
            $crate::location!(),
            $crate::color::fore(format!(">"), 160),
        ]
        .join(" ")
    };
}
/// colofully prints the filename of the macro call
#[macro_export]
macro_rules! filename {
    () => {{
        let mut parts = file!()
            .split(std::path::MAIN_SEPARATOR_STR)
            .map(String::from)
            .map(|part| $crate::color::auto(part.to_string()))
            .collect::<Vec<String>>();
        let (folder, filename) = if parts.len() > 1 {
            let last = parts.remove(parts.len() - 1);
            let parts = parts.iter().map(Clone::clone).collect::<Vec<String>>();
            (parts, last)
        } else {
            (Vec::<String>::new(), parts[0].to_string())
        };
        if folder.len() > 1 {
            format!(
                "{}{}{}",
                filename,
                $crate::color::fore(" in ", 7),
                folder.join(std::path::MAIN_SEPARATOR_STR)
            )
        } else {
            filename
        }
    }};
}
/// colorfully wraps the given text in "<", ">": "<{text}>"
#[macro_export]
macro_rules! tag {

    (@open, $arg:expr) => {{
        $crate::tag!(@open, $arg, 7, @color=fore)
    }};
    (@open, $arg:expr, @color=auto) => {{
        $crate::tag!(@open, $arg, 7, @color=auto)
    }};
    (@open, $arg:expr, @color=fore) => {{
        $crate::tag!(@open, $arg, 7, @color=fore)
    }};
    (@open, $arg:expr, @color=$color:expr) => {{
        $crate::tag!(@open, $arg, 7, @color=$color)
    }};

    (@open, $arg:expr, $color:expr) => {{
        $crate::tag!(@open, $arg, $color, @color=$color)
    }};
    (@open, $arg:expr, $color:expr, @color=auto) => {{
        let auto_color = $crate::color::from_display($arg) as usize;
        format!(
            "{}{}{}",
            $crate::color::fore("<", $color),
            $crate::color::fore($arg, auto_color),
            $crate::color::fore(">", $color),
        )
    }};
    (@open, $arg:expr, $color:expr, @color=fore) => {{
        format!(
            "{}{}{}",
            $crate::color::fore("<", $color),
            $crate::color::fore($arg, $color),
            $crate::color::fore(">", $color),
        )
    }};
    (@open, $arg:expr, $color:expr, @color=$fore:expr) => {{
        format!(
            "{}{}{}",
            $crate::color::fore("<", $color),
            $crate::color::fore($arg, $fore),
            $crate::color::fore(">", $color),
        )
    }};

    (@close, $arg:expr) => {{
        $crate::tag!(@close, $arg, 7, @color=fore)
    }};
    (@close, $arg:expr, @color=auto) => {{
        $crate::tag!(@close, $arg, 7, @color=auto)
    }};
    (@close, $arg:expr, @color=fore) => {{
        $crate::tag!(@close, $arg, 7, @color=fore)
    }};
    (@close, $arg:expr, @color=$color:expr) => {{
        $crate::tag!(@close, $arg, 7, @color=$color)
    }};

    (@close, $arg:expr, $color:expr) => {{
        $crate::tag!(@close, $arg, $color, @color=$color)
    }};
    (@close, $arg:expr, $color:expr, @color=auto) => {{
        let auto_color = $crate::color::from_display($arg) as usize;
        format!(
            "{}{}{}",
            $crate::color::fore("</", $color),
            $crate::color::fore($arg, auto_color),
            $crate::color::fore(">", $color),
        )
    }};
    (@close, $arg:expr, $color:expr, @color=fore) => {{
        format!(
            "{}{}{}",
            $crate::color::fore("</", $color),
            $crate::color::fore($arg, $color),
            $crate::color::fore(">", $color),
        )
    }};
    (@close, $arg:expr, $color:expr, @color=$fore:expr) => {{
        format!(
            "{}{}{}",
            $crate::color::fore("</", $color),
            $crate::color::fore($arg, $fore),
            $crate::color::fore(">", $color),
        )
    }};


    (@wrap, $tag:expr, $arg:expr) => {{
        [
            $crate::tag!(@open, $tag),
            $crate::indent!($crate::color::fore($arg, 7)),
            $crate::tag!(@close, $tag),
        ].join("\n").to_string()
    }};
    (@wrap, $tag:expr, $arg:expr, @color=auto) => {{
        [
            $crate::tag!(@open, $tag, 7, @color=auto),
            $crate::indent!($crate::color::auto($arg)),
            $crate::tag!(@close, $tag, 7, @color=auto),
        ].join("\n").to_string()

    }};
    (@wrap, $tag:expr, $arg:expr, @color=fore) => {{
        [
            $crate::tag!(@open, $tag, 7, @color=fore),
            $crate::indent!($crate::color::fore($arg, 7)),
            $crate::tag!(@close, $tag, 7, @color=fore),

        ].join("\n").to_string()
    }};
    (@wrap, $tag:expr, $arg:expr, @color=$color:expr) => {{
        [
            $crate::tag!(@open, $arg, $color, @color=$color),
            $crate::indent!($crate::color::fore($arg, $color)),
            $crate::tag!(@close, $arg, $color, @color=$color),
        ].join("\n").to_string()
    }};

    (@wrap, $tag:expr, $arg:expr, $color:expr) => {{
        [
            $crate::tag!(@open, $tag, $color, @color=$color),
            $crate::indent!($crate::color::fore($arg, $color)),
            $crate::tag!(@close, $tag, $color, @color=$color),
        ].join("\n").to_string()
    }};
    (@wrap, $tag:expr, $arg:expr, $color:expr, @color=auto) => {{
        let auto_tag_color = $crate::color::from_display($tag) as usize;
        let auto_arg_color = $crate::color::from_display($arg) as usize;
        [
            $crate::tag!(@open, $tag, $color, @color=auto_tag_color),
            $crate::indent!($crate::color::fore($arg, auto_arg_color)),
            $crate::tag!(@close, $tag, $color, @color=auto_tag_color),
        ].join("\n").to_string()
    }};
    (@wrap, $tag:expr, $arg:expr, $color:expr, @color=fore) => {{
        [
            $crate::tag!(@open, $tag, $color, @color=fore),
            $crate::indent!($crate::color::fore($arg, $color)),
            $crate::tag!(@close, $tag, $color, @color=fore),
        ].join("\n").to_string()
    }};
    (@wrap, $tag:expr, $arg:expr, $color:expr, @color=$fore:expr) => {{
        [
            $crate::tag!(@open, $tag, $color, @color=$fore),
            $crate::indent!($crate::color::fore($arg, $color)),
            $crate::tag!(@close, $tag, $color, @color=$fore),
        ].join("\n").to_string()
    }};

    // (@wrap, $tag:expr, $arg:expr) => {{
    //     [
    //         $crate::tag!(@open, $arg, 7, @color=fore)
    //     ].join("")
    // }};
    // (@wrap, $tag:expr, $arg:expr, $color:expr) => {{
    //     $crate::tag!(@wrap, $arg, $color, @color=$color)
    // }};
    // (@wrap, $tag:expr, $arg:expr, @color=auto) => {{
    //     $crate::tag!(@wrap, $arg, 7, @color=auto)
    // }};
    // (@wrap, $tag:expr, $arg:expr, $color:expr, @color=auto) => {{
    //     $crate::tag!(@wrap, $arg, $color, @color=auto)
    // }};
    // (@wrap, $tag:expr, $arg:expr, $color:expr, @color=auto) => {{
    //     let auto_color = $crate::color::from_display($arg);
    //     format!(
    //         "{}{}{}",
    //         $crate::color::fore("<", $color),
    //         $crate::color::fore($arg, auto_color),
    //         $crate::color::fore(">", $color),
    //     )
    // }};
    // (@wrap, $tag:expr, $arg:expr, $color:expr, @color=fore) => {{
    //     format!(
    //         "{}{}{}",
    //         $crate::color::fore("<", $color),
    //         $crate::color::fore($arg, $color),
    //         $crate::color::fore(">", $color),
    //     )
    // }};
    // (@wrap, $tag:expr, $arg:expr, $color:expr, @color=$fore:expr) => {{
    //     format!(
    //         "{}{}{}",
    //         $crate::color::fore("<", $color),
    //         $crate::color::fore($arg, $fore),
    //         $crate::color::fore(">", $color),
    //     )
    // }};





    ($arg:expr) => {{
        $crate::tag!(@open, $arg, 7)
    }};
    ($arg:expr, $color:expr) => {{
        $crate::tag!(@open, $arg, $color)
    }};
    ($arg:expr, @color=auto) => {{
        $crate::tag!(@open, $arg, 7, @color=auto)
    }};
    ($arg:expr, @color=fore) => {{
        $crate::tag!(@open, $arg, 7, @color=fore)
    }};
    ($arg:expr, @color=$fore:expr) => {{
        $crate::tag!(@open, $arg, 7, @color=$fore)
    }};
    ($arg:expr, $color:expr, @color=$fore:expr) => {{
        $crate::tag!(@open, $arg, $color, @color=$fore)
    }};

}

/// colorful alternative to [std::dbg]
#[macro_export]
macro_rules! dbg {
    ($arg:expr $(,)? ) => {{
        eprintln!("{}", $crate::format_dbg_location!($arg));
        $arg
    }};
    ($( $arg:expr ),* $(,)? ) => {{
        eprintln!("{}", $crate::format_dbg_location!($($arg),*));
    }};
}

#[macro_export]
macro_rules! format_dbg {
    ($arg:expr $(,)? ) => {{
        $crate::indent!(
                format!(
                    "{} = {}\n",
                    $crate::color::auto(stringify!(&$arg)),
                    $crate::color::auto(format!("{:#?}", &$arg))))

    }};
    ($( $arg:expr ),* $(,)? ) => {{
        [$($crate::format_dbg!($arg)),*].join("\n")
    }};
}
#[macro_export]
macro_rules! format_dbg_location {
    ($arg:expr $(,)? ) => {{
        format!("{}", $crate::color::reset([$crate::location!(begin), $crate::format_dbg!($arg), $crate::location!(end)].join("\n")))
    }};
    ($( $arg:expr ),* $(,)? ) => {{
        [$crate::location!(begin), $($crate::format_dbg!($arg)),*, $crate::location!(end)].join("\n")
    }};
}

/// indents an implementor of [std::fmt::Display]
#[macro_export]
macro_rules! indent {
    ($indentation:literal, $obj:expr) => {{
        format!("{}", $obj)
            .lines()
            .map(|line| format!("{}{}", " ".repeat($indentation), line))
            .collect::<Vec<String>>()
            .join("\n")
    }};
    ($obj:expr) => {{
        $crate::indent!(4, $obj)
    }};
}
/// indents an implementor of [std::fmt::Debug]
#[macro_export]
macro_rules! indent_objdump {
    ($indentation:literal, $obj:expr) => {{
        format!("{:#?}", $obj)
            .lines()
            .map(|line| format!("{}{}", " ".repeat($indentation), line))
            .collect::<Vec<String>>()
            .join("\n")
    }};
    ($obj:expr) => {{
        $crate::indent_objdump!(4, $obj)
    }};
}

/// returns a [String] with the name of the function calling the macro
#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        let name = name
            .strip_suffix("::f")
            .unwrap()
            .replace(format!("{}::", module_path!()).as_str(), "");
        name
    }};
}

/// colorfully steps through code
#[macro_export]
macro_rules! step {
    ($text:expr $(,)?) => {{
        $crate::step!(length=$crate::color::term_cols(), $text)
    }};
    (fg=$fg:expr, $text:expr $(,)?) => {{
        let fg=$crate::color::wrap($fg as usize) as usize;
        $crate::step!(bg=fg, fg=$crate::color::invert_ansi(fg), length=$crate::color::term_cols(), $text)
    }};
    (bg=$bg:expr, fg=$fg:expr, $text:expr $(,)?) => {{
        let bg=$crate::color::wrap($bg as usize) as usize;
        let fg=$crate::color::wrap($fg as usize) as usize;
        $crate::step!(bg=bg, fg=fg, length=$crate::color::term_cols(), $text)
    }};
    (length=$length:expr, $text:expr $(,)?) => {{
        let (bg, fg) = $crate::color::couple(line!() as usize);
        $crate::step!(bg=bg, fg=fg, length=$length, $text)
    }};
    (bg=$bg:expr, fg=$fg:expr, length=$length:expr, $text:expr $(,)?) => {{
        let bg = $crate::color::wrap($bg as usize) as usize;
        let fg = $crate::color::wrap($fg as usize) as usize;

        let text = $text.to_string();
        let bar = $crate::color::ansi(
            " ".repeat($length),
            $fg as usize,
            $bg as usize,
        );
        eprintln!(
            "\n{}",
            [
                bar.clone(),
                $crate::color::ansi(
                    $crate::color::pad_columns(
                        [
                            $crate::function_name!(),
                            [
                                file!().to_string(),
                                line!().to_string(),
                            ].join(":")
                        ].join(" ").to_string()
                    ),
                    $fg as usize,
                    $bg as usize,
                ),
                $crate::color::ansi(
                    $crate::color::pad_columns(
                        if text.is_empty() { String::new() } else { format!("{}", text) }
                    ),
                    $bg as usize,
                    $fg as usize,
                ),
                bar.clone(),
            ].join("\n")
        );
    }};
    (length=$length:expr, $text:expr, $( $arg:expr ),* $(,)? ) => {{
        $crate::step!(length=$length, format_args!($text, $($arg,)*))
    }};
    () => {{
        $crate::step!("")
    }};
}
/// colorfully steps through code debugging given expressions
#[macro_export]
macro_rules! step_dbg {
    (bg=$bg:expr, fg=$fg:expr, length=$length:expr, $($arg:expr),* $(,)?) => {{
        let bg=$crate::color::wrap($bg as usize);
        let fg=$crate::color::wrap($fg as usize);
        let text = format!("{}{}", $crate::reset(""), [
            $($crate::indent!(format!("{} = {}", $crate::color::auto(stringify!($arg)), $crate::color::auto(format!("{:#?}", $arg))))),*
        ].join("\n"));
        $crate::step!(bg=bg, fg=fg, length=$length, text);
    }};
    (bg=$bg:expr, fg=$fg:expr, $($arg:expr),* $(,)?) => {{
        let bg=$crate::color::wrap($bg as usize);
        let fg=$crate::color::wrap($fg as usize);
        $crate::step_dbg!(bg=bg, fg=fg, length=$crate::color::term_cols(), $($arg),*)
    }};
    (fg=$fg:expr, $($arg:expr),* $(,)?) => {{
        let fg=$crate::color::wrap($fg as usize) as usize;
        $crate::step_dbg!(bg=fg, fg=$crate::color::invert_ansi(fg), length=$crate::color::term_cols(), $($arg),*)
    }};
    ($($arg:expr),* $(,)?) => {{
        let fg=$crate::color::wrap(line!() as usize) as usize;
        $crate::step_dbg!(bg=fg, fg=$crate::color::invert_ansi(fg), length=$crate::color::term_cols(), $($arg),*)
    }};
    () => {{
        $crate::step!("")
    }};
}

/// colorfully prints an admonition
#[macro_export]
macro_rules! admonition {
    ($color:literal, $message:expr) => {
        $crate::admonition!($color, "{}", $message);
    };
    ($color:literal, $title:literal, $message:expr) => {
        $crate::admonition!($color, title=$title, $message);
    };

    ($color:literal, title=$title:literal, $message:expr) => {
        $crate::admonition!($color, title=$title, "{}", $message);
    };
    ($color:literal, title=$title:literal, $format:literal, $($arg:expr),* $(,)?) => {{
        use crate::color;
        eprintln!(
            "\n{}",
            [
                color::ansi(
                    format!("{}:{} {}", crate::function_name!(), line!(), $title),
                    color::invert_ansi($color).into(),
                    $color,
                ),
                color::ansi(
                    format!($format, $($arg),*),
                    $color,
                    color::invert_ansi($color).into(),
                )
            ]
            .join(" ")
        );
    }};
    ($color:literal, $format:literal, $($arg:expr),* $(,)?) => {{
        use crate::color;
        eprintln!(
            "\n{}",
            [
                color::ansi(
                    format!("{}:{}", crate::function_name!(), line!()),
                    color::invert_ansi($color).into(),
                    $color,
                ),
                color::ansi(
                    format!($format, $($arg),*),
                    $color,
                    color::invert_ansi($color).into(),
                )
            ]
            .join(" ")
        );
    }};
}

/// colorfully prints a "WARN" admonition
#[macro_export]
macro_rules! warn {
    ($color:literal, $format:literal, $($arg:expr),* $(,)?) => {
        $crate::admonition!($color, title="WARNING", $format, $($arg),*);
    };
    ($color:literal, $message:expr) => {
        $crate::admonition!($color, title="WARNING", $message);
    };
    ($message:expr) => {
        $crate::warn!(220, $message);
    };
}

/// colorfully prints an "INFO" admonition
#[macro_export]
macro_rules! info {
    ($color:literal, $format:literal, $($arg:expr),* $(,)?) => {
        $crate::admonition!($color, title="INFO", $format, $($arg),*);
    };
    ($color:literal, $message:expr) => {
        $crate::admonition!($color, title="INFO", $message);
    };
    ($message:expr) => {
        $crate::info!(74, $message);
    };
}

/// colorfully formats a [u8] as hex => binary => decimal (=> char (if ascii))
#[macro_export]
macro_rules! format_byte {
    (hex_only, $byte:expr $(,)? ) => {{
        use $crate::color::{auto, fore, from_byte, pad};
        let color = $crate::color::from_byte($byte);
        $crate::color::fore(format!("0x{:02x}", $byte), color.into())
    }};
    (hex, $byte:expr $(,)? ) => {{
        use $crate::color::{auto, fore, from_bytes, pad};
        let color = $crate::color::from_bytes(&[$byte]);
        [
            $crate::color::fore(format!("0x{:02x}", $byte), color.into()),
            if $byte < 127 {
                $crate::color::fore(
                    format!("{:#?}", char::from($byte).to_string()),
                    color.into(),
                )
            } else {
                String::new()
            },
        ]
        .iter()
        .filter(|c| !c.is_empty())
        .map(String::from)
        .collect::<Vec<String>>()
        .join(" => ")
    }};
    (bin, $byte:expr $(,)? ) => {{
        use $crate::color::{auto, fore, from_bytes, pad};
        let color = $crate::color::from_bytes(&[$byte]);
        [
            $crate::color::fore(format!("0b{:08b}", $byte), color.into()),
            if $byte < 127 {
                $crate::color::fore(
                    format!("{:#?}", char::from($byte).to_string()),
                    color.into(),
                )
            } else {
                String::new()
            },
        ]
        .iter()
        .filter(|c| !c.is_empty())
        .map(String::from)
        .collect::<Vec<String>>()
        .join(" => ")
    }};
    ($byte:expr $(,)? ) => {{
        use $crate::color::{auto, fore, from_bytes, pad};
        let color = $crate::color::from_bytes(&[$byte]);
        [
            $crate::color::fore(format!("0x{:02x}", $byte), color.into()),
            $crate::color::fore(format!("0b{:08b}", $byte), color.into()),
            $crate::color::fore(format!("{:#?}", $byte), color.into()),
            if $byte < 127 {
                $crate::color::fore(
                    format!("{:#?}", char::from($byte).to_string()),
                    color.into(),
                )
            } else {
                String::new()
            },
        ]
        .iter()
        .filter(|c| !c.is_empty())
        .map(String::from)
        .collect::<Vec<String>>()
        .join(" => ")
    }};
}
/// [std::dbg] equivalent for u8 which uses [format_byte] to display the byte
#[macro_export]
macro_rules! dbg_byte {
    ($byte:expr $(,)? ) => {{
        use $crate::color::{auto, fore, from_display};
        let color = $crate::color::from_display($byte);
        $crate::step!(format!(
            "{} = {}",
            $crate::color::auto(stringify!($byte)),
            $crate::format_byte!($byte)
        ));
        $byte
    }};
}

/// [std::dbg] equivalent for `&[u8]` which uses [format_bytes] to display the byte slice
#[macro_export]
macro_rules! dbg_bytes {
    ($slice:expr $(,)? ) => {{
        use $crate::color::{auto, back, fore, from_display, pad};
        $crate::step!($crate::indent!(format!(
            "{} = {}",
            $crate::color::auto(stringify!($slice)),
            $crate::format_bytes!($slice)
        )));
        $slice
    }};
}
/// [std::dbg] equivalent for `&[u8]` which uses [format_bytes] to display the byte slice in base 16 and string
#[macro_export]
macro_rules! dbg_bytes_str {
    ($slice:expr $(,)? ) => {{
        use $crate::color::{auto, back, fore, from_display, pad};
        use $crate::indent;
        eprintln!(
            "\n{}",
            [
                $crate::location!(begin),
                String::new(),
                $crate::color::auto(stringify!($slice)),
                $crate::format_bytes_str!($slice),
                String::new(),
                $crate::location!(end),
            ]
            .join("\n")
        );
        $slice
    }};
}
/// [std::dbg_bytes_str] equivalent which only displays debug message if the given bytes are valid UTF-8
#[macro_export]
macro_rules! dbg_bytes_if_str {
    ($slice:expr $(,)? ) => {
        if let Ok(c) = std::str::from_utf8($slice) {
            $crate::dbg_bytes!($slice)
        } else {
            $slice
        }
    };
}
/// colorfully formats a slice or vector of [u8] as hex => binary => decimal (=> char (if ascii))
#[macro_export]
macro_rules! format_bytes {
    ($slice:expr $(,)? ) => {
        $crate::format_bytes!($slice, " => ");
    };
    (hex, $slice:expr $(,)? ) => {
        $crate::format_bytes!(hex, $slice, " => ");
    };
    (bin, $slice:expr $(,)? ) => {
        $crate::format_bytes!(bin, $slice, " => ");
    };
    ($slice:expr, $sep:literal $(,)? ) => {{
        [
            format!(
                "[\n{}]",
                $slice
                    .iter()
                    .map(Clone::clone)
                    .map(|byte| format!(
                        "{}, // {}\n",
                        $crate::indent!($crate::format_byte!(byte)),
                        $crate::color::fore(format!("{:#?}", char::from(byte).to_string()), 237),
                    ))
                    .collect::<Vec<String>>()
                    .join("")
            ),
            format!("{} bytes", $slice.len()),
            std::str::from_utf8($slice)
                .map(|s| {
                    let chars = s.chars().collect::<Vec<char>>();
                    format!(
                        "\"{s}\" => {} chars => [{}]",
                        chars.len(),
                        chars
                            .iter()
                            .map(|c| format!("{c:?}"))
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                })
                .unwrap_or_default(),
        ]
        .iter()
        .filter(|c| !c.is_empty())
        .map(String::from)
        .collect::<Vec<String>>()
        .join($sep.to_string().as_str())
    }};
    (hex, $slice:expr, $sep:literal $(,)? ) => {{
        [
            format!(
                "[\n{}]",
                $slice
                    .iter()
                    .map(Clone::clone)
                    .map(|byte| format!(
                        "{}, // {}\n",
                        $crate::indent!($crate::format_byte!(hex, byte)),
                        $crate::color::fore(format!("{:#?}", char::from(byte).to_string()), 237),
                    ))
                    .collect::<Vec<String>>()
                    .join("")
            ),
            std::str::from_utf8($slice)
                .map(|s| format!("{s:#?}"))
                .unwrap_or_default(),
        ]
        .iter()
        .filter(|c| !c.is_empty())
        .map(String::from)
        .collect::<Vec<String>>()
        .join($sep.to_string().as_str())
    }};
    (bin, $slice:expr, $sep:literal $(,)? ) => {{
        [
            format!(
                "[\n{}]",
                $slice
                    .iter()
                    .map(Clone::clone)
                    .map(|byte| format!(
                        "{}, // {}\n",
                        $crate::indent!($crate::format_byte!(bin, byte)),
                        $crate::color::fore(format!("{:#?}", char::from(byte).to_string()), 237),
                    ))
                    .collect::<Vec<String>>()
                    .join("")
            ),
            std::str::from_utf8($slice)
                .map(|s| format!("{s:#?}"))
                .unwrap_or_default(),
        ]
        .iter()
        .filter(|c| !c.is_empty())
        .map(String::from)
        .collect::<Vec<String>>()
        .join($sep.to_string().as_str())
    }};
}

/// colorfully formats a slice or vector of [u8] as hex
#[macro_export]
macro_rules! format_bytes_str {
    ($slice:expr $(,)? ) => {
        $crate::format_bytes_str!($slice, " => ");
    };
    ($slice:expr, $sep:literal $(,)? ) => {{
        [
            format!(
                "[{}]",
                $slice
                    .iter()
                    .map(Clone::clone)
                    .map(|byte| $crate::format_byte!(hex_only, byte))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            std::str::from_utf8($slice)
                .map(|s| format!("{s:#?}"))
                .unwrap_or_default(),
        ]
        .iter()
        .filter(|c| !c.is_empty())
        .map(String::from)
        .collect::<Vec<String>>()
        .join($sep.to_string().as_str())
    }};
}
