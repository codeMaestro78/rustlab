//! Hand-written data structures. One module per structure, each with tests.
//!
//! Order: dynamic array → ring buffer → hash map → binary heap → LRU → skiplist.
//! No `std` collections for the structure being implemented.

fn two_sums(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                return Some((i, j));
            }
        }
    }
    None
}

fn main() {
    let nums = vec![2, 7, 11, 15];

    match two_sums(&nums, 9) {
        Some((i, j)) => println!("{}"),
    }
}
