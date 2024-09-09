fn main() {
    // Loop through a range from 1 to 2 (1..3 is exclusive of the upper bound)
    for i in 1..3 {
        println!("i: {}", i);
    }

    // Loop through a range from 1 to 3 (1..=3 is inclusive of the upper bound)
    for i in 1..=3 {
        println!("i: {}", i);
    }

    // Array of integers
    let numbers = [1, 2, 3];

    // Iterate through each element in the array using the .iter() method
    for num in numbers.iter() {
        println!("Number: {}", num);
    }

    // Vector of integers
    let vec = vec![10, 20, 30];

    // Iterate through each element in the vector by reference
    for element in &vec {
        println!("Element: {}", element);
    }

    // Vector of strings
    let strings = vec!["a", "b", "c"];

    // Iterate through the vector using .enumerate() to get both the index and value
    for (index, value) in strings.iter().enumerate() {
        println!("Index: {} Value: {}", index, value);
    }
}