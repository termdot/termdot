#![allow(dead_code)]
use crate::utils::charmap::*;
use common::escape_sequence::*;

/// Building syled string texts with [Ansi Escape Code Sequence](https://gist.github.com/Joezeo/ce688cf42636376650ead73266256336) for terminal.  
///
/// ### Functions:
/// - Change the text **foreground/background** color.
/// - Set/Reset **bold/underline/italic/blinking/strikethrough** style mode.
/// - **Move/save/restore** the cursor position to display inputing text on terminal.
/// ### Example
/// ```ignore
/// use utils::ansi_string::ShAnsiString;
///
/// let mut ansi_string = ShAnsiString::new();
/// ansi_string.foreground_256(45) // Change the foreground color to 45(256-Color).
///             // Append text, and the text "Hello World!" will display in foreground color 45(256-color).
///             .append("Hello World!")
///             // Clear all the style mode (foreground/background/bold/italic...)
///             .clear_style()
///             // Change the background color to (12, 12, 12) (RGB-Color).
///             .background_rgb(12, 12, 12)
///             // Set text style bold.
///             .bold()
///             // Set text tyle italic.
///             .italic()
///             // Append text, and the text "Hello You!" will display in background color (12,12,12)(RGB-color), bold and italic.
///             .append("Hello you!")
///             // Clear all the style mode (foreground/background/bold/italic...)
///             .clear_style();
/// println!("{}", ansi_string.as_str());
/// ```
pub struct ShAnsiString {
    pub(crate) builder: String,
    pub(crate) bg_256: i16,
    pub(crate) fg_256: i16,
    pub(crate) bg_r: i16,
    pub(crate) bg_g: i16,
    pub(crate) bg_b: i16,
    pub(crate) fg_r: i16,
    pub(crate) fg_g: i16,
    pub(crate) fg_b: i16,
}

impl Default for ShAnsiString {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl ShAnsiString {
    #[inline]
    pub fn new() -> Self {
        ShAnsiString {
            builder: String::new(),
            bg_256: -1,
            fg_256: -1,
            bg_r: -1,
            bg_g: -1,
            bg_b: -1,
            fg_r: -1,
            fg_g: -1,
            fg_b: -1,
        }
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        self.builder.as_str()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.builder.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.builder.len() == 0
    }

    #[inline]
    pub fn foreground_256(mut self, color: i16) -> Self {
        if !check_range(&color) {
            return self;
        }
        self.fg_r = -1;
        self.fg_g = -1;
        self.fg_b = -1;
        self.fg_256 = color;
        self
    }

    #[inline]
    pub fn background_256(mut self, color: i16) -> Self {
        if !check_range(&color) {
            return self;
        }
        self.bg_r = -1;
        self.bg_g = -1;
        self.bg_b = -1;
        self.bg_256 = color;
        self
    }

    #[inline]
    pub fn foreground_rgb(mut self, r: i16, g: i16, b: i16) -> Self {
        if !check_range(&r) || !check_range(&g) || !check_range(&b) {
            return self;
        }
        self.fg_r = r;
        self.fg_g = g;
        self.fg_b = b;
        self.fg_256 = -1;
        self
    }

    #[inline]
    pub fn background_rgb(mut self, r: i16, g: i16, b: i16) -> Self {
        if !check_range(&r) || !check_range(&g) || !check_range(&b) {
            return self;
        }
        self.bg_r = r;
        self.bg_g = g;
        self.bg_b = b;
        self.bg_256 = -1;
        self
    }

    #[inline]
    fn de_foreground(mut self) -> Self {
        self.fg_256 = -1;
        self.fg_r = -1;
        self.fg_g = -1;
        self.fg_b = -1;
        self
    }

    #[inline]
    fn de_background(mut self) -> Self {
        self.bg_256 = -1;
        self.bg_r = -1;
        self.bg_g = -1;
        self.bg_b = -1;
        self
    }

    #[inline]
    pub fn clear_style(mut self) -> Self {
        self.builder.push_str(ESC0M);
        self.de_background().de_foreground()
    }

    #[inline]
    fn fill_color(&self, str: &str) -> String {
        let mut filled = String::from(str);
        if self.fg_256 != -1 {
            filled = ColorHelper::foreground_256(&filled, self.fg_256);
        }
        if self.bg_256 != -1 {
            filled = ColorHelper::background_256(&filled, self.bg_256);
        }
        if self.fg_r != -1 && self.fg_g != -1 && self.fg_b != -1 {
            filled = ColorHelper::foreground_rgb(&filled, self.fg_r, self.fg_g, self.fg_b);
        }
        if self.bg_r != -1 && self.bg_g != -1 && self.bg_b != -1 {
            filled = ColorHelper::background_rgb(&filled, self.bg_r, self.bg_g, self.bg_b);
        }
        filled
    }

    #[inline]
    pub fn append(mut self, str: &str) -> Self {
        if str.is_empty() {
            return self;
        }
        self.builder.push_str(self.fill_color(str).as_str());
        self
    }

    #[inline]
    pub fn cursor_move_to(mut self, line: i32, column: i32) -> Self {
        let changed = CursorPositionHelper::cursor_move(line, column);
        self.builder.push_str(changed.as_str());
        self
    }

    #[inline]
    pub fn append_char(self, ch: char) -> Self {
        self.append(ch.to_string().as_str())
    }

    #[inline]
    pub fn append_i8(self, val: i8) -> Self {
        self.append(val.to_string().as_str())
    }

    #[inline]
    pub fn append_u8(self, val: u8) -> Self {
        self.append(val.to_string().as_str())
    }

    #[inline]
    pub fn append_i16(self, val: i16) -> Self {
        self.append(val.to_string().as_str())
    }

    #[inline]
    pub fn append_u16(self, val: u16) -> Self {
        self.append(val.to_string().as_str())
    }

    #[inline]
    pub fn append_i32(self, val: i32) -> Self {
        self.append(val.to_string().as_str())
    }

    #[inline]
    pub fn append_u32(self, val: u32) -> Self {
        self.append(val.to_string().as_str())
    }

    #[inline]
    pub fn append_i64(self, val: i64) -> Self {
        self.append(val.to_string().as_str())
    }

    #[inline]
    pub fn append_u64(self, val: u64) -> Self {
        self.append(val.to_string().as_str())
    }

    #[inline]
    pub fn append_f32(self, val: f32) -> Self {
        self.append(val.to_string().as_str())
    }

    #[inline]
    pub fn append_f64(self, val: f64) -> Self {
        self.append(val.to_string().as_str())
    }

    #[inline]
    pub fn append_bool(self, val: bool) -> Self {
        self.append(val.to_string().as_str())
    }

    #[inline]
    pub fn bold(mut self) -> Self {
        self.builder.push_str(ESC1M);
        self
    }

    #[inline]
    pub fn de_bold(mut self) -> Self {
        self.builder.push_str(ESC22M);
        self
    }

    #[inline]
    pub fn italic(mut self) -> Self {
        self.builder.push_str(ESC3M);
        self
    }

    #[inline]
    pub fn de_italic(mut self) -> Self {
        self.builder.push_str(ESC23M);
        self
    }

    #[inline]
    pub fn underline(mut self) -> Self {
        self.builder.push_str(ESC4M);
        self
    }

    #[inline]
    pub fn de_underline(mut self) -> Self {
        self.builder.push_str(ESC24M);
        self
    }

    #[inline]
    pub fn blinking(mut self) -> Self {
        self.builder.push_str(ESC5M);
        self
    }

    #[inline]
    pub fn de_blinking(mut self) -> Self {
        self.builder.push_str(ESC25M);
        self
    }

    #[inline]
    pub fn strikethrough(mut self) -> Self {
        self.builder.push_str(ESC9M);
        self
    }

    #[inline]
    pub fn de_strikethrough(mut self) -> Self {
        self.builder.push_str(ESC29M);
        self
    }

    #[inline]
    pub fn save_cursor_position(mut self) -> Self {
        self.builder.push_str(ESCS);
        self
    }

    #[inline]
    pub fn restore_cursor_position(mut self) -> Self {
        self.builder.push_str(ESCU);
        self
    }

    #[inline]
    pub fn crlf(mut self) -> Self {
        self.builder.push_str(CRLF);
        self
    }

    #[inline]
    pub fn tab(mut self) -> Self {
        self.builder.push_str(TAB);
        self
    }

    #[inline]
    pub fn space(mut self) -> Self {
        self.builder.push_str(SPACE);
        self
    }

    #[inline]
    pub fn space_in(mut self, cnt: usize) -> Self {
        self.builder.push_str(SPACE.repeat(cnt).as_str());
        self
    }

    #[inline]
    pub fn clear_cursor_to_end(mut self) -> Self {
        self.builder.push_str(ESC0K);
        self
    }

    #[inline]
    pub fn clear_cursor_to_start(mut self) -> Self {
        self.builder.push_str(ESC1K);
        self
    }

    #[inline]
    pub fn clear_line(mut self) -> Self {
        self.builder.push_str(ESC2K);
        self
    }

    #[inline]
    pub fn clear_str(mut self) -> Self {
        self.builder.clear();
        self
    }
}

struct ColorHelper {}
impl ColorHelper {
    // 256-Color mode
    fn foreground_256(msg: &str, color: i16) -> String {
        format!("\u{001b}[38;5;{}m{}", color, msg)
    }
    fn background_256(msg: &str, color: i16) -> String {
        format!("\u{001b}[48;5;{}m{}", color, msg)
    }

    // RGB-color mode
    fn foreground_rgb(msg: &str, r: i16, g: i16, b: i16) -> String {
        format!("\u{001b}[38;2;{};{};{}m{}", r, g, b, msg)
    }
    fn background_rgb(msg: &str, r: i16, g: i16, b: i16) -> String {
        format!("\u{001b}[48;2;{};{};{}m{}", r, g, b, msg)
    }
}

struct CursorPositionHelper {}
impl CursorPositionHelper {
    fn cursor_move(line: i32, column: i32) -> String {
        format!("\u{001b}[{};{}H", line, column)
    }
}

#[inline]
fn check_range(color: &i16) -> bool {
    (0..=255).contains(color)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ansi_string() {
        let ansi_str = ShAnsiString::new()
            .foreground_256(15)
            .background_rgb(112, 112, 112)
            .italic()
            .underline()
            .strikethrough()
            .append("Hello World!")
            .tab()
            .de_underline()
            .de_strikethrough()
            .bold()
            .foreground_rgb(175, 0, 0)
            .background_256(32)
            .append(" Hello you!")
            .clear_style()
            .space()
            .crlf()
            .de_bold()
            .de_italic()
            .blinking()
            .background_256(64)
            .append_bool(true)
            .space_in(1)
            .append_char('💖')
            .space()
            .append_f32(0.43)
            .space()
            .append_f64(0.64)
            .space()
            .append_i16(12)
            .space()
            .append_i32(44)
            .space()
            .append_i64(32)
            .space()
            .append_i8(8)
            .space()
            .append_u8(123)
            .space()
            .append_u16(32)
            .space()
            .append_u32(42)
            .space()
            .append_u64(32)
            .save_cursor_position()
            .cursor_move_to(0, 0)
            .append("Home")
            .restore_cursor_position()
            .append_i32(0xffffff)
            .clear_style();
        println!("{}", ansi_str.as_str());
        assert_eq!(ansi_str.clear_str().as_str(), "");
    }
}
