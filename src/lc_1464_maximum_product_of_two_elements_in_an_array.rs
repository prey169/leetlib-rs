pub fn max_product(nums: Vec<i32>) -> i32 {
    let length = nums.len();
    let mut ans = 0;
    for i in 0..length - 1 {
        for j in (i + 1)..(length) {
            let result = (nums[i] - 1) * (nums[j] - 1);
            if result > ans {
                ans = result;
            }
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use crate::max_product;

    #[test]
    fn ex1() {
        assert_eq!(max_product(vec![3, 4, 5, 2]), 12)
    }

    #[test]
    fn ex2() {
        assert_eq!(max_product(vec![1, 5, 4, 5]), 16)
    }
    
    #[test]
    fn ex3() {
        assert_eq!(max_product(vec![3, 7]), 12)
    }
}
