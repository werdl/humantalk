//! # Humantalk
//! Designed to make talking with the user easier
//! [crates.io](https://crates.io/crates/humantalk)
//! [github](https://github.com/werdl/humantalk)
 
/// console crate styling to customise the output of humantalk
/// 
pub use console::{style, Color};
use std::{collections::HashMap, io::Write};

/// version of humantalk, manually updated each release
pub const VERSION: &str = "0.1.1";

use rustc_version::version_meta;

/// severity enum to denote severity of logging
/// 
/// # Examples
/// ```rust
/// use humantalk::{Severity, Config};
/// let config = Config::default();
/// 
/// config.write(Severity::Error, "oh no!"); // non fatal
/// config.write(Severity::Debug, "this will not trigger if compiled with --release");
/// ```
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Debug,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
            Severity::Debug => "debug",
        };
        write!(f, "{}", s)
    }
}

/// Bug report struct, printed at fatal error
#[derive(Debug, Clone)]
pub struct HowToBugReport {
    /// the message to be displayed on crash
    pub message: String,

    /// url where users are pointed
    pub url: String,
}

impl HowToBugReport {
    /// create a new bug report
    pub fn new(message: String, url: String) -> Self {
        HowToBugReport { message, url }
    }
}


/// configuration struct for humantalk
#[derive(Clone, Debug)]
pub struct Config {
    /// colors hashmap for each severity level
    pub colors: HashMap<Severity, Color>,

    /// the bug reporting struct
    pub bug_report: Option<HowToBugReport>,
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
            Color::Color256(code) => *code,
        }
    }
}

impl Config {
    /// create a new configuration, with default colors and no bug report (auto-filled with default values on use)
    pub fn default() -> Config {
        let mut colors = HashMap::new();
        colors.insert(Severity::Error, Color::Red);
        colors.insert(Severity::Warning, Color::Yellow);
        colors.insert(Severity::Info, Color::Green);
        colors.insert(Severity::Debug, Color::Blue);
        Config {
            colors,
            bug_report: None,
        }
    }

    /// create a custom config, with your own colors and bug report. If you just want custom bug report, just use this code (inverse for colors):
    /// ```
    /// use humantalk::{Config, Severity, HowToBugReport};
    /// 
    /// let my_config = Config::custom(
    ///     Config::default().colors,
    ///     HowToBugReport::new(
    ///         "message".to_string(),
    ///         "url".to_string()
    ///     )
    /// );
    /// ```
    pub fn custom(
        colors: HashMap<Severity, Color>,
        bug_report: HowToBugReport,
    ) -> Config {
        Config {
            colors,
            bug_report: Some(bug_report),
        }
    }

    /// find the specified color for a given severity
    pub fn get_color(&self, severity: &Severity) -> Color {
        match self.colors.get(severity) {
            Some(color) => *color,
            None => Color::White,
        }
    }

    /// set the color for the specified severity level
    pub fn set_color(&mut self, severity: Severity, color: Color) {
        let _ = self.colors.remove(&severity);
        self.colors.insert(severity, color);
    }

    /// write a logging message to stdout. if the binary has been compiled with --release, it will not print debug assertions.
    pub fn write(&self, severity: Severity, message: &str) {
        #[cfg(not(debug_assertions))]
        if severity == Severity::Debug {
            return;
        }

        let color = self.get_color(&severity);
        let styled = style(format!("[{}] {}", severity, message)).color256(color.to_color256());
        println!("{}", styled);
    }
    
    /// shorthand for `config.write(Severity::Debug, ...)`
    pub fn debug(&self, message: &str) {
        self.write(Severity::Debug, message);
    }

    /// shorthand for `config.write(Severity::Info, ...)`
    pub fn info(&self, message: &str) {
        self.write(Severity::Info, message);
    }

    /// shorthand for `config.write(Severity::Error, ...)`
    pub fn error(&self, message: &str) {
        self.write(Severity::Error, message);
    } 

    /// shorthand for `config.write(Severity::Warning, ...)`
    pub fn warning(&self, message: &str) {
        self.write(Severity::Warning, message);
    }

    /// get machine info represented as a string. Contains info including OS family, os, arch, rust version, llvm version and humantalk version
    pub fn machine_info(&self) -> String {
        let arch = std::env::consts::ARCH.to_string();
        let os = std::env::consts::OS.to_string();
        let family = std::env::consts::FAMILY.to_string();

        let rustc_info = version_meta().unwrap_or_else(|e: rustc_version::Error| {
            println!("humantalk has failed (with message {e}). Please submit an issue at github.com/werdl/humantalk");

            std::process::exit(-1)
        }
        );

        let llvm_version_string = match rustc_info.llvm_version {
            Some(version) => {
                format!("{}.{}", version.major, version.minor)
            }
            None => "unknown".to_string(),
        };

        format!(
            "{family}-{os}-{arch} - Rust version {}, running on LLVM {}. information stuff generated by humantalk {}",
            rustc_info.short_version_string,
            llvm_version_string,
            VERSION
        )
    }

    /// error fatally, crashing the program. then exits with error code `3`, indincating that erroring out has succeeded
    pub fn fatal_error(&self, message: &str) {
        let bug_report = match self.bug_report.clone() {
            Some(x) => x,
            None => HowToBugReport {
                message: "Oh no! The program has crashed".to_string(),
                url: "the appropriate place".to_string(),
            },
        };
        let styled = style(format!(
            "[FATAL] {}
{}. Please submit a report to {}, along with a copy of this error message, which can also be found in crash_report.log as plaintext.

",
            message, bug_report.message, bug_report.url
        ))
        .red();

        println!("{}", styled);

        println!(
            "{}",
            style(format!("[PLATFORM INFO]\n{}", self.machine_info())).cyan()
        );

        let mut debug_file = std::fs::File::create("crash_report.log").unwrap_or_else(|_| {
            println!("Failed to create debug file - just copy the information displayed above.");

            std::process::exit(-1);
        });

        let _ = debug_file
            .write(format!(
                "[FATAL] {}
{}. Please submit a report to {}, along with a copy of this error message, which can also be found in crash_report.log as plaintext.",
                message, bug_report.message, bug_report.url
            ).as_bytes())
            .unwrap_or_else(|_| {
                println!(
                    "Failed to write to debug file - just copy the information displayed above."
                );

                std::process::exit(-1);
            });

        let _ = debug_file
            .write(format!("\n[PLATFORM INFO]\n{}", self.machine_info()).as_bytes())
            .unwrap_or_else(|_| {
                println!(
                    "Failed to write to debug file - just copy the information displayed above."
                );

                std::process::exit(-1);
            });

        std::process::exit(3)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_write() {
        let config = Config::custom(
            Config::default().colors,
            HowToBugReport::new(
                "Oh no! Humantalk testing has crashed (don't worry it was manually induced)".to_string(),
                "https://github.com/werdl/humantalk".to_string()
            )
        );
        config.write(Severity::Debug, "hello world!");
        config.write(Severity::Info, "hello information world!")
    }
}
