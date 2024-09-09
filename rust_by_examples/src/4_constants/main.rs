// Define a constant string `S` with the value "constant".
// Constants in Rust are immutable and have a 'static lifetime.
const S: &str = "constant";

fn main() {
    // Define a constant floating-point number `N` with the value 50,000,000.0.
    const N: f64 = 50000000.0;
    
    // Define another constant `D` which is the result of the division of 3e20 by `N`.
    // The value of `D` will be calculated at compile time.
    const D: f64 = 3e20 / N;
    
    // Print the value of `S` ("constant") to the console.
    println!("{}", S);
    
    // Print the value of `N` (50,000,000.0) to the console.
    println!("{}", N);
    
    // Print the value of `D` (which is 6e12 or 6 trillion) to the console.
    println!("{}", D);
}