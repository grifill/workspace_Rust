
fn main() {
    let arr = [1, 3, 5, 7, 9, 11, 15];
    let target = 7;

    match binary_search(&arr, target) {
        Some(index) => println!("Index element found {}", index),
        None => println!("None element in array"),
    }
}


fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let (mut left, mut right) = (0, arr.len()); // Set search interval

    while left < right {
        let mid = left + (right - left) / 2; // Find median

        if arr[mid] == target {
            return Some(mid); // If found return index
        } else if arr[mid] < target {
            left = mid + 1; // Search right side
        } else {
            right = mid; // Search left side
        }
    }
    None // If not found return None
}