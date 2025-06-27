pub fn is_palindrome(s: String) -> bool {
    let s = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();
    let s_rev = s.chars().rev().collect::<String>();
    s_rev == s
}

#[cfg(test)]
mod test {
    use super::is_palindrome;

    #[test]
    fn ex1() {
        assert!(is_palindrome("A man, a plan, a canal: Panama".to_string()))
    }
}
