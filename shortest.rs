// Function to find the shortest word in a string of words
fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|&word| word.len())
}

fn main() {
    let test_string = "The quick brown fox jumps over the lazy dog";

    match shortest_word(test_string) {
        Some(shortest) => println!("Shortest word in the string: {}", shortest),
        None => println!("The string is empty"),
    }
}

