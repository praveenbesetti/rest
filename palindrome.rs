fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn main() {
    let string1 = "mams";
    

    if is_palindrome(string1) {
        println!("Is '{}' a palindrome", string1);
    } else {
        println!("'{}' is not a palindrome", string1);
    }
   
}
