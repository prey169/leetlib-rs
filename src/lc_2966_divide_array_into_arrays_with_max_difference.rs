pub fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    fn process_chunks(v: &[i32], k: i32) -> Option<Vec<Vec<i32>>> {
        if v.is_empty() {
            return Some(vec![]);
        }
        let chunk: Vec<i32> = v.iter().take(3).cloned().collect();
        let mut result = vec![chunk.clone()];
        let min = chunk.iter().min().unwrap();
        let max = chunk.iter().max().unwrap();
        if max - min <= k {
            let next = process_chunks(&v[3..], k);
            if next.is_none() {
                return None;
            } else {
                result.extend(next.unwrap());
            }
        } else {
            return None;
        }
        Some(result)
    }
    let mut nums = nums;
    nums.sort_unstable();
    match process_chunks(&nums, k) {
        Some(ans) => ans,
        None => Vec::new(),
    }
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
