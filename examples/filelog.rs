extern crate fastlog;
#[macro_use]
extern crate log;
extern crate time;

use fastlog::LogBuilder;
use log::LogRecord;
use time::{ at, Timespec };

fn init() {
    let format = |ts: Timespec, record: &LogRecord| {
        let tm = at(ts);
        let tm_millisec = tm.tm_nsec / 1_000_000;
        let tm_microsec = tm.tm_nsec / 1_000 - tm_millisec * 1_000;
        format!("{:0>4}{:0>2}{:0>2}T{:0>2}{:0>2}{:0>2}.{:0>3}{:0>3}{:>+03} {}{}/{}:{}[{}] {}",
                tm.tm_year + 1900,
                tm.tm_mon + 1,
                tm.tm_mday,
                tm.tm_hour,
                tm.tm_min,
                tm.tm_sec,
                tm_millisec,
                tm_microsec,
                tm.tm_utcoff/3600,
                std::thread::current().name().unwrap_or_default(),
                record.location().module_path(),
                record.location().file(),
                record.location().line(),
                record.level(),
                record.args())
    };
    let mut log_builder = LogBuilder::new();
    log_builder.format(format);
    let logger = log_builder.build().expect("logger build failed");
    logger.init().expect("set logger failed");
}

fn main() {
    init();
    info!("Hello, world.");
    log::shutdown_logger().unwrap();
}
