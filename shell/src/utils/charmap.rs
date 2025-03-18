/// Common character code points
pub const ASCII_SPACE: u16 = 0x0020; // Space
pub const ASCII_TILDE: u16 = 0x007E; // ~
pub const ASCII_LEFT_SQUARE_BRACKET: u16 = 0x005B; // [

pub const UNICODE_NBSP: u16 = 0x00A0; // Non-breaking space
pub const UNICODE_EM_DASH: u16 = 0x2014; // Em dash
pub const UNICODE_BULLET: u16 = 0x2022; // Bullet point

// Common control character code points:
/// Null character (NUL)
pub const ASCII_NULL: u16 = 0x0000; 
/// Bell (BEL)
pub const ASCII_BELL: u16 = 0x0007; 
/// Backspace (BS)
pub const ASCII_BACKSPACE: u16 = 0x0008; 
/// Tab (TAB)
pub const ASCII_TAB: u16 = 0x0009; 
/// Line feed (LF)
pub const ASCII_NEWLINE: u16 = 0x000A; 
/// Carriage return (CR)
pub const ASCII_CARRIAGE_RETURN: u16 = 0x000D;
/// ESC
pub const ASCII_ESCAPE: u16 = 0x001B; 

// Common keyboard control character code points:
/// ↑
pub const KEY_UP: u16 = 0x2191;
/// ↓
pub const KEY_DOWN: u16 = 0x2193; 
/// ←
pub const KEY_LEFT: u16 = 0x2190; 
/// →
pub const KEY_RIGHT: u16 = 0x2192; 
/// `H` at the end of escape sequence \X1B[H
pub const KEY_HOME: u16 = 0x0048;
/// `F` at the end of escape sequence \X1B[F
pub const KEY_END: u16 = 0x0046;

/// Determines if a character is printable
pub fn is_printable(ch: u16) -> bool {
    match ch {
        // Printable ASCII characters (0x20-0x7E)
        0x0020..=0x007E => true,
        
        // Common Unicode printable characters
        0x00A0..=0x00FF | // Latin-1 Supplement
        0x2000..=0x206F | // Punctuation
        0x3000..=0x30FF | // CJK symbols and Japanese kana
        0x4E00..=0x9FFF    // Common CJK ideographs
        => true,
        
        _ => false,
    }
}