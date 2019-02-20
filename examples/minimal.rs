extern crate fastlog;
#[macro_use]
extern crate log;

fn main() {
    fastlog::LogBuilder::new().build().unwrap().init().unwrap();
    info!("Hello, world.");
    log::logger().flush();
}
