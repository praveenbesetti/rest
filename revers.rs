fn reverse_string(input: &str) -> String {
    let mut reversed_chars: Vec<char> = input.chars().collect();
    reversed_chars.reverse();
    reversed_chars.iter().collect()
}

fn main() {
    let input_string = "hello";
    let reversed_string = reverse_string(input_string);
    println!("Original string: {}", input_string);
    println!("Reversed string: {}", reversed_string);
}
