use colored::*;
use log::{ Level, LevelFilter, Metadata, Record };

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let message = match record.level() {
                Level::Error => record.args().to_string().red().bold(),
                Level::Warn => record.args().to_string().yellow().bold(),
                Level::Info => record.args().to_string().green(),
                Level::Debug => record.args().to_string().blue(),
                Level::Trace => record.args().to_string().magenta(),
            };
            println!("[{}] {}", record.level().to_string().bold(), message);
        }
    }

    fn flush(&self) {}
}

pub fn initialize() {
    log::set_logger(&Logger)
        .map(|()| log::set_max_level(LevelFilter::Debug))
        .expect("Failed to initialize logger");
}
