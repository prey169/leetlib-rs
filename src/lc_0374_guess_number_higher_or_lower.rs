/* I commented this out to avoid any issues with guess
unsafe fn guessNumber(n: i32) -> i32 {
    let mut right: i32 = 1;
    let mut left: i32 = n;
    loop {
        let mid: i32 = right + (left - right) / 2;
        match guess(mid) {
            0 => return mid,
            1 => {
                right = mid + 1;
            }
            _ => {
                left = mid - 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(guessNumber(10), 6);
    }
}
*/
