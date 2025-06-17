pub fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
    const MOD: i64 = 1000000007;
    fn factorial(i: i64) -> i64 {
        if i == 0 {
            return 1;
        };
        i * factorial(i - 1) % MOD
    }
    fn binaryexp(a: i64, b: i64) -> i64 {
        let mut ans = 1;
        let mut i = a;
        let mut b = b;
        while b > 0 {
            if b & 1 == 1 {
                ans = ans * i % MOD;
            }
            i = i.pow(2) % MOD;
            b >>= 1;
        }
        ans
    }
    let a = factorial(n as i64 - 1);
    let b = factorial(k as i64);
    let c = factorial(n as i64 - k as i64 - 1);
    let d = binaryexp(m as i64 - 1, (n - k - 1) as i64);
    let mut ans = (m as i64 * d) % MOD;
    ans = (ans * a) % MOD;

    let e = binaryexp(b, MOD - 2);
    let f = binaryexp(c, MOD - 2);
    ans = (ans * e) % MOD;
    ans = (ans * f) % MOD;
    ans as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(count_good_arrays(3, 2, 1), 4)
    }

    #[test]
    fn ex2() {
        assert_eq!(count_good_arrays(4, 2, 2), 6)
    }

    #[test]
    fn ex3() {
        assert_eq!(count_good_arrays(5581, 58624, 4766), 846088010)
    }
}
