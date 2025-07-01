/// colofully prints the "location" of the macro call (function name, filename and line number) in the code
#[macro_export]
macro_rules! location {
    () => {{
        let location = format!(
            "{}{}{}:{}",
            $crate::color::auto($crate::function_name!()),
            $crate::color::fore(" @ ", 220),
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
    ($arg:expr) => {{
        $crate::tag!($arg, 7)
    }};
    (close, $arg:expr) => {{
        $crate::tag!(close, $arg, 7)
    }};
    ($arg:expr, $color:literal) => {{
        format!(
            "{}{}{}",
            $crate::color::fore("<", $color),
            $crate::color::auto($arg),
            $crate::color::fore(">", $color),
        )
    }};
    (close, $arg:expr, $color:literal) => {{
        format!(
            "{}{}{}",
            $crate::color::fore("</", $color),
            $arg,
            $crate::color::fore(">", $color),
        )
    }};
}

/// colorful alternative to [std::dbg]
#[macro_export]
macro_rules! dbg {
    ($arg:expr $(,)? ) => {{
        let obj = $crate::indent!(
                format!(
                    "{} = {}\n",
                    $crate::color::auto(stringify!(&$arg)),
                    $crate::color::auto(format!("{:#?}", &$arg))));
        eprintln!("{}", $crate::color::reset([$crate::location!(begin), obj, $crate::location!(end)].join("\n")));
        $arg
    }};
    ($( $arg:expr ),* $(,)? ) => {{
        $($crate::dbg!($arg))*
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
        $crate::step!(bg=$fg, fg=$crate::color::invert_bw($fg), length=$crate::color::term_cols(), $text)
    }};
    (bg=$bg:expr, fg=$fg:expr, $text:expr $(,)?) => {{
        $crate::step!(bg=$bg, fg=$fg, length=$crate::color::term_cols(), $text)
    }};
    (length=$length:expr, $text:expr $(,)?) => {{
        let (bg, fg) = $crate::color::couple(line!() as usize);
        let text = $text.to_string();
        let bar = $crate::color::ansi(
            " ".repeat($length),
            fg.into(),
            bg.into(),
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
                    fg.into(),
                    bg.into(),
                ),
                $crate::color::ansi(
                    $crate::color::pad_columns(
                        if text.is_empty() { String::new() } else { format!("{}", text) }
                    ),
                    bg.into(),
                    fg.into(),
                ),
                bar.clone(),
            ].join("\n")
        );

    }};
    (bg=$bg:expr, fg=$fg:expr, length=$length:expr, $text:expr $(,)?) => {{
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
                    color::invert_bw($color).into(),
                    $color,
                ),
                color::ansi(
                    format!($format, $($arg),*),
                    $color,
                    color::invert_bw($color).into(),
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
                    color::invert_bw($color).into(),
                    $color,
                ),
                color::ansi(
                    format!($format, $($arg),*),
                    $color,
                    color::invert_bw($color).into(),
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
