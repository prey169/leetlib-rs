pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
    let keys = nums
        .iter()
        .enumerate()
        .filter_map(|(index, &value)| {
            if value == key {
                Some(index as i32)
            } else {
                None
            }
        })
        .collect::<Vec<i32>>();
    let mut ans: Vec<i32> = Vec::new();
    for (index, _) in nums.iter().enumerate() {
        for idx in keys.iter() {
            if (index as i32 - idx).abs() <= k {
                ans.push(index as i32);
                break;
            }
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::find_k_distant_indices;

    #[test]
    fn ex1() {
        assert_eq!(
            find_k_distant_indices(vec![3, 4, 9, 1, 3, 9, 5], 9, 1),
            vec![1, 2, 3, 4, 5, 6]
        )
    }

    #[test]
    fn ex2() {
        assert_eq!(
            find_k_distant_indices(vec![2, 2, 2, 2, 2], 2, 2),
            vec![0, 1, 2, 3, 4]
        )
    }
}
