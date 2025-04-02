use godot::prelude::*;

#[derive(GodotClass)]
#[class(init)]
pub struct Color256;

#[godot_api]
/// Standard basic color index in terminal:
impl Color256 {
    #[constant]
    pub const BLACK: i16 = 0;
    #[constant]
    pub const RED: i16 = 1;
    #[constant]
    pub const GREEN: i16 = 2;
    #[constant]
    pub const YELLOW: i16 = 3;
    #[constant]
    pub const BLUE: i16 = 4;
    #[constant]
    pub const MAGENTA: i16 = 5;
    #[constant]
    pub const CYAN: i16 = 6;
    #[constant]
    pub const WHITE: i16 = 7;
    #[constant]
    pub const BRIGHT_BLACK: i16 = 8;
    #[constant]
    pub const BRIGHT_RED: i16 = 9;
    #[constant]
    pub const BRIGHT_GREEN: i16 = 10;
    #[constant]
    pub const BRIGHT_YELLOW: i16 = 11;
    #[constant]
    pub const BRIGHT_BLUE: i16 = 12;
    #[constant]
    pub const BRIGHT_MAGENTA: i16 = 13;
    #[constant]
    pub const BRIGHT_CYAN: i16 = 14;
    #[constant]
    pub const BRIGHT_WHITE: i16 = 15;
}
