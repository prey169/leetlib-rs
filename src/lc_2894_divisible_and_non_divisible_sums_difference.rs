pub fn difference_of_sums(n: i32, m: i32) -> i32 {
    let mut sum = 0;
    let mut div = 0;

    for i in 0..n + 1 {
        if i % m == 0 {
            div += i;
        } else {
            sum += i
        }
    }
    println!("{}", div);
    sum - div
}

#[cfg(test)]
mod test {
    use crate::difference_of_sums;

    #[test]
    fn ex1() {
        assert_eq!(difference_of_sums(10, 3), 19)
    }

    #[test]
    fn ex2() {
        assert_eq!(difference_of_sums(5, 6), 15)
    }

    #[test]
    fn ex3() {
        assert_eq!(difference_of_sums(5, 1), -15)
    }
}
