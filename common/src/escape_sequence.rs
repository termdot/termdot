/// #### Esc[0m
/// - Reset **all** modes (styles and colors).
pub const ESC0M: &str = "\u{001b}[0m";

/// #### Esc[1m
/// - Set **bold** mode.
pub const ESC1M: &str = "\u{001b}[1m";

/// #### Esc[2m
/// - Set **dim/faint** mode.
pub const ESC2M: &str = "\u{001b}[2m";

/// #### Esc[3m
/// - Set **italic** mode.
pub const ESC3M: &str = "\u{001b}[3m";

/// #### Esc[4m
/// - Set **underline** mode.
pub const ESC4M: &str = "\u{001b}[4m";

/// #### Esc[5m
/// - Set **blinking** mode.
pub const ESC5M: &str = "\u{001b}[5m";

/// #### Esc[7m
/// - Set **inverse/reverse** mode.
pub const ESC7M: &str = "\u{001b}[7m";

/// #### Esc[8m
/// - Set **hidden/visible** mode.
pub const ESC8M: &str = "\u{001b}[8m";

/// #### Esc[9m
/// - Set **strikethrough** mode.
pub const ESC9M: &str = "\u{001b}[9m";

/// #### Esc[22m
/// - Reset **bold/dim/faint** mode.
pub const ESC22M: &str = "\u{001b}[22m";

/// #### Esc[23m
/// - Reset **italic** mode.
pub const ESC23M: &str = "\u{001b}[23m";

/// #### Esc[24m
/// - Reset **underline** mode.
pub const ESC24M: &str = "\u{001b}[24m";

/// #### Esc[25m
/// - Reset **blinking** mode.
pub const ESC25M: &str = "\u{001b}[25m";

/// #### Esc[27m
/// - Reset **inverse/reverse** mode.
pub const ESC27M: &str = "\u{001b}[27m";

/// #### Esc[28m
/// - Reset **hidden/visible** mode.
pub const ESC28M: &str = "\u{001b}[28m";

/// #### Esc[29m
/// - Reset **strikethrough** mode.
pub const ESC29M: &str = "\u{001b}[29m";

/// #### Esc[J
/// - **Erase the display** ( same as Esc[0J ).
pub const ESCJ: &str = "\u{001b}[J";

/// #### Esc[0J
/// - Erase **from cursor** until **end of screen**.
pub const ESC0J: &str = "\u{001b}[0J";

/// #### Esc[1J
/// - Erase **from cursor** to **beginning of screen**.
pub const ESC1J: &str = "\u{001b}[1J";

/// #### Esc[2J
/// - Erase **entire screen**.
pub const ESC2J: &str = "\u{001b}[2J";

/// #### Esc[3J
/// - Erase **saved lines**.
pub const ESC3J: &str = "\u{001b}[3J";

/// #### Esc[K
/// - Erase **in line** ( same as Esc[0K ).
pub const ESCK: &str = "\u{001b}[K";

/// #### Esc[0K
/// - Erase *from cursor** to **end of line**.
pub const ESC0K: &str = "\u{001b}[0K";

/// #### Esc[1K
/// - Erase *start of line to the cursor**.
pub const ESC1K: &str = "\u{001b}[1K";

/// #### Esc[2K
/// - Erase *the entire line**.
pub const ESC2K: &str = "\u{001b}[2K";

/// #### Esc[H
/// - Move cursor to the home position **(0, 0)**.
pub const ESCH: &str = "\u{001b}[H";

/// #### Esc[s
/// - Save cursor position(DEC).
pub const ESC7: &str = "\u{001b} 7";

/// #### Esc[s
/// - Restore cursor to the last saved position(DEC).
pub const ESC8: &str = "\u{001b} 8";

/// #### Esc[s
/// - Save cursor position(SCO).
pub const ESCS: &str = "\u{001b}[s";

/// #### Esc[s
/// - Restore cursor to the last saved position(SCO).
pub const ESCU: &str = "\u{001b}[u";
