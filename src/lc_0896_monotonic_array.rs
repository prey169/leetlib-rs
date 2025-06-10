pub fn is_monotonic(nums: Vec<i32>) -> bool {
    nums.is_sorted() || nums.iter().rev().is_sorted()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn ex1() {
        assert!(is_monotonic(vec![1, 2, 2, 3]))
    }

    #[test]
    fn ex2() {
        assert!(is_monotonic(vec![6, 5, 4, 4]))
    }

    #[test]
    fn ex3() {
        assert!(!is_monotonic(vec![1, 3, 2]))
    }
}
