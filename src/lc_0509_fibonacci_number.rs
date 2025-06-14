pub fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    let mut total = 0;
    let mut last_sum = 0;
    let mut sum = 1;
    for _i in 2..=n {
        total = sum + last_sum;
        last_sum = sum;
        sum = total;
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(fib(2), 1);
    }

    #[test]
    fn ex2() {
        assert_eq!(fib(3), 2);
    }

    #[test]
    fn ex3() {
        assert_eq!(fib(4), 3)
    }
}
