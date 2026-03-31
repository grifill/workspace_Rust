
fn main() {
    let arr = [1, 3, 5, 7, 9, 11, 15];
    let target = 7;

    match binary_search(&arr, target) {
        Some(index) => println!("Index element found {}", index),
        None => println!("None element in array"),
    }
}


fn binary_search(arr: &[i32], target: i32) -> Option<usize> {

    // Set search interval
    let (mut left, mut right) = (0, arr.len());

    while left < right {
        
        // Find median
        let mid = left + (right - left) / 2;

        // If found return index
        if arr[mid] == target {
            return Some(mid);
        }

        // Search right side
        else if arr[mid] < target {
            left = mid + 1;
        } 
        
        // Search left side
        else {
            right = mid;
        }
    }
    
    // If not found return None
    None
}