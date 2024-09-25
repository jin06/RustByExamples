/// This program performs several checks on numbers.
/// It checks if specific numbers are even or odd,
/// whether they are divisible by a certain value,
/// and provides information about the number of digits in a given number.
fn main() {
    // Check if 7 is even or odd
    if 7 % 2 == 0 {
        println!("7 is even");
    } else {
        println!("7 is odd");
    }

    // Check if 8 is divisible by 4
    if 8 % 4 == 0 {
        println!("8 is divisible by 4");
    }

    // Check if either 8 or 7 is even
    if 8 % 2 == 0 || 7 % 2 == 0 {
        println!("either 8 or 7 are even");
    }

    // Using let for conditional checking
    let num = 9;
    if num < 0 {
        println!("{} is negative", num);
    } else if num < 10 {
        println!("{} has 1 digit", num);
    } else {
        println!("{} has multiple digits", num);
    }
}
