pub fn merge_sort_iterative(arr: &mut Vec<i32>) {
    let mut step = 1;
    let len = arr.len();

    while step < len {
        let mut temp = arr.clone();

        let mut left = 0;
        while left < len {
            let mid = usize::min(left + step, len);
            let right = usize::min(left + 2 * step, len);

            let (mut i, mut j, mut k) = (left, mid, left);
            
            while i < mid && j < right {
                if arr[i] <= arr[j] {
                    temp[k] = arr[i];
                    i += 1;
                } else {
                    temp[k] = arr[j];
                    j += 1;
                }
                k += 1;
            }

            while i < mid {
                temp[k] = arr[i];
                i += 1;
                k += 1;
            }

            while j < right {
                temp[k] = arr[j];
                j += 1;
                k += 1;
            }

            left += 2 * step;
        }

        arr.copy_from_slice(&temp);
        step *= 2;
    }
}
