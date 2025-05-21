# fastlog

[![crates.io](http://meritbadge.herokuapp.com/fastlog)](https://crates.io/crates/fastlog)
[![Build Status](https://travis-ci.org/WiSaGaN/fastlog.svg?branch=master)](https://travis-ci.org/WiSaGaN/fastlog)
[![Coverage Status](https://coveralls.io/repos/github/WiSaGaN/fastlog/badge.svg?branch=master)](https://coveralls.io/github/WiSaGaN/fastlog?branch=master)

A high performance Rust library for asynchronous logging

Currently this is still a work in progress.

## Usage

Fastlog requires a minimum rustc version of 1.32.0.

To use fastlog, first add this to your `Cargo.toml`;

```toml
[dependencies]
fastlog = "0.2"
log = "0.4"
```

Then, add this to your crate root:

```rust
extern crate fastlog;
#[macro_use]
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
    log::logger().flush();
}
```

More examples can be found under `examples` directory.

## Handling Owned Formatting Arguments

The standard `std::fmt::Arguments` type in Rust is tied to the lifetime of the data being formatted. This makes it unsuitable for scenarios where the formatted arguments need to be stored or sent across threads (i.e., require a `'static` lifetime).

For these situations, you can use the `lazy_format` crate. It allows you to create formatting objects that capture their arguments (by moving or cloning them). If all captured arguments are themselves owned and `'static`, the resulting `lazy_format` object can also be `'static`.

**Example:**

```rust
use lazy_format::lazy_format;
use std::fmt::Display;
use std::thread;

fn create_owned_formatter(name: String, count: i32) -> Box<dyn Display + Send + 'static> {
    Box::new(lazy_format!("User: {}, Count: {}", name, count))
}

fn main() {
    let name = String::from("Alice");
    let formatter = create_owned_formatter(name, 42);

    let handle = thread::spawn(move || {
        // This formatter can be sent to another thread
        println!("{}", formatter); // Outputs: "User: Alice, Count: 42"
    });
    handle.join().unwrap();
}
```

First, add `lazy_format` to your `Cargo.toml`:

```toml
[dependencies]
lazy_format = "2.0.3" # Use the latest version
```

This approach provides a flexible way to handle formatting needs where lifetimes would otherwise be an issue.
