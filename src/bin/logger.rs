use std::sync::Mutex;

use linefeed::{Reader, Terminal};
use linefeed::reader::LogSender;
use log::{set_boxed_logger, set_max_level, Level, LevelFilter, Log, Metadata,
          Record};

/// Initializes the logger.
pub fn init<Term: Terminal>(
    reader: &mut Reader<Term>,
    level: LevelFilter,
) -> bool {
    let r = set_boxed_logger(Box::new(Logger {
        level,
        sender: Mutex::new(reader.get_log_sender()),
    }));
    if r.is_ok() {
        set_max_level(level);
        true
    } else {
        false
    }
}

struct Logger {
    level: LevelFilter,
    sender: Mutex<LogSender>,
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let level = match record.level() {
            Level::Error => "ERR",
            Level::Warn => "WRN",
            Level::Info => "INF",
            Level::Debug => "DBG",
            Level::Trace => "TRC",
        };

        writeln!(self.sender.lock().unwrap(), "[{}] {}", level, record.args(),)
            .ok();
    }

    fn flush(&self) {
        // This is a no-op.
    }
}
