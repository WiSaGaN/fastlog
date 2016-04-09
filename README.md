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

## Example

```rust
#[macro_use]
extern crate log;
extern crate fastlog;

fn main() {
    fastlog::LogBuilder::new().build().unwrap().init().unwrap();
    info!("Hello, world.");
}
```

More examples can be found under `examples` directory.
