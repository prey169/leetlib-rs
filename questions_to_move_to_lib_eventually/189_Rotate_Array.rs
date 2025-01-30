impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let mut count = 0;
        while count < k {
            count = count + 1;
            let item = nums.pop().unwrap();
            nums.insert(0, item);
        }
    }
}
