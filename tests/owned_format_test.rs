use lazy_format::lazy_format;
use std::fmt::Display;
use std::thread;

#[test]
fn test_owned_format_across_threads() {
    let owned_string = String::from("Rustacean");
    let value = 100;

    // Create a lazy_format object capturing owned data.
    // Ensure it's Send + 'static by boxing it.
    let formatter: Box<dyn Display + Send + 'static> =
        Box::new(lazy_format!("Greetings, {}! Your score is {}.", owned_string, value));

    // Move to another thread and format.
    let handle = thread::spawn(move || {
        formatter.to_string()
    });

    let result_from_thread = handle.join().expect("Thread panicked during test");

    assert_eq!(result_from_thread, "Greetings, Rustacean! Your score is 100.");
}

#[test]
fn test_owned_format_static_lifetime() {
    // This test demonstrates that if all inputs are 'static, the formatter can be 'static.
    fn create_static_formatter() -> Box<dyn Display + Send + 'static> {
        Box::new(lazy_format!("This is a static message from {}.", "static data"))
    }

    let formatter = create_static_formatter();
    
    // Move to another thread
    let handle = thread::spawn(move || {
        formatter.to_string()
    });

    let result = handle.join().unwrap();
    assert_eq!(result, "This is a static message from static data.");
}

#[test]
fn test_format_with_owned_moved_value() {
    let my_data = String::from("owned and moved");

    // my_data is moved into the formatter
    let formatter: Box<dyn Display + Send + 'static> = 
        Box::new(lazy_format!("Data: {}", my_data)); 
    
    // If my_data was not moved, this would be a compile error:
    // drop(my_data); // uncommenting this should cause a compile error if lazy_format doesn't take ownership

    let handle = thread::spawn(move || {
        formatter.to_string()
    });
    
    let result = handle.join().unwrap();
    assert_eq!(result, "Data: owned and moved");
}
