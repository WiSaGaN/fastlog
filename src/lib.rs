extern crate log;
extern crate time;

mod details;

use log::{ Log, LogLevelFilter, LogMetadata, LogRecord, SetLoggerError };
use std::fs::OpenOptions;
use std::io::Error as IoError;
use std::io::Write;
use std::path::PathBuf;
use time::{ get_time, Timespec };

use details::Queue as BoundedQueue;

#[derive(Clone, Debug)]
enum LoggerInput {
    LogMsg(String),
    Quit,
}

pub struct Logger {
    format: Box<Fn(Timespec, &LogRecord) -> String + Sync + Send>,
    level: LogLevelFilter,
    queue: BoundedQueue<LoggerInput>,
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
        // TODO: add full policy: drop? or block?
        self.queue.push(LoggerInput::LogMsg(log_msg)).unwrap();
        self.worker_thread.as_ref().expect("logger thread empty, this is a bug").thread().unpark();
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.queue.push(LoggerInput::Quit).unwrap();
        self.worker_thread.as_ref().expect("logger thread empty, this is a bug").thread().unpark();
        let join_handle = self.worker_thread.take().expect("logger thread empty, this is a bug");
        join_handle.join().expect("failed to join logger thread");
    }
}

pub struct LogBuilder {
    format: Box<Fn(Timespec, &LogRecord) -> String + Sync + Send>,
    capacity: usize,
    level: LogLevelFilter,
    path: PathBuf,
}

impl LogBuilder {
    pub fn new() -> LogBuilder {
        LogBuilder {
            format: Box::new(|ts: Timespec, record: &LogRecord| {
                format!("{:?} {}:{}: {}", ts, record.level(),
                record.location().module_path(), record.args())
            }),
            capacity: 2048,
            level: LogLevelFilter::Info,
            path: PathBuf::from("./current.log"),
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

    pub fn build(self) -> Result<Logger, IoError> {
        let queue = BoundedQueue::with_capacity(self.capacity);
        let queue_receiver = queue.clone();
        let mut writer = try!(OpenOptions::new()
                              .create(true)
                              .append(true)
                              .open(self.path));
        let worker_thread = try!(std::thread::Builder::new().
            name("logger".to_string()).
            spawn(move || loop {
                match queue_receiver.pop() {
                    Some(LoggerInput::LogMsg(msg)) => {
                        // TODO: handle error
                        writeln!(&mut writer, "{}", msg).expect("writeln failed");
                    },
                    Some(LoggerInput::Quit) => {
                        break;
                    },
                    None => {
                        std::thread::park();
                    },
                }
            }));
        Ok(Logger{
            format: self.format,
            level: self.level,
            queue: queue,
            worker_thread: Some(worker_thread)
        })
    }
}
