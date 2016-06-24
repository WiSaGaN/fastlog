# fastlog

[![crates.io](http://meritbadge.herokuapp.com/fastlog)](https://crates.io/crates/fastlog)
[![Build Status](https://travis-ci.org/WiSaGaN/fastlog.svg?branch=master)](https://travis-ci.org/WiSaGaN/fastlog)
[![Coverage Status](https://coveralls.io/repos/github/WiSaGaN/fastlog/badge.svg?branch=master)](https://coveralls.io/github/WiSaGaN/fastlog?branch=master)

A high performance Rust library for asynchronous logging

Currently this is still a work in progress.

## Usage

Fastlog requires a minimum rustc version of 1.4.0.

To use fastlog, first add this to your `Cargo.toml`;

```toml
[dependencies]
fastlog = "0.1"
log = "0.3"
```

Then, add this to your crate root:

```rust
extern crate fastlog;
#[marcro_use]
extern crate log
```

Finally initialize the logger, and use it like any other log implementation.

## Example

```rust
extern crate fastlog;
#[macro_use]
extern crate log;

fn main() {
    fastlog::LogBuilder::new().build().unwrap().init().unwrap();
    info!("Hello, world.");
    log::shutdown_logger().unwrap();
}
```

More examples can be found under `examples` directory.
