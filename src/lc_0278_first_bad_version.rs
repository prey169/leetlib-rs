/*
pub fn first_bad_version(&self, n: i32) -> i32 {
    let mut left: i32 = 1;
    let mut right: i32 = n;
    let mut mid: i32 = 1;

    while left < right {
        mid = left + (right - left) / 2;
        if !self.isBadVersion(mid) {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}
*/
