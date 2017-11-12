//! colors for the debugging commands

/// color type for debug commands
pub type Color = (u8, u8, u8);

/// white color
pub const WHITE:    Color = (0xFF, 0xFF, 0xFF);
/// red color
pub const RED:      Color = (0xFF, 0x00, 0x00);
/// green color
pub const GREEN:    Color = (0x00, 0xFF, 0x00);
/// yellow color
pub const YELLOW:   Color = (0xFF, 0xFF, 0x00);
/// blue color
pub const BLUE:     Color = (0x00, 0x00, 0xFF);
/// teal color
pub const TEAL:     Color = (0x00, 0xFF, 0xFF);
/// purple color
pub const PURPLE:   Color = (0xFF, 0x00, 0xFF);
/// black color
pub const BLACK:    Color = (0x00, 0x00, 0x00);
/// gray color
pub const GRAY:     Color = (0x80, 0x80, 0x80);
