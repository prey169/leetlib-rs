pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut right: i32 = (nums.len() - 1) as i32;
    let mut left: i32 = 0;
    let mut mid: i32;
    let mut value: i32;

    while left <= right {
        mid = left + (right - left) / 2;
        value = nums[mid as usize];

        if value == target {
            return mid;
        } else if value < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    -1
}

#[cfg(test)]
mod test {
    use super::search;

    #[test]
    fn ex1() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn ex2() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}
