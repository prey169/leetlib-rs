pub fn is_anagram(s: String, t: String) -> bool {
    let mut s: Vec<char> = s.chars().collect();
    s.sort();
    let mut t: Vec<char> = t.chars().collect();
    t.sort();
    s == t
}

#[cfg(test)]
mod test {
    use crate::is_anagram;

    #[test]
    fn ex1() {
        assert!(is_anagram("anagram".to_string(), "nagaram".to_string()))
    }

    #[test]
    fn ex2() {
        assert!(!is_anagram("rat".to_string(), "car".to_string()))
    }
}
