use std::fmt::Display;

const DEFAULT_COLUMNS: usize = 130;

/// reset the ANSI colors of the given test
pub fn reset<T: Display>(text: T) -> String {
    format!("{}\x1b[0m", text)
}

/// colorize the foreground of the given text without resetting (ANSI 256 colors)
pub fn fg<T: Display>(text: T, fg: usize) -> String {
    format!("\x1b[1;38;5;{}m{}", wrap(fg), text)
}
/// colorize the background of the given text without resetting (ANSI 256 colors)
pub fn bg<T: Display>(text: T, bg: usize) -> String {
    format!("\x1b[1;48;5;{}m{}", wrap(bg), text)
}
/// colorize the foreground and backrground of the given text without resetting
///
/// > shorthand combination of [bg] and [fg]
pub fn bgfg<T: Display>(text: T, fore: usize, back: usize) -> String {
    bg(fg(text, wrap(fore) as usize), wrap(back) as usize)
}
/// colorize the foreground and backrground of the given text and reset the colors afterwards
pub fn ansi<T: Display>(text: T, fore: usize, back: usize) -> String {
    reset(bgfg(text, fore as usize, back as usize))
}
/// pad text by the number of columns determined by [term_cols]
pub fn pad_columns<T: Display>(text: T) -> String {
    let text = text.to_string();
    let cols = term_cols();
    pad(text, cols)
}
/// pad text
pub fn pad<T: Display>(text: T, length: usize) -> String {
    let text = text.to_string();
    let len = text
        .as_bytes()
        .iter()
        .map(|c| char::from(*c))
        .map(|c| {
            u32::from(c)
                .to_ne_bytes()
                .iter()
                .map(Clone::clone)
                .filter(|c| *c > 0)
                .collect::<Vec<u8>>()
        })
        .flatten()
        .count();

    format!(
        "{}{}",
        text,
        " ".repeat(if length > len {
            length - len
        } else if len < length {
            0
        } else {
            0
        })
    )
}
/// clear the screen
pub fn ansi_clear() -> String {
    "\x1b[2J\x1b[3J\x1b[H".to_string()
}
/// colorize the foreground of the given text and reset afterwards
pub fn fore<T: Display>(text: T, fore: usize) -> String {
    let (fore, back) = couple(fore);
    ansi(text, fore as usize, back as usize)
}
/// colorize the backrground of the given text and reset afterwards
pub fn back<T: Display>(text: T, back: usize) -> String {
    let (back, fore) = couple(back);
    ansi(text, fore as usize, back as usize)
}
/// auto-colorize the given text with the color determined by [from_display]
pub fn auto<T: Display>(word: T) -> String {
    fore(
        word.to_string(),
        bright(
            u8::from_str_radix(&word.to_string(), 10)
                .unwrap_or_else(|_| from_display(word.to_string()))
                .into(),
        )
        .into(),
    )
}
/// auto-colorize the underlying bytes of given text with the color determined by [from_bytes]
pub fn from_display<T: Display>(word: T) -> u8 {
    from_bytes(word.to_string().as_bytes())
}
/// determine a triple of RGB colors of a string determined by [rgb_from_bytes]
pub fn rgb_from_display<T: Display>(word: T) -> [u8; 3] {
    rgb_from_bytes(word.to_string().as_bytes())
}

/// determine an ANSI-256 color determined by [rgb_from_bytes]
pub fn from_bytes(bytes: &[u8]) -> u8 {
    let mut color: u8 = 0;
    for rgb in rgb_from_bytes(bytes) {
        color ^= rgb
    }
    color
}
/// simple and naive algorithm to determine a triple of RGB colors
/// based on XOR'ing the given slice of bytes;
pub fn rgb_from_bytes(bytes: &[u8]) -> [u8; 3] {
    let mut color: [u8; 3] = [0, 0, 0];
    let mut iter = 0;
    while iter < 3 {
        for (index, byte) in bytes.iter().enumerate() {
            color[index % 3] ^= *byte
        }
        iter += 1;
    }
    color
}

/// returns a tuple of (foreground, backrground) color by taking one
/// unsigned integer, wrapping that around [u8::MAX] to determine the
/// foreground color then uses [invert_bw] to
/// determine the background color.
pub fn couple(color: usize) -> (u8, u8) {
    let fore = bright(wrap(color));
    let back = invert_bw(fore);
    (fore, back)
}

/// naive heuristic to return the brightest opposite of the given color.
pub fn invert_bw(color: u8) -> u8 {
    match color {
        0 | 8 | 16..21 | 52..61 | 88..93 | 232..239 => 231,
        _ => 16,
    }
}
/// naive heuristic to return a brighter color near the given one.
pub fn bright(color: u8) -> u8 {
    match color {
        0 | 8 => color + 100,
        16..21 => color + 100,
        52..61 => color + 40,
        88..93 => color + 50,
        232..239 => 249,
        _ => color,
    }
}

/// wraps the given usize via remainder
pub fn wrap(color: usize) -> u8 {
    (if color > 0 { color % 255 } else { color }) as u8
}

/// naive function for unix terminals that calls stty to determine the
/// number of columns of the terminal.
///
/// The rationale of the approach is to avoid linking to libc in order
/// to do ioctl FFI calls and keeping the rust crate lightweight.
///
/// This function might be rewritten using a more sophisticated
/// approach in the future.
fn io_term_cols() -> std::io::Result<usize> {
    if let Ok(cols) = std::env::var("COLUMNS") {
        if let Ok(cols) = usize::from_str_radix(&cols, 10) {
            return Ok(cols);
        }
    }
    use std::process::{Command, Stdio};
    let mut cmd = Command::new("/bin/stty");
    let cmd = cmd.args(vec!["-a"]);
    let cmd = cmd.stdin(Stdio::inherit());
    let cmd = cmd.stdout(Stdio::piped());
    let cmd = cmd.stderr(Stdio::piped());
    let child = cmd.spawn()?;
    let output = child.wait_with_output()?;
    let lines = String::from_utf8_lossy(&output.stdout)
        .to_string()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();
    let lines = lines[0]
        .split(';')
        .map(String::from)
        .collect::<Vec<String>>();
    if lines.len() > 2 {
        let fields = lines[2]
            .split(' ')
            .map(String::from)
            .collect::<Vec<String>>();
        if let Ok(cols) = usize::from_str_radix(&fields[1], 10) {
            return Ok(cols);
        }
    }
    Ok(DEFAULT_COLUMNS)
}

/// tries to obtain the number of columns of the terminal via
/// [io_term_cols] and falls back to
/// [DEFAULT_COLUMNS] in case of error.
pub fn term_cols() -> usize {
    io_term_cols().unwrap_or_else(|_| DEFAULT_COLUMNS)
}

/// determine an ANSI-256 color determined by [`from_bytes(&[byte])`]
pub fn from_byte(byte: u8) -> u8 {
    from_bytes(&[byte]).into()
}

/// auto-colorize the given byte with the color determined by [from_byte]
pub fn byte(byte: u8) -> String {
    let (fg, bg) = couple(from_byte(byte).into());
    ansi(byte, fg as usize, bg as usize)
}

/// auto-colorize the given byte in hex format with the color determined by [from_byte]
pub fn byte_hex(byte: u8) -> String {
    let (fg, bg) = couple(from_byte(byte).into());
    ansi(format!("0x{byte:02x}"), fg as usize, bg as usize)
}

/// auto-colorize the given byte in bin format with the color determined by [from_byte]
pub fn byte_bin(byte: u8) -> String {
    let (fg, bg) = couple(from_byte(byte).into());
    ansi(format!("0b{byte:08b}"), fg as usize, bg as usize)
}
