pub use console::{style, Color};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Debug,
}

pub struct Config {
    pub default_severity: Severity,
    colors: HashMap<Severity, Color>,
}

trait ColorToColor256 {
    fn to_color256(&self) -> u8;
}

impl ColorToColor256 for Color {
    fn to_color256(&self) -> u8 {
        match self {
            Color::Black => 0,
        Color::Red => 1,
        Color::Green => 2,
        Color::Yellow => 3,
        Color::Blue => 4,
        Color::Magenta => 5,
        Color::Cyan => 6,
        Color::White => 7,
        Color::Ansi256(code) => *code,
    }
}
        

impl Config {
    pub fn default() -> Config {
        let mut colors = HashMap::new();
        colors.insert(Severity::Error, Color::Red);
        colors.insert(Severity::Warning, Color::Yellow);
        colors.insert(Severity::Info, Color::Green);
        colors.insert(Severity::Debug, Color::Blue);
        Config {
            default_severity: Severity::Info,
            colors,
        }
    }

    pub fn custom(default_severity: Severity, colors: HashMap<Severity, Color>) -> Config {
        Config {
            default_severity,
            colors,
        }
    }

    pub fn get_color(&self, severity: &Severity) -> Color {
        match self.colors.get(severity) {
            Some(color) => *color,
            None => Color::White,
        }
    }

    pub fn set_severity(&mut self, severity: Severity) {
        self.default_severity = severity;
    }

    pub fn set_color(&mut self, severity: Severity, color: Color) {
        self.colors.insert(severity, color);
    }

    pub fn write(&self, severity: Severity, message: &str) {
        let color = self.get_color(&severity);
        let styled = style(message).color256(color.to_color256());
        println!("{}", styled);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
