use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fastlog::{LogBuilder}; // Logger is not directly used for initialization here
use log::{Level, LevelFilter, Record}; // Import Level for Record builder
use std::path::PathBuf;
// Timespec is not directly needed in the benchmark file if using default formatter
// and directly calling logger.log()

fn bench_logging(c: &mut Criterion) {
    let mut group = c.benchmark_group("logging");
    let sizes = [10, 100, 1000, 10000];

    for size in sizes.iter() {
        group.bench_function(format!("log_{}_chars", size), |b| {
            // Initialize logger for each iteration to ensure flushing and re-creation if necessary,
            // though for /dev/null it might not matter as much.
            // However, the logger has a worker thread, re-creating it each time might be too much overhead.
            // Let's create it once outside b.iter()
            
            let logger = LogBuilder::new()
                .file(PathBuf::from(if cfg!(windows) { "NUL" } else { "/dev/null" }))
                .max_log_level(LevelFilter::Info)
                .build()
                .expect("Failed to build logger for benchmarking.");

            let message_to_log = String::from_utf8(vec![b'a'; *size]).unwrap();
            
            // The Record needs to be created inside b.iter if its content (like timestamp) should vary per iteration,
            // but for benchmarking the logging of a fixed-size message, creating it once is fine.
            // However, format_args! captures its arguments by reference.
            // To be safe and typical for iter, we'll prepare the static parts outside 
            // and only construct the record with the message inside.
            
            b.iter(|| {
                // Construct the record inside the iter block.
                // The `log` crate's Record builder is suitable here.
                let record = Record::builder()
                    .args(format_args!("{}", message_to_log))
                    .level(Level::Info) // Use Level::Info here
                    .target("benchmark_target") // Provide a target
                    .module_path(Some("benchmark_module"))
                    .file(Some("benches/logging_benchmark.rs"))
                    .line(Some(line!())) // Use actual line number
                    .build();
                
                // Call the log method directly on the logger instance
                logger.log(black_box(&record));
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_logging);
criterion_main!(benches);
