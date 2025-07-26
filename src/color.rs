use ansi_colours::{ansi256_from_rgb, rgb_from_ansi256};
use std::fmt::{Debug, Display, LowerHex};
use std::iter::{IntoIterator, Iterator};

#[cfg(feature = "colorsys")]
use colorsys::{Ansi256, ColorAlpha, ColorTransform, Hsl, Rgb};

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
        u8::from_str_radix(&word.to_string(), 10)
            .unwrap_or_else(|_| from_display(word.to_string()))
            .into(),
    )
}
/// brighter version of [auto]
pub fn auto_bright<T: Display>(word: T) -> String {
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
/// darker version of [auto]
pub fn auto_dark<T: Display>(word: T) -> String {
    fore(
        word.to_string(),
        dark(
            u8::from_str_radix(&word.to_string(), 10)
                .unwrap_or_else(|_| from_display(word.to_string()))
                .into(),
        )
        .into(),
    )
}
/// auto-colorize the underlying bytes of given text with the color determined by [from_bytes]
pub fn from_display<T: Display>(word: T) -> u8 {
    let string = format!("{word}");
    from_bytes(
        &u8::from_str_radix(&string, 10)
            .ok()
            .or_else(|| u8::from_str_radix(&string, 16).ok())
            .map(|byte| vec![byte])
            .or_else(|| {
                if string.to_lowercase().starts_with("0x") {
                    u8::from_str_radix(string.to_lowercase().replacen("0x", "", 1).as_str(), 16)
                        .map(|byte| vec![byte])
                        .ok()
                } else {
                    None
                }
            })
            .map(|byte| vec![byte].into_iter().flatten().collect::<Vec<u8>>())
            .or_else(|| {
                u16::from_str_radix(&string, 16)
                    .map(|u| u.to_ne_bytes().to_vec())
                    .ok()
            })
            .or_else(|| {
                if string.to_lowercase().starts_with("0x") {
                    u16::from_str_radix(string.to_lowercase().replacen("0x", "", 1).as_str(), 16)
                        .map(|u| u.to_ne_bytes().to_vec())
                        .ok()
                } else {
                    None
                }
            })
            .or_else(|| {
                u32::from_str_radix(&string, 16)
                    .ok()
                    .map(|u| u.to_ne_bytes().to_vec())
            })
            .or_else(|| {
                if string.to_lowercase().starts_with("0x") {
                    u32::from_str_radix(string.to_lowercase().replacen("0x", "", 1).as_str(), 16)
                        .map(|u| u.to_ne_bytes().to_vec())
                        .ok()
                } else {
                    None
                }
            })
            .or_else(|| {
                u64::from_str_radix(&string, 16)
                    .ok()
                    .map(|u| u.to_ne_bytes().to_vec())
            })
            .or_else(|| {
                if string.to_lowercase().starts_with("0x") {
                    u64::from_str_radix(string.to_lowercase().replacen("0x", "", 1).as_str(), 16)
                        .map(|u| u.to_ne_bytes().to_vec())
                        .ok()
                } else {
                    None
                }
            })
            .unwrap_or_else(|| string.as_bytes().to_vec()),
    )
}
/// auto-colorize the underlying bytes of given text with the color determined by [from_bytes]
pub fn from_debug<T: Debug>(word: T) -> u8 {
    from_bytes(format!("{word:#?}").as_bytes())
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
    merge_rgb(bytes.into_iter().map(|byte| rgb_from_byte(*byte)), false)
}
/// returns a `[red, green, blue]` slice `[u8; 3]` from a single byte
pub fn rgb_from_byte(byte: u8) -> [u8; 3] {
    let tuple = rgb_from_ansi256(byte);
    [tuple.0, tuple.1, tuple.2]
}

/// returns a `[red, green, blue]` slice `[u8; 3]` from a single byte
pub fn rgb_to_byte(rgb: [u8; 3]) -> u8 {
    ansi256_from_rgb(rgb)
}
/// merges a sequence of slice `[u8; 3]` into a single slice `[u8; 3]`
pub fn merge_rgb<I: IntoIterator<Item = [u8; 3]> + Clone>(rgbs: I, extra: bool) -> [u8; 3] {
    let mut result = [0u8; 3];
    for rgb in rgbs.clone().into_iter() {
        result[0] ^= rgb[0];
        result[1] ^= rgb[1];
        result[2] ^= rgb[2];
    }
    if extra {
        for triple in rgbs.clone().into_iter() {
            for byte in triple.into_iter() {
                let rgb = rgb_from_byte(byte);
                result[0] ^= rgb[0];
                result[1] ^= rgb[1];
                result[2] ^= rgb[2];
            }
        }
    }
    result
}

/// returns a tuple of (foreground, backrground) color by taking one
/// unsigned integer, wrapping that around [u8::MAX] to determine the
/// foreground color then uses [invert_bw] to
/// determine the background color.
pub fn couple(color: usize) -> (u8, u8) {
    let fore = wrap(color);
    let back = invert_bw(fore as usize);
    (fore, back)
}

/// converts the given color to rgb triple then inverts the rgb and converts back to ansi256
#[cfg(feature = "colorsys")]
pub fn invert_ansi(color: usize) -> u8 {
    let color = Ansi256::new(wrap(color));
    let mut hsl = Hsl::from(&Rgb::from(color));
    hsl.set_lightness(100.0 - hsl.lightness());
    let mut rgb = Rgb::from(&hsl);
    rgb.invert();
    let color = Ansi256::from(&rgb);
    color.code()
}
/// converts the given color to rgb triple then inverts the rgb and converts back to ansi256
#[cfg(not(feature = "colorsys"))]
pub fn invert_ansi(color: usize) -> u8 {
    if is_dark_rgb_band(color) {
        bright(rgb_to_byte(invert_rgb(rgb_from_byte(wrap(color)))) as usize)
    } else {
        dark(rgb_to_byte(invert_rgb(rgb_from_byte(wrap(color)))) as usize)
    }
}

/// converts the given color to rgb triple then inverts the rgb and converts back to ansi256
#[cfg(feature = "colorsys")]
pub fn invert_rgb(color: [u8; 3]) -> [u8; 3] {
    let mut rgb = Rgb::from(color);
    rgb.invert();
    rgb.into()
}

/// converts the given color to rgb triple then inverts the rgb and converts back to ansi256
#[cfg(not(feature = "colorsys"))]
pub fn invert_rgb(color: [u8; 3]) -> [u8; 3] {
    [255u8 - color[0], 255u8 - color[1], 255u8 - color[2]]
}

/// naive heuristic to return the brightest opposite of the given color.
#[cfg(not(feature = "colorsys"))]
pub fn invert_bw(color: usize) -> u8 {
    match color {
        0 | 8 | 16..21 | 52..61 | 88..93 | 232..239 => 231,
        _ => 16,
    }
}
/// brings the color to grayscale then inverts it
#[cfg(feature = "colorsys")]
pub fn invert_bw(color: usize) -> u8 {
    let color = Ansi256::new(wrap(color));
    let mut rgb = Rgb::from(color);
    rgb.grayscale_simple();
    rgb.invert();
    Ansi256::from(&rgb).code()
}

/// return true if the given rgb band is bright
pub fn is_bright_rgb_band(color: usize) -> bool {
    let color = wrap(color);
    color >= 0x75
}
/// return a brighter color from the given rgb band
pub fn bright_rgb_band(color: usize) -> u8 {
    if !is_bright_rgb_band(color) {
        0xff - wrap(color)
    } else {
        wrap(color)
    }
}
/// return a brighter color near the given one via [bright_rgb_band].
#[cfg(feature = "colorsys")]
pub fn bright(color: usize) -> u8 {
    let color = wrap(color);
    let mut rgb = Rgb::from(Ansi256::new(color));
    rgb.lighten(50.0);
    Ansi256::from(rgb).code()
}
/// return a brighter color near the given one via [bright_rgb_band].
#[cfg(not(feature = "colorsys"))]
pub fn bright(color: usize) -> u8 {
    let color = wrap(color);
    let [r, g, b] = rgb_from_byte(color);
    rgb_to_byte([
        bright_rgb_band(r as usize),
        bright_rgb_band(g as usize),
        bright_rgb_band(b as usize),
    ])
}
/// return true if the given rgb band is dark
pub fn is_dark_rgb_band(color: usize) -> bool {
    let color = wrap(color);
    color <= 0x75
}
/// return a darker color from the given rgb band
pub fn dark_rgb_band(color: usize) -> u8 {
    let color = wrap(color);
    if !is_dark_rgb_band(color as usize) {
        color - ((color / 4) * 3)
    } else {
        color
    }
}
/// return a darker color near the given one via [dark_rgb_band].
#[cfg(feature = "colorsys")]
pub fn dark(color: usize) -> u8 {
    let color = wrap(color);
    let mut rgb = Rgb::from(Ansi256::new(color));
    rgb.lighten(-50.0);
    Ansi256::from(rgb).code()
}
#[cfg(not(feature = "colorsys"))]
pub fn dark(color: usize) -> u8 {
    let color = wrap(color);
    let [r, g, b] = rgb_from_byte(color);
    rgb_to_byte([
        dark_rgb_band(r as usize),
        dark_rgb_band(g as usize),
        dark_rgb_band(b as usize),
    ])
}

/// wraps the given usize via remainder
pub fn wrap(color: usize) -> u8 {
    if color <= u8::MAX.into() {
        color as u8
    } else {
        from_bytes(&color.to_ne_bytes())
    }
}

pub fn non_zero_be_bytes(color: usize) -> Vec<u8> {
    let mut bytes = color.to_be_bytes().to_vec();
    while bytes.len() > 1 && bytes[0] == 0 {
        bytes.remove(0);
    }
    bytes
}

/// tries to obtain the number of columns of the [DEFAULT_COLUMNS] in case of error.
pub fn term_cols() -> usize {
    match ioctl_term_light::cols() {
        0 => DEFAULT_COLUMNS,
        cols => cols as usize,
    }
}

/// determine an ANSI-256 color determined by [`from_bytes(&[byte])`]
pub fn from_byte(byte: u8) -> u8 {
    byte
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

pub const STD_COLORS: [u8; 48] = [
    0x00u8, 0x00u8, 0x00u8, //  0x00u8 black
    0x80u8, 0x00u8, 0x00u8, //  1 red
    0x00u8, 0x80u8, 0x00u8, //  2 green
    0x80u8, 0x80u8, 0x00u8, //  3 yellow
    0x00u8, 0x00u8, 0x80u8, //  4 blue
    0x80u8, 0x00u8, 0x80u8, //  5 magenta
    0x00u8, 0x80u8, 0x80u8, //  6 cyan
    0xc0u8, 0xc0u8, 0xc0u8, //  7 white (light grey)
    0x80u8, 0x80u8, 0x80u8, //  8 grey
    0xffu8, 0x00u8, 0x00u8, //  9 bright red
    0xffu8, 0xffu8, 0x00u8, // 10 bright green
    0x00u8, 0xffu8, 0x00u8, // 11 bright yellow
    0x00u8, 0x00u8, 0xffu8, // 12 bright blue
    0xffu8, 0x00u8, 0xffu8, // 13 bright magenta
    0x00u8, 0xffu8, 0xffu8, // 14 bright cyan
    0xffu8, 0xffu8, 0xffu8, // 15 bright white
];
pub fn cube_ansi_256(color: usize, op: usize) -> u8 {
    let color = wrap(color) as usize;
    let cube = ((color - 16) / op) % 6;
    if cube == 0 {
        0u8
    } else {
        wrap((14135 + 10280 * cube) / 256)
    }
}
pub fn get_ansi_rgb(color: usize) -> [u8; 3] {
    let tuple = rgb_from_ansi256(wrap(color));
    [tuple.0, tuple.1, tuple.2]
}

pub fn format_slice_hex<I: IntoIterator<Item: LowerHex>>(items: I, color: bool) -> String {
    format!(
        "[{}]",
        items
            .into_iter()
            .map(|el| {
                let byte = format!("0x{el:02x}");
                if color {
                    fore(
                        byte,
                        from_byte(
                            u8::from_str_radix(&format!("{el:02x}"), 16).unwrap_or_default(),
                            // .unwrap_or_else(|_| from_display(format!("{el:x}"))),
                        )
                        .into(),
                    )
                } else {
                    byte
                }
            })
            .collect::<Vec<String>>()
            .join(", ")
    )
}

pub fn format_slice_display<I: IntoIterator<Item: Display>>(items: I, color: bool) -> String {
    format!(
        "[{}]",
        items
            .into_iter()
            .map(|el| {
                let byte = format!("{el}");
                if color {
                    fore(byte, from_display(el).into())
                } else {
                    byte
                }
            })
            .collect::<Vec<String>>()
            .join(", ")
    )
}
pub fn format_slice_debug<I: IntoIterator<Item: Debug>>(items: I, color: bool) -> String {
    format!(
        "[{}]",
        items
            .into_iter()
            .map(|el| {
                let byte = format!("{el:#?}");
                if color {
                    fore(byte, from_debug(el).into())
                } else {
                    byte
                }
            })
            .collect::<Vec<String>>()
            .join(", ")
    )
}

// SEEALSO:
// https://gist.github.com/XVilka/8346728
// https://github.com/termstandard/colors?tab=readme-ov-file
// https://gist.github.com/lilydjwg/fdeaf79e921c2f413f44b6f613f6ad53
// https://github.com/robertknight/konsole/blob/master/tests/color-spaces.pl
// https://github.com/JohnMorales/dotfiles/blob/master/colors/24-bit-color.sh
// https://gitlab.gnome.org/GNOME/vte/-/blob/master/perf/img.sh
