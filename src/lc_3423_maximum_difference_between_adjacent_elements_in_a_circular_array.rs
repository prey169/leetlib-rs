pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if nums.is_empty() {
        return 0;
    }
    let mut sum = i32::MIN;
    let mut count = 0;
    let mut first: i32 = 0;
    for num in nums.clone() {
        if count == 0 {
            first = num;
        }
        if count == len - 1 {
            if (num - first) > sum {
                sum = num - first;
            }
            if (first - num) > sum {
                sum = first - num;
            };
            break;
        } else {
            if (num - nums[count + 1]) > sum {
                sum = num - nums[count + 1];
            }
            if (nums[count + 1] - num) > sum {
                sum = nums[count + 1] - num;
            }
        }
        count += 1;
    }
    sum
}
#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn ex1() {
        assert_eq!(max_adjacent_distance(vec![1, 2, 4]), 3)
    }

    #[test]
    fn ex2() {
        assert_eq!(max_adjacent_distance(vec![-5, -10, -5]), 5)
    }

    #[test]
    fn ex3() {
        assert_eq!(max_adjacent_distance(vec![-4, -2, -3]), 2)
    }
}
