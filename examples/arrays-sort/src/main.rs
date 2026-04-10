mod binary_sort;

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let target = 7;

    match binary_sort::binary_search(&arr, target) {
        Some(index) => println!("Index element found {}", index),
        None => println!("Target element {} not found in array", target),
    }
}

