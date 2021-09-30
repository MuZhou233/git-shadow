use log::{self, Level, LevelFilter, Log, Metadata, Record};

/// Set logging level
pub fn init(level: u8) {
    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match level {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    });
}

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        #[cfg(debug_assertions)]
        print!(
            "\u{1B}[{}m{}\u{1B}[0m",
            level_to_color_code(record.level()),
            format!("{:>5}: {}\n", record.level(), record.args())
        );

        #[cfg(not(debug_assertions))]
        println!("{}", record.args())
    }
    fn flush(&self) {}
}

#[allow(dead_code)]
fn level_to_color_code(level: Level) -> u8 {
    match level {
        Level::Error => 31, // Red
        Level::Warn => 93,  // BrightYellow
        Level::Info => 34,  // Blue
        Level::Debug => 32, // Green
        Level::Trace => 90, // BrightBlack
    }
}
