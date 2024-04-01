// Function to check if a given number is prime
fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=num / 2 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let number = 13;
    if is_prime(number) {
        println!("{} is prime", number);
    } else {
        println!("{} is not prime", number);
    }
}
