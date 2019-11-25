extern crate fastlog;
#[macro_use]
extern crate log;
extern crate time;

use fastlog::LogBuilder;
use log::Record;
use time::{at, get_time, Timespec};

fn init() {
    let format = |ts: Timespec, record: &Record| {
        let tm = at(ts);
        let tm_millisec = tm.tm_nsec / 1_000_000;
        let tm_microsec = tm.tm_nsec / 1_000 - tm_millisec * 1_000;
        format!(
            "{:0>4}{:0>2}{:0>2}T{:0>2}{:0>2}{:0>2}.{:0>3}{:0>3}{:>+03} {}{}/{}:{}[{}] {}",
            tm.tm_year + 1900,
            tm.tm_mon + 1,
            tm.tm_mday,
            tm.tm_hour,
            tm.tm_min,
            tm.tm_sec,
            tm_millisec,
            tm_microsec,
            tm.tm_utcoff / 3600,
            std::thread::current().name().unwrap_or_default(),
            record.module_path().unwrap_or(""),
            record.file().unwrap_or(""),
            record.line().unwrap_or(0),
            record.level(),
            record.args()
        )
    };
    let mut log_builder = LogBuilder::new();
    log_builder.format(format);
    log_builder.capacity(1024 * 1024);
    let logger = log_builder.build().expect("logger build failed");
    logger.init().expect("set logger failed");
}

fn main() {
    init();
    let start = get_time();
    let num = 100000;
    for i in 0..num {
        info!("Hello, world, this is the {}th time I am logging.", i);
    }
    let end = get_time();
    let latency = (end - start) / num;
    println!("{:?}", latency);
    log::logger().flush();
}
