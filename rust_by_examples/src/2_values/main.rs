// The code demonstrates various fundamental Rust operations, including string formatting, arithmetic, and logical expressions.
// The use of println! with formatting placeholders ({}) allows for flexible output formatting for different data types, including strings, integers, floats, and booleans.
fn main() {
    /*
        This line uses the format! 
        macro to concatenate the strings "rust" and "lang" with a hyphen "-" in between. The result is a new String object "rust-lang".
        The format! macro works similarly to println!, but instead of printing to the console, it returns a formatted string. 
        The result of format! is then passed to println!, which prints the string.
     */
    println!("{}", format!("{}-{}", "rust", "lang"));
    // This line performs an integer addition of 1 + 1 and inserts the result into the string "1+1={}" at the {} placeholder.
    println!("1+1={}", 1+1);
    // This line performs floating-point division with 7.0 / 3.0 and inserts the result into the string "7.0/3.0={}" at the {} placeholder.
    println!("7.0/3.0={}", 7.0/3.0);
    // This line evaluates the logical AND expression true && false, which results in false.
    // In Rust (as in many other programming languages), the logical AND operator (&&) returns true only if both operands are true.
    // Since one operand is false, the overall expression evaluates to false.
    println!("{}", true && false);
    // This line evaluates the logical OR expression true || false, which results in true.
    // The logical OR operator (||) returns true if at least one of the operands is true. 
    // Since one operand is true, the overall expression evaluates to true.
    println!("{}", true || false);
    // This line evaluates the logical NOT expression !true, which results in false.
    // The logical NOT operator (!) negates the boolean value. Since the operand is true, the result of !true is false.
    println!("{}", !true);
}