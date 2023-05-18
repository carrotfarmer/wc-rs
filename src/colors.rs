use std::fmt::Display;

pub enum Color {
    Red,
    GreenBold,
    Blue,
    MagentaBold,
    Yellow,
}

pub fn colorize<T: Display>(text: T, color: Color) -> String {
    match color {
        Color::Red => format!("\x1b[31m{}\x1b[0m", text),
        Color::GreenBold => format!("\x1b[1;32m{}\x1b[0m", text), 
        Color::Blue => format!("\x1b[34m{}\x1b[0m", text),
        Color::MagentaBold => format!("\x1b[1;35m{}\x1b[0m", text),
        Color::Yellow => format!("\x1b[33m{}\x1b[0m", text),
    }
}
