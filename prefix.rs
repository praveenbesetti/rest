// Function to find the longest common prefix of a set of strings
fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let mut prefix = String::new();
    for (idx, char) in strings[0].chars().enumerate() {
        if strings[1..].iter().all(|s| s.chars().nth(idx) == Some(char)) {
            prefix.push(char);
        } else {
            break;
        }
    }
    prefix
}

fn main() {
    let strings = vec!["flower", "flow", "flight"];
    let common_prefix = longest_common_prefix(&strings);
    println!("Longest common prefix: {}", common_prefix);
}
