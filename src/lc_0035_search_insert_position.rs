pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let length = nums.len();
    if length == 0 {
        return 0;
    }
    let mut min = 0;
    let mut max = length - 1;
    let mut mid = length / 2;

    while nums[mid] != target {
        if mid == min || mid == max {
            if nums[max] < target {
                return (max + 1) as i32;
            } else if nums[min] < target {
                return (min + 1) as i32;
            } else {
                return (min) as i32;
            };
        } else if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] > target {
            max = mid as usize;
            mid = (min + max) / 2;
        } else {
            // if nums[mid] < target {
            min = mid as usize;
            mid = (max + min) / 2;
        };
    }
    mid as i32
}

#[cfg(test)]
mod test {
    use super::search_insert;

    #[test]
    fn ex1() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
    }

    #[test]
    fn ex2() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
    }

    #[test]
    fn ex3() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
    }

    #[test]
    fn ex4() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 0), 0);
    }
}
