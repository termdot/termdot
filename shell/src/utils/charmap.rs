use wchar::{wch, wchar_t};

/// Common character str
pub const DOT: &str = ".";
pub const SLASH: &str = "/";
pub const CR: &str = "\r";
pub const LF: &str = "\n";
pub const CRLF: &str = "\r\n";
pub const TAB: &str = "\t";
pub const EMPTY: &str = "";
pub const SPACE: &str = " ";

/// Common character code points
pub const ASCII_SPACE: wchar_t = 0x0020; // Space
pub const ASCII_TILDE: wchar_t = 0x007E; // ~
pub const ASCII_LEFT_SQUARE_BRACKET: wchar_t = 0x005B; // [

pub const UNICODE_NBSP: wchar_t = 0x00A0; // Non-breaking space
pub const UNICODE_EM_DASH: wchar_t = 0x2014; // Em dash
pub const UNICODE_BULLET: wchar_t = 0x2022; // Bullet point

// Common control character code points:
/// Null character (NUL)
pub const CTL_NULL: wchar_t = 0x0000; 
/// Bell (BEL)
pub const CTL_BELL: wchar_t = 0x0007; 
/// Backspace (BS)
pub const CTL_BACKSPACE: wchar_t = 0x0008; 
/// Tab (TAB)
pub const CTL_TAB: wchar_t = 0x0009; 
/// Line feed (LF)
pub const CTL_NEWLINE: wchar_t = 0x000A; 
/// Carriage return (CR \r)
pub const CTL_CARRIAGE_RETURN: wchar_t = 0x000D;
/// ESC
pub const CTL_ESCAPE: wchar_t = 0x001B; 
/// Control+C
pub const CTL_SIGINT: wchar_t = 0x0003;
/// Single shift 3
pub const CTL_SS3: wchar_t = wch!('O');

// Common keyboard control character code points:
/// ↑ `A` at the end of escape sequence \x1B[A
pub const KEY_UP: wchar_t = 0x0041;
/// ↓ `B` at the end of escape sequence \x1B[B
pub const KEY_DOWN: wchar_t = 0x0042; 
/// → `C` at the end of escape sequence \x1B[C
pub const KEY_RIGHT: wchar_t = 0x0043; 
/// ← `D` at the end of escape sequence \x1B[D
pub const KEY_LEFT: wchar_t = 0x0044; 
/// `H` at the end of escape sequence \X1B[H
pub const KEY_HOME: wchar_t = 0x0048;
/// `F` at the end of escape sequence \X1B[F
pub const KEY_END: wchar_t = 0x0046;

/// Determines if a character is printable
pub fn is_printable(ch: wchar_t) -> bool {
    match ch {
        // Printable ASCII characters (0x20-0x7E)
        0x0020..=0x007E => true,
        
        // Common Unicode printable characters
        0x00A0..=0x00FF | // Latin-1 Supplement
        0x2000..=0x206F | // Punctuation
        0x3000..=0x30FF | // CJK symbols and Japanese kana
        0x4E00..=0x9FFF   // Common CJK ideographs
        => true,
        
        _ => false,
    }
}

#[inline]
pub fn is_csi_final_byte(ch: wchar_t) -> bool {
    (0x40..=0x7E).contains(&ch)
}