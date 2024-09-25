use std::time::{SystemTime, UNIX_EPOCH}; // Import for time handling

fn main() {
    let i = 2;
    print!("Write {} as ", i);
    
    // Match statement to print the corresponding word for the number
    match i {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => {} // Catch-all for other cases (do nothing)
    }

    // Get the current time
    let now = SystemTime::now();
    // Get the current weekday (you can only get the timestamp)
    let timestamp = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
    
    // Simple approximation of day of the week (not perfect, just for illustration)
    let weekday = (timestamp / 86400 % 7) as u8; // 86400 seconds in a day

    // Check the day of the week (0 = Sunday, 6 = Saturday)
    match weekday {
        0 | 6 => {
            println!("It's the weekend");
        }
        _ => {
            println!("It's a weekday");
        }
    }

    // Get the current hour (this is just a rough approximation)
    let hour = (timestamp / 3600 % 24) as u8; // 3600 seconds in an hour

    // Match statement to determine if it's before or after noon
    match hour {
        hour if hour < 12 => println!("It's before noon"),
        _ => println!("It's after noon"),
    }

    // Function to determine the type of the provided value
    fn what_am_i(i: &dyn std::any::Any) {
        // Use type downcasting to check the type
        if let Some(_) = i.downcast_ref::<bool>() {
            println!("I'm a bool");
        } else if let Some(_) = i.downcast_ref::<i32>() {
            println!("I'm an int");
        } else {
            println!("Don't know type");
        }
    }

    // Call the type-checking function with different types
    what_am_i(&true);
    what_am_i(&1);
    what_am_i(&"hey");
}