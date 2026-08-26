extern crate rand;
use rand::RngExt;

mod binary_sort;
mod merge_sort;

fn main() {
    let mut arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
    let target = 3;

    let mut rng = rand::rng();
    let m: u32 = rng.random_range(0..100);

    println!("Random u32: {}", m);

    // Case 1. Binary sort
    match binary_sort::binary_search(&arr, target) {
        Some(index) => println!("Index element found {}", index),
        None => println!("Target element {} not found in array", target),
    }

    // Case 2. Merge sort
    merge_sort::merge_sort_iterative(&mut arr);
    println!("New buff: {:?}", arr);
}
