#![feature(test)]

extern crate fastlog;
#[macro_use]
extern crate log;
extern crate test;

use test::Bencher;

#[bench]
fn calling_latency(b: &mut Bencher) {
    fastlog::init().unwrap();
    b.iter(|| { info!("Hello, world!"); });
}
