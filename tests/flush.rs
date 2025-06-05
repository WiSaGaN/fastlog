use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use fastlog::LogBuilder;

#[test]
fn flush_writes_message_to_file() {
    let path = std::env::temp_dir().join(format!(
        "fastlog-test-{}.log",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));

    let mut builder = LogBuilder::new();
    builder.file(path.clone());
    builder
        .build()
        .expect("failed to build logger")
        .init()
        .expect("failed to init logger");

    log::info!("flushed message");
    log::logger().flush();

    let content = fs::read_to_string(&path).expect("failed to read log file");
    assert!(content.contains("flushed message"));

    fs::remove_file(path).expect("failed to remove log file");
}
