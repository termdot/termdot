#![allow(dead_code)]
use std::str::FromStr;

use crate::utils::charmap::*;
use crate::utils::escape_sequence::*;
use godot::prelude::*;

use super::rust::ShAnsiString;

#[derive(GodotClass)]
#[class(init, base = Node)]
/// Building syled string texts with [Ansi Escape Code Sequence](https://gist.github.com/Joezeo/ce688cf42636376650ead73266256336) for terminal.  
///
/// ### Functions:
/// - Change the text **foreground/background** color.
/// - Set/Reset **bold/underline/italic/blinking/strikethrough** style mode.
/// - **Move/save/restore** the cursor position to display inputing text on terminal.
pub struct AnsiString {
    pub(crate) builder: String,
    #[init(val = -1)]
    pub(crate) bg_256: i16,
    #[init(val = -1)]
    pub(crate) fg_256: i16,
    #[init(val = -1)]
    pub(crate) bg_r: i16,
    #[init(val = -1)]
    pub(crate) bg_g: i16,
    #[init(val = -1)]
    pub(crate) bg_b: i16,
    #[init(val = -1)]
    pub(crate) fg_r: i16,
    #[init(val = -1)]
    pub(crate) fg_g: i16,
    #[init(val = -1)]
    pub(crate) fg_b: i16,

    base: Base<Node>,
}

#[godot_api]
impl AnsiString {
    #[func]
    #[inline]
    pub fn as_str(&self) -> GString {
        GString::from_str(self.builder.as_str()).unwrap()
    }

    #[func]
    #[inline]
    pub fn len(&self) -> i32 {
        self.builder.len() as i32
    }

    #[func]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.builder.len() == 0
    }

    #[func]
    #[inline]
    pub fn foreground_256(&mut self, color: i16) -> Gd<Self> {
        if !check_range(&color) {
            return self.to_gd();
        }
        self.fg_r = -1;
        self.fg_g = -1;
        self.fg_b = -1;
        self.fg_256 = color;
        self.to_gd()
    }

    #[func]
    pub fn background_256(&mut self, color: i16) -> Gd<Self> {
        if !check_range(&color) {
            return self.to_gd();
        }
        self.bg_r = -1;
        self.bg_g = -1;
        self.bg_b = -1;
        self.bg_256 = color;
        self.to_gd()
    }

    #[func]
    pub fn foreground_rgb(&mut self, r: i16, g: i16, b: i16) -> Gd<Self> {
        if !check_range(&r) || !check_range(&g) || !check_range(&b) {
            return self.to_gd();
        }
        self.fg_r = r;
        self.fg_g = g;
        self.fg_b = b;
        self.fg_256 = -1;
        self.to_gd()
    }

    #[func]
    pub fn background_rgb(&mut self, r: i16, g: i16, b: i16) -> Gd<Self> {
        if !check_range(&r) || !check_range(&g) || !check_range(&b) {
            return self.to_gd();
        }
        self.bg_r = r;
        self.bg_g = g;
        self.bg_b = b;
        self.bg_256 = -1;
        self.to_gd()
    }

    fn de_foreground(&mut self) -> &mut Self {
        self.fg_256 = -1;
        self.fg_r = -1;
        self.fg_g = -1;
        self.fg_b = -1;
        self
    }

    fn de_background(&mut self) -> &mut Self {
        self.bg_256 = -1;
        self.bg_r = -1;
        self.bg_g = -1;
        self.bg_b = -1;
        self
    }

    #[func]
    pub fn clear_style(&mut self) -> Gd<Self> {
        self.builder.push_str(ESC0M);
        self.de_background().de_foreground().to_gd()
    }

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

    #[func]
    pub fn append(&mut self, str: GString) -> Gd<Self> {
        if str.is_empty() {
            return self.to_gd();
        }
        self.builder
            .push_str(self.fill_color(&str.to_string()).as_str());
        self.to_gd()
    }

    #[func]
    pub fn cursor_move_to(&mut self, line: i32, column: i32) -> Gd<Self> {
        let changed = CursorPositionHelper::cursor_move(line, column);
        self.builder.push_str(changed.as_str());
        self.to_gd()
    }

    #[func]
    pub fn append_int(&mut self, val: i32) -> Gd<Self> {
        self.append(GString::from_str(val.to_string().as_str()).unwrap());
        self.to_gd()
    }

    #[func]
    pub fn append_float(&mut self, val: f32) -> Gd<Self> {
        self.append(GString::from_str(val.to_string().as_str()).unwrap());
        self.to_gd()
    }

    #[func]
    pub fn append_bool(&mut self, val: bool) -> Gd<Self> {
        self.append(GString::from_str(val.to_string().as_str()).unwrap());
        self.to_gd()
    }

    #[func]
    pub fn bold(&mut self) -> Gd<Self> {
        self.builder.push_str(ESC1M);
        self.to_gd()
    }

    #[func]
    pub fn de_bold(&mut self) -> Gd<Self> {
        self.builder.push_str(ESC22M);
        self.to_gd()
    }

    #[func]
    pub fn italic(&mut self) -> Gd<Self> {
        self.builder.push_str(ESC3M);
        self.to_gd()
    }

    #[func]
    pub fn de_italic(&mut self) -> Gd<Self> {
        self.builder.push_str(ESC23M);
        self.to_gd()
    }

    #[func]
    pub fn underline(&mut self) -> Gd<Self> {
        self.builder.push_str(ESC4M);
        self.to_gd()
    }

    #[func]
    pub fn de_underline(&mut self) -> Gd<Self> {
        self.builder.push_str(ESC24M);
        self.to_gd()
    }

    #[func]
    pub fn blinking(&mut self) -> Gd<Self> {
        self.builder.push_str(ESC5M);
        self.to_gd()
    }

    #[func]
    pub fn de_blinking(&mut self) -> Gd<Self> {
        self.builder.push_str(ESC25M);
        self.to_gd()
    }

    #[func]
    pub fn strikethrough(&mut self) -> Gd<Self> {
        self.builder.push_str(ESC9M);
        self.to_gd()
    }

    #[func]
    pub fn de_strikethrough(&mut self) -> Gd<Self> {
        self.builder.push_str(ESC29M);
        self.to_gd()
    }

    #[func]
    pub fn save_cursor_position(&mut self) -> Gd<Self> {
        self.builder.push_str(ESCS);
        self.to_gd()
    }

    #[func]
    pub fn restore_cursor_position(&mut self) -> Gd<Self> {
        self.builder.push_str(ESCU);
        self.to_gd()
    }

    #[func]
    pub fn crlf(&mut self) -> Gd<Self> {
        self.builder.push_str(CRLF);
        self.to_gd()
    }

    #[func]
    pub fn tab(&mut self) -> Gd<Self> {
        self.builder.push_str(TAB);
        self.to_gd()
    }

    #[func]
    pub fn space(&mut self) -> Gd<Self> {
        self.builder.push_str(SPACE);
        self.to_gd()
    }

    #[func]
    pub fn space_in(&mut self, cnt: u32) -> Gd<Self> {
        self.builder.push_str(SPACE.repeat(cnt as usize).as_str());
        self.to_gd()
    }

    #[func]
    pub fn clear_cursor_to_end(&mut self) -> Gd<Self> {
        self.builder.push_str(ESC0K);
        self.to_gd()
    }

    #[func]
    pub fn clear_cursor_to_start(&mut self) -> Gd<Self> {
        self.builder.push_str(ESC1K);
        self.to_gd()
    }

    #[func]
    pub fn clear_line(&mut self) -> Gd<Self> {
        self.builder.push_str(ESC2K);
        self.to_gd()
    }

    #[func]
    pub fn clear_str(&mut self) -> Gd<Self> {
        self.builder.clear();
        self.to_gd()
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

impl From<AnsiString> for ShAnsiString {
    #[inline]
    fn from(value: AnsiString) -> Self {
        ShAnsiString {
            builder: value.builder.clone(),
            bg_256: value.bg_256,
            fg_256: value.fg_256,
            bg_r: value.bg_r,
            bg_g: value.bg_g,
            bg_b: value.bg_b,
            fg_r: value.fg_r,
            fg_g: value.fg_g,
            fg_b: value.fg_b,
        }
    }
}
