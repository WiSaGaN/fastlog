extern crate log;
extern crate time;

use log::{Log, LogLevelFilter, LogMetadata, LogRecord, SetLoggerError};
use std::fs::OpenOptions;
use std::io::Error as IoError;
use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc::{SyncSender, sync_channel};
use time::{get_time, Timespec};


#[derive(Clone, Debug)]
enum LoggerInput {
    LogMsg(String),
    Quit,
}

pub struct Logger {
    format: Box<Fn(Timespec, &LogRecord) -> String + Sync + Send>,
    level: LogLevelFilter,
    queue: SyncSender<LoggerInput>,
    worker_thread: Option<std::thread::JoinHandle<()>>,
}

unsafe impl Send for Logger {}
unsafe impl Sync for Logger {}

impl Logger {
    fn max_log_level(&self) -> LogLevelFilter {
        self.level
    }

    pub fn init(self) -> Result<(), SetLoggerError> {
        log::set_logger(|max_log_level| {
            max_log_level.set(self.max_log_level());
            Box::new(self)
        })
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        self.level >= metadata.level()
    }
    fn log(&self, record: &LogRecord) {
        let log_msg = (self.format)(get_time(), record);
        self.queue
            .send(LoggerInput::LogMsg(log_msg))
            .expect("logger queue closed when logging, this is a bug");
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.queue
            .send(LoggerInput::Quit)
            .expect("logger queue closed before joining logger thread, this is a bug");
        let join_handle = self.worker_thread
            .take()
            .expect("logger thread empty when dropping logger, this is a bug");
        join_handle.join()
            .expect("failed to join logger thread when dropping logger, this is a bug");
    }
}

pub struct LogBuilder {
    format: Box<Fn(Timespec, &LogRecord) -> String + Sync + Send>,
    capacity: usize,
    level: LogLevelFilter,
    path: PathBuf,
    header: Vec<String>,
}

impl LogBuilder {
    pub fn new() -> LogBuilder {
        LogBuilder {
            format: Box::new(|ts: Timespec, record: &LogRecord| {
                format!("{:?} {}:{}: {}",
                        ts,
                        record.level(),
                        record.location().module_path(),
                        record.args())
            }),
            capacity: 2048,
            level: LogLevelFilter::Info,
            path: PathBuf::from("./current.log"),
            header: Vec::new(),
        }
    }

    pub fn format<F: 'static>(&mut self, format: F) -> &mut LogBuilder
        where F: Fn(Timespec, &LogRecord) -> String + Sync + Send
    {
        self.format = Box::new(format);
        self
    }

    pub fn capacity(&mut self, capacity: usize) -> &mut LogBuilder {
        self.capacity = capacity;
        self
    }

    pub fn file(&mut self, path: PathBuf) -> &mut LogBuilder {
        self.path = path;
        self
    }

    pub fn max_log_level(&mut self, level: LogLevelFilter) -> &mut LogBuilder {
        self.level = level;
        self
    }

    pub fn header(&mut self, header: Vec<String>) -> &mut LogBuilder {
        self.header = header;
        self
    }

    pub fn build(self) -> Result<Logger, IoError> {
        let (sync_sender, receiver) = sync_channel(self.capacity);
        let mut writer = try!(OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.path));
        for line in &self.header {
            try!(writeln!(&mut writer, "{}", line));
        }
        let worker_thread = try!(std::thread::Builder::new()
            .name("logger".to_string())
            .spawn(move || loop {
                match receiver.recv() {
                    Ok(LoggerInput::LogMsg(msg)) => {
                        writeln!(&mut writer, "{}", msg).expect("logger write message failed");
                    }
                    Ok(LoggerInput::Quit) => {
                        break;
                    }
                    Err(_) => {
                        panic!("sender closed without sending a Quit first, this is a bug");
                    }
                }
            }));
        Ok(Logger {
            format: self.format,
            level: self.level,
            queue: sync_sender,
            worker_thread: Some(worker_thread),
        })
    }
}
