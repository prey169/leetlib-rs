pub fn array_sign(nums: Vec<i32>) -> i32 {
    let mut sum = 1;
    if nums.is_empty() {
        return 0;
    }
    for n in nums {
        match n {
            0 => return 0,
            ..0 => sum *= -1,
            1.. => continue,
        }
    }
    match sum {
        0 => 0,
        1.. => 1,
        ..0 => -1,
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn ex1() {
        assert_eq!(array_sign(vec![-1, -2, -3, -4, 3, 2, 1]), 1)
    }

    #[test]
    fn ex2() {
        assert_eq!(array_sign(vec![1, 2, 5, 0, -3]), 0)
    }

    #[test]
    fn ex3() {
        assert_eq!(
            array_sign(vec![
                41, 65, 14, 80, 20, 10, 55, 58, 24, 56, 28, 86, 96, 10, 3, 84, 4, 41, 13, 32, 42,
                43, 83, 78, 82, 70, 15, -41
            ]),
            -1
        )
    }
}
