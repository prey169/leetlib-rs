pub fn repeated_substring_pattern(s: String) -> bool {
    let w = format!("{}{}", s, s);
    w[1..(w.len() - 1)].contains(&s)
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn ex1() {
        assert!(repeated_substring_pattern("abab".to_string()))
    }

    #[test]
    fn ex2() {
        assert!(!repeated_substring_pattern("aba".to_string()))
    }

    #[test]
    fn ex3() {
        assert!(!repeated_substring_pattern("ab".to_string()))
    }
}
