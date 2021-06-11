use core::fmt;

use log::{set_max_level, Level, LevelFilter, Log};

pub fn init() {
    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).unwrap();

    set_max_level(match option_env!("LOG") {
        Some("error") => LevelFilter::Error,
        Some("warn") => LevelFilter::Warn,
        Some("info") => LevelFilter::Info,
        Some("debug") => LevelFilter::Debug,
        Some("trace") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });
}

struct SimpleLogger;

fn print_in_color(args: fmt::Arguments, color_code: u8) {
    println!("\u{1B}[{}m{}\u{1B}[0m", color_code as u8, args);
}

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        print_in_color(
            format_args!("[{:>5}] {}", record.level(), record.args()),
            level_to_color_code(record.level()),
        )
    }

    fn flush(&self) {}
}

fn level_to_color_code(level: Level) -> u8 {
    match level {
        Level::Error => 31, // Red
        Level::Warn => 93,  // BrightYellow
        Level::Info => 34,  // Blue
        Level::Debug => 32, // Green
        Level::Trace => 90, // BrightBlack
    }
}
