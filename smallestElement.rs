use std::collections::HashSet;

fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k == 0 || k > arr.len() {
        return None; 
    }

    let mut unique_set: HashSet<&i32> = HashSet::new();
    for num in arr {
        unique_set.insert(num);
    }

    let mut unique_sorted_arr: Vec<i32> = unique_set.into_iter().cloned().collect();
    unique_sorted_arr.sort();
    
    Some(unique_sorted_arr[k - 1]) 
}

fn main() {
    let arr = [3, 1, 4, 1, 5, 9, 2, 6, 5];
    let k = 7;
    match kth_smallest(&arr, k) {
        Some(smallest) => println!("The {}th smallest element is: {}", k, smallest),
        None => println!("Invalid value of k"),
    }
}
