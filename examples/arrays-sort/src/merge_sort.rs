

pub fn binary_search_first(arr: &[i32], target: i32) -> Option<usize> {
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
*/
