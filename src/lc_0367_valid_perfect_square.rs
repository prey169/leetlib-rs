pub fn is_perfect_square(num: i32) -> bool {
    let mut n: f64 = 1.0;
    let mut square: f64 = f64::MAX;
    while square > n {
        square = num as f64 / n;
        if square == n {
            return true;
        } else {
            n += 1.0;
        };
    }
    false
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn ex1() {
        assert!(is_perfect_square(16));
    }

    #[test]
    fn ex2() {
        assert!(!is_perfect_square(14));
    }

    #[test]
    fn ex3() {
        assert!(!is_perfect_square(2147483647));
    }

    #[test]
    fn ex4() {
        assert!(!is_perfect_square(5));
    }

    #[test]
    fn ex5() {
        assert!(!is_perfect_square(100000001));
    }
}
