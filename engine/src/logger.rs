use std::collections::VecDeque;
use std::sync::{Mutex, OnceLock};
use log::{Log, Metadata, Record, Level};

#[derive(Clone)]
pub struct LogMessage {
    pub level: Level,
    pub message: String,
}

struct Logger {
    records: Mutex<VecDeque<LogMessage>>,
    max_logs: usize
}

static LOGGER: OnceLock<Logger> = OnceLock::new();

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.target().starts_with("smq_engine") | metadata.target().starts_with("smq_editor")
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let msg_text = format!("{}", record.args());
            
            match record.level() {
                Level::Error => eprintln!("[ERROR] {}", msg_text),
                Level::Warn => println!("[WARN]  {}", msg_text),
                Level::Info => println!("[INFO]  {}", msg_text),
                Level::Debug => println!("[DEBUG] {}", msg_text),
                Level::Trace => println!("[TRACE] {}", msg_text),
            }

            if let Ok(mut records) = self.records.lock() {
                records.push_back(LogMessage {
                    level: record.level(),
                    message: msg_text,
                });
                
                while records.len() > self.max_logs {
                    records.pop_front();
                }
            }
        }
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), log::SetLoggerError> {
    let logger = LOGGER.get_or_init(|| Logger {
        records: Mutex::new(VecDeque::new()),
        max_logs: 1000
    });
    log::set_logger(logger)?;
    
    log::set_max_level(log::LevelFilter::Info); 
    Ok(())
}

pub fn get_logs() -> Vec<LogMessage> {
    if let Some(logger) = LOGGER.get() {
        if let Ok(records) = logger.records.lock() {
            return records.iter().cloned().collect();
        }
    }
    Vec::new()
}

pub fn get_logs_as_string() -> String {
    let logs = get_logs();
    let mut clipboard_text = String::with_capacity(logs.len() * 50);
    
    for log in logs {
        let prefix = match log.level {
            Level::Error => "[ERROR] ",
            Level::Warn =>  "[WARN]  ",
            Level::Info =>  "[INFO]  ",
            Level::Debug => "[DEBUG] ",
            Level::Trace => "[TRACE] ",
        };
        clipboard_text.push_str(&format!("{}{}\n", prefix, log.message));
    }
    
    clipboard_text
}

pub fn clear_logs() {
    if let Some(logger) = LOGGER.get() {
        if let Ok(mut records) = logger.records.lock() {
            records.clear();
        }
    }
}