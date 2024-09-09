// This code snippet demonstrates basic variable declarations, type annotations, and printing values in Rust.
fn main() {
    // This line declares a variable a and initializes it with the string literal "initial". 
    // In Rust, string literals are of type &'static str, which is a string slice with a static lifetime.
    let a = "initial";
    println!("{}", a);

    // This line declares a variable a2 with an explicit type annotation &str and initializes it with the string literal "initial2".
    let a2: &str = "initial2";
    println!("{}", a2);

    // This line declares a tuple (b, c) where b and c are both of type i32. It initializes the tuple with the values (1, 2).
    let (b, c):(i32, i32) = (1, 2);
    println!("b: {}, c: {}", b, c);

    // This line declares a variable d and initializes it with the boolean value true.
    let d = true;
    println!("{}", d);

    // This line declares a variable e with an explicit type annotation i32 and initializes it with the integer value 0.
    // Rust enforces strict rules regarding variable initialization to ensure safety and prevent undefined behavior. 
    let e: i32 = 0;
    println!("{}", e);
}