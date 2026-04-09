fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let target = 7;

    match binary_search(&arr, target) {
        Some(index) => println!("Index element found {}", index),
        None => println!("Target element {} not found in array", target),
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

fn binary_search_recursive(arr: &[i32], target: i32, left: usize, right: usize) -> Option<usize> {
    if left >= right {
        return None;
    }

    let mid = left + (right - left) / 2;

    if arr[mid] == target {
        return Some(mid);
    } else if arr[mid] < target {
        return binary_search_recursive(arr, target, mid + 1, right);
    } else {
        return binary_search_recursive(arr, target, left, mid);
    }
}

fn binary_search_first(arr: &[i32], target: i32) -> Option<usize> {
    let (mut left, mut right) = (0, arr.len());
    let mut result = None;

    while left < right {
        let mid = left + (right - left) / 2;

        if arr[mid] >= target {
            if arr[mid] == target {
                result = Some(mid);
            }
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    result
}

fn merge_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);

    merge(arr, &left, &right);
}

fn merge(arr: &mut [i32], left: &[i32], right: &[i32]) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}
