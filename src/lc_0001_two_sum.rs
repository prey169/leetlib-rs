pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut count = 0;
    for i in &nums {
        let num_to_find = target - i;
        if let Some(index) = nums.iter().position(|num| *num == num_to_find) {
            let index: i32 = index.try_into().unwrap();
            if count == index {
                count += 1;
                continue;
            }
            return vec![count, index];
        }
        count += 1;
    }
    vec![0, 0]
}

#[cfg(test)]
mod test {
    use crate::two_sum;

    #[test]
    fn ex1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15, 9], 9), vec![0, 1])
    }
}
