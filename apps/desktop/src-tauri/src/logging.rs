//! Minimal local file logger. Privacy-first: logs stay on the device and contain
//! no hardware identifiers or user content beyond operational messages.

use log::{Log, Metadata, Record};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

struct FileLogger {
    file: Mutex<File>,
}

impl Log for FileLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        if let Ok(mut f) = self.file.lock() {
            let _ = writeln!(f, "[{}] {}: {}", record.level(), record.target(), record.args());
        }
    }
    fn flush(&self) {
        if let Ok(mut f) = self.file.lock() {
            let _ = f.flush();
        }
    }
}

pub fn init(dir: &Path) {
    let path = dir.join("vyronix.log");
    if let Ok(file) = OpenOptions::new().create(true).append(true).open(path) {
        let logger = FileLogger { file: Mutex::new(file) };
        if log::set_boxed_logger(Box::new(logger)).is_ok() {
            log::set_max_level(log::LevelFilter::Info);
            log::info!("VYRONIX logging initialised.");
        }
    }
}
