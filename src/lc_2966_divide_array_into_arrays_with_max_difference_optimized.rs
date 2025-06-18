pub fn divide_array_optimized(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();
    nums.chunks(3)
        .map(|chunk| {
            if chunk[2] - chunk[0] <= k {
                Some(chunk.to_vec())
            } else {
                None
            }
        })
        .collect::<Option<Vec<Vec<i32>>>>()
        .unwrap_or(vec![])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            divide_array(vec![1, 3, 4, 8, 7, 9, 3, 5, 1], 2),
            vec![vec![1, 1, 3], vec![3, 4, 5], vec![7, 8, 9]]
        )
    }

    #[test]
    fn ex2() {
        assert!(divide_array(vec![2, 4, 2, 2, 5, 2], 2).is_empty())
    }
}
