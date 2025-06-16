pub fn minimize_max(nums: Vec<i32>, p: i32) -> i32 {
    fn get_matches(d: i32, nums: &[i32], len: usize, p: i32) -> i32 {
        let mut index: usize = 1;
        let mut count = 0;
        while index < len && count < p {
            if nums[index] - nums[index - 1] <= d {
                count += 1;
                index += 2;
            } else {
                index += 1;
            }
        }
        count
    }
    let len = nums.len();
    let mut nums = nums.clone();
    nums.sort_unstable();
    let mut min_ans = 0;
    let mut maximum_ans = nums[len - 1] - nums[0];
    let mut mid: i32;
    while min_ans < maximum_ans {
        mid = min_ans + (maximum_ans - min_ans) / 2;
        println!("{mid}");
        if get_matches(mid, &nums, len, p) >= p {
            maximum_ans = mid;
        } else {
            min_ans = mid + 1;
        }
    }

    min_ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(minimize_max(vec![10, 1, 2, 7, 1, 3], 2), 1)
    }

    #[test]
    fn ex2() {
        assert_eq!(minimize_max(vec![4, 2, 1, 2], 1), 0)
    }
}
