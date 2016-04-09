#[macro_use]
extern crate log;
extern crate fastlog;

fn main() {
    fastlog::LogBuilder::new().build().unwrap().init().unwrap();
    info!("Hello, world.");
}
