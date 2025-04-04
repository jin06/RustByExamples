fn main() {
    for i in 1..5 {
        // Range 1 to 4 (excluding 5)
        print!("{i} "); // Corrected placeholder
    }
    println!(); // Print a newline

    for i in 1..=5 {
        // Range 1 to 5 (inclusive)
        print!("{i} "); // Corrected placeholder
    }
    println!(); // Print a newline

    for c in 'a'..='d' {
        // Includes 'd'
        print!("{c} ");
    }
    println!(); // Print a newline

    let arr = [10, 20, 30, 40, 50];

    // Using a range to iterate with indices
    for i in 0..arr.len() {
        println!("Index {}: Value {}", i, arr[i]);
    }

    // Using a range to create a slice
    let slice = &arr[1..4]; // [20, 30, 40]
    println!("Slice: {:?}", slice);
    // Reverse range iteration
    for i in (1..=5).rev() {
        print!("{} ", i);
    }
    println!();

    // Step by 2
    for i in (1..=10).step_by(2) {
        print!("{} ", i);
    }
    println!();

    // Enumerate indices with a range
    let nums = [100, 200, 300];
    for (index, value) in (0..nums.len()).zip(nums.iter()) {
        println!("Index {}: Value {}", index, value);
    }
}
