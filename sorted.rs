fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

fn main() {
    let sorted_array = [1, 2, 3, 4, 4, 5, 6];
    let target_number = 4;

    match first_occurrence(&sorted_array, target_number) {
        Some(index) => println!("The first occurrence of {} is at index {}", target_number, index),
        None => println!("{} not found in the array", target_number),
    }
}
