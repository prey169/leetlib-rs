use std::collections::HashMap;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut counts = HashMap::new();

    // Count occurrences of each element
    for &num in nums.iter() {
        *counts.entry(num).or_insert(0) += 1;
    }

    // Retain elements with count <= 2
    nums.retain(|&num| {
        let count = counts[&num];
        if count <= 2 {
            true
        } else {
            counts.insert(num, count - 1);
            false
        }
    });
    nums.len().try_into().unwrap()
    }
}
