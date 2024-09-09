fn main() {
    // Initialize a mutable variable 'count' to 0
    let mut count = 0;
    // Infinite loop
    loop {
        // Print the current value of 'count'
        println!("count: {}", count);
        // Increment 'count' by 1
        count += 1;
        // Exit the loop if 'count' equals 3
        if count == 3 {
            break;
        }
    }

    // Initialize a mutable variable 'number' to 0
    let mut number = 0;
    // Another infinite loop
    loop {
        // Increment 'number' by 1
        number += 1;
        // Skip the current iteration if 'number' is even
        if number % 2 == 0 {
            continue;
        }
        // Print the odd number
        println!("Odd number: {}", number);
        // Exit the loop if 'number' is greater than 3
        if number > 3 {
            break;
        }
    }
}