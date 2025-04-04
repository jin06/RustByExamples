// Function to add two integers and return the result
fn add(a: i32, b: i32) -> i32 {
    a + b // The last expression is the return value (no need for `return` keyword)
}

// Function to swap two integers and return them as a tuple
fn swap(x: i32, y: i32) -> (i32, i32) {
    (y, x) // Returns a tuple with swapped values
}

// Function to increase the given integer by 1 using a mutable reference
fn increase(x: &mut i32) {
    *x += 1; // Dereference `x` to modify its value
}

fn main() {
    // Calling `add` function
    let result = add(5, 10);
    println!("5 + 10 = {}", result); // Prints: 5 + 10 = 15

    // Calling `swap` function
    let (a, b) = swap(3, 7);
    println!("Swapped value: a = {}, b = {}", a, b); // Prints: Swapped value: a = 7, b = 3

    // Using a mutable variable and calling `increase` function
    let mut num = 5;
    increase(&mut num);
    println!("Increased value: {}", num); // Prints: Increased value: 6

    // Defining and using a closure (anonymous function) to multiply two numbers
    let multiply = |x: i32, y: i32| x * y;
    println!("3 * 4 = {}", multiply(3, 4)); // Prints: 3 * 4 = 12
}
