pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut length = nums.len();
    let mut count = 0;

    while count < length {
        if nums[count] == 0 {
            nums.remove(count);
            nums.push(0);
            length -= 1;
            continue;
        }
        count += 1;
    }
}
#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn ex1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0])
    }
}
