// Recursive function to compute factorial: n! = n * (n-1)!
fn factorial(n: u32) -> u32 {
    if n == 0 {
        1 // Base case: 0! = 1
    } else {
        n * factorial(n - 1) // Recursive call
    }
}

fn main() {
    println!("5! = {}", factorial(5)); // Output: 5! = 120
}

