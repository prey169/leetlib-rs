/* https://leetcode.com/problems/partition-array-such-that-maximum-difference-is-k/solutions/6859638/rust-solution-using-iterators-by-frossi-md4o */

pub fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let ans = nums.iter().fold(vec![Vec::new()], |mut acc, num| {
        if num - *acc.last().and_then(|v| v.first()).unwrap_or(&num) > k {
            acc.push(vec![])
        }
        acc.last_mut().unwrap().push(num);
        acc
    });
    ans.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(partition_array(vec![3, 6, 1, 2, 5], 2), 2)
    }

    #[test]
    fn ex2() {
        assert_eq!(partition_array(vec![1, 2, 3], 1), 2)
    }

    #[test]
    fn ex3() {
        assert_eq!(partition_array(vec![2, 2, 4, 5], 0), 3)
    }
}
