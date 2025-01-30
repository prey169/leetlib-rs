pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut dict = std::collections::HashMap::new();

    for c in magazine.chars() {
        dict.entry(c).and_modify(|count| *count += 1).or_insert(1);
    }

    for c in ransom_note.chars() {
        match dict.get_mut(&c) {
            Some(count) if *count > 0 => *count -= 1,
            _ => return false,
        }
    }
    true
}

#[cfg(test)]
mod test {
    use crate::can_construct;

    #[test]
    fn ex1() {
        assert!(!can_construct("a".to_string(), "b".to_string()))
    }

    #[test]
    fn ex2() {
        assert!(!can_construct("aa".to_string(), "ab".to_string()))
    }

    #[test]
    fn ex3() {
        assert!(can_construct("aa".to_string(), "aab".to_string()))
    }

    #[test]
    fn ex4() {
        assert!(can_construct("aab".to_string(), "baa".to_string()))
    }
}
