# fastlog

A high performance Rust library for asynchronous logging

Currently this is still a work in progress.

## Usage

To use fastlog, first add this to your `Cargo.toml`;

```toml
[dependencies]
fastlog = "0.0"
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
}
```

More examples can be found under `examples` directory.
