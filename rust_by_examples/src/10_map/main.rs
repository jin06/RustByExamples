use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    
    // Insert key-value pairs into the HashMap
    scores.insert("Alice", 90);
    scores.insert("Bob", 85);

    // Access values using `get()`
    if let Some(score) = scores.get("Alice") {
        println!("Alice's score: {}", score);
    }

    // Iterate over the HashMap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
