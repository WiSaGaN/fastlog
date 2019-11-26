use std::fs::OpenOptions;
use std::io::Error as IoError;
use std::io::Write;
use std::path::PathBuf;

use crossbeam_channel as channel;
use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};
use time::{get_time, Timespec};

#[derive(Clone, Debug)]
enum LoggerInput {
    LogMsg(String),
    Flush,
    Quit,
}

#[derive(Clone, Debug)]
enum LoggerOutput {
    Flushed,
}

pub struct Logger {
    format: Box<dyn Fn(Timespec, &Record) -> String + Sync + Send>,
    level: LevelFilter,
    queue: channel::Sender<LoggerInput>,
    notification: channel::Receiver<LoggerOutput>,
    worker_thread: Option<std::thread::JoinHandle<()>>,
}

unsafe impl Send for Logger {}
unsafe impl Sync for Logger {}

impl Logger {
    pub fn init(self) -> Result<(), SetLoggerError> {
        log::set_max_level(self.level);
        let boxed = Box::new(self);
        log::set_boxed_logger(boxed)
    }
}

impl Log for Logger {
    #[inline]
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.level >= metadata.level()
    }

    fn log(&self, record: &Record) {
        let log_msg = (self.format)(get_time(), record);
        self.queue
            .send(LoggerInput::LogMsg(log_msg))
            .expect("logger queue closed when logging, this is a bug")
    }

    fn flush(&self) {
        self.queue
            .send(LoggerInput::Flush)
            .expect("logger queue closed when flushing, this is a bug");
        self.notification
            .recv()
            .expect("logger notification closed, this is a bug");
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.queue
            .send(LoggerInput::Quit)
            .expect("logger queue closed before joining logger thread, this is a bug");
        let join_handle = self
            .worker_thread
            .take()
            .expect("logger thread empty when dropping logger, this is a bug");
        join_handle
            .join()
            .expect("failed to join logger thread when dropping logger, this is a bug");
    }
}

pub struct LogBuilder {
    format: Box<dyn Fn(Timespec, &Record) -> String + Sync + Send>,
    capacity: usize,
    level: LevelFilter,
    path: PathBuf,
    header: Vec<String>,
}

impl LogBuilder {
    #[inline]
    pub fn new() -> LogBuilder {
        LogBuilder {
            format: Box::new(|ts: Timespec, record: &Record| {
                format!(
                    "{:?} {}:{}: {}",
                    ts,
                    record.level(),
                    record.module_path().unwrap_or(""),
                    record.args()
                )
            }),
            capacity: 2048,
            level: LevelFilter::Info,
            path: PathBuf::from("./current.log"),
            header: Vec::new(),
        }
    }

    #[inline]
    pub fn format<F: 'static>(&mut self, format: F) -> &mut LogBuilder
    where
        F: Fn(Timespec, &Record) -> String + Sync + Send,
    {
        self.format = Box::new(format);
        self
    }

    #[inline]
    pub fn capacity(&mut self, capacity: usize) -> &mut LogBuilder {
        self.capacity = capacity;
        self
    }

    #[inline]
    pub fn file(&mut self, path: PathBuf) -> &mut LogBuilder {
        self.path = path;
        self
    }

    #[inline]
    pub fn max_log_level(&mut self, level: LevelFilter) -> &mut LogBuilder {
        self.level = level;
        self
    }

    #[inline]
    pub fn header(&mut self, header: Vec<String>) -> &mut LogBuilder {
        self.header = header;
        self
    }

    pub fn build(self) -> Result<Logger, IoError> {
        let (sync_sender, receiver) = channel::bounded(self.capacity);
        let (notification_sender, notification_receiver) = channel::bounded(1);
        let mut writer = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.path)?;
        for line in &self.header {
            writeln!(&mut writer, "{}", line)?;
        }
        let worker_thread = std::thread::Builder::new()
            .name("logger".to_string())
            .spawn(move || loop {
                match receiver.recv() {
                    Ok(LoggerInput::LogMsg(msg)) => {
                        writeln!(&mut writer, "{}", msg).expect("logger write message failed");
                    }
                    Ok(LoggerInput::Flush) => {
                        notification_sender
                            .send(LoggerOutput::Flushed)
                            .expect("logger notification failed");
                    }
                    Ok(LoggerInput::Quit) => {
                        break;
                    }
                    Err(e) => {
                        panic!(
                            "sender closed without sending a Quit first, this is a bug, {}",
                            e
                        );
                    }
                }
            })?;
        Ok(Logger {
            format: self.format,
            level: self.level,
            queue: sync_sender,
            notification: notification_receiver,
            worker_thread: Some(worker_thread),
        })
    }
}

impl Default for LogBuilder {
    #[inline]
    fn default() -> Self {
        LogBuilder::new()
    }
}
