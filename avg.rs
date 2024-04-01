fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        let mid = len / 2;
        (arr[mid - 1] as f64 + arr[mid] as f64) / 2.0
    } else {
        arr[len / 2] as f64
    }
}

fn main() {
    let sorted_array = vec![1, 2, 3, 4, 5];
    let median_value = median(&sorted_array);
    println!("Median of the array: {}", median_value);
}
