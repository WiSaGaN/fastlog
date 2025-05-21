use lazy_format::lazy_format;
use std::fmt::Display;
use std::thread;

fn main() {
    let owned_string = String::from("world");
    let number = 42;

    // Create a lazy_format object that captures owned data.
    // This object itself should be Send + 'static.
    let formatter: Box<dyn Display + Send + 'static> =
        Box::new(lazy_format!("Hello, {}! The number is {}.", owned_string, number));

    // Demonstrate that it can be sent to another thread.
    let handle = thread::spawn(move || {
        // Format the object in the other thread.
        let formatted_string = formatter.to_string();
        println!("From thread: {}", formatted_string);
        formatted_string
    });

    let result_from_thread = handle.join().expect("Thread panicked");

    println!("In main thread, got back: {}", result_from_thread);

    assert_eq!(result_from_thread, "Hello, world! The number is 42.");
    println!("Successfully created and used an owned formatter across threads!");
}
