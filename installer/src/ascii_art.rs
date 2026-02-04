//! Strange and artful ASCII art for the hacker installer
//! These constants are used selectively by effects and render modules.

// Matrix rain characters
pub const MATRIX_CHARS: &[char] = &[
    'ア', 'イ', 'ウ', 'エ', 'オ', 'カ', 'キ', 'ク', 'ケ', 'コ', 'サ', 'シ', 'ス', 'セ', 'ソ', 'タ',
    'チ', 'ツ', 'テ', 'ト', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '@', '#', '$', '%',
    '&', '*', '+', '=', '<', '>',
];

// Glitch characters for future visual effects
#[allow(dead_code)]
pub const GLITCH_CHARS: &[char] = &[
    '░', '▒', '▓', '█', '▄', '▀', '▌', '▐', '■', '□', '▪', '▫', '╳', '╱', '╲', '┃', '━', '┏', '┓',
    '┗', '┛', '╋', '┣', '┫', '◢', '◣', '◤', '◥', '◆', '◇', '○', '●', '◎', '◉', '⊕', '⊗',
];

// Spinner frames
#[allow(dead_code)]
pub const SPINNER_DOTS: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
pub const SPINNER_BRAILLE: &[&str] = &["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"];
#[allow(dead_code)]
pub const SPINNER_BLOCKS: &[&str] = &["▖", "▘", "▝", "▗"];
