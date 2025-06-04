pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut output: Vec<i32> = vec![];
    let mut sum: i32 = 0;
    for num in &nums {
        sum += num;
        output.push(sum);
    }
    output
}

#[cfg(test)]
mod test {
    use crate::running_sum;

    #[test]
    fn ex1() {
        assert_eq!(running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10])
    }

    #[test]
    fn ex2() {
        assert_eq!(running_sum(vec![1, 1, 1, 1, 1]), vec![1, 2, 3, 4, 5])
    }
}
